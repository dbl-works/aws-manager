use aws_sdk_rds as rds;
use aws_sdk_rds::Error;

use crate::aws::rds::RDSInstance;

pub async fn list() -> Result<Vec<RDSInstance>, Error> {
  let config = ::aws_config::load_from_env().await;
  let client = rds::Client::new(&config);
  let result = client.describe_db_instances().send().await?;
  let mut instances:Vec<RDSInstance> = Vec::new();

  for db_instance in result.db_instances().unwrap_or_default() {
    let identifier = db_instance.db_instance_identifier().unwrap_or_default().to_string();
    let project = identifier.split("-").next().unwrap_or_default().to_string();
    let environment = identifier.split("-").nth(1).unwrap_or_default().to_string();
    let username = format!("{}_{}_readonly", project, environment);

    let instance = RDSInstance {
      endpoint: db_instance.endpoint().unwrap().address().unwrap_or_default().to_string(),
      engine: db_instance.engine().unwrap_or_default().to_string(),
      instance_class: db_instance.db_instance_class().unwrap_or_default().to_string(),
      name: db_instance.db_name().unwrap_or_default().to_string(),
      port: db_instance.endpoint().unwrap().port(),
      status: db_instance.db_instance_status().unwrap_or_default().to_string(),
      username: username,
    };

    instances.push(instance);
  };

  Ok(instances)
}
