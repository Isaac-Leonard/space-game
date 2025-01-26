use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::models::_entities::users;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct LoginResponse {
    pub token: String,
    pub pid: String,
    pub name: String,
    pub is_verified: bool,
    pub role: String,
}

impl LoginResponse {
    #[must_use]
    pub fn new(user: &users::Model, token: &String) -> Self {
        Self {
            token: token.to_string(),
            pid: user.pid.to_string(),
            name: user.name.clone(),
            is_verified: user.email_verified_at.is_some(),
            role: user.role.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CurrentResponse {
    pub pid: String,
    pub name: String,
    pub email: String,
}

impl CurrentResponse {
    #[must_use]
    pub fn new(user: &users::Model) -> Self {
        Self {
            pid: user.pid.to_string(),
            name: user.name.clone(),
            email: user.email.clone(),
        }
    }
}
