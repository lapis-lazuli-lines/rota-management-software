pub mod users;
pub mod db_users;

use axum::{
    extract::Extension,
    http::StatusCode,
    routing::get,
    Router, response::IntoResponse,
};
use crate::middleware::request_id::RequestId;

// Root route handler
async fn root(Extension(request_id): Extension<RequestId>) -> impl IntoResponse {
    let id = &request_id.0; // Now we're using the field!
    format!("Welcome to our Axum API! Request ID: {}", id)
}

// Health check endpoint
async fn health_check() -> StatusCode {
    StatusCode::OK
}

// Combine all routes
pub fn app_routes() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .merge(users::user_routes())
        .merge(db_users::db_user_routes())
}