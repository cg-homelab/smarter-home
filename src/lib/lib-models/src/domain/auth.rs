use serde::{Deserialize, Serialize};

// the response that we pass back to HTTP client once successfully authorised
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthBody {
    access_token: String,
    token_type: String,
    header_value: String,
}
impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token: access_token.clone(),
            token_type: "Bearer".to_string(),
            header_value: format!("Bearer {}", access_token),
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
