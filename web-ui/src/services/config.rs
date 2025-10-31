//! Backend configuration and endpoint management
//!
//! This module manages backend service endpoints and configuration
//! with environment-aware settings and security controls.

use serde::{Deserialize, Serialize};

/// Backend service endpoints configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendConfig {
    /// API service base URL
    pub api_url: String,
    /// Indexer service base URL  
    pub indexer_url: String,
    /// MEV monitor service URL
    pub mev_monitor_url: String,
    /// IPFS service URL
    pub ipfs_url: String,
    /// Keepers service URL
    pub keepers_url: String,
    /// WebSocket base URL
    pub ws_url: String,
    /// API timeout in milliseconds
    pub timeout_ms: u64,
    /// Maximum retry attempts
    pub max_retries: u32,
}

impl BackendConfig {
    /// Create development configuration
    pub fn development() -> Self {
        Self {
            api_url: "http://localhost:3000".to_string(),
            indexer_url: "http://localhost:3001".to_string(),
            mev_monitor_url: "http://localhost:3002".to_string(),
            ipfs_url: "http://localhost:5001".to_string(),
            keepers_url: "http://localhost:3003".to_string(),
            ws_url: "ws://localhost:3000".to_string(),
            timeout_ms: 30000, // 30 seconds
            max_retries: 3,
        }
    }
    
    /// Create production configuration
    pub fn production() -> Self {
        Self {
            api_url: "https://api.defi-platform.com".to_string(),
            indexer_url: "https://indexer.defi-platform.com".to_string(),
            mev_monitor_url: "https://mev.defi-platform.com".to_string(),
            ipfs_url: "https://ipfs.defi-platform.com".to_string(),
            keepers_url: "https://keepers.defi-platform.com".to_string(),
            ws_url: "wss://api.defi-platform.com".to_string(),
            timeout_ms: 10000, // 10 seconds
            max_retries: 5,
        }
    }
    
    /// Create test configuration
    pub fn test() -> Self {
        Self {
            api_url: "http://localhost:8080".to_string(),
            indexer_url: "http://localhost:8081".to_string(),
            mev_monitor_url: "http://localhost:8082".to_string(),
            ipfs_url: "http://localhost:8083".to_string(),
            keepers_url: "http://localhost:8084".to_string(),
            ws_url: "ws://localhost:8080".to_string(),
            timeout_ms: 5000,
            max_retries: 2,
        }
    }
    
    /// Get configuration based on environment
    pub fn from_environment() -> Self {
        // In a real implementation, this would read from environment variables
        // For now, we default to development
        #[cfg(debug_assertions)]
        {
            Self::development()
        }
        
        #[cfg(not(debug_assertions))]
        {
            Self::production()
        }
    }
    
    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        // Security: Ensure URLs are valid
        if self.api_url.is_empty() {
            return Err("API URL cannot be empty".to_string());
        }
        
        if !self.api_url.starts_with("http://") && !self.api_url.starts_with("https://") {
            return Err("API URL must start with http:// or https://".to_string());
        }
        
        if !self.ws_url.starts_with("ws://") && !self.ws_url.starts_with("wss://") {
            return Err("WebSocket URL must start with ws:// or wss://".to_string());
        }
        
        // Security: Timeout must be reasonable
        if self.timeout_ms == 0 || self.timeout_ms > 300000 {
            return Err("Timeout must be between 1ms and 5 minutes".to_string());
        }
        
        // Security: Retry attempts must be limited
        if self.max_retries > 10 {
            return Err("Maximum retries cannot exceed 10".to_string());
        }
        
        Ok(())
    }
}

impl Default for BackendConfig {
    fn default() -> Self {
        Self::from_environment()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_development_config() {
        let config = BackendConfig::development();
        assert_eq!(config.api_url, "http://localhost:3000");
        assert_eq!(config.timeout_ms, 30000);
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_production_config() {
        let config = BackendConfig::production();
        assert!(config.api_url.starts_with("https://"));
        assert!(config.ws_url.starts_with("wss://"));
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_config_validation() {
        let mut config = BackendConfig::development();
        
        // Test empty URL
        config.api_url = String::new();
        assert!(config.validate().is_err());
        
        // Test invalid protocol
        config.api_url = "ftp://invalid.com".to_string();
        assert!(config.validate().is_err());
        
        // Test invalid timeout
        config.api_url = "http://valid.com".to_string();
        config.timeout_ms = 0;
        assert!(config.validate().is_err());
        
        config.timeout_ms = 400000;
        assert!(config.validate().is_err());
        
        // Test invalid retries
        config.timeout_ms = 5000;
        config.max_retries = 15;
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_test_config() {
        let config = BackendConfig::test();
        assert_eq!(config.api_url, "http://localhost:8080");
        assert_eq!(config.max_retries, 2);
    }
}
