use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectEnv {
    pub name: String,
    pub path: String,
}

#[tauri::command]
pub fn read_env_config() -> Result<Vec<ProjectEnv>, String> {
    let content = fs::read_to_string("env_config.json")
        .map_err(|e| format!("Failed to read env_config.json: {}", e))?;
    
    let config: Vec<ProjectEnv> = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse env_config.json: {}", e))?;
        
    Ok(config)
}
