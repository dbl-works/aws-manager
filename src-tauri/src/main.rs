#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod aws;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      aws::credentials::reader::get_credentials,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
