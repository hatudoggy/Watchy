use rusqlite::Connection;

use crate::db::{
    data_transfer::{
        batch_insert_channels, batch_insert_tags, batch_insert_video_tags, batch_insert_videos,
        clear_all_tables, list_all_channels, list_all_tags, list_all_video_tags, list_all_videos,
    },
    schema::DatabaseData,
};

pub fn export_db_data(conn: &Connection) -> Result<DatabaseData, String> {
    let videos = list_all_videos(conn)?;
    let channels = list_all_channels(conn)?;
    let tags = list_all_tags(conn)?;
    let video_tags = list_all_video_tags(conn)?;

    Ok(DatabaseData {
        videos,
        channels,
        tags,
        video_tags,
    })
}

pub fn import_db_data(conn: &mut Connection, data: DatabaseData) -> Result<(), String> {
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    // Clear data for now to prevent key conflict
    clear_all_tables(&tx)?;

    batch_insert_channels(&tx, &data.channels)?;
    batch_insert_videos(&tx, &data.videos)?;
    batch_insert_tags(&tx, &data.tags)?;
    batch_insert_video_tags(&tx, &data.video_tags)?;

    tx.commit().map_err(|e| e.to_string())?;

    Ok(())
}
