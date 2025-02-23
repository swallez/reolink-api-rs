use serde::Serialize;
use crate::api::{AuthenticationType, BinaryEndpoint};

impl BinaryEndpoint for DownloadRequest {
    const CMD: &'static str = "Download";
    const AUTH: AuthenticationType = AuthenticationType::Token;
}

/// Download a video file
#[derive(Debug, Serialize)]
pub struct DownloadRequest {
    /// Name of the source file
    pub source: String,

    /// File storage name, will be returned as the `Content-Disposition: attachment;filename=` header
    pub output: Option<String>,
}
