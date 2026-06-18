use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// DomainHomeLocation stores the home coordinates used for spatial hashing.
/// # Fields
/// * `latitude` - Latitude in decimal degrees
/// * `longitude` - Longitude in decimal degrees
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DomainHomeLocation {
    pub latitude: f64,
    pub longitude: f64,
}

/// DomainHome struct representing a home in the domain layer
/// # Fields
/// * `id` - Home ID
/// * `name` - Home name
/// * `address` - Home address
/// * `location` - Home coordinates
/// * `location_hash_high` - Highest precision location hash
/// * `location_hash_medium` - Medium precision location hash
/// * `location_hash_low` - Lowest precision location hash
/// * `write_token` - access token for writing data to the home
/// * `is_favorite` - whether the home is marked as favorite by the requesting user
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DomainHome {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub location: DomainHomeLocation,
    pub location_hash_high: String,
    pub location_hash_medium: String,
    pub location_hash_low: String,
    pub write_token: String,
    pub is_favorite: bool,
}

/// DomainNewHome struct representing a new home to be created in the domain layer
/// # Fields
/// * `name` - Home name
/// * `address` - Home address
/// * `location` - Home coordinates
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DomainNewHome {
    pub name: String,
    pub address: String,
    pub location: DomainHomeLocation,
}

/// DomainUpdateHome struct representing the fields that can be updated on a home
/// # Fields
/// * `name` - Home name
/// * `address` - Home address
/// * `location` - Home coordinates
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DomainUpdateHome {
    pub name: String,
    pub address: String,
    pub location: DomainHomeLocation,
}

/// DomainSetFavoriteHome struct for toggling the favorite status of a home
/// # Fields
/// * `is_favorite` - whether to mark the home as favorite
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DomainSetFavoriteHome {
    pub is_favorite: bool,
}
