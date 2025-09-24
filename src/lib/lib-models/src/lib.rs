pub mod domain;
pub mod entity;
pub mod error;

pub struct HashedPassword {
    pub hash: String,
    pub salt: String,
}
