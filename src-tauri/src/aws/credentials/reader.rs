extern crate dirs;
use std::fs;
use crate::aws::credentials::{
  AWSCredential,
  credentials_path,
};

#[tauri::command]
pub fn get_credentials() -> String {
  let mut credentials:Vec<AWSCredential> = Vec::new();

  // @TODO(lud, 2022-11-07): parse all configs from "data" and push into vector
  let data = load_credentials_into_file();
  println!("{}", data); // @TODO(lud, 2022-11-08): for debugging, remove this line
  let dummy_credential = build_aws_credential_template(); // for testing
  credentials.push(dummy_credential);

  let json = serde_json::to_string(&credentials);

  json.unwrap()
}

// private methods

fn build_aws_credential_template() -> AWSCredential {
  AWSCredential {
    profile_name: String::from("default"),
    aws_access_key_id: String::from("YOUR-KEY-ID-HERE"),
    aws_secret_access_key: String::from("YOUR-SECRET-KEY-HERE"),
    region: String::from("eu-central-1"),
    output: String::from("json"),
  }
}

fn load_credentials_into_file() -> String {
  let path = credentials_path();
  let error_msg = String::from(
    "Error while reading credentials file from "
  ) + &path;

  fs::read_to_string(path).expect(&error_msg)
}
