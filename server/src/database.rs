use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;
use std::time::Duration;

pub type DbPool = Pool<Postgres>;

pub async fn create_db_pool() -> Result<DbPool, sqlx::Error> {
    // Get the DATABASE_URL from environment variables
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable is required");

    tracing::info!("Connecting to database...");
    
    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await?;
    
    // Test the connection
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await?;
    
    tracing::info!("Database connection successful");
    
    // For Supabase, we'll skip automated migrations
    // You should manage your schema through Supabase's UI or SQL editor
    
    Ok(pool)
}