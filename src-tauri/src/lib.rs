pub mod crawl;
pub mod reader;
pub mod syshandler;
pub mod terminal;
pub mod envirment;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            reader::get_current_env_vars,
            reader::count_env_vars,
            reader::read_env_config,
            crawl::intiate_crawl,
            syshandler::load_env_to_system_process,
            terminal::launch_terminal_with_env,
            envirment::add_user_env_var
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
