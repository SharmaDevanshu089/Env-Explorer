use std::fs;
use serde::{Serialize, Deserialize};
use tauri::Manager;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectEnv {
    pub name: String,
    pub path: String,
}

#[tauri::command]
pub fn read_env_config(app_handle: tauri::AppHandle) -> Result<Vec<ProjectEnv>, String> {
    let path = app_handle.path().app_local_data_dir()
        .map_err(|e| format!("Failed to get app local data dir: {}", e))?
        .join("env_config.json");
    
    log::info!("Reading env config from: {}", path.display());
    
    if !path.exists() {
        return Ok(Vec::new());
    }
    
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read env_config.json at {}: {}", path.display(), e))?;
    
    let config: Vec<ProjectEnv> = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse env_config.json from {}: {}", path.display(), e))?;
        
    Ok(config)
}
