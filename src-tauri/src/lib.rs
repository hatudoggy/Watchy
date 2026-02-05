use std::sync::Mutex;

use rusqlite::Connection;
use tauri::Manager;

pub mod commands;
pub mod db;
pub mod services;

pub struct Db(pub Mutex<Connection>);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app data dir");

            let conn = db::connection::open_db(app_dir).expect("failed to open database");

            app.manage(Db(Mutex::new(conn)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::youtube::fetch_youtube_metadata,
            commands::library::list_videos,
            commands::library::add_video,
            commands::library::get_video,
            commands::library::edit_video,
            commands::library::delete_video,
            commands::library::add_tag_to_video,
            commands::library::remove_tag_from_video,
            commands::library::get_channel_details,
            commands::library::list_tags,
            commands::library::add_tag,
            commands::library::edit_tag,
            commands::library::delete_tag,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
