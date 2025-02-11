use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use crate::api::{JsonEndpoint, NotApplicable};
use crate::api::record::Time;

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
    pub channel: usize,
    /// The bitstream type of the file to download, `"main"` or `"sub"`.
    #[serde(rename = "streamType")]
    pub stream_type: String,
    #[serde(rename = "StartTime")]
    pub start_time: Time,
    #[serde(rename = "EndTime")]
    pub end_time: Time,
}

//----- Result

#[derive(Debug, Deserialize)]
pub struct NvrDownloadResponse {
    #[serde(rename = "fileCount")]
    pub file_count: usize,
    #[serde(rename = "fileList")]
    pub file_list: Vec<NvrFile>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct NvrFile {
    #[serde(rename = "fileName")]
    pub name: String,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "fileSize")]
    pub size: u64,
}
