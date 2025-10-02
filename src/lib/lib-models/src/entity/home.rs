use base64::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewHome {
    name: String,
    address: String,
}
impl NewHome {
    pub fn from_new_domain(model: crate::domain::home::DomainNewHome) -> Self {
        Self {
            name: model.name,
            address: model.address,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Home {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub write_token: String, // base64 encoded random token
}
impl Home {
    pub fn to_domain(&self) -> crate::domain::home::DomainHome {
        domain::home::DomainHome {
            id: self.id,
            name: self.name.clone(),
            address: self.address.clone(),
            write_token: self.write_token.clone(),
        }
    }

    pub fn from_new(model: NewHome) -> Self {
        let write_token = BASE64_URL_SAFE.encode(uuid::Uuid::new_v4().as_bytes());

        Self {
            id: Uuid::new_v4(),
            name: model.name,
            address: model.address,
            write_token,
        }
    }
    pub fn from_domain(model: crate::domain::home::DomainHome) -> Self {
        Self {
            id: model.id,
            name: model.name,
            address: model.address,
            write_token: model.write_token,
        }
    }
}
