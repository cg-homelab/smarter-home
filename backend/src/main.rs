use dotenvy::dotenv;
pub mod error;
pub mod models;
pub mod repository;

#[tokio::main()]
async fn main() {
    dotenv().ok();

    println!("Hello, world!");
}
