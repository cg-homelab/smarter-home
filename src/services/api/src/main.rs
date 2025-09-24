use surrealdb::{engine::remote::ws::Client, Surreal};
use tokio::net::TcpListener;
use tracing::Level;

pub mod error;
mod routes;
pub mod store;

use std::sync::LazyLock;
static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    // Add logging
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    // Set up server address from env var or default to 3001
    let address = "0.0.0.0:".to_string()
        + std::env::var("BACKEND_PORT")
            .unwrap_or_else(|_| "3001".to_string())
            .as_str();

    // Set up db connection
    store::init().await?;

    // Log db connected
    tracing::debug!("Connected to database");

    // Create tcp listener
    let listener = TcpListener::bind(address.as_str()).await.unwrap();

    // Set up app with routing
    let router = routes::create_router();

    //Log Startup
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    // Start Tcp app on port
    axum::serve(listener, router)
        .await
        .map_err(|_| error::Error::AxumServerError)?;

    Ok(())
}
