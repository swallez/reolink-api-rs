use serde::{Deserialize, Serialize};
use crate::api::NotApplicable;
use crate::api::JsonEndpoint;

impl JsonEndpoint for GetDevInfoRequest {
    const CMD: &'static str = "GetDevinfo"; // Typo intentional
    type Response = GetChannelStatusResponse;
    type Initial = NotApplicable;
    type Range = NotApplicable;
}

/// Get information on the device
#[derive(Debug, Clone, Serialize)]
pub struct GetDevInfoRequest;

//----- Response

#[derive(Debug, Clone, Deserialize)]
pub struct GetChannelStatusResponse {
    #[serde(rename = "DevInfo")]
    pub dev_info: DevInfo,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevInfo {
    /// Has 485?
    #[serde(rename = "B485", with = "crate::serde::bool_as_number")]
    pub b485: bool,

    /// Number of IO input ports
    #[serde(rename = "IOInputNum")]
    pub io_input_num: usize,

    /// Number of IO output ports
    #[serde(rename = "IOOutputNum")]
    pub io_output_num: usize,

    pub audio_num: usize,

    pub build_day: String,

    #[serde(rename = "cfgVer")]
    pub config_version: String,

    pub channel_num: usize,

    pub detail: String,

    /// Number of USB disks or SD cards
    pub disk_num: usize,

    pub exact_type: String,

    /// Version number of the firmware
    #[serde(rename = "firmVer")]
    pub firmware_version: String,

    #[serde(rename = "frameworkVer")]
    pub framework_version: usize,

    #[serde(rename = "hardVer")]
    pub hardware_version: String,

    pub model: String,

    pub name: String,

    pub pak_suffix: String,

    pub serial: String,

    /// Device type (e.g. `HOMEHUB`)
    #[serde(rename = "type")]
    pub type_: String,

    /// Whether Wi-Fi is supported
    #[serde(with = "crate::serde::bool_as_number")]
    pub wifi: bool,
}
