use serde::{Deserialize, Serialize};
use crate::api::JsonEndpoint;
use crate::api::NotApplicable;
use crate::api::AuthenticationType;

impl JsonEndpoint for LoginRequest<'_> {
    const CMD: &'static str = "Login";
    // Do NOT send login & password in the URL, even if they're the same as in the payload:
    // the request will succeed, but the resulting token will be invalid (tested on Home Hub).
    const AUTH: AuthenticationType = AuthenticationType::None;
    type Response = LoginResult;
    type Initial = NotApplicable;
    type Range = NotApplicable;
}

//----- Request

#[derive(Debug, Clone, Serialize)]
pub struct LoginRequest<'a> {
    #[serde(rename = "User")]
    pub user: LoginUser<'a>
}

impl <'a> LoginRequest<'a> {
    pub fn new(login: &'a str, password: &'a str) -> Self {
        LoginRequest {
            user: LoginUser {
                version: "0",
                user_name: login,
                password
            }
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginUser<'a> {
    /// Must be `"0"`
    #[serde(rename = "Version")]
    pub version: &'static str,
    #[serde(rename = "userName")]
    pub user_name: &'a str,
    pub password: &'a str,
}

//----- Response

#[derive(Debug, Clone, Deserialize)]
pub struct LoginResult {
    #[serde(rename="Token")]
    pub token: Token
}

#[derive(Debug, Clone, Deserialize)]
pub struct Token {
    #[serde(rename="leaseTime")]
    pub lease_time: usize,
    pub name: String,
}
