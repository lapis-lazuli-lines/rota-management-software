use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use crate::{
    database::DbPool,
    error::AppError,
    models::user::{CreateUserRequest, User, UserResponse, AdminUserResponse},
};

pub fn db_user_routes() -> Router {
    Router::new()
        .route("/db/users", post(create_user).get(list_users))
        .route("/db/users/:id", get(get_user_by_id))
        .route("/db/users/:id/admin", get(admin_user_details))
}

// Handler to create a new user
async fn create_user(
    Extension(pool): Extension<DbPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    // Validate user input
    if payload.name.is_empty() {
        return Err(AppError::BadRequest("Name cannot be empty".to_string()));
    }
    
    if payload.email.is_empty() {
        return Err(AppError::BadRequest("Email cannot be empty".to_string()));
    }
    
    if !payload.email.contains('@') {
        return Err(AppError::BadRequest("Invalid email format".to_string()));
    }
    
    let user = User::create(&pool, payload).await?;
    let response = user.into_response();
    
    Ok((StatusCode::CREATED, Json(response)))
}

// Handler to list all users
async fn list_users(
    Extension(pool): Extension<DbPool>,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    let users = User::list_all(&pool).await?;
    
    let responses = users.into_iter()
        .map(|user| user.into_response())
        .collect();
    
    Ok(Json(responses))
}

// Handler to get a user by ID
async fn get_user_by_id(
    Extension(pool): Extension<DbPool>,
    Path(id): Path<i32>,
) -> Result<Json<UserResponse>, AppError> {
    let user = User::find_by_id(&pool, id).await?
        .ok_or(AppError::NotFound)?;
    
    Ok(Json(user.into_response()))
}

// Example of an admin-only endpoint that uses the Unauthorized and Forbidden errors
async fn admin_user_details(
    Extension(pool): Extension<DbPool>,
    Path(id): Path<i32>,
    headers: axum::http::HeaderMap,
) -> Result<Json<AdminUserResponse>, AppError> {
    // Check for authorization header (simplified example)
    let auth_header = headers.get("Authorization")
        .ok_or(AppError::Unauthorized)?
        .to_str()
        .map_err(|_| AppError::Unauthorized)?;
    
    // Very simplified auth check - in a real app, you'd verify a JWT or session
    if !auth_header.starts_with("Bearer admin-token") {
        return Err(AppError::Forbidden);
    }
    
    // Get the user
    let user = User::find_by_id(&pool, id).await?
        .ok_or(AppError::NotFound)?;
    
    Ok(Json(user.into_admin_response()))
}