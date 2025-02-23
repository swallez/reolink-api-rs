use serde::{Deserialize, Serialize};
use crate::api::{Channel, NotApplicable};
use crate::api::JsonEndpoint;

impl JsonEndpoint for GetChannelStatusRequest {
    const CMD: &'static str = "GetChannelstatus"; // Typo intentional
    type Response = GetChannelStatusResponse;
    type Initial = NotApplicable;
    type Range = NotApplicable;
}

/// Get the status of all channels.
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
    pub channel: Channel,
    pub name: String,

    /// Is this channel online?
    #[serde(with = "crate::serde::bool_as_number")]
    pub online: bool,

    // Not present on Home Hub
    #[serde(rename = "typeInfo")]
    pub type_info: Option<String>,

    /// Unique id of the device when the channel is a device on a hub
    // Not in the spec, but present in Home Hub
    #[serde(default)]
    pub uid: Option<String>,

    /// Is this channel sleeping? Used when a channel represents a battery-powered camera.
    // Not in the spec, but present in Home Hub
    #[serde(default, with = "crate::serde::bool_as_number")]
    pub sleep: bool,
}
