use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::error::AppError;

// In-memory storage for users (for demonstration purposes)
// In a real application, this would be replaced with a database
pub struct AppState {
    users: Mutex<Vec<User>>,
}

// User models
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<usize>,
    pub name: String,
    pub email: String,
    pub role: Option<String>,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: usize,
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct AdminUserResponse {
    pub id: usize,
    pub name: String,
    pub email: String,
    pub role: String,
    pub created_at: String,
    pub last_login: Option<String>,
}

// Initialize router with user-related routes
pub fn user_routes() -> Router {
    let app_state = Arc::new(AppState {
        users: Mutex::new(Vec::new()),
    });

    Router::new()
        .route("/users", post(create_user).get(list_users))
        .route("/users/:id", get(get_user_by_id))
        .route("/users/:id/admin", get(admin_user_details))
        .with_state(app_state)
}

// Handler to create a new user
async fn create_user(
    state: axum::extract::State<Arc<AppState>>,
    Json(payload): Json<User>,
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
    
    let mut users = state.users.lock().unwrap();
    
    // Check if email already exists
    if users.iter().any(|u| u.email == payload.email) {
        return Err(AppError::BadRequest("Email already in use".to_string()));
    }
    
    // Generate a new ID
    let id = users.len() + 1;
    
    // Create the new user
    let user = User {
        id: Some(id),
        name: payload.name,
        email: payload.email,
        role: None,
    };
    
    // Add to our in-memory store
    users.push(user.clone());
    
    // Return the created user
    let response = UserResponse {
        id,
        name: user.name,
        email: user.email,
    };
    
    Ok((StatusCode::CREATED, Json(response)))
}

// Handler to list all users
async fn list_users(
    state: axum::extract::State<Arc<AppState>>,
) -> Json<Vec<UserResponse>> {
    let users = state.users.lock().unwrap();
    
    let response: Vec<UserResponse> = users
        .iter()
        .map(|user| UserResponse {
            id: user.id.unwrap(),
            name: user.name.clone(),
            email: user.email.clone(),
        })
        .collect();
    
    Json(response)
}

// Handler to get a user by ID
async fn get_user_by_id(
    state: axum::extract::State<Arc<AppState>>,
    Path(id): Path<usize>,
) -> Result<Json<UserResponse>, AppError> {
    let users = state.users.lock().unwrap();
    
    let user = users
        .iter()
        .find(|u| u.id == Some(id))
        .ok_or(AppError::NotFound)?;
    
    let response = UserResponse {
        id: user.id.unwrap(),
        name: user.name.clone(),
        email: user.email.clone(),
    };
    
    Ok(Json(response))
}

// Example of an admin-only endpoint that uses the Unauthorized and Forbidden errors
async fn admin_user_details(
    state: axum::extract::State<Arc<AppState>>,
    Path(id): Path<usize>,
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
    let users = state.users.lock().unwrap();
    let user = users
        .iter()
        .find(|u| u.id == Some(id))
        .ok_or(AppError::NotFound)?;
    
    // Create an admin response with extra fields
    let response = AdminUserResponse {
        id: user.id.unwrap(),
        name: user.name.clone(),
        email: user.email.clone(),
        role: user.role.clone().unwrap_or_else(|| "user".to_string()),
        created_at: "2024-03-10T12:00:00Z".to_string(), // In a real app, this would be a timestamp
        last_login: Some("2024-03-10T12:30:00Z".to_string()),
    };
    
    Ok(Json(response))
}