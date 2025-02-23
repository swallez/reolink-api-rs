use serde::{Deserialize, Serialize};
use crate::api::{Channel, JsonEndpoint, NotApplicable};
use crate::api::record::DateTime;

impl JsonEndpoint for NvrDownloadRequest {
    const CMD: &'static str = "NvrDownload";
    type Response = NvrDownloadResponse;
    type Initial = NotApplicable;
    type Range = NotApplicable;
}

#[derive(Debug, Serialize)]
pub struct NvrDownloadRequest {
    #[serde(rename = "NvrDownload")]
    pub nvr_download: NvrDownload,
}

#[derive(Debug, Serialize)]
pub struct NvrDownload {
    pub channel: Channel,
    /// The bitstream type of the file to download, `"main"` or `"sub"`.
    #[serde(rename = "streamType")]
    pub stream_type: String,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime,
}

//----- Result

#[derive(Debug, Deserialize)]
pub struct NvrDownloadResponse {
    #[serde(rename = "fileCount")]
    pub file_count: usize,
    #[serde(rename = "fileList")]
    pub file_list: Vec<NvrFile>,
}

#[derive(Debug, Deserialize)]
pub struct NvrFile {
    #[serde(rename = "fileName")]
    pub name: String,
    #[serde(rename = "fileSize",with = "crate::serde::from_str")]
    pub size: u64,
}
