extern crate dirs;
extern crate ini;

use crate::aws::credentials::{
  AWSCredential,
  credentials_path,
};

use ini::{Ini, Properties};
use regex::Regex;
use std::fs;

pub fn get_credentials() -> Vec<AWSCredential> {
  let mut credentials:Vec<AWSCredential> = Vec::new();

  let data = load_credentials_file();

  for (sec, props) in &data {
    if sec.is_none() {
      continue;
    }

    let profile_name = sec.unwrap().to_string();
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

// @NOTE: this is totally broken and horrible. Better approach:
//        iterate over every line, start a new profile once we hit a row that matches "[.*]"
//        and then parse the key/value pairs from the following rows, filling default values if missing
fn parse_credential(sec: Option<&str>, props: Properties) -> AWSCredential {
  let re = Regex::new(r"^aws_(.+)").unwrap();
  let sec = sec.unwrap().to_string();

  let mut aws_access_key_id = "".to_string();
  let mut aws_secret_access_key = "".to_string();
  let mut region = None;
  let mut output = None;

  for (key, value) in props.iter() {
    if let Some(caps) = re.captures(key) {
      let key = caps.get(1).map_or("", |m| m.as_str());
      match key {
        "access_key_id" => aws_access_key_id = value.to_string(),
        "secret_access_key" => aws_secret_access_key = value.to_string(),
        "region" => region = Some(value.to_string()),
        "output" => output = Some(value.to_string()),
        _ => (),
      }
    }
  }

  AWSCredential {
    profile_name: sec,
    aws_access_key_id: aws_access_key_id,
    aws_secret_access_key: aws_secret_access_key,
    region: region.unwrap_or_else(|| "eu-central-1".to_string()).to_string(),
    output: output.unwrap_or_else(|| "json".to_string()).to_string(),
  }
}
