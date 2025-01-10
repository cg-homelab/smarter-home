use axum::{
    extract::FromRef,
    routing::{get, post},
};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

pub mod power;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: PgPool,
}

pub fn create_router(db: PgPool) -> axum::Router {
    let state = AppState { db };

    axum::Router::new()
        // Health check endpoint
        .route("/health", get(|| async { "healthy" }))
        // Power endpoints
        .route("/power", post(power::post::post_power_metric))
        // TODO: User endpoints
        .route("/user", get(|| async { "todo" }))
        // TODO: Add Home endpoints
        .route("/home", get(|| async { "todo" }))
        // Add request logging to app
        .layer(TraceLayer::new_for_http())
        // Bind postgres connection pool to app
        .with_state(state)
}
