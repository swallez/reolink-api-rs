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

/// Get system ability of the current user
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
    /// Abilities for each channel
    #[serde(rename = "abilityChn")]
    pub channels: Vec<HashMap<String, Ability>>,
    /// Abilities for the sevice
    #[serde(flatten)]
    pub device: HashMap<String, Ability>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Ability {
    /// Access right, validating in least significant three bits: the most significant bit
    /// indicates execution permission, the first bit indicates revision permission, and the
    /// second bit indicates read/write permission.
    pub permit: usize,

    /// 0 means the feature is not supported in that version, nonzero means the
    /// feature is supported. Different version numbers indicate that certain functional
    /// modules support different functional options.
    pub ver: usize,
}
