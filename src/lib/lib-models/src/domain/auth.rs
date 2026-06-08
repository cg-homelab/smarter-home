use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Struct representing the authentication response body
/// # Fields
/// * `message` - Message
/// * `access_token` - Access token
/// * `token_type` - Token type
/// # Methods
/// * `new` - Creates a new AuthBody
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthBody {
    pub message: String,
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: usize,
}
impl AuthBody {
    /// Creates a new AuthBody
    /// # Arguments
    /// * `access_token` - Access token
    /// * `refresh_token` - Refresh token
    /// * `expires_in` - Access token expiration time in seconds
    /// * `message` - Message
    /// # Returns
    /// * `AuthBody` - New AuthBody
    pub fn new(
        access_token: String,
        refresh_token: String,
        expires_in: usize,
        message: String,
    ) -> Self {
        Self {
            message,
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LogoutRequest {
    pub refresh_token: String,
}

// TODO: Implement this instead of AuthUser for more generic OAuth2-style authentication
// the request type - "client_id" is analogous to a username, client_secret can also be interpreted as a password
// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct AuthPayload {
//     client_id: String,
//     client_secret: String,
// }
