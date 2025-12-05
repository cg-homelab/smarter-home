use crate::Db;
use lib_models::domain::home::{DomainHome, DomainNewHome};
use lib_models::error::Error;
use uuid::Uuid;

pub struct Home {
    id: Uuid,
    name: String,
    address: String,
    token: String, // base64 encoded random token
    #[allow(dead_code)]
    tibber_token: Option<String>,
}
impl Home {
    /// Check if a home with the given address already exists
    /// # Arguments
    /// * `db` - Database connection
    /// * `address` - Home address
    /// # Returns
    /// * `Result<bool, Error>` - True if home exists, false otherwise
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

    /// Check if a user is part of a specific home.
    ///
    /// # Arguments
    /// * `db` - Database connection
    /// * `home_id` - The UUID of the home
    /// * `user_id` - The UUID of the user
    ///
    /// # Returns
    /// * `Result<bool, Error>` - True if the user is part of the home, false otherwise
    pub async fn check_user_on_home(db: &Db, home_id: Uuid, user_id: Uuid) -> Result<bool, Error> {
        // Check if user is part of home
        let result = sqlx::query!(
            "SELECT user_id FROM user_homes WHERE home_id = $1 AND user_id = $2",
            home_id,
            user_id
        )
        .fetch_all(&db.pool)
        .await?;
        if result.is_empty() {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    /// Insert a new home into the database and connect it to a user
    /// # Arguments
    /// * `db` - Database connection
    /// * `new_home` - New home data
    /// * `user_id` - User ID
    /// # Returns
    /// * `Result<DomainHome, Error>` - Inserted home or error
    pub async fn insert_home(
        db: &Db,
        new_home: &DomainNewHome,
        user_id: Uuid,
    ) -> Result<DomainHome, Error> {
        // Generate a random write token
        let id = Uuid::new_v4();
        let write_token = lib_utils::crypto::generate_jwt(
            new_home.address.clone(),
            lib_models::Role::Home,
            Some(id),
            true,
        );

        let home = sqlx::query_as!(
            Home,
            r#"
            INSERT INTO homes (id, name, address, token)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, address, token, tibber_token
            "#,
            id,
            new_home.name,
            new_home.address,
            write_token,
        )
        .fetch_one(&db.pool)
        .await?;
        let _user_home = sqlx::query!(
            r#"
            INSERT INTO user_homes (user_id, home_id)
            VALUES ($1, $2)
            "#,
            user_id,
            id,
        )
        .fetch_optional(&db.pool)
        .await?;

        Ok(DomainHome {
            id: home.id,
            name: home.name,
            address: home.address,
            write_token: home.token,
        })
    }

    /// Get all homes for a user
    /// # Arguments
    /// * `db` - Database connection
    /// * `user_id` - User ID
    /// # Returns
    /// * `Result<Vec<DomainHome>, Error>` - List of homes or error
    pub async fn get_homes(db: &Db, user_id: Uuid) -> Result<Vec<DomainHome>, Error> {
        // Get all homes for a user
        let homes = sqlx::query_as!(
            Home,
            r#"
            SELECT h.id, h.name, h.address, h.token, h.tibber_token
            FROM homes h
            JOIN user_homes uh ON h.id = uh.home_id
            WHERE uh.user_id = $1
            "#,
            user_id,
        )
        .fetch_all(&db.pool)
        .await?;
        let domain_homes = homes
            .into_iter()
            .map(|home| DomainHome {
                id: home.id,
                name: home.name,
                address: home.address,
                write_token: home.token,
            })
            .collect();
        Ok(domain_homes)
    }
}
