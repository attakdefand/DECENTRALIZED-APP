//! Integration tests for services
//!
//! Tests caching, retry logic, and service integration

use wasm_bindgen_test::*;
use wasm_bindgen_futures::JsFuture;
use crate::services::{
    cache::CacheService,
    retry::{RetryService, RetryConfig},
};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_cache_memory_storage() {
    let mut cache = CacheService::new();
    
    let data = vec!["item1".to_string(), "item2".to_string(), "item3".to_string()];
    let result = cache.set_memory("test_key", data.clone(), 5000.0);
    
    assert!(result.is_ok(), "Cache set should succeed");
    
    let retrieved: Option<Vec<String>> = cache.get_memory("test_key");
    assert!(retrieved.is_some(), "Should retrieve cached data");
    assert_eq!(retrieved.unwrap(), data, "Retrieved data should match stored data");
}

#[wasm_bindgen_test]
fn test_cache_local_storage() {
    let cache = CacheService::new();
    
    let data = vec![1, 2, 3, 4, 5];
    let result = cache.set_local("test_numbers", &data, 10000.0);
    
    assert!(result.is_ok(), "Local storage set should succeed");
    
    let retrieved: Option<Vec<i32>> = cache.get_local("test_numbers");
    assert!(retrieved.is_some(), "Should retrieve from local storage");
    assert_eq!(retrieved.unwrap(), data, "Retrieved data should match");
}

#[wasm_bindgen_test]
fn test_cache_ttl_expiration() {
    let mut cache = CacheService::new();
    
    // Set cache with 0ms TTL (already expired)
    let data = "test_data".to_string();
    let _ = cache.set_memory("expired_key", data, 0.0);
    
    // Should return None for expired data
    let retrieved: Option<String> = cache.get_memory("expired_key");
    assert!(retrieved.is_none(), "Expired cache should return None");
}

#[wasm_bindgen_test]
fn test_cache_nonexistent_key() {
    let mut cache = CacheService::new();
    
    let retrieved: Option<String> = cache.get_memory("nonexistent_key");
    assert!(retrieved.is_none(), "Nonexistent key should return None");
}

#[wasm_bindgen_test]
async fn test_retry_service_success_on_first_attempt() {
    let retry = RetryService::default();
    let mut attempt_count = 0;
    
    let operation = || {
        attempt_count += 1;
        async move {
            Ok::<String, String>("success".to_string())
        }
    };
    
    let result = retry.retry(operation).await;
    assert!(result.is_ok(), "Should succeed on first attempt");
    assert_eq!(attempt_count, 1, "Should only attempt once");
}

#[wasm_bindgen_test]
async fn test_retry_service_with_failures() {
    let retry = RetryService::with_config(RetryConfig {
        max_attempts: 3,
        base_delay: 10.0, // Short delay for testing
        max_delay: 50.0,
        backoff_multiplier: 2.0,
    });
    
    let mut attempt_count = 0;
    
    let operation = || {
        attempt_count += 1;
        async move {
            if attempt_count < 2 {
                Err::<String, String>("temporary error".to_string())
            } else {
                Ok::<String, String>("success".to_string())
            }
        }
    };
    
    let result = retry.retry(operation).await;
    assert!(result.is_ok(), "Should succeed after retries");
    assert!(attempt_count >= 2, "Should retry at least once");
}

#[wasm_bindgen_test]
async fn test_retry_service_max_attempts() {
    let retry = RetryService::with_config(RetryConfig {
        max_attempts: 2,
        base_delay: 10.0,
        max_delay: 50.0,
        backoff_multiplier: 2.0,
    });
    
    let mut attempt_count = 0;
    
    let operation = || {
        attempt_count += 1;
        async move {
            Err::<String, String>("persistent error".to_string())
        }
    };
    
    let result = retry.retry(operation).await;
    assert!(result.is_err(), "Should fail after max attempts");
    assert_eq!(attempt_count, 2, "Should attempt exactly max_attempts times");
}

#[wasm_bindgen_test]
fn test_cache_different_types() {
    let mut cache = CacheService::new();
    
    // Test with different data types
    let _ = cache.set_memory("string", "test".to_string(), 5000.0);
    let _ = cache.set_memory("number", 42, 5000.0);
    let _ = cache.set_memory("bool", true, 5000.0);
    let _ = cache.set_memory("vec", vec![1, 2, 3], 5000.0);
    
    let str_val: Option<String> = cache.get_memory("string");
    let num_val: Option<i32> = cache.get_memory("number");
    let bool_val: Option<bool> = cache.get_memory("bool");
    let vec_val: Option<Vec<i32>> = cache.get_memory("vec");
    
    assert_eq!(str_val, Some("test".to_string()));
    assert_eq!(num_val, Some(42));
    assert_eq!(bool_val, Some(true));
    assert_eq!(vec_val, Some(vec![1, 2, 3]));
}

#[wasm_bindgen_test]
fn test_retry_config_validation() {
    let config = RetryConfig {
        max_attempts: 5,
        base_delay: 100.0,
        max_delay: 5000.0,
        backoff_multiplier: 2.0,
    };
    
    assert_eq!(config.max_attempts, 5);
    assert_eq!(config.base_delay, 100.0);
    assert_eq!(config.max_delay, 5000.0);
    assert_eq!(config.backoff_multiplier, 2.0);
}

#[wasm_bindgen_test]
fn test_cache_overwrite() {
    let mut cache = CacheService::new();
    
    let _ = cache.set_memory("key", "value1".to_string(), 5000.0);
    let first: Option<String> = cache.get_memory("key");
    assert_eq!(first, Some("value1".to_string()));
    
    // Overwrite
    let _ = cache.set_memory("key", "value2".to_string(), 5000.0);
    let second: Option<String> = cache.get_memory("key");
    assert_eq!(second, Some("value2".to_string()));
}

#[wasm_bindgen_test]
fn test_multiple_cache_instances() {
    let mut cache1 = CacheService::new();
    let mut cache2 = CacheService::new();
    
    let _ = cache1.set_memory("shared_key", "cache1_value".to_string(), 5000.0);
    let _ = cache2.set_memory("shared_key", "cache2_value".to_string(), 5000.0);
    
    let val1: Option<String> = cache1.get_memory("shared_key");
    let val2: Option<String> = cache2.get_memory("shared_key");
    
    // Memory caches are instance-specific
    assert_eq!(val1, Some("cache1_value".to_string()));
    assert_eq!(val2, Some("cache2_value".to_string()));
}
