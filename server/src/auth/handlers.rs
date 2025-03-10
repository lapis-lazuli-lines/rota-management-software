use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::auth::jwt::{
    validate_token, AuthError, Claims, TokenType, create_tokens,
};
use crate::models::user::{User, UserRole};

// Login request payload
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// Registration request payload
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

// Refresh token request payload
#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

// Generic response structure
#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

// User registration handler
pub async fn register(
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AuthError> {
    // This would typically interact with a database
    // For now, we'll simulate creating a user
    
    // In a real application, you would:
    // 1. Check if user already exists
    // 2. Hash the password (now done in User::new)
    // 3. Store user in database
    // 4. Generate tokens

    // Create a new user with a hashed password
    let user = User::new(
        uuid::Uuid::new_v4().to_string(),
        payload.username,
        payload.email,
        &payload.password,
        UserRole::User,
    ).map_err(|_| AuthError::TokenCreation)?; // Using TokenCreation as a general error

    // Generate tokens
    let token_response = create_tokens(&user.id, &user.role.to_string())?;

    // Return tokens
    Ok((StatusCode::CREATED, Json(token_response)))
}

// User login handler
pub async fn login(
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthError> {
    // This would typically:
    // 1. Query the database for the user
    // 2. Verify password hash
    // 3. Generate tokens if valid
    
    // For now, simulate a stored user
    // In a real application, you would query the database
    let stored_user = User::new(
        "user_123".to_string(),
        "testuser".to_string(),
        "test@example.com".to_string(),
        "password", // The expected password
        UserRole::User,
    ).map_err(|_| AuthError::TokenCreation)?;
    
    // Check if email matches
    if payload.email != stored_user.email {
        return Err(AuthError::WrongCredentials);
    }
    
    // Verify password using bcrypt
    if !stored_user.verify_password(&payload.password) {
        return Err(AuthError::WrongCredentials);
    }

    // Create tokens
    let token_response = create_tokens(&stored_user.id, &stored_user.role.to_string())?;

    // Return tokens
    Ok((StatusCode::OK, Json(token_response)))
}

// Token refresh handler
pub async fn refresh_token(
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<impl IntoResponse, AuthError> {
    // Validate the refresh token
    let claims = validate_token(&payload.refresh_token, Some(TokenType::Refresh))?;
    
    // Generate new tokens
    let token_response = create_tokens(&claims.sub, &claims.role)?;
    
    Ok((StatusCode::OK, Json(token_response)))
}

// Protected route example
pub async fn protected(claims: Claims) -> impl IntoResponse {
    // The Claims extractor ensures this route is protected
    // If we get here, the token is valid
    
    let message = format!(
        "Protected route accessed by user {} with role {}",
        claims.sub, claims.role
    );
    
    Json(MessageResponse { message })
}

// Logout handler - For JWT, this is typically handled client-side by removing the token
// However, you could implement a token blacklist for added security
pub async fn logout() -> impl IntoResponse {
    // In a real implementation, you might add the token to a blacklist
    // or invalidate sessions in a database
    
    (
        StatusCode::OK,
        Json(MessageResponse {
            message: "Successfully logged out".to_string(),
        }),
    )
}