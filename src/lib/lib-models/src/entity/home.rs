use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::domain;
use crate::entity::HOME_TABLE;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewHome {
    name: String,
    address: String,
}
impl NewHome {
    pub fn from_new_domain(model: crate::domain::home::NewHome) -> Self {
        Self {
            name: model.name,
            address: model.address,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Home {
    pub id: RecordId,
    name: String,
    address: String,
}
impl Home {
    pub fn to_domain(&self) -> crate::domain::home::Home {
        domain::home::Home {
            id: self.id.key().to_string(),
            name: self.name.clone(),
            address: self.address.clone(),
        }
    }
    pub fn from_domain(model: crate::domain::home::Home) -> Self {
        Self {
            id: RecordId::from((HOME_TABLE, model.id)),
            name: model.name,
            address: model.address,
        }
    }
}
