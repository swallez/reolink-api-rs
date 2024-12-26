use serde::{Deserialize, Serialize};
use crate::api::JsonEndpoint;

impl JsonEndpoint for GetRecordingRequest {
    const CMD: &'static str = "GetRec";
    type Response = GetRecordingResponse;
    type Initial = GetRecordingResponse;
    type Range = GetRecordingRange;
}

#[derive(Debug, Clone, Serialize)]
pub struct GetRecordingRequest {
    pub channel: usize,
}

//----- Result & Initial

#[derive(Debug, Clone, Deserialize)]
pub struct GetRecordingResponse {
    #[serde(rename = "Rec")]
    pub rec: RecordingConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecordingConfig {
    pub channel: usize,
    pub overwrite: usize,
    #[serde(rename = "packTime")]
    pub pack_time: String, // NVR
    #[serde(rename = "postRec")]
    pub post_rec: String,
    #[serde(rename = "preRec")]
    pub pre_rec: usize,
    pub schedule: RecordingSchedule,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecordingSchedule {
    pub enable: usize,
    pub table: String,
}

//----- Range

#[derive(Debug, Clone, Deserialize)]
pub struct GetRecordingRange {
    pub rec: RecordingRange,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecordingRange {
    pub channel: usize,
    pub overwrite: String, // "boolean"
    #[serde(rename = "packTime")]
    pub pack_time: Vec<String>, // NVR
    #[serde(rename = "postRec")]
    pub post_rec: Vec<String>,
    #[serde(rename = "preRec")]
    pub pre_rec: String, // "boolean",
    #[serde(rename = "schedule")]
    pub schedule: ScheduleRange,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScheduleRange {
    pub enable: usize,
    pub table: String,
}
