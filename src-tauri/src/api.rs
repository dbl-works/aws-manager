use crate::aws;

#[tauri::command]
pub fn aws_credentials_index() -> String {
  let credentials = aws::credentials::reader::get_credentials();
  let json = serde_json::to_string(&credentials);
  json.unwrap()
}
