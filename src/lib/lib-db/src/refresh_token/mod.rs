use crate::Db;
use chrono::{DateTime, Utc};
use lib_models::error::Error;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub replaced_by_id: Option<Uuid>,
}

impl RefreshToken {
    pub async fn create(
        db: &Db,
        user_id: Uuid,
        token_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<Self, Error> {
        let token = sqlx::query_as::<_, RefreshToken>(
            r#"
            INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, token_hash, created_at, expires_at, revoked_at, last_used_at, replaced_by_id
            "#,
        )
        .bind(user_id)
        .bind(token_hash)
        .bind(expires_at)
        .fetch_one(&db.pool)
        .await?;

        Ok(token)
    }

    pub async fn get_active_by_hash(db: &Db, token_hash: &str) -> Result<Self, Error> {
        let token = sqlx::query_as::<_, RefreshToken>(
            r#"
            SELECT id, user_id, token_hash, created_at, expires_at, revoked_at, last_used_at, replaced_by_id
            FROM refresh_tokens
            WHERE token_hash = $1
              AND revoked_at IS NULL
              AND expires_at > NOW()
            "#,
        )
        .bind(token_hash)
        .fetch_optional(&db.pool)
        .await?;

        match token {
            Some(token) => Ok(token),
            None => Err(Error::Unauthorized),
        }
    }

    pub async fn rotate(
        db: &Db,
        current_id: Uuid,
        new_token_hash: &str,
        new_expires_at: DateTime<Utc>,
    ) -> Result<Self, Error> {
        let mut tx = db.pool.begin().await?;

        let old_token = sqlx::query_as::<_, RefreshToken>(
            r#"
            SELECT id, user_id, token_hash, created_at, expires_at, revoked_at, last_used_at, replaced_by_id
            FROM refresh_tokens
            WHERE id = $1
              AND revoked_at IS NULL
              AND expires_at > NOW()
            FOR UPDATE
            "#,
        )
        .bind(current_id)
        .fetch_optional(&mut *tx)
        .await?;

        let old_token = match old_token {
            Some(token) => token,
            None => return Err(Error::Unauthorized),
        };

        let new_token = sqlx::query_as::<_, RefreshToken>(
            r#"
            INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, token_hash, created_at, expires_at, revoked_at, last_used_at, replaced_by_id
            "#,
        )
        .bind(old_token.user_id)
        .bind(new_token_hash)
        .bind(new_expires_at)
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = NOW(),
                last_used_at = NOW(),
                replaced_by_id = $2
            WHERE id = $1
            "#,
        )
        .bind(old_token.id)
        .bind(new_token.id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(new_token)
    }

    pub async fn revoke_by_hash(db: &Db, token_hash: &str) -> Result<(), Error> {
        sqlx::query(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = NOW(),
                last_used_at = NOW()
            WHERE token_hash = $1
              AND revoked_at IS NULL
            "#,
        )
        .bind(token_hash)
        .execute(&db.pool)
        .await?;

        Ok(())
    }
}
