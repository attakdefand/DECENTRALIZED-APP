//! Throttle service
//!
//! This module provides rate limiting and throttling functionality for the web UI.

use gloo_storage::{LocalStorage, Storage};
use js_sys::Date;
use serde::{Deserialize, Serialize};

/// Rate limit information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RateLimitInfo {
    pub count: u32,
    pub last_reset: f64,
    pub limit: u32,
    pub window_ms: f64,
}

impl RateLimitInfo {
    /// Create new rate limit info
    pub fn new(limit: u32, window_ms: f64) -> Self {
        Self {
            count: 0,
            last_reset: Date::now(),
            limit,
            window_ms,
        }
    }

    /// Check if the rate limit window has expired
    pub fn is_window_expired(&self) -> bool {
        Date::now() > self.last_reset + self.window_ms
    }

    /// Reset the rate limit counter
    pub fn reset(&mut self) {
        self.count = 0;
        self.last_reset = Date::now();
    }

    /// Increment the counter
    pub fn increment(&mut self) {
        self.count += 1;
    }

    /// Check if the rate limit has been exceeded
    pub fn is_limited(&self) -> bool {
        self.count >= self.limit
    }

    /// Get remaining requests
    pub fn remaining(&self) -> u32 {
        if self.is_limited() {
            0
        } else {
            self.limit - self.count
        }
    }
}

/// Throttle service
pub struct ThrottleService {
    limits: std::collections::HashMap<String, RateLimitInfo>,
    storage_key_prefix: String,
}

impl ThrottleService {
    /// Create a new throttle service
    pub fn new() -> Self {
        Self {
            limits: std::collections::HashMap::new(),
            storage_key_prefix: "throttle_".to_string(),
        }
    }

    /// Configure a rate limit for a specific operation
    pub fn configure_limit(&mut self, operation: &str, limit: u32, window_ms: f64) {
        let key = format!("{}_{}", self.storage_key_prefix, operation);
        let limit_info = RateLimitInfo::new(limit, window_ms);
        
        // Try to load from storage first
        if let Ok(stored) = LocalStorage::get::<RateLimitInfo>(&key) {
            self.limits.insert(operation.to_string(), stored);
        } else {
            self.limits.insert(operation.to_string(), limit_info.clone());
            let _ = LocalStorage::set(&key, &limit_info);
        }
    }

    /// Check if an operation is allowed
    pub fn is_allowed(&mut self, operation: &str) -> bool {
        if let Some(limit_info) = self.limits.get_mut(operation) {
            // Check if window has expired
            if limit_info.is_window_expired() {
                limit_info.reset();
            }

            // Check if limit is exceeded
            if limit_info.is_limited() {
                false
            } else {
                limit_info.increment();
                
                // Save to storage
                let key = format!("{}_{}", self.storage_key_prefix, operation);
                let _ = LocalStorage::set(&key, limit_info);
                
                true
            }
        } else {
            // No limit configured, allow by default
            true
        }
    }

    /// Get rate limit information for an operation
    pub fn get_limit_info(&self, operation: &str) -> Option<&RateLimitInfo> {
        self.limits.get(operation)
    }

    /// Clear all rate limit information
    pub fn clear_all(&mut self) {
        self.limits.clear();
        // Note: In a real implementation, you might want to clear storage as well
    }
}

impl Default for ThrottleService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_throttle_service_creation() {
        let throttle = ThrottleService::new();
        assert!(true); // Just test that it can be created
    }

    #[wasm_bindgen_test]
    fn test_rate_limit_info() {
        let mut limit_info = RateLimitInfo::new(5, 1000.0);
        assert!(!limit_info.is_limited());
        assert_eq!(limit_info.remaining(), 5);
        
        for _ in 0..5 {
            limit_info.increment();
        }
        
        assert!(limit_info.is_limited());
        assert_eq!(limit_info.remaining(), 0);
    }
}