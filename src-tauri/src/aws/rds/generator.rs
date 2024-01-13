use aws_types::region::Region;
use aws_credential_types::Credentials;
use std::time::SystemTime;
use std::error::Error;
use aws_sigv4::{
    http_request::{sign, SignableBody, SignableRequest, SigningSettings},
    sign::v4,
};

pub fn generate_password(
    hostname: &str,
    region: Region,
    port: u16,
    username: &str,
    credentials: &Credentials,
) -> Result<String, Box<dyn Error>> {
    // let identity = credentials.into();
    let identity = credentials.clone().into();
    let region_string = region.to_string();

    let mut signing_settings = SigningSettings::default();
    signing_settings.expires_in = Some(std::time::Duration::from_secs(900));
    signing_settings.signature_location = aws_sigv4::http_request::SignatureLocation::QueryParams;

    let signing_params = v4::SigningParams::builder()
        .identity(&identity)
        .region(&region_string)
        .name("rds-db")
        .time(SystemTime::now())
        .settings(signing_settings)
        .build()?;

    let url = format!(
        "https://{hostname}:{port}/?Action=connect&DBUser={db_user}",
        hostname = hostname,
        port = port,
        db_user = username
    );

    let signable_request =
        SignableRequest::new("GET", &url, std::iter::empty(), SignableBody::Bytes(&[]))
            .expect("signable request");

    let (signing_instructions, _signature) = sign(signable_request, &signing_params.into())?.into_parts();

    let mut url = url::Url::parse(&url).unwrap();
    for (name, value) in signing_instructions.params() {
        url.query_pairs_mut().append_pair(name, value);
    }

    let response = url.to_string().split_off("https://".len());

    Ok(response)
}
