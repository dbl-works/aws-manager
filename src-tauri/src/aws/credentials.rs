use serde::Serialize;
use aws_credential_types::Credentials;

pub mod reader;
pub mod writer;
pub mod setter;

#[derive(Serialize)]
pub struct AWSCredential {
  pub profile_name: String,
  pub region: String,
  pub output: String,
  #[serde(skip_serializing)]
  pub credential: Credentials, // We can't serialize this, so we skip it
}

fn credentials_path() -> String {
  let home_dir = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();
  String::from(home_dir) + "/.aws/credentials"
}
