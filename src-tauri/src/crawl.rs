use std::fs;
use std::path::PathBuf;
use log::error;
use log::info;
use log::warn;

extern crate dirs;

#[derive(serde::Serialize, Debug, Clone)]
pub struct ProjectEnv {
    pub name: String,
    pub path: String,
}

#[tauri::command]
pub fn intiate_crawl() {
    match dirs::home_dir() {
        Some(path) => {
            info!("Home directory: {}", path.display());
            let mut env_files = Vec::new();
            scan(path, &mut env_files);
            
            // Serialize to JSON and write to config
            match serde_json::to_string_pretty(&env_files) {
                Ok(json_data) => {
                    match fs::write("env_config.json", json_data) {
                        Ok(_) => info!("Successfully wrote env config to env_config.json"),
                        Err(e) => error!("Failed to write env config to env_config.json: {}", e),
                    }
                }
                Err(e) => error!("Failed to serialize env files: {}", e),
            }
        }
        #[allow(non_snake_case)]
        None => {
            error!("Could not find home directory");
            panic!("Error, Check Logs")
        }
    }
}

pub fn scan(next_directory_path: PathBuf, env_files: &mut Vec<ProjectEnv>) {
    info!("Crawling {}", next_directory_path.to_string_lossy());
    let directory = match fs::read_dir(&next_directory_path) {
        Ok(dir) => dir,
        Err(e) => {
            warn!(
                "Could not read directory {}: {}",
                next_directory_path.display(),
                e
            );
            return;
        }
    };
    for entry in directory {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let file_name = entry.file_name();
        let file_path = next_directory_path.join(&file_name);
        let string_file_name = file_name.to_string_lossy();
        if file_path.is_dir() {
            if string_file_name.starts_with('.') {
                info!("Skipping hidden directory: {}", string_file_name);
            } else {
                match string_file_name.as_ref() {
                    "node_modules"
                    | "target"
                    | "vendor"
                    | "build"
                    | "dist"
                    | "out"
                    | "bin"
                    | "obj"
                    | "venv"
                    | "env"
                    | "__pycache__"
                    | "coverage"
                    | "tmp"
                    | "temp"
                    | "packages"
                    | "usr"
                    | "sys"
                    | "proc"
                    | "dev"
                    | "run"
                    | "var"
                    | "boot"
                    | "etc"
                    | "lib"
                    | "lib64"
                    | "mnt"
                    | "media"
                    | "opt"
                    | "snap"
                    | "lost+found"
                    | "Windows"
                    | "Program Files"
                    | "Program Files (x86)"
                    | "ProgramData"
                    | "System32"
                    | "AppData"
                    | "Applications"
                    | "Library"
                    | "System"
                    | "Volumes"
                    | "Downloads"
                    | "Pictures"
                    | "Videos"
                    | "Music"
                    | "CrossDevice" => {
                        info!("Skipping ignored directory: {}", string_file_name);
                    }
                    _ => {
                        scan(file_path, env_files);
                    }
                }
            }
        } else {
            //check for env
            if file_name == ".env" {
                info!("Found env at {}", file_path.to_string_lossy());
                let parent = file_path.parent();
                let project_name = parent
                    .and_then(|p| p.file_name())
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "Unknown".to_string());
                
                env_files.push(ProjectEnv {
                    name: project_name,
                    path: file_path.to_string_lossy().into_owned(),
                });
            }
        }
    }
}
