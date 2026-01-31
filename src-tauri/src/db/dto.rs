use serde::{Deserialize, Serialize};

use crate::db::schema::{Tag, VideoStatus};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoFilter {
    pub channel_id: Option<i64>,
    pub status: Option<VideoStatus>,
    pub tag_ids: Option<Vec<i64>>,
    pub search: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum VideoSort {
    CreatedAtDesc,
    CreatedAtAsc,
    UpdatedAtDesc,
    UpdatedAtAsc,
    UploadedAtDesc,
    UploadedAtAsc,
    WatchedAtDesc,
    WatchedAtAsc,
    DurationDesc,
    DurationAsc,
    TitleAsc,
    TitleDesc,
}

impl VideoSort {
    pub fn to_sql(&self) -> &'static str {
        match self {
            VideoSort::CreatedAtDesc => "v.created_at DESC",
            VideoSort::CreatedAtAsc => "v.created_at ASC",
            VideoSort::UpdatedAtDesc => "v.updated_at DESC",
            VideoSort::UpdatedAtAsc => "v.updated_at ASC",
            VideoSort::UploadedAtDesc => "v.uploaded_at DESC",
            VideoSort::UploadedAtAsc => "v.uploaded_at ASC",
            VideoSort::WatchedAtDesc => "v.watched_at DESC",
            VideoSort::WatchedAtAsc => "v.watched_at ASC",
            VideoSort::DurationDesc => "v.duration DESC",
            VideoSort::DurationAsc => "v.duration ASC",
            VideoSort::TitleAsc => "v.title COLLATE NOCASE ASC",
            VideoSort::TitleDesc => "v.title COLLATE NOCASE DESC",
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoItem {
    pub id: i64,
    pub channel_id: i64,
    pub channel_name: String,
    pub channel_link: String,
    pub link: String,
    pub title: String,
    pub thumbnail: String,
    pub duration: Option<u64>,
    pub views: Option<u64>,
    pub likes: Option<u64>,
    pub uploaded_at: String,
    pub status: VideoStatus,
    pub created_at: String,
    pub updated_at: String,
    pub watched_at: Option<String>,
}

impl VideoItem {
    pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            channel_id: row.get(1)?,
            channel_name: row.get(2)?,
            channel_link: row.get(3)?,
            link: row.get(4)?,
            title: row.get(5)?,
            thumbnail: row.get(6)?,
            duration: row.get::<_, Option<i64>>(7)?.map(|v| v as u64),
            views: row.get::<_, Option<i64>>(8)?.map(|v| v as u64),
            likes: row.get::<_, Option<i64>>(9)?.map(|v| v as u64),
            uploaded_at: row.get(10)?,
            status: row
                .get::<_, String>(11)?
                .parse::<VideoStatus>()
                .unwrap_or(VideoStatus::SAVED),
            created_at: row.get(12)?,
            updated_at: row.get(13)?,
            watched_at: row.get(14)?,
        })
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoItemWithTags {
    pub video: VideoItem,
    pub tags: Vec<Tag>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateVideo {
    pub channel_id: i64,
    pub link: String,
    pub title: String,
    pub thumbnail: String,
    pub duration: Option<u64>,
    pub views: Option<u64>,
    pub likes: Option<u64>,
    pub uploaded_at: String,
    pub status: VideoStatus,
}

#[derive(Deserialize)]
pub struct UpdateVideo {
    pub link: Option<String>,
    pub title: Option<String>,
    pub thumbnail: Option<String>,
    pub duration: Option<u64>,
    pub views: Option<u64>,
    pub likes: Option<u64>,
    pub uploaded_at: Option<String>,
    pub status: Option<VideoStatus>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateChannel {
    pub link: String,
    pub name: String,
    pub avatar: String,
    pub verified: bool,
    pub subscribers: Option<u64>,
}

#[derive(Deserialize)]
pub struct UpdateChannel {
    pub link: String,
    pub name: String,
    pub avatar: String,
    pub verified: bool,
    pub subscribers: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTag {
    pub name: String,
    pub color: String,
}

#[derive(Deserialize)]
pub struct UpdateTag {
    pub name: String,
    pub color: String,
}
