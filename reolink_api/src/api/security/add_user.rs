use serde::Serialize;
use crate::api::{NotApplicable, SimpleResult};
use crate::api::JsonEndpoint;

impl JsonEndpoint for AddUserRequest {
    const CMD: &'static str = "AddUser";
    type Response = SimpleResult;
    type Initial = NotApplicable;
    type Range = NotApplicable;
}

/// Used to set the configuration of a user.
#[derive(Debug, Clone, Serialize)]
pub struct AddUserRequest {
    #[serde(rename = "User")]
    pub user: AddUser,
}

#[derive(Debug, Clone, Serialize)]
pub struct AddUser {
    /// User name
    #[serde(rename = "userName")]
    pub username: String,
    /// Password
    pub password: String,
    /// Access level (`admin` or `guest`)
    pub level: String,
}

