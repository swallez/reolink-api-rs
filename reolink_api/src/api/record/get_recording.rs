use serde::{Deserialize, Serialize};
use crate::api::{Channel, JsonEndpoint};
use crate::api::record::ScheduleTable;

impl JsonEndpoint for GetRecordingRequest {
    const CMD: &'static str = "GetRec";
    type Response = GetRecordingResponse;
    type Initial = GetRecordingResponse;
    type Range = GetRecordingRange;
}

/// Get the recording configuration. Note: This command supports models 52X only. When
/// `scheduleVersion=1` in the capability set, use `get_recording_v20`.
#[derive(Debug, Clone, Serialize)]
pub struct GetRecordingRequest {
    /// Channel number
    pub channel: Channel,
}

//----- Result & Initial

#[derive(Debug, Clone, Deserialize)]
pub struct GetRecordingResponse {
    #[serde(rename = "Rec")]
    pub rec: RecordingConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecordingConfig {
    /// Channel number
    pub channel: Channel,

    /// Whether the video files can be overwritten
    #[serde(with = "crate::serde::bool_as_number")]
    pub overwrite: bool,

    /// Packaging cycle period as a string, e.g. "30 Minutes".
    /// Possible values are listed in `RecordingRange`.
    #[serde(rename = "packTime")]
    pub pack_time: String, // NVR

    /// Post record time as a string, e.g. "1 Minute".
    /// Possible values are listed in `RecordingRange`.
    #[serde(rename = "postRec")]
    pub post_rec: String,

    /// Enable pre record
    #[serde(rename = "preRec", with = "crate::serde::bool_as_number")]
    pub pre_rec: bool,

    /// Weekly scheduling table: 7 days * 24 hours. Each byte indicates whether it’s recording.
    /// With the value of 0 the recording is off, otherwise the recording is on.
    pub schedule: RecordingSchedule,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecordingSchedule {
    /// Is this schedule enabled?
    #[serde(with = "crate::serde::bool_as_number")]
    pub enable: bool,
    pub table: ScheduleTable,
}

//----- Range

#[derive(Debug, Clone, Deserialize)]
pub struct GetRecordingRange {
    pub rec: RecordingRange,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecordingRange {
    pub channel: Channel,

    pub overwrite: String, // Constant string "boolean"

    #[serde(rename = "packTime")]
    pub pack_time: Vec<String>, // NVR

    #[serde(rename = "postRec")]
    pub post_rec: Vec<String>,

    #[serde(rename = "preRec")]
    pub pre_rec: String, // "boolean",

    #[serde(rename = "schedule")]
    pub schedule: ScheduleTable,
}
