use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub name: String,
    pub age: u8,
    pub homes: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub age: u8,
    pub homes: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct AuthUser {
    pub email: String,
    pub password: String,
}
