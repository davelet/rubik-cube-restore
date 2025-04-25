// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod rubiks;

use chrono::Local;
use tauri::{command, PhysicalSize, Size, Window};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            rubiks::init_get_get_state,
            rubiks::shuffle,
            rubiks::turn,
            rubiks::solve,
            get_current_time,
            resize_window,
            get_window_size,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[command]
fn resize_window(width: u32, height: u32, window: Window) {
    let _ = window.set_size(Size::Physical(PhysicalSize { width, height }));
}

#[command]
fn get_window_size(window: tauri::Window) -> (u32, u32) {
    match window.inner_size() {
        Ok(size) => (size.width, size.height),
        Err(e) => panic!("Error: {}", e.to_string()),
    }
}

#[command]
fn get_current_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
