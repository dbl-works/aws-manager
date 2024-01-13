// use aws_smithy_types::body::SdkBody;
// use aws_types::region::{Region, SigningRegion};
// use aws_types::SigningName;
// use aws_sigv4::http_request::{sign, SigningParams, SigningSettings, PayloadChecksumKind, SignatureLocation, UriPathNormalizationMode, SessionTokenMode};
// use aws_smithy_runtime_api::box_error::BoxError;
use aws_runtime::auth::SigningOptions;
use aws_smithy_runtime_api::client::identity::Identity;
// use aws_credential_types::Credentials;
// use std::time::Duration;
// use http::Request;

use aws_smithy_types::body::SdkBody;
// use aws_sigv4::http_request::{sign, SigningParams, SigningSettings, SignatureLocation};
use aws_smithy_runtime_api::box_error::BoxError;
use aws_types::region::Region;
use aws_credential_types::Credentials;
// use http::Request;
use std::time::Duration;
// use aws_sigv4::sign::v4;

// use aws_sigv4::http_request::{SignableRequest, SignableBody, SigningError};
use http::{Request, Method};
use std::convert::TryFrom;
use std::sync::Arc;
use std::time::SystemTime;
use std::any::Any;
use std::fmt::Debug;
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
