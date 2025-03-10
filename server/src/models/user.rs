use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use bcrypt::{hash, verify, DEFAULT_COST};

// User roles
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UserRole {
    User,
    Admin,
}

impl Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::User => write!(f, "user"),
            UserRole::Admin => write!(f, "admin"),
        }
    }
}

// User model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)] // Don't include password hash in serialized output
    pub password_hash: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    // Create a new user with a hashed password
    pub fn new(
        id: String,
        username: String, 
        email: String, 
        password: &str,
        role: UserRole
    ) -> Result<Self, bcrypt::BcryptError> {
        // Hash the password with bcrypt
        let password_hash = hash(password, DEFAULT_COST)?;
        
        Ok(Self {
            id,
            username,
            email,
            password_hash,
            role,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    // Verify a password against the stored hash
    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password_hash).unwrap_or(false)
    }
}

// For user data to return in responses (excludes sensitive information)
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            role: user.role,
            created_at: user.created_at,
        }
    }
}