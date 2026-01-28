use std::collections::HashMap;

use rusqlite::{named_params, params_from_iter, Connection, ToSql};

use crate::db::{
    dto::{
        CreateChannel, CreateTag, CreateVideo, UpdateChannel, UpdateTag, UpdateVideo, VideoFilter,
        VideoItem, VideoSort,
    },
    schema::{Channel, Tag, Video},
};

pub fn list_videos(
    conn: &Connection,
    filter: Option<VideoFilter>,
    sort: Option<VideoSort>,
) -> Result<Vec<VideoItem>, String> {
    let mut sql = String::from(
        "
            SELECT
                v.id,
                v.channel_id,
                c.name AS channel_name,
                c.link AS channel_link,
                v.link,
                v.title,
                v.thumbnail,
                v.duration,
                v.views,
                v.likes,
                v.uploaded_at,
                v.status,
                v.created_at,
                v.updated_at,
                v.watched_at
            FROM videos v
            JOIN channels c ON c.id = v.channel_id
            ",
    );

    let mut params: Vec<Box<dyn ToSql>> = Vec::new();
    let mut where_clauses: Vec<String> = Vec::new();

    if let Some(filter) = filter {
        if let Some(channel_id) = filter.channel_id {
            where_clauses.push("v.channel_id = ?".into());
            params.push(Box::new(channel_id));
        }

        if let Some(status) = filter.status {
            where_clauses.push("v.status = ?".into());
            params.push(Box::new(status.to_string()));
        }

        if let Some(tag_ids) = &filter.tag_ids {
            if !tag_ids.is_empty() {
                let placeholders = std::iter::repeat("?")
                    .take(tag_ids.len())
                    .collect::<Vec<_>>()
                    .join(",");

                where_clauses.push(format!(
                    "
                    EXISTS (
                        SELECT 1
                        FROM video_tags vt
                        WHERE vt.video_id = v.id
                            AND vt.tag_id IN ({})
                    )
                    ",
                    placeholders
                ));

                for tag_id in tag_ids {
                    params.push(Box::new(*tag_id));
                }
            }
        }

        if let Some(search) = filter.search {
            where_clauses.push(
                "
                (
                    v.link = ?
                    OR v.title LIKE '%' || ? || '%' COLLATE NOCASE
                )
                "
                .into(),
            );

            params.push(Box::new(search.clone()));
            params.push(Box::new(search));
        }
    }

    if !where_clauses.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&where_clauses.join(" AND "));
    }

    let order_by = sort.unwrap_or(VideoSort::CreatedAtDesc);

    sql.push_str(" ORDER BY ");
    sql.push_str(order_by.to_sql());

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

    let params_ref: Vec<&dyn ToSql> = params.iter().map(|p| p.as_ref()).collect();

    let rows = stmt
        .query_map(&params_ref[..], |row| VideoItem::from_row(row))
        .map_err(|e| e.to_string())?;

    let mut out = Vec::new();
    for v in rows {
        out.push(v.map_err(|e| e.to_string())?);
    }

    Ok(out)
}

pub fn get_video(conn: &Connection, id: i64) -> Result<VideoItem, String> {
    let mut stmt = conn
        .prepare(
            "
            SELECT
                v.id,
                v.channel_id,
                c.name AS channel_name,
                c.link AS channel_link,
                v.link,
                v.title,
                v.thumbnail,
                v.duration,
                v.views,
                v.likes,
                v.uploaded_at,
                v.status,
                v.created_at,
                v.updated_at,
                v.watched_at
            FROM videos v
            JOIN channels c ON c.id = v.channel_id
            WHERE v.id = :id
            ",
        )
        .map_err(|e| e.to_string())?;

    let video = stmt
        .query_row(named_params! { ":id": id}, |row| VideoItem::from_row(row))
        .map_err(|e| e.to_string())?;

    Ok(video)
}

pub fn create_video(conn: &Connection, video: CreateVideo) -> Result<Video, String> {
    conn.execute(
        "
        INSERT INTO videos (
            channel_id,
            link,
            title,
            thumbnail,
            duration,
            views,
            likes,
            uploaded_at,
            status
        )
        VALUES (:channel_id, :link, :title, :thumbnail, :duration, :views, :likes, :uploaded_at, :status)
        ",
    named_params! {
        ":channel_id": video.channel_id,
        ":link": video.link,
        ":title": video.title,
        ":thumbnail": video.thumbnail,
        ":duration": video.duration.map(|v| v as i64),
        ":views": video.views.map(|v| v as i64),
        ":likes": video.likes.map(|v| v as i64),
        ":uploaded_at": video.uploaded_at,
        ":status": video.status.to_string(),
    },
    )
    .map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();

    let mut stmt = conn
        .prepare(
            "
            SELECT
                id,
                channel_id,
                link,
                title,
                thumbnail,
                duration,
                views,
                likes,
                uploaded_at,
                status,
                created_at,
                updated_at,
                watched_at
            FROM videos
            WHERE id = :id
            ",
        )
        .map_err(|e| e.to_string())?;

    let video = stmt
        .query_row(named_params! { ":id": id }, |row| Video::from_row(row))
        .map_err(|e| e.to_string())?;

    Ok(video)
}

pub fn update_video(conn: &Connection, id: i64, video: UpdateVideo) -> Result<Video, String> {
    let mut updates = Vec::new();
    let mut params = Vec::new();

    if let Some(link) = &video.link {
        updates.push("link = ?");
        params.push(link.clone());
    }
    if let Some(title) = &video.title {
        updates.push("title = ?");
        params.push(title.clone());
    }
    if let Some(thumbnail) = &video.thumbnail {
        updates.push("thumbnail = ?");
        params.push(thumbnail.clone());
    }
    if let Some(uploaded_at) = &video.uploaded_at {
        updates.push("uploaded_at = ?");
        params.push(uploaded_at.clone());
    }
    if let Some(status) = video.status {
        updates.push("status = ?");
        params.push(status.to_string());
    }

    if updates.is_empty() {
        return Err("No fields to update".to_string());
    }

    let sql = format!("UPDATE videos SET {} WHERE id = ?", updates.join(", "));

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let mut param_refs: Vec<&dyn rusqlite::ToSql> =
        params.iter().map(|p| p as &dyn rusqlite::ToSql).collect();
    param_refs.push(&id);

    stmt.execute(param_refs.as_slice())
        .map_err(|e| e.to_string())?;

    if video.duration.is_some() || video.views.is_some() || video.likes.is_some() {
        let mut numeric_updates = Vec::new();
        if video.duration.is_some() {
            numeric_updates.push("duration = :duration");
        }
        if video.views.is_some() {
            numeric_updates.push("views = :views");
        }
        if video.likes.is_some() {
            numeric_updates.push("likes = :likes");
        }

        let numeric_sql = format!(
            "UPDATE videos SET {} WHERE id = :id",
            numeric_updates.join(", ")
        );

        conn.execute(
            &numeric_sql,
            named_params! {
                ":id": id,
                ":duration": video.duration.map(|v| v as i64),
                ":views": video.views.map(|v| v as i64),
                ":likes": video.likes.map(|v| v as i64),
            },
        )
        .map_err(|e| e.to_string())?;
    }

    let mut stmt = conn
        .prepare(
            "
            SELECT
                id,
                channel_id,
                link,
                title,
                thumbnail,
                duration,
                views,
                likes,
                uploaded_at,
                status,
                created_at,
                updated_at,
                watched_at
            FROM videos
            WHERE id = :id
            ",
        )
        .map_err(|e| e.to_string())?;

    let video = stmt
        .query_row(named_params! { ":id": id }, |row| Video::from_row(row))
        .map_err(|e| e.to_string())?;

    Ok(video)
}

pub fn delete_video(conn: &Connection, id: i64) -> Result<(), String> {
    let deleted = conn
        .execute(
            "DELETE FROM videos WHERE id = :id",
            named_params! { ":id": id },
        )
        .map_err(|e| e.to_string())?;

    if deleted == 0 {
        return Err(format!("Video with id {} not found", id));
    }

    Ok(())
}

pub fn get_channel(conn: &Connection, id: i64) -> Result<Channel, String> {
    let mut stmt = conn
        .prepare(
            "
            SELECT
                id,
                link,
                name,
                avatar,
                verified,
                subscribers,
                created_at,
                updated_at
            FROM channels
            WHERE id = :id
            ",
        )
        .map_err(|e| e.to_string())?;

    let channel = stmt
        .query_row(named_params! { ":id": id }, |row| Channel::from_row(row))
        .map_err(|e| e.to_string())?;

    Ok(channel)
}

pub fn get_channel_by_name(conn: &Connection, name: &str) -> Result<Option<Channel>, String> {
    let mut stmt = conn
        .prepare(
            "
            SELECT
                id,
                link,
                name,
                avatar,
                verified,
                subscribers,
                created_at,
                updated_at
            FROM channels
            WHERE name = :name
            ",
        )
        .map_err(|e| e.to_string())?;

    let result = stmt.query_row(named_params! { ":name": name }, |row| {
        Channel::from_row(row)
    });

    match result {
        Ok(channel) => Ok(Some(channel)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

pub fn create_channel(conn: &Connection, channel: CreateChannel) -> Result<Channel, String> {
    conn.execute(
        "
        INSERT INTO channels (
            link,
            name,
            avatar,
            verified,
            subscribers
        )
        VALUES (:link, :name, :avatar, :verified, :subscribers)
        ",
        named_params! {
            ":link": channel.link,
            ":name": channel.name,
            ":avatar": channel.avatar,
            ":verified": channel.verified,
            ":subscribers": channel.subscribers.map(|v| v as i64),
        },
    )
    .map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();

    let mut stmt = conn
        .prepare(
            "
            SELECT
                id,
                link,
                name,
                avatar,
                verified,
                subscribers,
                created_at,
                updated_at
            FROM channels
            WHERE id = :id
            ",
        )
        .map_err(|e| e.to_string())?;

    let channel = stmt
        .query_row(named_params! { ":id": id }, |row| Channel::from_row(row))
        .map_err(|e| e.to_string())?;

    Ok(channel)
}

pub fn update_channel(
    conn: &Connection,
    id: i64,
    channel: UpdateChannel,
) -> Result<Channel, String> {
    conn.execute(
        "
        UPDATE channels SET
            link = :link,
            name = :name,
            avatar = :avatar,
            verified = :verified,
            subscribers = :subscribers
        WHERE id = :id
        ",
        named_params! {
            ":id": id,
            ":link": channel.link,
            ":name": channel.name,
            ":avatar": channel.avatar,
            ":verified": channel.verified,
            ":subscribers": channel.subscribers.map(|v| v as i64),
        },
    )
    .map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "
            SELECT
                id,
                link,
                name,
                avatar,
                verified,
                subscribers,
                created_at,
                updated_at
            FROM channels
            WHERE id = :id
            ",
        )
        .map_err(|e| e.to_string())?;

    let channel = stmt
        .query_row(named_params! { ":id": id}, |row| Channel::from_row(row))
        .map_err(|e| e.to_string())?;

    Ok(channel)
}

pub fn delete_channel(conn: &Connection, id: i64) -> Result<(), String> {
    let deleted = conn
        .execute(
            "DELETE FROM channels WHERE id = :id",
            named_params! { ":id": id },
        )
        .map_err(|e| e.to_string())?;

    if deleted == 0 {
        return Err(format!("Channel with id {} not found", id));
    }

    Ok(())
}

pub fn list_tags(conn: &Connection) -> Result<Vec<Tag>, String> {
    let mut stmt = conn
        .prepare(
            "
            SELECT
                id,
                name,
                color,
                created_at,
                updated_at
            FROM tags
            ORDER BY created_at ASC
            ",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| Tag::from_row(row))
        .map_err(|e| e.to_string())?;

    let mut out = Vec::new();
    for v in rows {
        out.push(v.map_err(|e| e.to_string())?);
    }

    Ok(out)
}

pub fn get_tag(conn: &Connection, id: i64) -> Result<Tag, String> {
    let mut stmt = conn
        .prepare(
            "
            SELECT
                id,
                name,
                color,
                created_at,
                updated_at
            FROM tags
            WHERE id = :id
            ",
        )
        .map_err(|e| e.to_string())?;

    let tag = stmt
        .query_row(named_params! { ":id": id }, |row| Tag::from_row(row))
        .map_err(|e| e.to_string())?;

    Ok(tag)
}

pub fn get_tag_by_name(conn: &Connection, name: &str) -> Result<Option<Tag>, String> {
    let mut stmt = conn
        .prepare(
            "
            SELECT
                id,
                name,
                color,
                created_at,
                updated_at
            FROM tags
            WHERE name = :name
            ",
        )
        .map_err(|e| e.to_string())?;

    let result = stmt.query_row(named_params! { ":name": name }, |row| Tag::from_row(row));

    match result {
        Ok(tag) => Ok(Some(tag)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

pub fn create_tag(conn: &Connection, tag: CreateTag) -> Result<Tag, String> {
    conn.execute(
        "
        INSERT INTO tags (
        name,
        color
    )
    VALUES (:name, :color)
        ",
        named_params! {
            ":name": tag.name,
            ":color": tag.color
        },
    )
    .map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();

    let mut stmt = conn
        .prepare(
            "
            SELECT
                id,
                name,
                color,
                created_at,
                updated_at
            FROM tags
            WHERE id = :id
        ",
        )
        .map_err(|e| e.to_string())?;

    let tag = stmt
        .query_row(named_params! { ":id": id }, |row| Tag::from_row(row))
        .map_err(|e| e.to_string())?;

    Ok(tag)
}

pub fn update_tag(conn: &Connection, id: i64, tag: UpdateTag) -> Result<Tag, String> {
    conn.execute(
        "
        UPDATE tags SET
            name = :name,
            color = :color
        WHERE id = :id
        ",
        named_params! {
            ":id": id,
            ":name": tag.name,
            ":color": tag.color
        },
    )
    .map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "
            SELECT
                id,
                name,
                color,
                created_at,
                updated_at
            FROM tags
            WHERE id = :id
        ",
        )
        .map_err(|e| e.to_string())?;

    let tag = stmt
        .query_row(named_params! { ":id": id }, |row| Tag::from_row(row))
        .map_err(|e| e.to_string())?;

    Ok(tag)
}

pub fn delete_tag(conn: &Connection, id: i64) -> Result<(), String> {
    let deleted = conn
        .execute(
            "DELETE FROM tags WHERE id = :id",
            named_params! { ":id": id },
        )
        .map_err(|e| e.to_string())?;

    if deleted == 0 {
        return Err(format!("Tag with id {} not found", id));
    }

    Ok(())
}

pub fn list_video_tags(
    conn: &Connection,
    video_ids: &[i64],
) -> Result<HashMap<i64, Vec<Tag>>, String> {
    if video_ids.is_empty() {
        return Ok(HashMap::new());
    }

    let placeholders = std::iter::repeat("?")
        .take(video_ids.len())
        .collect::<Vec<_>>()
        .join(",");

    let sql = format!(
        "
        SELECT
            vt.video_id,
            t.id,
            t.name,
            t.color,
            t.created_at,
            t.updated_at
        FROM video_tags vt
        JOIN tags t ON t.id = vt.tag_id
        WHERE vt.video_id IN ({})
        ",
        placeholders
    );

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(params_from_iter(video_ids.iter()), |row| {
            Ok((row.get::<_, i64>(0)?, Tag::from_row_offset(row, 1)?))
        })
        .map_err(|e| e.to_string())?;

    let mut map: HashMap<i64, Vec<Tag>> = HashMap::new();
    for row in rows {
        let (video_id, tag) = row.map_err(|e| e.to_string())?;
        map.entry(video_id).or_default().push(tag);
    }

    Ok(map)
}

pub fn create_video_tag(conn: &Connection, video_id: i64, tag_id: i64) -> Result<(), String> {
    match conn.execute(
        "
        INSERT OR IGNORE INTO video_tags (
            video_id,
            tag_id
        )
        VALUES (:video_id, :tag_id)
        ",
        named_params! {
            ":video_id": video_id,
            ":tag_id": tag_id
        },
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn delete_video_tag(conn: &Connection, video_id: i64, tag_id: i64) -> Result<(), String> {
    let deleted = conn
        .execute(
            "
            DELETE FROM video_tags 
            WHERE video_id = :video_id AND tag_id = :tag_id
            ",
            named_params! {
                ":video_id": video_id,
                ":tag_id": tag_id
            },
        )
        .map_err(|e| e.to_string())?;

    if deleted == 0 {
        return Err("Video tag relation not found".to_string());
    }

    Ok(())
}
