use crate::aws::credentials::AWSCredential;
use crate::aws::credentials::credentials_path;

#[tauri::command]
pub fn set_credentials(records: Vec<AWSCredential>) {
  let path = credentials_path();
  // TODO: persist serialized credentials to file
}

// private methods

// TODO?
