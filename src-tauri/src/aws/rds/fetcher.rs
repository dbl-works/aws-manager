use aws_sdk_rds as rds;
use aws_sdk_rds::Error;

use crate::aws::rds::RDSInstance;

pub async fn get_instances() -> Result<Vec<RDSInstance>, Error> {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let client = rds::Client::new(&config);

    let result = client.describe_db_instances().send().await?;

    let db_instances_slice = result.db_instances();

    let instances: Vec<RDSInstance> = db_instances_slice.iter().map(|db_instance| {
        RDSInstance {
            endpoint: db_instance.endpoint().and_then(|e| e.address()).unwrap_or_default().to_string(),
            engine: db_instance.engine().unwrap_or_default().to_string(),
            instance_class: db_instance.db_instance_class().unwrap_or_default().to_string(),
            name: db_instance.db_name().unwrap_or_default().to_string(),
            port: db_instance.endpoint().map_or(0, |e| e.port().unwrap_or(0)),
            status: db_instance.db_instance_status().unwrap_or_default().to_string(),
            username: db_instance.master_username().unwrap_or_default().to_string(),
        }
    }).collect();

    Ok(instances)
}
