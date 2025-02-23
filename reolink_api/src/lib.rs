mod common;
pub mod api;

// Re-export dependencies that are part of our public API
pub use reqwest;
pub use anyhow;
pub use bytes;
#[cfg(feature = "chrono")]
pub use chrono;

#[cfg(feature = "blocking")]
pub mod blocking;
mod serde;

#[cfg(feature = "blocking")]
/// A blocking client for the Reolink API.
pub type ReolinkBlockingClient = blocking::ReolinkClient;
