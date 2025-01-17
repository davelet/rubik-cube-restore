// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod lib_ext;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, lib_ext::my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
