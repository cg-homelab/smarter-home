use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// NewDomainUser struct representing a new user to be created in the domain layer
/// # Fields
/// * `email` - User email
/// * `password` - User password
/// * `first_name` - User first name
/// * `last_name` - User last name
#[derive(Clone, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NewDomainUser {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

/// DomainUser struct representing a user in the domain layer
/// # Fields
/// * `id` - User ID
/// * `email` - User email
/// * `role` - User role
/// * `first_name` - User first name
/// * `last_name` - User last name
/// * `homes` - List of home IDs associated with the user
/// * `created_at` - Creation timestamp
/// * `updated_at` - Update timestamp
#[derive(Clone, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DomainUser {
    pub id: Uuid,
    pub email: String,
    pub role: String,
    pub first_name: String,
    pub last_name: String,
    pub homes: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// AuthUser struct representing user credentials for authentication
/// # Fields
/// * `email` - User email
/// * `password` - User password
#[derive(Clone, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthUser {
    pub email: String,
    pub password: String,
}
