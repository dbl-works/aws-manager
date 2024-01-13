use aws_smithy_http::body::SdkBody;
use aws_types::region::{Region, SigningRegion};
use aws_types::SigningService;
use aws_credential_types::Credentials;
use std::time::{Duration, SystemTime};
use aws_sig_auth::signer::{self, SigningError, OperationSigningConfig, HttpSignatureType, RequestConfig};
use http::Request;

pub fn generate(
    hostname: &str,
    region: Region,
    port: u16,
    username: &str,
    credentials: &Credentials,
) -> Result<String, SigningError> {
    let signer = signer::SigV4Signer::new();
    let mut operation_config = OperationSigningConfig::default_config();
    operation_config.signature_type = HttpSignatureType::HttpRequestQueryParams;
    operation_config.expires_in = Some(Duration::from_secs(15 * 60));
    let request_config = RequestConfig {
        request_ts: SystemTime::now(),
        region: &SigningRegion::from(region),
        service: &SigningService::from_static("rds-db"),
        payload_override: None,
    };
    let mut request = Request::builder()
        .uri(format!(
            "http://{hostname}:{port}/?Action=connect&DBUser={db_user}",
            hostname = hostname,
            port = port,
            db_user = username
        ))
        .body(SdkBody::empty())
        .expect("valid request");
    let _signature = signer.sign(
        &operation_config,
        &request_config,
        &credentials,
        &mut request,
    )?;
    let uri = request.uri().to_string();
    let uri = uri.replace("http://", "");
    Ok(uri)
}
