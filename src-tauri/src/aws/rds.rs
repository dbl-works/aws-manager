use serde::Serialize;

pub mod instances;
pub mod session_password;

#[derive(Serialize)]
pub struct RDSInstance {
  endpoint: String,
  engine: String,
  instance_class: String,
  name: String,
  port: i32,
  status: String,
  username: String,
}
