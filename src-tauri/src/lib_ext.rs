use chrono::Local;

// #[tauri::command]
// pub fn my_custom_command() {
//   println!("I was invoked from JavaScript!");
// }

#[tauri::command]
pub fn get_current_time() -> String {
  Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}