use serde::Serialize;
use crate::api::{AuthenticationType, BinaryEndpoint};

impl BinaryEndpoint for DownloadRequest {
    const CMD: &'static str = "Download";
    const AUTH: AuthenticationType = AuthenticationType::Token;
}

#[derive(Debug, Serialize)]
pub struct DownloadRequest {
    pub source: String,
    pub output: Option<String>,
}
