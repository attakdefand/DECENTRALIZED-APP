//! Cache service
//!
//! This module provides caching functionality for the web UI.

use serde::{Deserialize, Serialize};
use gloo_storage::{LocalStorage, SessionStorage, Storage};
use std::collections::HashMap;

/// Cache entry with expiration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CacheEntry<T> {
    pub data: T,
    pub timestamp: f64,
    pub ttl: f64, // time to live in milliseconds
}

impl<T> CacheEntry<T> {
    /// Create a new cache entry
    pub fn new(data: T, ttl: f64) -> Self {
        Self {
            data,
            timestamp: js_sys::Date::now(),
            ttl,
        }
    }

    /// Check if the entry is expired
    pub fn is_expired(&self) -> bool {
        js_sys::Date::now() > self.timestamp + self.ttl
    }
}

/// Cache service with different storage options
pub struct CacheService {
    local_cache: HashMap<String, String>, // Store serialized data as strings
    session_cache: HashMap<String, String>,
}

impl CacheService {
    /// Create a new cache service
    pub fn new() -> Self {
        Self {
            local_cache: HashMap::new(),
            session_cache: HashMap::new(),
        }
    }

    /// Store data in memory cache
    pub fn set_memory<T: Serialize>(&mut self, key: &str, data: T, ttl: f64) -> Result<(), anyhow::Error> {
        let entry = CacheEntry::new(data, ttl);
        let serialized = serde_json::to_string(&entry)?;
        self.local_cache.insert(key.to_string(), serialized);
        Ok(())
    }

    /// Retrieve data from memory cache
    pub fn get_memory<T: for<'de> Deserialize<'de>>(&mut self, key: &str) -> Option<T> {
        if let Some(value) = self.local_cache.get(key) {
            if let Ok(entry) = serde_json::from_str::<CacheEntry<T>>(value) {
                if !entry.is_expired() {
                    return Some(entry.data);
                } else {
                    // Remove expired entry
                    self.local_cache.remove(key);
                }
            }
        }
        None
    }

    /// Store data in local storage cache
    pub fn set_local<T: Serialize>(&self, key: &str, data: T, ttl: f64) -> Result<(), gloo_storage::errors::StorageError> {
        let entry = CacheEntry::new(data, ttl);
        LocalStorage::set(key, entry)
    }

    /// Retrieve data from local storage cache
    pub fn get_local<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        if let Ok(entry) = LocalStorage::get::<CacheEntry<T>>(key) {
            if !entry.is_expired() {
                return Some(entry.data);
            } else {
                // Remove expired entry
                LocalStorage::delete(key);
            }
        }
        None
    }

    /// Store data in session storage cache
    pub fn set_session<T: Serialize>(&self, key: &str, data: T, ttl: f64) -> Result<(), gloo_storage::errors::StorageError> {
        let entry = CacheEntry::new(data, ttl);
        SessionStorage::set(key, entry)
    }

    /// Retrieve data from session storage cache
    pub fn get_session<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        if let Ok(entry) = SessionStorage::get::<CacheEntry<T>>(key) {
            if !entry.is_expired() {
                return Some(entry.data);
            } else {
                // Remove expired entry
                SessionStorage::delete(key);
            }
        }
        None
    }

    /// Clear all cached data
    pub fn clear_all(&mut self) {
        self.local_cache.clear();
        // Note: We don't clear browser storage as it might affect other data
    }

    /// Clear expired entries from memory cache
    pub fn clear_expired_memory(&mut self) {
        self.local_cache.retain(|_key, value| {
            if let Ok(entry) = serde_json::from_str::<CacheEntry<serde_json::Value>>(value) {
                !entry.is_expired()
            } else {
                false
            }
        });
    }
}

impl Default for CacheService {
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
    fn test_cache_service_creation() {
        let cache = CacheService::new();
        assert!(true); // Just test that it can be created
    }

    #[wasm_bindgen_test]
    fn test_memory_cache() {
        let mut cache = CacheService::new();
        let result = cache.set_memory("test_key", "test_value", 1000.0);
        assert!(result.is_ok());

        let retrieved: Option<String> = cache.get_memory("test_key");
        assert_eq!(retrieved, Some("test_value".to_string()));
    }
}