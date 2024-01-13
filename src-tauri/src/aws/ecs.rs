use serde::Serialize;

pub mod start_session;

#[derive(Serialize)]
pub struct ECSTerminalSession {
  stream_url: String,
  container_uptime: String,
}
