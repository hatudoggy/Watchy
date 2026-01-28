use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum VideoStatus {
    SAVED,
    TOWATCH,
    WATCHED,
}

impl fmt::Display for VideoStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            VideoStatus::SAVED => "SAVED",
            VideoStatus::TOWATCH => "TOWATCH",
            VideoStatus::WATCHED => "WATCHED",
        })
    }
}

impl FromStr for VideoStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SAVED" => Ok(VideoStatus::SAVED),
            "TOWATCH" => Ok(VideoStatus::TOWATCH),
            "WATCHED" => Ok(VideoStatus::WATCHED),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub id: i64,
    pub channel_id: i64,
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

impl Video {
    pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            channel_id: row.get(1)?,
            link: row.get(2)?,
            title: row.get(3)?,
            thumbnail: row.get(4)?,
            duration: row.get::<_, Option<i64>>(5)?.map(|v| v as u64),
            views: row.get::<_, Option<i64>>(6)?.map(|v| v as u64),
            likes: row.get::<_, Option<i64>>(7)?.map(|v| v as u64),
            uploaded_at: row.get(8)?,
            status: VideoStatus::from_str(&row.get::<_, String>(9)?).unwrap_or(VideoStatus::SAVED),
            created_at: row.get(10)?,
            updated_at: row.get(11)?,
            watched_at: row.get(12)?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Tag {
    pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            name: row.get(1)?,
            color: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    }

    pub fn from_row_offset(row: &rusqlite::Row, offset: usize) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(offset)?,
            name: row.get(offset + 1)?,
            color: row.get(offset + 2)?,
            created_at: row.get(offset + 3)?,
            updated_at: row.get(offset + 4)?,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub id: i64,
    pub link: String,
    pub name: String,
    pub avatar: String,
    pub verified: bool,
    pub subscribers: Option<u64>,
    pub created_at: String,
    pub updated_at: String,
}

impl Channel {
    pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            link: row.get(1)?,
            name: row.get(2)?,
            avatar: row.get(3)?,
            verified: row.get::<_, bool>(4)?,
            subscribers: row.get::<_, Option<i64>>(5)?.map(|v| v as u64),
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    }
}
