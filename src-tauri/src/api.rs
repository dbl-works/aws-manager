use crate::aws;

#[tauri::command]
pub fn aws_credentials_index() -> String {
  let credentials = aws::credentials::reader::get_credentials();
  let json = serde_json::to_string(&credentials);
  json.unwrap()
}

#[tauri::command]
pub fn set_aws_profile(profile_name: String) -> () {
  println!("Setting AWS_PROFILE to {}", profile_name);
  aws::credentials::setter::set_profile(profile_name);
}

#[tauri::command]
pub async fn aws_rds_index() -> String {
  print!("Fetching RDS instances...");
  let rds_instances = match aws::rds::fetcher::get_instances().await {
    Ok(v) => v,
    Err(e) => return format!("Error: {:?}", e),
  };

  match serde_json::to_string(&rds_instances) {
    Ok(v) => v,
    Err(e) => format!("Error: {:?}", e),
  }
}

#[tauri::command]
pub fn aws_credentials_update() -> String {
  // TODO: accept user input, build a Vec of AWSCredential objects to ensure
  //       valid input. Then write to file using the aws::credentials::writer
  String::from("Hello from Rust")
}
