use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize)]
pub struct NewDomainUser {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

pub struct DomainUser {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub homes: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct AuthUser {
    pub email: String,
    pub password: String,
}
