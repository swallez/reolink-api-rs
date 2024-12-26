use serde::Serialize;
use crate::api::BinaryEndpoint;

impl BinaryEndpoint for SnapshotRequest {
    const CMD: &'static str = "Snap";
}

#[derive(Debug, Serialize)]
pub struct SnapshotRequest {
    pub channel: usize,
    /// Random string with fixed length. It's used to prevent browser caching.
    pub rs: String,
}
