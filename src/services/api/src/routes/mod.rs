use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};
use tower_http::trace::TraceLayer;

pub mod auth;
pub mod home;
pub mod power;

#[derive(Clone)]
pub struct AppState {
    pub encoding_key: String,
}

pub fn create_router() -> axum::Router {
    let app_state = AppState {
        encoding_key: std::env::var("AUTH_SECRET").unwrap(),
    };

    let autherized_routes: axum::Router = axum::Router::new()
        .route("/home/{id}", post(home::post_home))
        .route("/home", get(home::get_homes))
        // Apply JWT authentication middleware to protected routes
        .layer(from_fn_with_state(app_state.clone(), auth::jwt_auth));

    let unautherized_routes: axum::Router = axum::Router::new()
        .route("/power", post(power::post::post_power_metric))
        // Health check endpoint
        .route("/health", get(|| async { "healthy" }))
        .route("/user/signup", post(auth::sign_up))
        .with_state(app_state);

    axum::Router::new()
        // Health check endpoint
        .merge(autherized_routes)
        .merge(unautherized_routes)
        .layer(TraceLayer::new_for_http())
}
