use serde::Serialize;
use crate::api::{NotApplicable, SimpleResult};
use crate::api::JsonEndpoint;

impl JsonEndpoint for AddUserRequest {
    const CMD: &'static str = "AddUser";
    type Response = SimpleResult;
    type Initial = NotApplicable;
    type Range = NotApplicable;
}

#[derive(Debug, Clone, Serialize)]
pub struct AddUserRequest {
    #[serde(rename = "User")]
    pub user: AddUser,
}

#[derive(Debug, Clone, Serialize)]
pub struct AddUser {
    #[serde(rename = "userName")]
    pub username: String,
    pub password: String,
    pub level: String,
}

