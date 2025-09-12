use tokio::net::TcpListener;
use tracing::Level;

pub mod error;
mod routes;
pub mod store;

#[tokio::main]
async fn main() {
    // Add logging
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let address = "0.0.0.0:".to_string()
        + std::env::var("BACKEND_PORT")
            .unwrap_or_else(|_| "3001".to_string())
            .as_str();
    // Set up db connection
    let pool = store::init().await;

    // Log db connected
    tracing::debug!("Connected to database");

    // Create tcp listener
    let listener = TcpListener::bind(address.as_str()).await.unwrap();

    // Set up app with routing
    let app = routes::create_router(pool);

    //Log Startup
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    // Start Tcp app on port
    axum::serve(listener, app).await.unwrap();
}
