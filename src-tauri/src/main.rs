#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod aws;
mod api;

fn main() {
  let _ = fix_path_env::fix(); // fixes the app from crashing on macOS, see https://github.com/tauri-apps/tauri/issues/7063

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      api::aws_credentials_index,
      api::aws_credentials_update,
      api::aws_rds_index,
      api::generate_password,
      api::set_aws_profile,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
