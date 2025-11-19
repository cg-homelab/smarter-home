use serde::{Deserialize, Serialize};

pub mod domain;
pub mod entity;
pub mod error;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Role {
    Admin,
    User,
    Home,
    Unknown,
}
impl Role {
    pub fn as_str(&self) -> &str {
        match self {
            Role::Admin => "admin",
            Role::User => "user",
            Role::Home => "home",
            Role::Unknown => "unknown",
        }
    }
    pub fn from_text(role: &str) -> Role {
        match role {
            "admin" => Role::Admin,
            "user" => Role::User,
            "home" => Role::Home,
            _ => Role::Unknown,
        }
    }
}
pub struct HashedPassword {
    pub hash: String,
    pub salt: String,
}
