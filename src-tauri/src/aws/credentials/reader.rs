extern crate dirs;

use crate::aws::credentials::{
  AWSCredential,
  credentials_path,
};

use regex::Regex;
use std::fs;

pub fn get_credentials() -> Vec<AWSCredential> {
  let mut credentials:Vec<AWSCredential> = Vec::new();

  // @TODO(lud, 2022-11-07): parse all configs from "data" and push into vector
  let data = load_credentials_file();
  let test = parse_credential(data);
  let dummy_credential = build_aws_credential_template(); // for testing
  credentials.push(dummy_credential);

  credentials
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

fn load_credentials_file() -> String {
  let path = credentials_path();
  let error_msg = String::from(
    "Error while reading credentials file from "
  ) + &path;

  fs::read_to_string(path).expect(&error_msg)
}

// @NOTE: this is totally broken and horrible. Better approach:
//        iterate over every line, start a new profile once we hit a row that matches "[.*]"
//        and then parse the key/value pairs from the following rows, filling default values if missing
fn parse_credential(raw: String) -> AWSCredential {
  let template = build_aws_credential_template();
  let profile_name = String::from("default");

  // @NOTE: the regex only works if there is **some** char after the last value, e.g. a newline
  //        this especially might be a problem for the last line/entry of that profile.
  //        We want a non-greedy matcher to not match the optional quotes, but still have to stop at some point.
  let profile_name_regex = Regex::new(r#"\[(.*)\]"#).unwrap();
  let aws_access_key_id_regex = Regex::new(r#"aws_access_key_id\s{0,}=\s{0,}(.*?)\[\n\r\s]{1}"#).unwrap();
  let aws_secret_access_key_regex = Regex::new(r#"aws_secret_access_key\s{0,}=\s{0,}(.*?)\[\n\r\s]{1}"#).unwrap();
  let region_regex = Regex::new(r#"region\s{0,}=\s{0,}(.*?)\[\n\r\s]{1}"#).unwrap();
  let output_regex = Regex::new(r#"output\s{0,}=\s{0,}(.*?)\[\n\r\s]{1}"#).unwrap();


  // println!("profile_name: {}", first_match_or(&raw, &profile_name_regex, "default"));
  // println!("aws_access_key_id: {}", first_match_or(&raw, &profile_name_regex, ""));
  // println!("aws_secret_access_key: {}", first_match_or(&raw, &aws_secret_access_key_regex, ""));
  // println!("region: {}", first_match_or(&raw, &region_regex, &template.region));
  // println!("output: {}", first_match_or(&raw, &output_regex, &template.output));


  let credential = AWSCredential {
    profile_name: first_match_or(&raw, &profile_name_regex, ""),
    aws_access_key_id: first_match_or(&raw, &profile_name_regex, ""),
    aws_secret_access_key: first_match_or(&raw, &aws_secret_access_key_regex, ""),
    region: first_match_or(&raw, &region_regex, &template.region),
    output: first_match_or(&raw, &output_regex, &template.output),
  };

  credential
}

fn first_match_or(raw: &String, regex: &Regex, default: &str) -> String {
  raw.split(regex).collect::<Vec<&str>>().first().unwrap_or(&default).to_string()
}
