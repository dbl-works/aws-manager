use serde::{Serialize};

pub mod reader;
pub mod writer;

#[derive(Serialize)]
pub struct AWSCredential {
  profile_name: String,
  aws_access_key_id: String,
  aws_secret_access_key: String,
  region: String,
  output: String,
}

pub fn credentials_path() -> String {
  let home_dir = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();
  String::from(home_dir) + "/.aws/credentials"
}
