#![allow(clippy::significant_drop_tightening)]

mod domains;

use domains::BrowserApp;
use tauri::{generate_context, generate_handler};

/// # Panics
///
/// Panics if error launching
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(BrowserApp::new())
        // .on_window_event(window::on_window_event)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(generate_handler![
            domains::tabs::transport::get_opened::command_tabs_get_opened,
            domains::tabs::transport::open_url_in_tab::command_tabs_add,
            domains::tabs::transport::set_active::command_tabs_set_active,
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
}
