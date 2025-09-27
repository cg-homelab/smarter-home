use lib_models::{domain, entity, error::Error};
use lib_utils::crypto;

pub async fn insert_user(
    id: &str,
    model: domain::user::NewUser,
) -> Result<domain::user::User, Error> {
    let hash_result = crypto::hash_password(&model.password).await?;
    let new_user =
        entity::user::NewUser::from_new_domain(model, hash_result.hashed, hash_result.salt);

    let created: Option<entity::user::User> = crate::DB
        .create((entity::USER_TABLE, id))
        .content(new_user)
        .await?;
    dbg!(&created);
    match &created {
        Some(c) => Ok(c.to_domain()),
        None => Err(Error::DbReturnedNoRows),
    }
}
