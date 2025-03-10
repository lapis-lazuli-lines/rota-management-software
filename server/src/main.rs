mod config;
mod database;
mod error;
mod middleware;
mod models;
mod routes;
#[cfg(test)]
mod tests;

use axum::Extension;
use dotenv::dotenv;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Load .env file if it exists
    dotenv().ok();
    
    // Load configuration
    let config = config::Config::from_env();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| config.log_level.clone()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize database connection
    let db_pool = match database::create_db_pool().await {
        Ok(pool) => {
            tracing::info!("Connected to database");
            pool
        },
        Err(e) => {
            tracing::error!("Failed to connect to database: {:?}", e);
            std::process::exit(1);
        }
    };

    // Build our application router
    let app = routes::app_routes()
        .layer(Extension(db_pool))
        .layer(axum::middleware::from_fn(middleware::request_id::request_id_middleware))
        .layer(TraceLayer::new_for_http());

    // Run the server
    let addr = config.socket_addr();
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("server started successfully");
    axum::serve(listener, app).await.unwrap();
}