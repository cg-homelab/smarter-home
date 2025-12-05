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
pub mod user;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Smart Home API",
        version = "1.0.0",
        description = "API documentation for the Smart Home API.",
    ),
    paths(
        // Auth
        auth::log_in,
        auth::sign_up,
        // User
        user::get_me,
        // Home
        home::post_home,
        home::get_homes,
        // Power
        power::post::post_power_metric,
        power::get::get_metrics_for_period,
    ))]
struct ApiDoc;

#[derive(Clone)]
pub struct AppState {
    pub db: lib_db::Db,
}

pub fn create_router(db: lib_db::Db) -> axum::Router {
    let app_state = AppState { db };

    let base_routes: axum::Router = axum::Router::new()
        .route("/health", get(|| async { "healthy" }))
        .route("/openapi.json", get(openapi_json))
        .route("/docs", get(swagger_ui));

    let auth_routes: axum::Router = axum::Router::new()
        .route("/auth/login", post(auth::log_in))
        .route("/auth/signup", post(auth::sign_up))
        .with_state(app_state.clone());

    let user_routes: axum::Router = axum::Router::new()
        .route("/user/me", get(user::get_me))
        .with_state(app_state.clone());

    let power_routes: axum::Router = axum::Router::new()
        .route("/power", post(power::post::post_power_metric))
        .route("/power/metrics", get(power::get::get_metrics_for_period))
        .with_state(app_state.clone());

    let home_routes: axum::Router = axum::Router::new()
        .route("/home", post(home::post_home))
        .route("/home", get(home::get_homes))
        // .route("/home", get(home::get_homes))
        .with_state(app_state.clone());

    axum::Router::new()
        // Health check endpoint
        .merge(base_routes)
        .merge(auth_routes)
        .merge(user_routes)
        .merge(home_routes)
        .merge(power_routes)
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
