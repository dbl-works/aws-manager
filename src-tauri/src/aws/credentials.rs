use serde::Serialize;

pub mod reader;
pub mod writer;
pub mod setter;

#[derive(Serialize)]
pub struct AWSCredential {
  pub profile_name: String,
  pub aws_access_key_id: String,
  pub aws_secret_access_key: String,
  pub region: String,
  pub output: String,
}

fn credentials_path() -> String {
  let home_dir = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();
  String::from(home_dir) + "/.aws/credentials"
}
