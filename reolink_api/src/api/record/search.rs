use serde::{Deserialize, Serialize};
use serde_with::{ serde_as, DisplayFromStr };
use crate::api::NotApplicable;
use crate::api::JsonEndpoint;
use crate::api::record::Time;

impl JsonEndpoint for SearchRequest {
    const CMD: &'static str = "Search";
    type Response = SearchResponse;
    type Initial = NotApplicable;
    type Range = NotApplicable;
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchRequest {
    #[serde(rename = "Search")]
    pub search: Search
}

#[derive(Debug, Clone, Serialize)]
pub struct Search {
    pub channel: usize,
    /// Do we want only statuses (0), or also a file list (1)?
    /// **Warning**: a file list will be returned only if `start_time` and `end_time`
    /// are within the same day.
    #[serde(rename = "onlyStatus")]
    pub only_status: usize,
    #[serde(rename = "streamType")]
    pub stream_type: String,
    #[serde(rename = "StartTime")]
    pub start_time: Time,
    #[serde(rename = "EndTime")]
    pub end_time: Time,
}

//----- Response

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResponse {
    #[serde(rename = "SearchResult")]
    pub search_result: SearchResults,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResults {
    pub channel: usize,
    #[serde(rename = "Status")]
    pub status: Option<Vec<SearchStatus>>,
    #[serde(rename = "File")]
    pub file: Option<Vec<SearchFile>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchStatus {
    pub year: u16,
    pub mon: u8,
    /// Each byte in the string represent the days of the month,
    /// indicating whether it’s recording. With the value of 0, the
    /// recording is off, with the value of 1, the recording is on.
    pub table: String,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct SearchFile {
    #[serde(rename = "type")]
    pub stream_type: String,
    #[serde(rename = "StartTime")]
    pub start_time: Time,
    #[serde(rename = "EndTime")]
    pub end_time: Time,
    #[serde(rename = "frameRate")]
    pub frame_rate: usize,
    pub height: usize,
    pub width: usize,
    #[serde_as(as = "DisplayFromStr")]
    pub size: usize,
    pub name: String,
}
