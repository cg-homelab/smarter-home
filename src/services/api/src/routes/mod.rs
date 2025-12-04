// use axum::middleware::from_fn_with_state;
use axum::{
    response::{Html, IntoResponse},
    routing::{get, post},
};
use scalar_api_reference::scalar_html;
use serde_json::json;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;

pub mod auth;
pub mod home;
pub mod power;

#[derive(OpenApi)]
#[openapi(paths(
    // Auth
    auth::log_in,
    auth::sign_up,
    // Home
    home::post_home,
    // Power
    power::post::post_power_metric,
))]
struct ApiDoc;

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

    let routes: axum::Router = axum::Router::new()
        .route("/openapi.json", get(openapi_json))
        .route("/docs", get(swagger_ui))
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
        .merge(routes)
        .layer(TraceLayer::new_for_http())
}

pub async fn openapi_json() -> impl IntoResponse {
    ApiDoc::openapi().to_pretty_json().unwrap()
}

pub async fn swagger_ui() -> impl IntoResponse {
    // Generate HTML with configuration using CDN
    let configuration = json!({
        "url": "/openapi.json",
        "theme": "purple"
    });

    // Using CDN fallback
    let html = scalar_html(&configuration, None);

    Html(html)
}
