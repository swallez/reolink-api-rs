use serde::{Deserialize, Serialize};
use crate::api::JsonEndpoint;

impl JsonEndpoint for GetUserRequest {
    const CMD: &'static str = "GetUser";
    type Response = GetUserResult;
    type Initial = GetUserInitial;
    type Range = GetUserRange;
}

//----- Request

/// Get information about all users
#[derive(Debug, Clone, Serialize)]
pub struct GetUserRequest;

//----- Result

#[derive(Debug, Clone, Deserialize)]
pub struct GetUserResult {
    #[serde(rename = "User")]
    pub user: Vec<UserInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserInfo {
    /// User name
    #[serde(rename = "userName")]
    pub user_name: String,
    /// Access level (`admin` or `guest`)
    pub level: String,
}

//----- Initial

#[derive(Debug, Clone, Deserialize)]
pub struct GetUserInitial {
    #[serde(rename = "User")]
    pub user: UserInitial
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserInitial {
    pub level: String,
}

//----- Range

#[derive(Debug, Clone, Deserialize)]
pub struct GetUserRange {
    #[serde(rename = "User")]
    pub user: UserRange
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserRange {
    /// Available access levels
    pub level: Vec<String>,
    #[serde(rename = "userName")]
    /// Min and max length of a username
    pub user_name: LengthRange,
    /// Min and max length of a password
    pub password: LengthRange,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LengthRange {
    #[serde(rename = "minLen")]
    pub min_len: usize,
    #[serde(rename = "maxLen")]
    pub max_len: usize,
}
