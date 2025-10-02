use chrono::{DateTime, Utc};
use lib_models::domain::user::{AuthUser, DomainUser, NewDomainUser};
use lib_models::error::Error;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

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
    pub async fn check_exists(db: &PgPool, new_user: &NewDomainUser) -> Result<bool, Error> {
        // Check if user with email already exists
        let result = sqlx::query!("SELECT id FROM users WHERE email = $1", new_user.email)
            .fetch_all(db)
            .await?;
        if result.is_empty() {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    pub async fn insert(db: &PgPool, new_user: &NewDomainUser) -> Result<DomainUser, Error> {
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
        .fetch_one(db)
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
        );

        Ok(DomainUser {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            homes: None,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
    }

    pub async fn auth_user(
        db: &PgPool,
        auth_user: &AuthUser,
    ) -> Result<bool, lib_models::error::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE email = $1",
            auth_user.email
        )
        .fetch_one(db)
        .await?;

        Ok(lib_utils::crypto::verify_password(
            auth_user.password.as_str(),
            &user.password_hash,
        )?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserSalts {
    user_id: Uuid,
    salt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserSessions {
    session_id: Uuid,
    user_id: Uuid,
    token: String,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

struct UserHomes {
    user_id: Uuid,
    home_id: Uuid,
    role: String,
    added_at: DateTime<Utc>,
}
