use std::path::Path;

#[tauri::command]
pub fn load_env_to_system_process(path: String) -> Result<bool, String> {
    log::info!(
        "[syshandler] Initiating request to load env file from: {}",
        path
    );

    let target_path = Path::new(&path);

    if !target_path.exists() {
        let error_msg = format!(
            "The specified path does not exist on this machine: {}",
            path
        );
        log::error!("[syshandler] {}", error_msg);
        return Err(error_msg);
    }
    match dotenvy::from_path(target_path) {
        Ok(_) => {
            log::info!("[syshandler] Successfully loaded all environment variables from {} into the system process environment.", path);
            if let Ok(iter) = dotenvy::from_path_iter(target_path) {
                log::info!("[syshandler] Listing loaded variables keys:");
                for item in iter {
                    if let Ok((key, _)) = item {
                        log::info!("[syshandler] Loaded key: {}", key);
                    }
                }
            }

            Ok(true)
        }
        Err(error) => {
            let error_msg = format!(
                "dotenvy failed to parse and load env file at {}: {}",
                path, error
            );
            log::error!("[syshandler] {}", error_msg);
            Err(error_msg)
        }
    }
}
