use crate::aws;

#[tauri::command]
pub fn aws_credentials_index() -> String {
  let credentials = aws::credentials::reader::get_credentials();
  let json = serde_json::to_string(&credentials);
  json.unwrap()
}

#[tauri::command]
pub fn aws_credentials_update() -> String {
  // TODO: accept user input, build a Vec of AWSCredential objects to ensure
  //       valid input. Then write to file using the aws::credentials::writer
  String::from("Hello from Rust")
}
