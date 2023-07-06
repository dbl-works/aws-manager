use crate::aws::credentials::{
  AWSCredential,
  credentials_path,
};

#[tauri::command]
pub fn set_credentials(_records: Vec<AWSCredential>) {
  let _path = credentials_path();
  // TODO: persist serialized credentials to file
}

// private methods

// TODO?
