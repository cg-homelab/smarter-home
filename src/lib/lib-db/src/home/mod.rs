use crate::DatabaseState;
use lib_models::entity::home::{Home, NewHome};
use lib_models::error::Error;

// async fn check_home_exists(address: &str, db: &DatabaseState) -> Result<bool, Error> {
//     let doc = doc! { "address": address.to_string() };
//     let count = db
//         .mongo
//         .database(crate::MONGO_DB)
//         .collection::<Home>("homes")
//         .count_documents(doc)
//         .await?;
//     Ok(count > 0)
// }
//
// pub async fn create_home(db: DatabaseState, input: NewHome) -> Result<Home, Error> {
//     let home = Home::from_new(input);
//
//     if check_home_exists(&home.address, &db).await? {
//         return Err(Error::Conflict(format!(
//             "Home with address {} already exists",
//             &home.address
//         )));
//     }
//
//     db.mongo
//         .database(crate::MONGO_DB)
//         .collection::<Home>("homes")
//         .insert_one(home.clone())
//         .await?;
//     Ok(home)
// }
//
// pub async fn get_homes(db: &DatabaseState, ids: Vec<String>) -> Result<Vec<Home>, Error> {
//     let doc = doc! { "id": { "$in": ids } };
//
//     let mut cursor = db
//         .mongo
//         .database(crate::MONGO_DB)
//         .collection("homes")
//         .find(doc)
//         .await?;
//
//     let mut homes: Vec<Home> = Vec::new();
//
//     while let Some(home) =  cursor.advance().await? {
//         ho
//         homes.push(home);
//     }
//     Ok(homes)
// }
