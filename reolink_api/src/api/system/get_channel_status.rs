use serde::{Deserialize, Serialize};
use crate::api::NotApplicable;
use crate::api::JsonEndpoint;

impl JsonEndpoint for GetChannelStatusRequest {
    const CMD: &'static str = "GetChannelstatus"; // Typo intentional
    type Response = GetChannelStatusResponse;
    type Initial = NotApplicable;
    type Range = NotApplicable;
}

#[derive(Debug, Clone, Serialize)]
pub struct GetChannelStatusRequest;

//----- Response

#[derive(Debug, Clone, Deserialize)]
pub struct GetChannelStatusResponse {
    pub count: usize,
    pub status: Vec<ChannelStatus>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChannelStatus {
    pub channel: usize,
    pub name: String,
    pub online: usize,
    // Not present on Home Hub
    #[serde(rename = "typeInfo")]
    pub type_info: Option<String>,
    // Not in the spec, but present in Home Hub
    #[serde(default)]
    pub uid: String,
    // Not in the spec, but present in Home Hub
    #[serde(default)]
    pub sleep: usize,
}
