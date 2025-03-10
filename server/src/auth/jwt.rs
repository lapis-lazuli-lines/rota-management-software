use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode, header::{AUTHORIZATION, HeaderMap}},
    response::{IntoResponse, Response},
    Json,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Display;

// JWT secret key for encoding and decoding tokens
// In a production environment, this should be loaded from environment variables
static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "your_jwt_secret_key".to_string())
});

// Define token expiration times
const ACCESS_TOKEN_EXPIRATION: i64 = 15; // minutes
const REFRESH_TOKEN_EXPIRATION: i64 = 60 * 24 * 7; // 7 days in minutes

// Claims structure for JWT payload
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,         // Subject (usually user_id)
    pub role: String,        // User role
    pub exp: i64,            // Expiration time
    pub iat: i64,            // Issued at
    pub token_type: String,  // Token type: "access" or "refresh"
}

// Token types
#[derive(Debug, Serialize, Deserialize)]
pub enum TokenType {
    Access,
    Refresh,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Access => write!(f, "access"),
            TokenType::Refresh => write!(f, "refresh"),
        }
    }
}

// Response for authentication endpoints
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

// Custom error for authentication failures
#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    Expired,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthError::Expired => (StatusCode::UNAUTHORIZED, "Token has expired"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

// Extract Bearer token from Authorization header
fn extract_token_from_header(headers: &HeaderMap) -> Result<String, AuthError> {
    let header = headers
        .get(AUTHORIZATION)
        .ok_or(AuthError::MissingCredentials)?;
    
    let auth_value = header
        .to_str()
        .map_err(|_| AuthError::MissingCredentials)?;
    
    if !auth_value.starts_with("Bearer ") {
        return Err(AuthError::InvalidToken);
    }
    
    Ok(auth_value[7..].to_string())
}

// Generate a new JWT token
pub fn generate_token(user_id: &str, role: &str, token_type: TokenType) -> Result<String, AuthError> {
    let expiration = match token_type {
        TokenType::Access => ACCESS_TOKEN_EXPIRATION,
        TokenType::Refresh => REFRESH_TOKEN_EXPIRATION,
    };

    let now = Utc::now();
    let expires_at = now + Duration::minutes(expiration);

    let claims = Claims {
        sub: user_id.to_owned(),
        role: role.to_owned(),
        exp: expires_at.timestamp(),
        iat: now.timestamp(),
        token_type: token_type.to_string(),
    };

    let header = Header::default();
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .map_err(|_| AuthError::TokenCreation)
}

// Create tokens for successful authentication
pub fn create_tokens(user_id: &str, role: &str) -> Result<AuthResponse, AuthError> {
    let access_token = generate_token(user_id, role, TokenType::Access)?;
    let refresh_token = generate_token(user_id, role, TokenType::Refresh)?;

    Ok(AuthResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: ACCESS_TOKEN_EXPIRATION * 60, // Convert minutes to seconds
    })
}

// Validate JWT token and extract claims
pub fn validate_token(token: &str, expected_token_type: Option<TokenType>) -> Result<Claims, AuthError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| {
        match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::Expired,
            _ => AuthError::InvalidToken,
        }
    })?;

    // Check token type if specified
    if let Some(expected_type) = expected_token_type {
        if token_data.claims.token_type != expected_type.to_string() {
            return Err(AuthError::InvalidToken);
        }
    }

    Ok(token_data.claims)
}

// Extractor for JWT from Authorization header
#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let token = extract_token_from_header(&parts.headers)?;
        
        // Validate the token
        let claims = validate_token(&token, Some(TokenType::Access))?;
        
        Ok(claims)
    }
}