use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::api::{Channel, JsonEndpoint};
use crate::api::record::ScheduleTable;

impl JsonEndpoint for GetRecordingRequest {
    const CMD: &'static str = "GetRecV20";
    type Response = GetRecordingResponse;
    type Initial = GetRecordingResponse;
    type Range = GetRecordingRange;
}

/// Get the recording configuration.
#[derive(Debug, Clone, Serialize)]
pub struct GetRecordingRequest {
    /// Channel number
    pub channel: Channel,
}

//----- Response & Initial

#[derive(Debug, Clone, Deserialize)]
pub struct GetRecordingResponse {
    #[serde(rename = "Rec")]
    pub rec: RecordingConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecordingConfig {
    // Note: Home Hub also returns "scheduleEnable"
    #[serde(with = "crate::serde::bool_as_number")]
    pub enable: bool,

    /// Whether the video files can be overwritten
    #[serde(with = "crate::serde::bool_as_number")]
    pub overwrite: bool,

    /// Packaging cycle period as a string, e.g. "30 Minutes".
    /// Possible values are listed in `RecordingRange`.
    #[serde(rename = "packTime")]
    pub pack_time: Option<String>,

    /// Post record time as a string, e.g. "1 Minute".
    /// Possible values are listed in `RecordingRange`.
    #[serde(rename = "postRec")]
    pub post_rec: String,

    /// Enable pre record
    #[serde(rename = "preRec", with = "crate::serde::bool_as_number")]
    pub pre_rec: bool,

    /// Video retention duration in days.
    #[serde(rename = "saveDay")]
    pub save_day: usize,

    /// Weekly scheduling table: 7 days * 24 hours. Each byte indicates whether it’s recording.
    /// With the value of 0 the recording is off, otherwise the recording is on.
    pub schedule: RecordingSchedule,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecordingSchedule {
    pub channel: Channel,

    /// Weekly scheduling tables: 7 days * 24 hours. Each byte indicates whether it’s recording.
    /// With the value of 0 the recording is off, otherwise the recording is on.
    ///
    /// The map keys are the various detection methods, e.g. `AI_PEOPLE`, `AI_VEHICLE`, `MD`.
    #[serde(default)]
    pub table: HashMap<String, ScheduleTable>,
}

//----- Range

#[derive(Debug, Clone, Deserialize)]
pub struct GetRecordingRange {
    #[serde(rename = "Rec")]
    pub rec: RecordingRange,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecordingRange {
    pub enable: String, // Constant string "boolean"
    pub overwrite: String, // Constant string "boolean"
    #[serde(rename = "packTime")]
    pub pack_time: Option<Vec<String>>,
    #[serde(rename = "postRec")]
    pub post_rec: Vec<String>,
    #[serde(rename = "preRec")]
    pub pre_rec: String, // Constant string "boolean",
    #[serde(rename = "schedule")]
    pub schedule: ScheduleRange,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScheduleRange {
    pub channel: Channel,
    /// Values are all "boolean"
    pub table: Option<HashMap<String,String>>,
}
