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
    message: String,
    access_token: String,
    token_type: String,
}
impl AuthBody {
    /// Creates a new AuthBody
    /// # Arguments
    /// * `access_token` - Access token
    /// * `message` - Message
    /// # Returns
    /// * `AuthBody` - New AuthBody
    pub fn new(access_token: String, message: String) -> Self {
        Self {
            message,
            access_token: access_token.clone(),
            token_type: "Bearer".to_string(),
        }
    }
}

// TODO: Implement this instead of AuthUser for more generic OAuth2-style authentication
// the request type - "client_id" is analogous to a username, client_secret can also be interpreted as a password
// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct AuthPayload {
//     client_id: String,
//     client_secret: String,
// }
