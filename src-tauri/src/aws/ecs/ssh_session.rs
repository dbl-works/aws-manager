use aws_sdk_ecs as ecs;

pub async fn start_session() -> Result<String, Error> {
  let config = ::aws_config::load_from_env().await;
  let client = ecs::Client::new(&config);

  // to be tested
  let result = client.start_session().send().await?;
  let session = result.session().unwrap();
  let stream_url = session.stream_url().unwrap();
  let token_value = session.token_value().unwrap();
  let token_parameter_name = session.token_parameter_name().unwrap();
  let session_id = session.session_id().unwrap();
  let expiry = session.expiry().unwrap();
  let stream_url = sess;
}
