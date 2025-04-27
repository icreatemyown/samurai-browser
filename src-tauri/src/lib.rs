mod domains;

use domains::{tabs, tabs::repo::TabRepo, window};
use std::sync::Mutex;
use tauri::{generate_context, generate_handler};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(TabRepo::default()))
        .on_window_event(window::on_window_event)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(generate_handler![
            tabs::commands::tabs_add,
            tabs::commands::tabs_get,
            tabs::commands::tabs_set_active,
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
}
