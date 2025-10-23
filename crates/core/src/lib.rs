//! Core components for the decentralized application
//!
//! This crate contains shared components used across different modules of the dApp.

/// Common error types used across the application
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Json(serde_json::Error),
    Network(reqwest::Error),
    Ethereum(ethers::providers::ProviderError),
    Custom(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Json(e) => write!(f, "JSON error: {}", e),
            Error::Network(e) => write!(f, "Network error: {}", e),
            Error::Ethereum(e) => write!(f, "Ethereum error: {}", e),
            Error::Custom(e) => write!(f, "Custom error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Json(e) => Some(e),
            Error::Network(e) => Some(e),
            Error::Ethereum(e) => Some(e),
            Error::Custom(_) => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Json(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Network(error)
    }
}

impl From<ethers::providers::ProviderError> for Error {
    fn from(error: ethers::providers::ProviderError) -> Self {
        Error::Ethereum(error)
    }
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, Error>;

/// Common types used across the application
pub mod types {
    use serde::{Deserialize, Serialize};
    
    /// Represents a blockchain address
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub struct Address(pub String);
    
    /// Represents a token amount with decimals
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TokenAmount {
        pub value: u128,
        pub decimals: u8,
    }
}

/// Configuration management
pub mod config {
    use serde::{Deserialize, Serialize};
    use std::fs;
    
    /// Application configuration
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Config {
        pub rpc_url: String,
        pub chain_id: u64,
        pub database_url: String,
    }
    
    impl Config {
        /// Load configuration from a JSON file
        pub fn load(path: &str) -> crate::Result<Self> {
            let content = fs::read_to_string(path)?;
            let config: Config = serde_json::from_str(&content)?;
            Ok(config)
        }
    }
}

/// Logging and tracing utilities
pub mod logging {
    use tracing_subscriber::{fmt, EnvFilter};
    
    /// Initialize logging with default settings
    pub fn init() {
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));
            
        fmt::Subscriber::builder()
            .with_env_filter(filter)
            .init();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::Other, "test error");
        let error: Error = io_error.into();
        assert!(matches!(error, Error::Io(_)));
    }
}