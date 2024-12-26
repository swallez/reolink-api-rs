use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::api::{AuthenticationType, NotApplicable};
use crate::api::JsonEndpoint;

impl JsonEndpoint for GetAbilityRequest {
    const CMD: &'static str = "GetAbility";
    const AUTH: AuthenticationType = AuthenticationType::Token;
    type Response = GetAbilityResponse;
    type Initial = NotApplicable;
    type Range = NotApplicable;
}

#[derive(Debug, Clone, Serialize)]
pub struct GetAbilityRequest {
    #[serde(rename = "User")]
    pub user: GetAbility,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetAbility {
    /// If `"NULL"`, get the current user's abilities
    #[serde(rename = "userName")]
    pub user_name: String,
}

//----- Response

#[derive(Debug, Clone, Deserialize)]
pub struct GetAbilityResponse {
    #[serde(rename = "Ability")]
    pub ability: Abilities,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Abilities {
    #[serde(rename = "abilityChn")]
    pub channels: Vec<HashMap<String, Ability>>,
    #[serde(flatten)]
    pub device: HashMap<String, Ability>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Ability {
    pub permit: usize,
    pub ver: usize,
}
