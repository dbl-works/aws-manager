use serde::Serialize;

pub mod fetcher;
pub mod generator;

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

