use serde::{Deserialize, Serialize};
use std::fs;
use tauri::{AppHandle, State};
use tauri_plugin_dialog::{DialogExt, FilePath};

use crate::{
    db::schema::DatabaseData,
    services::settings::{export_db_data, import_db_data},
    store::settings::{
        get_all_settings, load_settings_store, set_all_settings, set_settings_download_path,
        SettingsStore,
    },
    Db,
};

#[tauri::command]
pub async fn set_download_path(app: AppHandle) -> Result<String, String> {
    let folder = retrieve_file_picker_folder_path(&app)?;
    let path: String = resolve_path_to_string(folder)?;

    // Save to store
    let store = load_settings_store(&app)?;
    set_settings_download_path(&store, &path)?;

    // Return path
    Ok(path)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AppData {
    pub database: DatabaseData,
    pub settings: SettingsStore,
}

#[tauri::command]
pub async fn export_data(app: AppHandle, db: State<'_, Db>) -> Result<(), String> {
    // Open file picker
    // Save output folder to state
    let folder = retrieve_file_picker_folder_path(&app)?;

    // Get database data
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let database = export_db_data(&conn)?;

    // Get settings data
    let store = load_settings_store(&app)?;
    let settings = get_all_settings(&store)?;

    // Create json of compiled data
    let data = AppData { database, settings };
    let json = serde_json::to_string(&data).map_err(|e| e.to_string())?;

    // Save json to output folder
    let file = folder
        .into_path()
        .map_err(|e| e.to_string())?
        .join("export.json");
    let _ = fs::write(file, json);

    // Return ok if success
    Ok(())
}

#[tauri::command]
pub async fn import_data(app: AppHandle, db: State<'_, Db>) -> Result<SettingsStore, String> {
    // Open file picker
    let file = retrieve_file_picker_file_path(&app)?;
    let path = file.into_path().map_err(|e| e.to_string())?;

    // Parse the input json if it is valid
    let json = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let data: AppData = serde_json::from_str(&json).map_err(|e| e.to_string())?;

    // Perform database insert operation for database data
    let mut conn = db.0.lock().map_err(|e| e.to_string())?;
    import_db_data(&mut conn, &data.database)?;

    // Perform store set for settings data
    let store = load_settings_store(&app)?;
    set_all_settings(&store, &data.settings)?;

    // Return ok if success
    Ok(data.settings)
}

fn retrieve_file_picker_folder_path(app: &AppHandle) -> Result<FilePath, String> {
    // Open file picker
    let folder = app
        .dialog()
        .file()
        .blocking_pick_folder()
        .ok_or("Folder selection cancelled")?;

    Ok(folder)
}

fn retrieve_file_picker_file_path(app: &AppHandle) -> Result<FilePath, String> {
    // Open file picker
    let folder = app
        .dialog()
        .file()
        .blocking_pick_file()
        .ok_or("File selection cancelled")?;

    Ok(folder)
}

fn resolve_path_to_string(path: FilePath) -> Result<String, String> {
    // Extract path
    let path_string = path
        .into_path()
        .map_err(|_| "Failed to resolve folder path")?
        .to_str()
        .ok_or("Path contains invalid UTF-8 characters")?
        .to_string();

    Ok(path_string)
}
