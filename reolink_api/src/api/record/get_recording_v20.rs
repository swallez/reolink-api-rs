use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::api::JsonEndpoint;

impl JsonEndpoint for GetRecordingRequest {
    const CMD: &'static str = "GetRecV20";
    type Response = GetRecordingResponse;
    type Initial = GetRecordingResponse;
    type Range = GetRecordingRange;
}

#[derive(Debug, Clone, Serialize)]
pub struct GetRecordingRequest {
    pub channel: usize,
}

//----- Response & Initial

#[derive(Debug, Clone, Deserialize)]
pub struct GetRecordingResponse {
    #[serde(rename = "Rec")]
    pub rec: RecordingConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecordingConfig {
    pub enable: usize,
    pub overwrite: usize,
    #[serde(rename = "packTime")]
    pub pack_time: Option<String>,
    #[serde(rename = "postRec")]
    pub post_rec: String,
    #[serde(rename = "preRec")]
    pub pre_rec: usize,
    #[serde(rename = "saveDay")]
    pub save_day: usize,
    pub schedule: RecordingSchedule,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecordingSchedule {
    pub channel: usize,
    pub table: Option<HashMap<String, String>>,
}

//----- Range

#[derive(Debug, Clone, Deserialize)]
pub struct GetRecordingRange {
    #[serde(rename = "Rec")]
    pub rec: RecordingRange,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecordingRange {
    pub enable: String, // "boolean"
    pub overwrite: Vec<usize>, // [0, 1, 2]
    #[serde(rename = "packTime")]
    pub pack_time: Option<Vec<String>>, // NVR
    #[serde(rename = "postRec")]
    pub post_rec: Vec<String>,
    #[serde(rename = "preRec")]
    pub pre_rec: String, // "boolean",
    #[serde(rename = "saveDay")]
    pub save_day: Vec<usize>,
    #[serde(rename = "schedule")]
    pub schedule: ScheduleRange,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScheduleRange {
    pub channel: usize,
    pub table: Option<HashMap<String,String>>,
}
