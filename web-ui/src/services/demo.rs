//! Demo module to showcase the new services
//!
//! This module demonstrates how to use the new WebSocket, authentication,
//! caching, retry, and throttling services.

use crate::services::{
    api::create_client,
    auth::AuthService,
    cache::CacheService,
    retry::{RetryService, RetryConfig},
    throttle::ThrottleService,
};

/// Demonstrate WebSocket connection
pub fn demo_websocket() {
    let client = create_client();
    match client.connect_websocket("/ws/events") {
        Ok(_ws) => {
            // In a real app, you would use the WebSocket for real-time updates
            // For this demo, we'll just log that the connection was successful
            web_sys::console::log_1(&"WebSocket connection successful".into());
        }
        Err(e) => {
            web_sys::console::log_1(&format!("WebSocket connection failed: {:?}", e).into());
        }
    }
}

/// Demonstrate authentication service
pub fn demo_authentication() {
    let auth = AuthService::new("demo_secret_key");
    
    match auth.generate_token("user123", "demo_user") {
        Ok(token) => {
            web_sys::console::log_1(&format!("Generated token: {}", token).into());
            
            // Store the token
            if AuthService::store_token(&token).is_ok() {
                web_sys::console::log_1(&"Token stored successfully".into());
            }
            
            // Verify the token
            match auth.verify_token(&token) {
                Ok(claims) => {
                    web_sys::console::log_1(&format!("Token verified for user: {}", claims.username).into());
                }
                Err(e) => {
                    web_sys::console::log_1(&format!("Token verification failed: {}", e).into());
                }
            }
        }
        Err(e) => {
            web_sys::console::log_1(&format!("Token generation failed: {}", e).into());
        }
    }
}

/// Demonstrate caching service
pub fn demo_caching() {
    let mut cache = CacheService::new();
    
    // Store data in cache
    let data = vec!["item1".to_string(), "item2".to_string(), "item3".to_string()];
    match cache.set_memory("demo_key", data, 5000.0) { // 5 second TTL
        Ok(_) => {
            web_sys::console::log_1(&"Data cached successfully".into());
        }
        Err(e) => {
            web_sys::console::log_1(&format!("Caching failed: {}", e).into());
        }
    }
    
    // Retrieve data from cache
    let retrieved: Option<Vec<String>> = cache.get_memory("demo_key");
    match retrieved {
        Some(data) => {
            web_sys::console::log_1(&format!("Retrieved from cache: {:?}", data).into());
        }
        None => {
            web_sys::console::log_1(&"No data found in cache".into());
        }
    }
}

/// Demonstrate retry service
pub fn demo_retry() {
    let _retry = RetryService::with_config(RetryConfig {
        max_attempts: 3,
        base_delay: 100.0,
        max_delay: 1000.0,
        backoff_multiplier: 2.0,
    });
    
    web_sys::console::log_1(&"Retry service configured".into());
    // In a real app, you would use this to retry failed operations
}

/// Demonstrate throttling service
pub fn demo_throttling() {
    let mut throttle = ThrottleService::new();
    throttle.configure_limit("demo_operation", 5, 10000.0); // 5 requests per 10 seconds
    
    // Test if operation is allowed
    if throttle.is_allowed("demo_operation") {
        web_sys::console::log_1(&"Operation allowed".into());
    } else {
        web_sys::console::log_1(&"Operation throttled".into());
    }
    
    // Get rate limit info
    if let Some(info) = throttle.get_limit_info("demo_operation") {
        web_sys::console::log_1(&format!("Remaining requests: {}", info.remaining()).into());
    }
}

/// Run all demos
pub fn run_all_demos() {
    web_sys::console::log_1(&"=== Running All Demos ===".into());
    
    demo_websocket();
    demo_authentication();
    demo_caching();
    demo_retry();
    demo_throttling();
    
    web_sys::console::log_1(&"=== All Demos Completed ===".into());
}