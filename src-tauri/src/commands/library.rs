use tauri::State;

use crate::{
    commands::youtube::YoutubeMetadata,
    db::{
        dto::{CreateTag, UpdateTag, UpdateVideo, VideoFilter, VideoItemWithTags, VideoSort},
        schema::{Channel, Tag, Video},
    },
    services::{tags, videos},
    Db,
};

// =====================
// VIDEO COMMANDS
// =====================

#[tauri::command]
pub fn list_videos(
    db: State<Db>,
    filter: Option<VideoFilter>,
    sort: Option<VideoSort>,
) -> Result<Vec<VideoItemWithTags>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    videos::list_videos(&conn, filter, sort)
}

#[tauri::command]
pub fn add_video(
    db: State<Db>,
    metadata: YoutubeMetadata,
    tags: Option<Vec<Tag>>,
) -> Result<Video, String> {
    let mut conn = db.0.lock().map_err(|e| e.to_string())?;
    videos::add_video(&mut conn, metadata, tags)
}

#[tauri::command]
pub fn edit_video(db: State<Db>, id: i64, video: UpdateVideo) -> Result<Video, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    videos::edit_video(&conn, id, video)
}

#[tauri::command]
pub fn delete_video(db: State<Db>, id: i64) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    videos::delete_video(&conn, id)
}

#[tauri::command]
pub fn add_tag_to_video(db: State<Db>, video_id: i64, name: String) -> Result<Tag, String> {
    let mut conn = db.0.lock().map_err(|e| e.to_string())?;
    videos::add_tag_to_video(&mut conn, video_id, name)
}

#[tauri::command]
pub fn remove_tag_from_video(db: State<Db>, video_id: i64, tag_id: i64) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    videos::remove_tag_from_video(&conn, video_id, tag_id)
}

#[tauri::command]
pub fn get_video(db: State<Db>, id: i64) -> Result<VideoItemWithTags, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    videos::get_video(&conn, id)
}

#[tauri::command]
pub fn get_channel_details(db: State<Db>, id: i64) -> Result<Channel, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    videos::get_channel_details(&conn, id)
}

// =====================
// TAG COMMANDS
// =====================

#[tauri::command]
pub fn list_tags(db: State<Db>) -> Result<Vec<Tag>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    tags::list_tags(&conn)
}

#[tauri::command]
pub fn add_tag(db: State<Db>, tag: CreateTag) -> Result<Tag, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    tags::add_tag(&conn, tag)
}

#[tauri::command]
pub fn edit_tag(db: State<Db>, id: i64, tag: UpdateTag) -> Result<Tag, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    tags::edit_tag(&conn, id, tag)
}

#[tauri::command]
pub fn delete_tag(db: State<Db>, id: i64) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    tags::delete_tag(&conn, id)
}
