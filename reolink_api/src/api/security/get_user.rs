use serde::{Deserialize, Serialize};
use crate::api::JsonEndpoint;

impl JsonEndpoint for GetUserRequest {
    const CMD: &'static str = "GetUser";
    type Response = GetUserResult;
    type Initial = GetUserInitial;
    type Range = GetUserRange;
}

//----- Request

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
    pub level: String,
    #[serde(rename = "userName")]
    pub user_name: String,
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
    pub level: Vec<String>,
    pub password: LengthRange,
    #[serde(rename = "userName")]
    pub user_name: LengthRange,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LengthRange {
    #[serde(rename = "minLen")]
    pub min_len: usize,
    #[serde(rename = "maxLen")]
    pub max_len: usize,
}
