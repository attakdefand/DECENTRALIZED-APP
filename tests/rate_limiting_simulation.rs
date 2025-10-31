//! Rate Limiting Simulation Tests
//!
//! This module contains tests that simulate various rate limiting scenarios
//! to verify the effectiveness of our security measures.

use core::rate_limiting::{TokenBucket, RateLimiter, IdempotencyManager, JobGuard, WAFRules, Request, WAFError, JobError, TokenBucketConfig};
use std::time::Duration;

/// Test token bucket rate limiting
#[test]
fn test_token_bucket_rate_limiting() {
    let mut bucket = TokenBucket::new(10, 5); // 10 tokens, refill 5 per second
    
    // Consume all tokens
    assert!(bucket.consume(10));
    assert!(!bucket.consume(1)); // Should fail
    
    // Simulate time passing for refill
    bucket.last_refill = std::time::Instant::now() - Duration::from_secs(2);
    
    // Should have refilled
    assert!(bucket.consume(5));
    assert_eq!(bucket.tokens(), 5);
    
    println!("Token bucket rate limiting test passed");
}

/// Test rate limiter with multiple identifiers
#[test]
fn test_rate_limiter_with_multiple_identifiers() {
    let config = TokenBucketConfig {
        max_tokens: 3,
        refill_rate: 1,
    };
    let mut limiter = RateLimiter::new(config);
    
    // Test user1
    assert!(limiter.is_allowed("user1"));
    assert!(limiter.is_allowed("user1"));
    assert!(limiter.is_allowed("user1"));
    assert!(!limiter.is_allowed("user1")); // Should be rate limited
    
    // Test user2 (separate bucket)
    assert!(limiter.is_allowed("user2"));
    assert!(limiter.is_allowed("user2"));
    assert!(limiter.is_allowed("user2"));
    assert!(!limiter.is_allowed("user2")); // Should be rate limited
    
    println!("Rate limiter with multiple identifiers test passed");
}

/// Test idempotency manager
#[test]
fn test_idempotency_manager() {
    let mut manager = IdempotencyManager::new(Duration::from_secs(60));
    
    // Process a request
    let result: Result<String, String> = manager.process_request("key1", || {
        Ok("Hello, World!".to_string())
    });
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello, World!");
    
    // Process the same request again - should return cached result
    let result2: Result<String, String> = manager.process_request("key1", || {
        Ok("Goodbye, World!".to_string()) // This should not be called
    });
    
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), "Hello, World!");
    
    println!("Idempotency manager test passed");
}

/// Test job guard concurrency control
#[tokio::test]
async fn test_job_guard_concurrency_control() {
    let guard = JobGuard::new(2, Duration::from_secs(1));
    
    // Execute multiple jobs concurrently
    let job1 = guard.execute_job(async {
        tokio::time::sleep(Duration::from_millis(100)).await;
        Ok("Job 1 completed")
    });
    
    let job2 = guard.execute_job(async {
        tokio::time::sleep(Duration::from_millis(100)).await;
        Ok("Job 2 completed")
    });
    
    let job3 = guard.execute_job(async {
        tokio::time::sleep(Duration::from_millis(100)).await;
        Ok("Job 3 completed")
    });
    
    // First two jobs should succeed
    let result1 = job1.await;
    let result2 = job2.await;
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    
    // Third job should fail due to concurrency limit
    let result3 = job3.await;
    assert!(result3.is_err());
    match result3.unwrap_err() {
        JobError::TooManyJobs => {},
        _ => panic!("Expected TooManyJobs error"),
    }
    
    println!("Job guard concurrency control test passed");
}

/// Test WAF rules
#[test]
fn test_waf_rules() {
    let mut waf = WAFRules::new(5, 60); // 5 requests per minute
    
    let request = Request {
        ip: "192.168.1.1".to_string(),
        country: "US".to_string(),
        path: "/api/test".to_string(),
        method: "GET".to_string(),
        user_agent: "Mozilla/5.0".to_string(),
        api_key: None,
    };
    
    // Normal requests should be allowed (up to limit)
    for _ in 0..5 {
        assert!(waf.check_request(&request).is_ok());
    }
    
    // 6th request should be rate limited
    match waf.check_request(&request) {
        Err(WAFError::RateLimitExceeded) => {},
        _ => panic!("Expected RateLimitExceeded error"),
    }
    
    println!("WAF rules test passed");
}

/// Test geographic restrictions
#[test]
fn test_geographic_restrictions() {
    let mut waf = WAFRules::new(10, 60);
    
    // Block requests from specific country
    waf.set_geo_restriction("CN", false);
    
    let request = Request {
        ip: "192.168.1.1".to_string(),
        country: "CN".to_string(),
        path: "/api/test".to_string(),
        method: "GET".to_string(),
        user_agent: "Mozilla/5.0".to_string(),
        api_key: None,
    };
    
    // Request from blocked country should be denied
    match waf.check_request(&request) {
        Err(WAFError::GeoBlocked) => {},
        _ => panic!("Expected GeoBlocked error"),
    }
    
    println!("Geographic restrictions test passed");
}

/// Test suspicious pattern detection
#[test]
fn test_suspicious_pattern_detection() {
    let mut waf = WAFRules::new(10, 60);
    
    let malicious_request = Request {
        ip: "192.168.1.1".to_string(),
        country: "US".to_string(),
        path: "/api/test".to_string(),
        method: "GET".to_string(),
        user_agent: "Mozilla/5.0".to_string(),
        api_key: Some("DROP TABLE users;".to_string()),
    };
    
    // Request with suspicious pattern should be denied
    match waf.check_request(&malicious_request) {
        Err(WAFError::SuspiciousPattern) => {},
        _ => panic!("Expected SuspiciousPattern error"),
    }
    
    println!("Suspicious pattern detection test passed");
}

/// Test IP blocking/unblocking
#[test]
fn test_ip_blocking_unblocking() {
    let mut waf = WAFRules::new(10, 60);
    
    let request = Request {
        ip: "192.168.1.100".to_string(),
        country: "US".to_string(),
        path: "/api/test".to_string(),
        method: "GET".to_string(),
        user_agent: "Mozilla/5.0".to_string(),
        api_key: None,
    };
    
    // Normal request should be allowed
    assert!(waf.check_request(&request).is_ok());
    
    // Block the IP
    waf.block_ip("192.168.1.100");
    
    // Request from blocked IP should be denied
    match waf.check_request(&request) {
        Err(WAFError::BlockedIP) => {},
        _ => panic!("Expected BlockedIP error"),
    }
    
    // Unblock the IP
    waf.unblock_ip("192.168.1.100");
    
    // Request should be allowed again
    assert!(waf.check_request(&request).is_ok());
    
    println!("IP blocking/unblocking test passed");
}

/// Integration test for complete rate limiting workflow
#[test]
fn test_rate_limiting_workflow() {
    // 1. Create rate limiter with default configuration
    let config = TokenBucketConfig {
        max_tokens: 5,
        refill_rate: 2,
    };
    let mut limiter = RateLimiter::new(config);
    
    // 2. Create WAF rules
    let mut waf = WAFRules::new(10, 60);
    
    // 3. Create idempotency manager
    let mut idempotency_manager = IdempotencyManager::new(Duration::from_secs(300));
    
    // 4. Create job guard
    let job_guard = JobGuard::new(3, Duration::from_secs(5));
    
    // 5. Test normal request flow
    let request = Request {
        ip: "192.168.1.50".to_string(),
        country: "US".to_string(),
        path: "/api/transfer".to_string(),
        method: "POST".to_string(),
        user_agent: "MyApp/1.0".to_string(),
        api_key: Some("api_key_123".to_string()),
    };
    
    // WAF check
    assert!(waf.check_request(&request).is_ok());
    
    // Rate limit check
    assert!(limiter.is_allowed("192.168.1.50"));
    
    // Idempotent request processing
    let result: Result<String, String> = idempotency_manager.process_request("transfer_001", || {
        Ok("Transfer successful".to_string())
    });
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Transfer successful");
    
    // Process same request again - should return cached result
    let result2: Result<String, String> = idempotency_manager.process_request("transfer_001", || {
        // This should not be executed
        Err("Should not be called".to_string())
    });
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), "Transfer successful");
    
    println!("Complete rate limiting workflow test passed");
}

/// Test spike handling
#[test]
fn test_spike_handling() {
    let config = TokenBucketConfig {
        max_tokens: 100,
        refill_rate: 10,
    };
    let mut limiter = RateLimiter::new(config);
    
    // Simulate a spike - many requests in a short time
    let mut allowed_count = 0;
    let mut denied_count = 0;
    
    for i in 0..150 {
        if limiter.is_allowed("spike_test") {
            allowed_count += 1;
        } else {
            denied_count += 1;
        }
        
        // Simulate some time passing occasionally
        if i % 20 == 0 {
            // Get the bucket and simulate time passing
            let bucket = limiter.get_bucket("spike_test");
            bucket.last_refill = std::time::Instant::now() - Duration::from_secs(1);
        }
    }
    
    // Should have allowed some requests and denied others
    assert!(allowed_count > 0);
    assert!(denied_count > 0);
    
    println!("Spike handling test passed");
}

/// Test rate limit accuracy
#[test]
fn test_rate_limit_accuracy() {
    let config = TokenBucketConfig {
        max_tokens: 5,
        refill_rate: 1,
    };
    let mut limiter = RateLimiter::new(config);
    
    // Allow exactly 5 requests
    for _ in 0..5 {
        assert!(limiter.is_allowed("accuracy_test"));
    }
    
    // 6th request should be denied
    assert!(!limiter.is_allowed("accuracy_test"));
    
    // Simulate time passing for refill
    let bucket = limiter.get_bucket("accuracy_test");
    bucket.last_refill = std::time::Instant::now() - Duration::from_secs(3);
    
    // Should allow more requests after refill
    assert!(limiter.is_allowed("accuracy_test"));
    
    println!("Rate limit accuracy test passed");
}