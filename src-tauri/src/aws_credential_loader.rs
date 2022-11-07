extern crate dirs;
use std::fs;
use serde::{Serialize};

#[derive(Serialize)]
pub struct AWSCredential {
  profile_name: String,
  aws_access_key_id: String,
  aws_secret_access_key: String,
  region: String,
  output: String,
}

fn build_aws_credential_template() -> AWSCredential {
  AWSCredential {
    profile_name: String::from("default"),
    aws_access_key_id: String::from("aws_access_key_id"),
    aws_secret_access_key: String::from("aws_secret_access_key"),
    region: String::from("eu-central-1"),
    output: String::from("json"),
  }
}

fn load_credentials_into_file() -> String {
  let home_dir = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();
  let credentials_path = String::from(home_dir) + "/.aws/credentials";
  let error_msg = String::from("Error while reading credentials file from ") + &credentials_path;

  fs::read_to_string(credentials_path).expect(&error_msg)
}

#[tauri::command]
pub fn load_credentials() -> String {
  let mut credentials:Vec<AWSCredential> = Vec::new();

  // @TODO(lud, 2022-11-07): parse all configs from "data" and push into vector
  // let data = load_credentials_into_file();
  let dummy_credential = build_aws_credential_template(); // for testing
  credentials.push(dummy_credential);

  let json = serde_json::to_string(&credentials);

  json.unwrap()
}
