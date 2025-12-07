/// Library for models used across the application(s)
/// This library contains domain models, error handling, and role definitions.
use serde::{Deserialize, Serialize};

pub mod domain;
pub mod error;

/// Enum representing user roles
/// # Variants
/// * `Admin` - Administrator role
/// * `User` - Regular user role
/// * `Home` - Home role
/// * `Unknown` - Unknown role
/// # Methods
/// * `as_str` - Returns the role as a string
/// * `from_text` - Creates a Role from a string
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Role {
    Admin,
    User,
    Home,
    Unknown,
}
impl Role {
    /// Returns the role as a string
    /// # Returns
    /// * `&str` - Role as string
    pub fn as_str(&self) -> &str {
        match self {
            Role::Admin => "admin",
            Role::User => "user",
            Role::Home => "home",
            Role::Unknown => "unknown",
        }
    }

    /// Creates a Role from a string
    /// # Arguments
    /// * `role` - Role as string
    /// # Returns
    /// * `Role` - Role enum
    pub fn from_text(role: &str) -> Role {
        match role {
            "admin" => Role::Admin,
            "user" => Role::User,
            "home" => Role::Home,
            _ => Role::Unknown,
        }
    }
}

/// Struct representing a hashed password and its salt
pub struct HashedPassword {
    pub hash: String,
    pub salt: String,
}
