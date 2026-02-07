use rusqlite::{named_params, Connection, Transaction};

use crate::db::schema::{Channel, Tag, Video, VideoTag};

pub fn list_all_videos(conn: &Connection) -> Result<Vec<Video>, String> {
    let sql = String::from("SELECT * FROM videos");

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(named_params! {}, |row| Video::from_row(row))
        .map_err(|e| e.to_string())?;

    let mut out = Vec::new();
    for v in rows {
        out.push(v.map_err(|e| e.to_string())?);
    }

    Ok(out)
}

pub fn list_all_channels(conn: &Connection) -> Result<Vec<Channel>, String> {
    let mut stmt = conn
        .prepare("SELECT * FROM channels")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(named_params! {}, |row| Channel::from_row(row))
        .map_err(|e| e.to_string())?;

    let mut out = Vec::new();
    for v in rows {
        out.push(v.map_err(|e| e.to_string())?);
    }

    Ok(out)
}

pub fn list_all_tags(conn: &Connection) -> Result<Vec<Tag>, String> {
    let mut stmt = conn
        .prepare("SELECT * FROM tags")
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

pub fn list_all_video_tags(conn: &Connection) -> Result<Vec<VideoTag>, String> {
    let mut stmt = conn
        .prepare("SELECT * FROM video_tags")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| VideoTag::from_row(row))
        .map_err(|e| e.to_string())?;

    let mut out = Vec::new();
    for v in rows {
        out.push(v.map_err(|e| e.to_string())?);
    }

    Ok(out)
}

pub fn batch_insert_videos(tx: &Transaction, videos: &[Video]) -> Result<(), String> {
    let mut stmt = tx
        .prepare(
            "
            INSERT INTO videos (
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
            )
            VALUES (
                :id,
                :channel_id,
                :link,
                :title,
                :thumbnail,
                :duration,
                :views,
                :likes,
                :uploaded_at,
                :status,
                :created_at,
                :updated_at,
                :watched_at
            )
        ",
        )
        .map_err(|e| e.to_string())?;

    for video in videos {
        stmt.execute(named_params! {
            ":id": video.id,
            ":channel_id": video.channel_id,
            ":link": video.link,
            ":title": video.title,
            ":thumbnail": video.thumbnail,
            ":duration": video.duration.map(|v| v as i64),
            ":views": video.views.map(|v| v as i64),
            ":likes": video.likes.map(|v| v as i64),
            ":uploaded_at": video.uploaded_at,
            ":status": video.status.to_string(),
            ":created_at": video.created_at,
            ":updated_at": video.updated_at,
            ":watched_at": video.watched_at,
        })
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub fn batch_insert_channels(tx: &Transaction, channels: &[Channel]) -> Result<(), String> {
    let mut stmt = tx
        .prepare(
            "
            INSERT INTO channels (
                id,
                link,
                name,
                avatar,
                verified,
                subscribers,
                created_at,
                updated_at
            )
            VALUES (
                :id,
                :link,
                :name,
                :avatar,
                :verified,
                :subscribers,
                :created_at,
                :updated_at
            )
        ",
        )
        .map_err(|e| e.to_string())?;

    for channel in channels {
        stmt.execute(named_params! {
            ":id": channel.id,
            ":link": channel.link,
            ":name": channel.name,
            ":avatar": channel.avatar,
            ":verified": channel.verified,
            ":subscribers": channel.subscribers.map(|v| v as i64),
            ":created_at": channel.created_at,
            ":updated_at": channel.updated_at,
        })
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub fn batch_insert_tags(tx: &Transaction, tags: &[Tag]) -> Result<(), String> {
    let mut stmt = tx
        .prepare(
            "
            INSERT INTO tags (
                id,
                name,
                color,
                created_at,
                updated_at
            )
            VALUES (
                :id,
                :name, 
                :color,
                :created_at,
                :updated_at
            )
        ",
        )
        .map_err(|e| e.to_string())?;

    for tag in tags {
        stmt.execute(named_params! {
            ":id": tag.id,
            ":name": tag.name,
            ":color": tag.color,
            ":created_at": tag.created_at,
            ":updated_at": tag.updated_at,
        })
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub fn batch_insert_video_tags(tx: &Transaction, video_tags: &[VideoTag]) -> Result<(), String> {
    let mut stmt = tx
        .prepare(
            "
            INSERT INTO video_tags (
                video_id,
                tag_id
            )
            VALUES (
                :video_id,
                :tag_id
            )
        ",
        )
        .map_err(|e| e.to_string())?;

    for video_tag in video_tags {
        stmt.execute(named_params! {
            ":video_id": video_tag.video_id,
            ":tag_id": video_tag.tag_id,

        })
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub fn clear_all_tables(tx: &Transaction) -> Result<(), String> {
    tx.execute("DELETE FROM video_tags", [])
        .map_err(|e| e.to_string())?;
    tx.execute("DELETE FROM videos", [])
        .map_err(|e| e.to_string())?;
    tx.execute("DELETE FROM channels", [])
        .map_err(|e| e.to_string())?;
    tx.execute("DELETE FROM tags", [])
        .map_err(|e| e.to_string())?;
    Ok(())
}
