use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use super::{from_diesel_error, ModelManager};
use crate::error::ApiError;
use crate::schema::users;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

impl ModelManager {
    pub fn create_user<'a>(&self, user: NewUser) -> Result<User, ApiError> {
        let mut conn = self.establish_connection()?;

        Ok(diesel::insert_into(users::table)
            .values(&user)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .map_err(from_diesel_error)?)
    }
}
