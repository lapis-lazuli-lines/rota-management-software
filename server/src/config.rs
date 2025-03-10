use std::env;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: IpAddr,
    pub port: u16,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Self {
        let host = env::var("HOST")
            .unwrap_or_else(|_| "127.0.0.1".to_string())
            .parse()
            .expect("HOST must be a valid IP address");

        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .expect("PORT must be a valid number");

        let log_level = env::var("RUST_LOG")
            .unwrap_or_else(|_| "info".to_string());

        Self {
            host,
            port,
            log_level,
        }
    }

    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}

// Implement Default for testing purposes
impl Default for Config {
    fn default() -> Self {
        Self {
            host: IpAddr::from_str("127.0.0.1").unwrap(),
            port: 3000,
            log_level: "info".to_string(),
        }
    }
}