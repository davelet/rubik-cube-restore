#[tauri::command]
pub fn my_custom_command() {
  println!("I was invoked from JavaScript!");
}