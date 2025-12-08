use lib_models::error::Error;
use tokio::net::TcpListener;
use tracing::Level;

mod routes;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Add logging
    let filter = match std::env::var("LOG_LEVEL") {
        Ok(level) => match level.as_str() {
            "trace" => Level::TRACE,
            "debug" => Level::DEBUG,
            "info" => Level::INFO,
            "warn" => Level::WARN,
            _ => Level::WARN,
        },
        Err(_) => Level::ERROR,
    };
    tracing_subscriber::fmt().with_max_level(filter).init();

    // Set up server address from env var or default to 3001
    let address = "0.0.0.0:".to_string()
        + std::env::var("BACKEND_PORT")
            .unwrap_or_else(|_| "3001".to_string())
            .as_str();

    // Set up db connection
    let db = lib_db::Db::new().await?;

    // Log db connected
    tracing::info!("Connected to database");

    // Create tcp listener
    let listener = TcpListener::bind(address.as_str()).await.unwrap();

    // Set up app with routing
    let router = routes::create_router(db);

    //Log Startup
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    // Start Tcp app on port
    axum::serve(listener, router)
        .await
        .map_err(|_| Error::AxumServerError)?;

    Ok(())
}

// #[cfg(test)]
// mod tests {
//
//     #[test]
//     fn check_token_encode() {
//         use lib_utils::crypto::generate_jwt;
//
//         let token = generate_jwt("user123@test.com".to_string(), false);
//         assert!(!token.is_empty());
//     }
//
//     #[test]
//     fn check_token_decode() {
//         use lib_utils::crypto::{generate_jwt, validate_jwt};
//
//         let token = generate_jwt("user123@test.com".to_string(), false);
//         println!("Generated Token: {}", token);
//
//         let claims = validate_jwt(&token).unwrap();
//         assert_eq!(claims.sub, "user123@test.com");
//     }
// }
