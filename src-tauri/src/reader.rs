use env_file_parser::read_env_file;
use log::Level::Info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectEnv {
    pub name: String,
    pub path: String,
}

#[tauri::command]
pub fn read_env_config(app_handle: tauri::AppHandle) -> Result<Vec<ProjectEnv>, String> {
    let path = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("Failed to get app local data dir: {}", e))?
        .join("env_config.json");

    log::info!("[reader] read_env_config called. Config path: {}", path.display());

    if !path.exists() {
        log::warn!("[reader] read_env_config: env_config.json does not exist yet.");
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(&path).map_err(|e| {
        let msg = format!("Failed to read env_config.json at {}: {}", path.display(), e);
        log::error!("[reader] {}", msg);
        msg
    })?;

    let config: Vec<ProjectEnv> = serde_json::from_str(&content).map_err(|e| {
        let msg = format!("Failed to parse env_config.json from {}: {}", path.display(), e);
        log::error!("[reader] {}", msg);
        msg
    })?;

    log::info!("[reader] read_env_config: successfully read {} projects.", config.len());
    Ok(config)
}

#[derive(Default)]
struct MyState {
    s: std::sync::Mutex<String>,
    t: std::sync::Mutex<std::collections::HashMap<String, String>>,
}
#[tauri::command]
pub async fn get_current_env_vars(path: String) -> HashMap<String, String> {
    log::info!("[reader] get_current_env_vars called for path: {}", path);
    match read_env_file(&path) {
        Ok(env_vars) => {
            let mut env_map = HashMap::new();
            for (key, value) in env_vars {
                env_map.insert(key, value);
            }
            log::info!("[reader] get_current_env_vars for path {} retrieved {} variables.", path, env_map.len());
            env_map
        }
        Err(e) => {
            log::error!("[reader] get_current_env_vars failed to read env file at {}: {}", path, e);
            HashMap::new()
        }
    }
}

#[tauri::command]
pub fn count_env_vars(path: String) -> usize {
    log::info!("[reader] count_env_vars called for path: {}", path);
    match read_env_file(&path) {
        Ok(vars) => {
            let count = vars.len();
            log::info!("[reader] count_env_vars: successfully parsed {} vars from {}", count, path);
            count
        }
        Err(e) => {
            log::warn!("[reader] count_env_vars: failed to parse env file at {}: {}", path, e);
            0
        }
    }
}

