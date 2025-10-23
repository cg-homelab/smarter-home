use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainHome {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub write_token: String, // base64 encoded random token
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainNewHome {
    pub name: String,
    pub address: String,
}
