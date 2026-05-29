use crate::Db;
use lib_models::domain::home::{DomainHome, DomainNewHome, DomainUpdateHome};
use lib_models::error::Error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Home struct representing a home in the database
/// # Fields
/// * `id` - Home ID
/// * `name` - Home name
/// * `address` - Home address
/// * `token` - Base64 encoded random token
/// * `tibber_token` - Optional Tibber token
#[derive(Debug, Clone, Serialize, Deserialize)]
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
            is_favorite: false,
        })
    }

    /// Get all homes for a user
    /// # Arguments
    /// * `db` - Database connection
    /// * `user_id` - User ID
    /// # Returns
    /// * `Result<Vec<DomainHome>, Error>` - List of homes or error
    pub async fn get_homes(db: &Db, user_id: Uuid) -> Result<Vec<DomainHome>, Error> {
        // Get all homes for a user, including per-user favorite status
        let homes = sqlx::query!(
            r#"
            SELECT h.id, h.name, h.address, h.token, h.tibber_token, uh.is_favorite
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
            .map(|r| DomainHome {
                id: r.id,
                name: r.name,
                address: r.address,
                write_token: r.token,
                is_favorite: r.is_favorite,
            })
            .collect();
        Ok(domain_homes)
    }

    /// Update the name and address of an existing home
    /// # Arguments
    /// * `db` - Database connection
    /// * `home_id` - Home ID to update
    /// * `user_id` - User ID (used to retrieve per-user favorite status)
    /// * `update` - Updated home data
    /// # Returns
    /// * `Result<DomainHome, Error>` - Updated home or error
    pub async fn update_home(
        db: &Db,
        home_id: Uuid,
        user_id: Uuid,
        update: &DomainUpdateHome,
    ) -> Result<DomainHome, Error> {
        let home = sqlx::query_as!(
            Home,
            r#"
            UPDATE homes
            SET name = $1, address = $2, updated_at = NOW()
            WHERE id = $3
            RETURNING id, name, address, token, tibber_token
            "#,
            update.name,
            update.address,
            home_id,
        )
        .fetch_one(&db.pool)
        .await?;

        let is_favorite = sqlx::query!(
            "SELECT is_favorite FROM user_homes WHERE home_id = $1 AND user_id = $2",
            home_id,
            user_id,
        )
        .fetch_one(&db.pool)
        .await?
        .is_favorite;

        Ok(DomainHome {
            id: home.id,
            name: home.name,
            address: home.address,
            write_token: home.token,
            is_favorite,
        })
    }

    /// Set the favorite status of a home for a specific user
    /// # Arguments
    /// * `db` - Database connection
    /// * `home_id` - Home ID
    /// * `user_id` - User ID
    /// * `is_favorite` - New favorite status
    /// # Returns
    /// * `Result<DomainHome, Error>` - Updated home or error
    pub async fn set_favorite_home(
        db: &Db,
        home_id: Uuid,
        user_id: Uuid,
        is_favorite: bool,
    ) -> Result<DomainHome, Error> {
        let result = sqlx::query!(
            "UPDATE user_homes SET is_favorite = $1 WHERE home_id = $2 AND user_id = $3",
            is_favorite,
            home_id,
            user_id,
        )
        .execute(&db.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(Error::EntityNotFound);
        }

        let home = sqlx::query_as!(
            Home,
            r#"
            SELECT id, name, address, token, tibber_token
            FROM homes
            WHERE id = $1
            "#,
            home_id,
        )
        .fetch_one(&db.pool)
        .await?;

        Ok(DomainHome {
            id: home.id,
            name: home.name,
            address: home.address,
            write_token: home.token,
            is_favorite,
        })
    }

    /// Delete a home by ID
    /// # Arguments
    /// * `db` - Database connection
    /// * `home_id` - Home ID to delete
    /// # Returns
    /// * `Result<(), Error>` - Ok if deleted, error otherwise
    pub async fn delete_home(db: &Db, home_id: Uuid) -> Result<(), Error> {
        let result = sqlx::query!("DELETE FROM homes WHERE id = $1", home_id,)
            .execute(&db.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(Error::EntityNotFound);
        }

        Ok(())
    }
}
