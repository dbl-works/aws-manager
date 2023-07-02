use std::env;

pub fn set_profile(profile: String) -> () {
  env::set_var("AWS_PROFILE", profile);
}
