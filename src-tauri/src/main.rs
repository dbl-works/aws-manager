#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod aws_credential_loader;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      aws_credential_loader::load_credentials,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
