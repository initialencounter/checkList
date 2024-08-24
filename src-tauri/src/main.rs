// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_autostart::MacosLauncher;

use config::get_config;
use handle::handle_setup;
use utils::{log_info, restart};

mod config;
mod handle;
mod menu;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app: &mut tauri::App| Ok(handle_setup(app)))
        .invoke_handler(tauri::generate_handler![restart, get_config, log_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
