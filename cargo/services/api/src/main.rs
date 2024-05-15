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

    // Set up db connection
    let pool = store::init().await;

    // Log db connected
    tracing::debug!("Connected to database");

    // Create tcp listener
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // Set up app with routing
    let app = routes::create_router(pool);

    //Log Startup
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    // Start Tcp app on port
    axum::serve(listener, app).await.unwrap();
}
