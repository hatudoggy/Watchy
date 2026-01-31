use rusqlite::Connection;

use crate::{
    commands::youtube::YoutubeMetadata,
    db::{
        dto::{
            CreateChannel, CreateTag, CreateVideo, UpdateVideo, VideoFilter, VideoItemWithTags,
            VideoSort,
        },
        queries,
        schema::{Channel, Tag, Video, VideoStatus},
    },
};

pub fn list_videos(
    conn: &Connection,
    filter: Option<VideoFilter>,
    sort: Option<VideoSort>,
) -> Result<Vec<VideoItemWithTags>, String> {
    let videos = queries::list_videos(conn, filter, sort)?;

    let video_ids: Vec<i64> = videos.iter().map(|v| v.id).collect();

    let tags_map = queries::list_video_tags(conn, &video_ids)?;

    let result = videos
        .into_iter()
        .map(|video| VideoItemWithTags {
            tags: tags_map.get(&video.id).cloned().unwrap_or_default(),
            video,
        })
        .collect();

    Ok(result)
}

pub fn add_video(
    conn: &mut Connection,
    metadata: YoutubeMetadata,
    tags: Option<Vec<Tag>>,
) -> Result<Video, String> {
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let channel = match queries::get_channel_by_name(&tx, &metadata.channel.name)? {
        Some(channel) => channel,
        None => queries::create_channel(
            &tx,
            CreateChannel {
                link: metadata.channel.link,
                name: metadata.channel.name,
                avatar: metadata.channel.avatar,
                verified: metadata.channel.verified,
                subscribers: metadata.channel.subscribers,
            },
        )?,
    };

    let video = queries::create_video(
        &tx,
        CreateVideo {
            channel_id: channel.id,
            link: metadata.video.link,
            title: metadata.video.title,
            thumbnail: metadata.video.thumbnail,
            duration: metadata.video.duration,
            views: metadata.video.views,
            likes: metadata.video.likes,
            uploaded_at: metadata.video.upload_date,
            status: VideoStatus::SAVED,
        },
    )
    .map_err(|e| {
        if e.contains("UNIQUE") {
            "Video already exists".to_string()
        } else {
            e
        }
    })?;

    if let Some(tags) = tags {
        for tag in tags {
            if tag.id <= 0 {
                return Err("Invalid tag id".to_string());
            }
            queries::create_video_tag(&tx, video.id, tag.id)?;
        }
    }

    tx.commit().map_err(|e| e.to_string())?;

    Ok(video)
}

pub fn edit_video(conn: &Connection, id: i64, video: UpdateVideo) -> Result<Video, String> {
    queries::update_video(conn, id, video)
}

pub fn delete_video(conn: &Connection, id: i64) -> Result<(), String> {
    queries::delete_video(conn, id)
}

pub fn add_tag_to_video(conn: &mut Connection, video_id: i64, name: String) -> Result<Tag, String> {
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let tag = match queries::get_tag_by_name(&tx, &name)? {
        Some(tag) => tag,
        None => queries::create_tag(
            &tx,
            CreateTag {
                name: name,
                color: "#000000".to_string(),
            },
        )?,
    };

    queries::create_video_tag(&tx, video_id, tag.id)?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok(tag)
}

pub fn remove_tag_from_video(conn: &Connection, video_id: i64, tag_id: i64) -> Result<(), String> {
    queries::delete_video_tag(conn, video_id, tag_id)
}

pub fn get_video(conn: &Connection, id: i64) -> Result<VideoItemWithTags, String> {
    let video = queries::get_video(conn, id)?;
    let tags = queries::list_video_tags(conn, &[video.id])?;

    Ok(VideoItemWithTags {
        video,
        tags: tags.get(&id).cloned().unwrap_or_default(),
    })
}

pub fn get_channel_details(conn: &Connection, id: i64) -> Result<Channel, String> {
    queries::get_channel(conn, id)
}
