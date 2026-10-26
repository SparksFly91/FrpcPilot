use std::collections::BTreeMap;
use std::sync::Mutex;

use tauri::State;
use tauri_helper::auto_collect_command;

use crate::services::storage::{StoragePaths, write_json};

#[derive(Default)]
pub struct PreferencesLock(pub Mutex<()>);

#[tauri::command]
#[auto_collect_command]
pub fn get_preferences(
    paths: State<'_, StoragePaths>,
    lock: State<'_, PreferencesLock>,
) -> Result<Option<BTreeMap<String, String>>, String> {
    let _guard = lock.0.lock().map_err(|e| e.to_string())?;
    let path = paths.data().join("preferences.json");
    match std::fs::read(path) {
        Ok(bytes) => serde_json::from_slice(&bytes)
            .map(Some)
            .map_err(|e| e.to_string()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
#[auto_collect_command]
pub fn set_preferences(
    paths: State<'_, StoragePaths>,
    lock: State<'_, PreferencesLock>,
    preferences: BTreeMap<String, String>,
) -> Result<(), String> {
    if preferences
        .iter()
        .any(|(key, value)| !key.starts_with("frpcpilot:") || key.len() + value.len() > 65536)
        || preferences.len() > 100
    {
        return Err("设置数据无效或过大".into());
    }
    let _guard = lock.0.lock().map_err(|e| e.to_string())?;
    write_json(&paths.data().join("preferences.json"), &preferences)
}
