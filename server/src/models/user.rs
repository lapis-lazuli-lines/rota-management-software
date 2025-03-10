use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use crate::database::DbPool;
use crate::error::AppError;

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub role: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminUserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub async fn find_by_id(pool: &DbPool, id: i32) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;

        Ok(user)
    }

    pub async fn find_by_email(pool: &DbPool, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;

        Ok(user)
    }

    pub async fn create(pool: &DbPool, request: CreateUserRequest) -> Result<User, AppError> {
        // Check if user with this email already exists
        if let Some(_) = Self::find_by_email(pool, &request.email).await? {
            return Err(AppError::BadRequest("Email already in use".to_string()));
        }

        let role = request.role.unwrap_or_else(|| "user".to_string());

        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (name, email, role) VALUES ($1, $2, $3) RETURNING *"
        )
        .bind(&request.name)
        .bind(&request.email)
        .bind(&role)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {:?}", e);
            AppError::InternalServerError
        })?;

        Ok(user)
    }

    pub async fn list_all(pool: &DbPool) -> Result<Vec<User>, AppError> {
        let users = sqlx::query_as::<_, User>(
            "SELECT * FROM users ORDER BY id"
        )
        .fetch_all(pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;

        Ok(users)
    }

    pub fn into_response(self) -> UserResponse {
        UserResponse {
            id: self.id,
            name: self.name,
            email: self.email,
            role: self.role,
            created_at: self.created_at,
        }
    }

    pub fn into_admin_response(self) -> AdminUserResponse {
        AdminUserResponse {
            id: self.id,
            name: self.name,
            email: self.email,
            role: self.role,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}