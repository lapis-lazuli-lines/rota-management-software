mod auth;
mod models;

use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing for logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Build our application with routes
    let app = Router::new()
        .route("/", get(health_check))
        // Public authentication routes
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/refresh", post(auth::refresh_token))
        .route("/api/auth/logout", post(auth::logout))
        // Protected routes
        .route("/api/protected", get(auth::protected))
        // Add tracing middleware
        .layer(TraceLayer::new_for_http());

    // Define the address to run the server on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server listening on {}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening on {}", addr);
    axum::Server::from_tcp(listener.into_std().unwrap())
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Health check endpoint
async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}