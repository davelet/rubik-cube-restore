use chrono::Local;
use tauri::{command, PhysicalSize, Size, Window};

// #[tauri::command]
// pub fn my_custom_command() {
//   println!("I was invoked from JavaScript!");
// }

#[command]
pub fn resize_window(width: u32, height: u32, window: Window) {
    let _ = window.set_size(Size::Physical(PhysicalSize { width, height }));
}

#[command]
pub fn get_window_size(window: tauri::Window) -> (u32, u32) {
    match window.inner_size() {
        Ok(size) => (size.width, size.height),
        Err(e) => panic!("Error: {}", e.to_string()),
    }
}

#[command]
pub fn get_current_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
