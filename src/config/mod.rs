use std::env;

/// Application-wide configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub redis_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub base_url: String,
    pub default_code_length: usize,
    pub max_custom_code_length: usize,
}

impl AppConfig {
    /// Loads configuration from environment variables.
    /// Panics on missing required variables — this is intentional:
    /// the app should not start with invalid configuration.
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("SERVER_PORT must be a valid u16"),
            base_url: env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string()),
            default_code_length: env::var("DEFAULT_CODE_LENGTH")
                .unwrap_or_else(|_| "7".to_string())
                .parse()
                .expect("DEFAULT_CODE_LENGTH must be a valid usize"),
            max_custom_code_length: env::var("MAX_CUSTOM_CODE_LENGTH")
                .unwrap_or_else(|_| "16".to_string())
                .parse()
                .expect("MAX_CUSTOM_CODE_LENGTH must be a valid usize"),
        }
    }

    /// Returns the full socket address for binding the server.
    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}
