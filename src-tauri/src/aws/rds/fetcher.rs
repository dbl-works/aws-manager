use aws_sdk_rds as rds;
use aws_sdk_rds::Error;

use crate::aws::rds::RDSInstance;

pub async fn get_instances() -> Result<Vec<RDSInstance>, Error> {
  let config = ::aws_config::load_from_env().await;
  let client = rds::Client::new(&config);
  let result = client.describe_db_instances().send().await?;
  let mut instances:Vec<RDSInstance> = Vec::new();

  for db_instance in result.db_instances().unwrap_or_default() {
    let instance = RDSInstance {
      identifier: db_instance.db_instance_identifier().unwrap_or_default().to_string(),
      engine: db_instance.engine().unwrap_or_default().to_string(),
      status: db_instance.db_instance_status().unwrap_or_default().to_string(),
      endpoint: db_instance.endpoint().unwrap().address().unwrap_or_default().to_string(),
    };

    instances.push(instance);
  };

  Ok(instances)
}
