use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use serde::de::DeserializeOwned;

pub mod security;
pub mod record;
pub mod system;

/// Response for endpoints that just return an execution status
#[derive(Debug, Clone, Deserialize)]
pub struct SimpleResult {
    #[serde(rename = "rspCode")]
    pub rsp_code: isize
}

/// Type to be used for `initial` and `range` for endpoints that don't return them.
pub type NotApplicable = Option<()>;

/// A request for an API endpoint returning JSON data.
pub trait JsonEndpoint : Serialize + Debug {
    /// Name of this endpoint
    const CMD: &'static str;
    /// Type of authentication this endpoint expects
    const AUTH: AuthenticationType = AuthenticationType::Any;
    /// Main response value
    type Response: DeserializeOwned;
    /// Response's `initial` value when details are requested
    type Initial: DeserializeOwned;
    /// Response's `range` value when details are requested
    type Range: DeserializeOwned;
}

/// A request for an API endpoint returning binary data
pub trait BinaryEndpoint : Serialize + Debug {
    const CMD: &'static str;
    const AUTH: AuthenticationType = AuthenticationType::Any;
}

/// The authentication type an endpoint expects
#[derive(Debug, PartialEq, Eq)]
pub enum AuthenticationType {
    /// No authentication
    None,
    /// login/password required (e.g. login)
    LoginPassword,
    /// Use of a token is required
    Token,
    /// Auth required, login/password and token are accepted
    Any,
}

/// Data returned by the server when it failed to execute the request
#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub code: isize,
    pub error: ApiErrorData,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", &self.error.detail, self.error.rsp_code)
    }
}

impl std::error::Error for ApiError {}

#[derive(Debug, Deserialize)]
pub struct ApiErrorData {
    #[serde(rename = "rspCode")]
    pub rsp_code: isize,
    pub detail: String,
}
