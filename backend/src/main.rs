use dotenvy::dotenv;
pub mod error;
pub mod models;
pub mod schema;

#[tokio::main()]
async fn main() {
    dotenv().ok();

    println!("Hello, world!");
}
