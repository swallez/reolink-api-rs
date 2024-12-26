use serde::Serialize;
use crate::api::{AuthenticationType, NotApplicable, SimpleResult};
use crate::api::JsonEndpoint;

impl JsonEndpoint for LogoutRequest {
    const CMD: &'static str = "Logout";
    const AUTH: AuthenticationType = AuthenticationType::Token;
    type Response = SimpleResult;
    type Initial = NotApplicable;
    type Range = NotApplicable;
}

// Note: this has to be an empty object
#[derive(Debug, Clone, Serialize)]
pub struct LogoutRequest {}
