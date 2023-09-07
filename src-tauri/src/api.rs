

use aws_types::region::Region;

use crate::aws;

#[tauri::command]
pub fn aws_credentials_index() -> String {
  let credentials = aws::credentials::reader::get_credentials();
  let json = serde_json::to_string(&credentials);
  json.unwrap()
}

#[tauri::command]
pub fn set_aws_profile(profile_name: String) -> () {
  aws::credentials::setter::set_profile(profile_name);
}

#[tauri::command]
pub async fn aws_rds_index() -> String {
  let rds_instances = match aws::rds::instances::list().await {
    Ok(v) => v,
    Err(e) => return format!("Error: {:?}", e),
  };

  match serde_json::to_string(&rds_instances) {
    Ok(v) => v,
    Err(e) => format!("Error: {:?}", e),
  }
}

#[tauri::command]
pub async fn generate_password(hostname: String, port: u16, username: String, profile: String) -> String {
  let credentials = aws::credentials::reader::get_credentials();
  let credential = credentials.iter().filter(|cred| cred.profile_name == profile).next().unwrap();
  let region = Region::new(credential.region.clone());

  let password = aws::rds::session_password::generate(
    &hostname, region, port, &username, &credential.credential
  ).expect("An error occurred generating the password");
  password
}

#[tauri::command]
pub fn aws_credentials_update() -> String {
  // TODO: accept user input, build a Vec of AWSCredential objects to ensure
  //       valid input. Then write to file using the aws::credentials::writer
  String::from("Hello from Rust")
}
