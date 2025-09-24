use axum::routing::{get, post};
use tower_http::trace::TraceLayer;

pub mod home;
pub mod power;
pub async fn paths() -> &'static str {
    r#"
-----------------------------------------------------------------------------------------------------------------------------------------
        PATH                |           SAMPLE COMMAND                                                                                  
-----------------------------------------------------------------------------------------------------------------------------------------
/session: See session data  |  curl -X GET    -H "Content-Type: application/json"                      http://localhost:8080/session
                            |
/person/{id}:               |
  Create a person           |  curl -X POST   -H "Content-Type: application/json" -d '{"name":"John"}' http://localhost:8080/person/one
  Update a person           |  curl -X PUT    -H "Content-Type: application/json" -d '{"name":"Jane"}' http://localhost:8080/person/one
  Get a person              |  curl -X GET    -H "Content-Type: application/json"                      http://localhost:8080/person/one
  Delete a person           |  curl -X DELETE -H "Content-Type: application/json"                      http://localhost:8080/person/one
                            |
/people: List all people    |  curl -X GET    -H "Content-Type: application/json"                      http://localhost:8080/people

/new_user:  Create a new record user
/new_token: Get instructions for a new token if yours has expired"#
}

pub fn create_router() -> axum::Router {
    axum::Router::new()
        .route("/", get(paths))
        // Health check endpoint
        .route("/health", get(|| async { "healthy" }))
        // Power endpoints
        .route("/power", post(power::post::post_power_metric))
        // TODO: User endpoints
        .route("/user", get(|| async { "todo" }))
        // TODO: Add Home endpoints
        .route("/home/{id}", post(home::post_home))
        .route("/home", get(home::get_homes))
        // Add request logging to app
        .layer(TraceLayer::new_for_http())
}
