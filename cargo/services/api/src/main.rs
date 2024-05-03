use axum::routing::{get, post};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::Level;

mod api;

#[tokio::main]
async fn main() {
    // Add logging
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    // Set up db connection
    let pool = lib_database::init().await;
    tracing::debug!("Connected to database");

    // Create tcp listener
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // Set up app with routing
    let app = axum::Router::new()
        // Health check endpoint
        .route("/health", get(|| async { "healthy" }))
        // Power endpoints
        .route("/power", post(api::power::post_power_metric))
        // TODO: User endpoints
        .route("/user", get(|| async { "todo" }))
        // TODO: Add Home endpoints
        .route("/home", get(|| async { "todo" }))
        // Add request logging to app
        .layer(TraceLayer::new_for_http())
        // Bind postgres connection pool to app
        .with_state(pool);

    //Log Startup
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    // Start Tcp app on port
    axum::serve(listener, app).await.unwrap();
}
