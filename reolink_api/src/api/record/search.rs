use serde::{Deserialize, Serialize};
use crate::api::{Channel, NotApplicable};
use crate::api::JsonEndpoint;
use crate::api::record::{DateTime, ScheduleTable};

impl JsonEndpoint for SearchRequest {
    const CMD: &'static str = "Search";
    type Response = SearchResponse;
    type Initial = NotApplicable;
    type Range = NotApplicable;
}

/// Search stored video files
#[derive(Debug, Clone, Serialize)]
pub struct SearchRequest {
    #[serde(rename = "Search")]
    pub search: Search
}

#[derive(Debug, Clone, Serialize)]
pub struct Search {
    pub channel: Channel,

    /// Do we want only per-day statuses (true), or also a file list (false)?
    /// **Warning**: a file list will be returned only if `start_time` and `end_time`
    /// are within the same day.
    #[serde(rename = "onlyStatus", with = "crate::serde::bool_as_number")]
    pub only_status: bool,

    /// The stream type of the recordings, `main` is for searching main stream,
    /// otherwise is for searching sub stream.
    #[serde(rename = "streamType")]
    pub stream_type: String,

    #[serde(rename = "StartTime")]
    pub start_time: DateTime,

    #[serde(rename = "EndTime")]
    pub end_time: DateTime,
}

//----- Response

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResponse {
    #[serde(rename = "SearchResult")]
    pub search_result: SearchResults,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResults {
    pub channel: Channel,

    #[serde(rename = "Status")]
    pub status: Option<Vec<SearchStatus>>,

    #[serde(rename = "File")]
    // No default since field can be omitted if the time range is > 1 day
    pub file: Option<Vec<SearchFile>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchStatus {
    pub year: u16,
    pub mon: u8,
    /// Each value represents a day of the month, indicating whether there are recordings available.
    pub table: ScheduleTable,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchFile {
    #[serde(rename = "type")]
    pub stream_type: String,
    #[serde(rename = "StartTime")]
    pub start_time: DateTime,
    #[serde(rename = "EndTime")]
    pub end_time: DateTime,
    #[serde(rename = "frameRate")]
    pub frame_rate: usize,
    pub height: usize,
    pub width: usize,
    #[serde(with="crate::serde::from_str")]
    pub size: usize,
    pub name: String,
}
