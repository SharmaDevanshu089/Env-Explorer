use std::process::Command;
use std::os::windows::process::CommandExt;
use std::path::Path;
use env_file_parser::read_env_file;

// Windows API creation flag to start the command in a new console window.
const CREATE_NEW_CONSOLE: u32 = 0x00000010;

#[tauri::command]
pub fn launch_terminal_with_env(path: String) -> Result<bool, String> {
    log::info!("[terminal] Initiating request to launch terminal with env from: {}", path);

    let env_path = Path::new(&path);
    if !env_path.exists() {
        let error_msg = format!("The specified path does not exist on this machine: {}", path);
        log::error!("[terminal] {}", error_msg);
        return Err(error_msg);
    }

    let project_dir = env_path.parent().ok_or_else(|| {
        let error_msg = format!("Could not determine parent directory of: {}", path);
        log::error!("[terminal] {}", error_msg);
        error_msg
    })?;

    // Parse environment variables using env_file_parser
    let vars = read_env_file(&path).map_err(|e| {
        let error_msg = format!("Failed to parse env file at {}: {}", path, e);
        log::error!("[terminal] {}", error_msg);
        error_msg
    })?;

    // Create standard command to spawn cmd.exe
    let mut cmd = Command::new("cmd.exe");
    cmd.current_dir(project_dir);
    cmd.creation_flags(CREATE_NEW_CONSOLE);

    // Load the environment variables from the file into the command environment
    for (key, value) in vars {
        cmd.env(key, value);
    }

    // Spawn the command process asynchronously
    match cmd.spawn() {
        Ok(_) => {
            log::info!("[terminal] Successfully launched terminal for project directory: {}", project_dir.display());
            Ok(true)
        }
        Err(e) => {
            let error_msg = format!("Failed to spawn cmd.exe process: {}", e);
            log::error!("[terminal] {}", error_msg);
            Err(error_msg)
        }
    }
}
