use lib_models::{domain, entity, error::Error};

use crate::DB;

pub async fn insert_home(
    id: &str,
    model: domain::home::NewHome,
) -> Result<domain::home::Home, Error> {
    let new_home = entity::home::NewHome::from_new_domain(model);

    let created: Option<entity::home::Home> = DB
        .create((entity::HOME_TABLE, id))
        .content(new_home)
        .await?;

    dbg!(&created);

    match &created {
        Some(c) => Ok(c.to_domain()),
        None => Err(Error::DbReturnedNoRows),
    }
}

pub async fn get_homes() -> Result<Vec<domain::home::Home>, Error> {
    let homes: Vec<entity::home::Home> = DB.select(entity::HOME_TABLE).await?;
    let domain_homes = homes.into_iter().map(|h| h.to_domain()).collect();
    Ok(domain_homes)
}
