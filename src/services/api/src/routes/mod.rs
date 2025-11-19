// use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};
use tower_http::trace::TraceLayer;

pub mod auth;
pub mod home;
pub mod power;

#[derive(Clone)]
pub struct AppState {
    pub db: lib_db::Db,
}

pub fn create_router(db: lib_db::Db) -> axum::Router {
    let app_state = AppState { db };

    // let authorized_routes: axum::Router = axum::Router::new()
    //     .route("/home", post(home::post_home))
    //     // .route("/home", get(home::get_homes))
    //     // Apply JWT authentication middleware to protected routes
    //     .layer(from_fn_with_state(app_state.clone(), auth::jwt_auth))
    //     .with_state(app_state.clone());

    let unauthorized_routes: axum::Router = axum::Router::new()
        .route("/home", post(home::post_home))
        .route("/power", post(power::post::post_power_metric))
        // Health check endpoint
        .route("/health", get(|| async { "healthy" }))
        .route("/user/login", post(auth::log_in))
        .route("/user/signup", post(auth::sign_up))
        .with_state(app_state);

    axum::Router::new()
        // Health check endpoint
        // .merge(authorized_routes)
        .merge(unauthorized_routes)
        .layer(TraceLayer::new_for_http())
}
