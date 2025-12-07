use crate::Db;
use chrono::{DateTime, Utc};
use lib_models::Role;
use lib_models::domain::user::{AuthUser, DomainUser, NewDomainUser};
use lib_models::error::Error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User struct representing a user in the database
/// # Fields
/// * `id` - User ID
/// * `first_name` - First name
/// * `last_name` - Last name
/// * `email` - Email
/// * `password_hash` - Hashed password
/// * `role` - User role
/// * `created_at` - Creation timestamp
/// * `updated_at` - Update timestamp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    password_hash: String,
    role: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
impl User {
    /// Check if a user with the given email already exists
    /// # Arguments
    /// * `db` - Database connection
    /// * `new_user` - New user data
    /// # Returns
    /// * `Result<bool, Error>` - True if user exists, false otherwise
    pub async fn check_exists(db: &Db, new_user: &NewDomainUser) -> Result<bool, Error> {
        // Check if user with email already exists
        let result = sqlx::query!("SELECT id FROM users WHERE email = $1", new_user.email)
            .fetch_all(&db.pool)
            .await?;
        if result.is_empty() {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    /// Insert a new user into the database
    /// # Arguments
    /// * `db` - Database connection
    /// * `new_user` - New user data
    /// # Returns
    /// * `Result<DomainUser, Error>` - Inserted user or error
    pub async fn insert(db: &Db, new_user: &NewDomainUser) -> Result<DomainUser, Error> {
        // Hash the password
        let password_hash = lib_utils::crypto::hash_password(new_user.password.as_str())?;

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (first_name, last_name, email, password_hash)
            VALUES ($1, $2, $3, $4)
            RETURNING id, first_name, last_name, email, password_hash, role, created_at, updated_at
            "#,
            new_user.first_name,
            new_user.last_name,
            new_user.email,
            password_hash.hash,
        )
        .fetch_one(&db.pool)
        .await?;

        let _salts = sqlx::query_as!(
            UserSalts,
            r#"
            INSERT INTO user_salts (user_id, salt)
            VALUES ($1, $2)
            RETURNING user_id, salt
            "#,
            user.id,
            password_hash.salt
        )
        .fetch_one(&db.pool)
        .await?;

        Ok(DomainUser {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            role: Role::User.as_str().to_string(),
            homes: None,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
    }

    /// Get a user by ID
    /// # Arguments
    /// * `db` - Database connection
    /// * `id` - User ID
    /// # Returns
    /// * `Result<DomainUser, Error>` - Retrieved user or error
    pub async fn get(db: &Db, id: Uuid) -> Result<DomainUser, Error> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_one(&db.pool)
            .await?;

        let return_user = DomainUser {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            role: user.role,
            homes: None,
            created_at: user.created_at,
            updated_at: user.updated_at,
        };

        Ok(return_user)
    }

    /// Authenticate a user
    /// # Arguments
    /// * `db` - Database connection
    /// * `auth_user` - Authentication data
    /// # Returns
    /// * `Result<DomainUser, Error>` - Authenticated user or error
    pub async fn auth_user(db: &Db, auth_user: &AuthUser) -> Result<DomainUser, Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE email = $1",
            auth_user.email
        )
        .fetch_one(&db.pool)
        .await?;

        let return_user = DomainUser {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            role: user.role,
            homes: None,
            created_at: user.created_at,
            updated_at: user.updated_at,
        };
        match lib_utils::crypto::verify_password(auth_user.password.as_str(), &user.password_hash) {
            Ok(valid) => {
                if valid {
                    Ok(return_user)
                } else {
                    Err(Error::WrongPassword)
                }
            }
            Err(_) => Err(Error::InternalServerError),
        }
    }
}

/// UserSalts struct for storing user salt information.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserSalts {
    user_id: Uuid,
    salt: String,
}
