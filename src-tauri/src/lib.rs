// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod lib_ext;
mod rubiks;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            rubiks::init_get_get_state,
            rubiks::shuffle,
            rubiks::turn,
            lib_ext::get_current_time,
            lib_ext::resize_window,
            lib_ext::get_window_size,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
