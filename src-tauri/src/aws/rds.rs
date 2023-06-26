use serde::Serialize;

pub mod fetcher;

#[derive(Serialize)]
pub struct RDSInstance {
  identifier: String,
  engine: String,
  status: String,
  endpoint: String
}

