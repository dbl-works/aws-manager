extern crate dirs;
extern crate ini;

use aws_credential_types::Credentials;
use crate::aws::credentials::{
  AWSCredential,
  credentials_path,
};

use ini::{Ini, Properties};
use std::fs;

pub fn get_credentials() -> Vec<AWSCredential> {
  let mut credentials:Vec<AWSCredential> = Vec::new();

  let data = load_credentials_file();

  for (sec, props) in &data {
    if sec.is_none() {
      continue;
    }

    credentials.push(parse_credential(sec, props.clone()));
  }

  credentials
}

fn load_credentials_file() -> Ini {
  let path = credentials_path();
  let error_msg = String::from(
    "Error while reading credentials file from "
  ) + &path;

  let buf = fs::read_to_string(path).expect(&error_msg);
  Ini::load_from_str(&buf).unwrap()
}

fn parse_credential(sec: Option<&str>, props: Properties) -> AWSCredential {
  let aws_access_key_id = props.get("aws_access_key_id").map(|v| v.clone()).unwrap_or_default();
  let aws_secret_access_key = props.get("aws_secret_access_key").map(|v| v.clone()).unwrap_or_default();
  let region = props.get("region").map(|v| v.to_string()).unwrap_or_else(|| "eu-central-1".to_string());
  let output = props.get("output").map(|v| v.to_string()).unwrap_or_else(|| "json".to_string());

  AWSCredential {
    profile_name: sec.unwrap_or_default().to_string(),
    region: region,
    output: output,
    credential: Credentials::new(
    aws_access_key_id,
    aws_secret_access_key,
    None,
    None,
    "default",
    ),
  }
}
