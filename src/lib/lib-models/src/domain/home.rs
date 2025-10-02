use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Home {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub write_token: String, // base64 encoded random token
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewHome {
    pub name: String,
    pub address: String,
}
