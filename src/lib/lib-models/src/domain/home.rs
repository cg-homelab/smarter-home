use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// DomainHome struct representing a home in the domain layer
/// # Fields
/// * `id` - Home ID
/// * `name` - Home name
/// * `address` - Home address
/// * `write_token` - access token for writing data to the home
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DomainHome {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub write_token: String,
}

/// DomainNewHome struct representing a new home to be created in the domain layer
/// # Fields
/// * `name` - Home name
/// * `address` - Home address
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DomainNewHome {
    pub name: String,
    pub address: String,
}
