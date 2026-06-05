// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
extern crate log;

extern crate dirs;
extern crate simplelog;

mod crawl;
const LOGGING: bool = true;
fn main() {
    if LOGGING {
        // intiate logging
        simplelog::CombinedLogger::init(vec![
            simplelog::WriteLogger::new(
                simplelog::LevelFilter::Debug,
                simplelog::Config::default(),
                std::fs::File::create("env_explorer.log").unwrap(),
            ),
            simplelog::SimpleLogger::new(
                simplelog::LevelFilter::Debug,
                simplelog::Config::default(),
            ),
        ])
        .unwrap();
    }
    info!("Application started");
    env_explorer_lib::run();
}
