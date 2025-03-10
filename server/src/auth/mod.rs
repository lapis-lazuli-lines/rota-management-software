pub mod handlers;
pub mod jwt;

// Re-export commonly used functions
pub use handlers::{login, register, refresh_token, protected, logout};