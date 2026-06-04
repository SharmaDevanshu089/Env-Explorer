use log;
use simplelog;
use std::fs;
use std::path::PathBuf;

extern crate dirs;

pub fn intiate_crawl() {
    match dirs::home_dir() {
        Some(path) => {
            info!("Home directory: {}", path.display());
            crawl(path);
        }
        #[allow(non_snake_case)]
        None => {
            error!("Could not find home directory");
            panic!("Error, Check Logs")
        }
    }
}
pub fn crawl(next_directory_path: PathBuf) {
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
        let entry = entry.unwrap();
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
                    | "Music" => {
                        info!("Skipping ignored directory: {}", string_file_name);
                    }
                    _ => {
                        info!("{}", file_path.to_string_lossy());
                        crawl(file_path);
                    }
                }
            }
        } else {
            //check for env
            if file_name == ".env" {
                info!("Found env at {}", file_path.to_string_lossy());
            }
        }
    }
}
