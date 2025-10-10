use crate::Db;
use lib_models::domain::home::{DomainHome, DomainNewHome};
use lib_models::error::Error;
use uuid::Uuid;

pub struct Home {
    id: Uuid,
    name: String,
    address: String,
    token: String, // base64 encoded random token
}
impl Home {
    pub async fn check_home_exists(db: &Db, address: &str) -> Result<bool, Error> {
        // Check if home with address already exists
        let result = sqlx::query!("SELECT id FROM homes WHERE address = $1", address)
            .fetch_all(&db.pool)
            .await?;
        if result.is_empty() {
            Ok(false)
        } else {
            Ok(true)
        }
    }
    pub async fn insert_home(db: &Db, new_home: &DomainNewHome) -> Result<DomainHome, Error> {
        // Generate a random write token
        let write_token = lib_utils::crypto::generate_base64_token();

        let home = sqlx::query_as!(
            Home,
            r#"
            INSERT INTO homes (name, address, token)
            VALUES ($1, $2, $3)
            RETURNING id, name, address, token
            "#,
            new_home.name,
            new_home.address,
            write_token,
        )
        .fetch_one(&db.pool)
        .await?;
        Ok(DomainHome {
            id: home.id,
            name: home.name,
            address: home.address,
            write_token: home.token,
        })
    }
}
