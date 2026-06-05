// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod reader;
pub mod crawl;
// use tauri::Manager;
// use window_vibrancy::apply_mica;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // .setup(|app| {
        //     let window = app.get_webview_window("main").unwrap();
        //     #[cfg(target_os = "windows")]
        //     {
        //         apply_mica(&window, None).expect("Failed to apply acrylic effect");
        //     }
        //     Ok(())
        // })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, reader::read_env_config , crawl::intiate_crawl])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
