use serde::Serialize;
use crate::api::{BinaryEndpoint, Channel};

impl BinaryEndpoint for SnapshotRequest {
    const CMD: &'static str = "Snap";
}

/// Capture an image.
#[derive(Debug, Serialize)]
pub struct SnapshotRequest {
    pub channel: Channel,
    /// Random string with fixed length. It's used to prevent browser caching.
    pub rs: String,
}
