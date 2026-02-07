use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Wry};
use tauri_plugin_store::{Store, StoreExt};

use crate::constants::{SETTINGS_FILE, SETTINGS_KEY_DOWNLOAD_PATH, SETTINGS_KEY_THEME};

pub fn load_settings_store(app: &AppHandle) -> Result<Arc<Store<Wry>>, String> {
    app.store(SETTINGS_FILE).map_err(|e| e.to_string())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsStore {
    pub theme: Option<String>,
    pub download_path: Option<String>,
}

pub fn get_all_settings(store: &Store<Wry>) -> Result<SettingsStore, String> {
    Ok(SettingsStore {
        theme: get_settings_theme(store),
        download_path: get_settings_download_path(store),
    })
}

pub fn set_all_settings(store: &Store<Wry>, settings: SettingsStore) -> Result<(), String> {
    if let Some(path) = &settings.download_path {
        store.set(SETTINGS_KEY_DOWNLOAD_PATH, path.as_str());
    }

    if let Some(theme) = &settings.theme {
        store.set(SETTINGS_KEY_THEME, theme.as_str());
    }

    store.save().map_err(|e| e.to_string())
}

pub fn get_settings_download_path(store: &Store<Wry>) -> Option<String> {
    store
        .get(SETTINGS_KEY_DOWNLOAD_PATH)
        .and_then(|v| v.as_str().map(str::to_string))
}

pub fn set_settings_download_path(store: &Store<Wry>, path: &str) -> Result<(), String> {
    store.set(SETTINGS_KEY_DOWNLOAD_PATH, path);
    store.save().map_err(|e| e.to_string())
}

pub fn get_settings_theme(store: &Store<Wry>) -> Option<String> {
    store
        .get(SETTINGS_KEY_THEME)
        .and_then(|v| v.as_str().map(str::to_string))
}

pub fn set_settings_theme(store: &Store<Wry>, theme: &str) -> Result<(), String> {
    store.set(SETTINGS_KEY_THEME, theme);
    store.save().map_err(|e| e.to_string())
}
