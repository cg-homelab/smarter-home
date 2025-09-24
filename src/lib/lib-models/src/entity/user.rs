use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use surrealdb::Uuid;

#[derive(Clone, Deserialize, Serialize)]
pub struct User {
    pub id: RecordId,
    pub email: String,
    pub password: String,
    pub salt: String,
    pub name: String,
    pub age: u8,
    pub homes: Option<Vec<RecordId>>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}
impl User {
    pub fn to_domain(&self) -> crate::domain::user::User {
        crate::domain::user::User {
            id: self.id.key().to_string(),
            email: self.email.clone(),
            name: self.name.clone(),
            age: self.age,
            homes: self
                .homes
                .as_ref()
                .map(|homes| homes.iter().map(|h| h.key().to_string()).collect()),
            created_at: self.created_at,
            modified_at: self.modified_at,
        }
    }
    pub fn from_domain(
        model: crate::domain::user::NewUser,
        password: String,
        salt: String,
    ) -> Self {
        Self {
            id: RecordId::from((crate::entity::USER_TABLE, Uuid::new_v4())),
            email: model.email,
            password,
            salt,
            name: model.name,
            age: model.age,
            homes: model.homes.map(|homes| {
                homes
                    .into_iter()
                    .map(|h| RecordId::from((crate::entity::HOME_TABLE, h)))
                    .collect()
            }),
            created_at: model.created_at,
            modified_at: model.modified_at,
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub name: String,
    pub age: u8,
    pub homes: Option<Vec<RecordId>>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}
