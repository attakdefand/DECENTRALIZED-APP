//! Rate Limiting Security Module
//!
//! This module implements security measures for rate limiting including
//! WAF rules, token bucket rate limiting, idempotency keys, and job guards.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tokio::time::timeout;
use anyhow::Error;

/// Represents a request for WAF evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    /// IP address of the requester
    pub ip: String,
    /// Country of the requester
    pub country: String,
    /// Request path
    pub path: String,
    /// Request method
    pub method: String,
    /// User agent
    pub user_agent: String,
    /// API key (if provided)
    pub api_key: Option<String>,
}

/// WAF error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WAFError {
    /// Blocked IP address
    BlockedIP,
    /// Geographic restriction
    GeoBlocked,
    /// Rate limit exceeded
    RateLimitExceeded,
    /// Suspicious pattern detected
    SuspiciousPattern,
}

/// Token bucket for rate limiting
#[derive(Debug)]
pub struct TokenBucket {
    /// Maximum tokens in the bucket
    max_tokens: u32,
    /// Current tokens in the bucket
    tokens: u32,
    /// Tokens added per second
    refill_rate: u32,
    /// Last refill timestamp
    last_refill: Instant,
}

/// Configuration for a token bucket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenBucketConfig {
    /// Maximum tokens
    pub max_tokens: u32,
    /// Refill rate (tokens per second)
    pub refill_rate: u32,
}

/// Rate limiter with multiple buckets
#[derive(Debug)]
pub struct RateLimiter {
    /// Token buckets for different identifiers
    buckets: HashMap<String, TokenBucket>,
    /// Default bucket configuration
    default_bucket: TokenBucketConfig,
}

/// Record of an idempotent request
#[derive(Debug, Clone)]
pub struct IdempotencyRecord {
    /// The result of the request
    pub result: Vec<u8>,
    /// Timestamp when the record was created
    pub timestamp: Instant,
    /// Whether the request is still being processed
    pub in_progress: bool,
}

/// Idempotency manager
#[derive(Debug)]
pub struct IdempotencyManager {
    /// Cache of idempotency keys and their results
    cache: HashMap<String, IdempotencyRecord>,
    /// Time-to-live for idempotency records
    ttl: Duration,
    /// Mutex for thread safety
    mutex: Mutex<()>,
}

/// Job error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobError {
    /// Too many concurrent jobs
    TooManyJobs,
    /// Job timeout
    Timeout,
    /// Job execution failed
    ExecutionFailed(String),
}

/// Job guard for concurrency control
#[derive(Debug)]
pub struct JobGuard {
    /// Maximum concurrent jobs
    max_concurrent: usize,
    /// Current job count
    current_jobs: Arc<AtomicUsize>,
    /// Job timeout
    timeout: Duration,
    /// Semaphore for concurrency control
    semaphore: Arc<Semaphore>,
}

/// WAF rules manager
#[derive(Debug)]
pub struct WAFRules {
    /// Maximum requests per IP per time window
    max_requests_per_ip: u32,
    /// Time window in seconds
    time_window: u64,
    /// Blocked IP addresses
    blocked_ips: HashSet<String>,
    /// Geographic restrictions
    geo_restrictions: HashMap<String, bool>,
    /// Request counters for rate limiting
    request_counters: HashMap<String, (u32, Instant)>,
    /// Suspicious patterns
    suspicious_patterns: Vec<String>,
}

impl TokenBucket {
    /// Create a new token bucket
    pub fn new(max_tokens: u32, refill_rate: u32) -> Self {
        Self {
            max_tokens,
            tokens: max_tokens,
            refill_rate,
            last_refill: Instant::now(),
        }
    }
    
    /// Try to consume tokens
    pub fn consume(&mut self, tokens: u32) -> bool {
        // Refill tokens
        self.refill();
        
        // Check if we have enough tokens
        if self.tokens >= tokens {
            self.tokens -= tokens;
            true
        } else {
            false
        }
    }
    
    /// Refill tokens based on elapsed time
    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs();
        
        if elapsed > 0 {
            let new_tokens = (elapsed * self.refill_rate as u64) as u32;
            self.tokens = (self.tokens + new_tokens).min(self.max_tokens);
            self.last_refill = now;
        }
    }
    
    /// Get current token count
    pub fn tokens(&self) -> u32 {
        self.tokens
    }
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(default_bucket: TokenBucketConfig) -> Self {
        Self {
            buckets: HashMap::new(),
            default_bucket,
        }
    }
    
    /// Check if a request is allowed for an identifier
    pub fn is_allowed(&mut self, identifier: &str) -> bool {
        let bucket = self.buckets.entry(identifier.to_string()).or_insert_with(|| {
            TokenBucket::new(self.default_bucket.max_tokens, self.default_bucket.refill_rate)
        });
        
        bucket.consume(1)
    }
    
    /// Get token bucket for an identifier
    pub fn get_bucket(&mut self, identifier: &str) -> &mut TokenBucket {
        self.buckets.entry(identifier.to_string()).or_insert_with(|| {
            TokenBucket::new(self.default_bucket.max_tokens, self.default_bucket.refill_rate)
        })
    }
}

impl IdempotencyManager {
    /// Create a new idempotency manager
    pub fn new(ttl: std::time::Duration) -> Self {
        Self {
            cache: std::collections::HashMap::new(),
            ttl,
            mutex: std::sync::Mutex::new(()),
        }
    }
    
    /// Process an idempotent request
    pub fn process_request<T, F>(&mut self, key: &str, handler: F) -> Result<T, String>
    where
        T: serde::de::DeserializeOwned + serde::Serialize + Clone,
        F: FnOnce() -> Result<T, String>,
    {
        {
            let _guard = self.mutex.lock().unwrap();
            
            // Check if we already have a result for this key
            if let Some(record) = self.cache.get(key) {
                if record.in_progress {
                    return Err("Request is still being processed".to_string());
                }
                
                // Check if record is expired
                if record.timestamp.elapsed() < self.ttl {
                    // Deserialize and return cached result
                    return serde_json::from_slice(&record.result)
                        .map_err(|e| format!("Failed to deserialize cached result: {}", e));
                }
            }
            
            // Remove expired records
            let _now = std::time::Instant::now();
            self.cache.retain(|_, record| {
                record.timestamp.elapsed() < self.ttl
            });
            
            // Mark as in progress
            let serialized_empty: Vec<u8> = vec![];
            self.cache.insert(key.to_string(), IdempotencyRecord {
                result: serialized_empty,
                timestamp: std::time::Instant::now(),
                in_progress: true,
            });
        } // Release lock before processing
        
        // Process the request
        let result = handler();
        
        {
            // Reacquire lock
            let _guard = self.mutex.lock().unwrap();
            
            match result {
                Ok(ref value) => {
                    // Serialize result
                    let serialized = serde_json::to_vec(value)
                        .map_err(|e| format!("Failed to serialize result: {}", e))?;
                    
                    // Store the result
                    self.cache.insert(key.to_string(), IdempotencyRecord {
                        result: serialized,
                        timestamp: std::time::Instant::now(),
                        in_progress: false,
                    });
                    
                    Ok(value.clone())
                }
                Err(e) => {
                    // Remove the in-progress record on error
                    self.cache.remove(key);
                    Err(e)
                }
            }
        }
    }
    
    /// Get cache size
    pub fn cache_size(&self) -> usize {
        let _guard = self.mutex.lock().unwrap();
        self.cache.len()
    }
}

impl JobGuard {
    /// Create a new job guard
    pub fn new(max_concurrent: usize, timeout_duration: Duration) -> Self {
        Self {
            max_concurrent,
            current_jobs: Arc::new(AtomicUsize::new(0)),
            timeout: timeout_duration,
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }
    
    /// Execute a job with guarding
    pub async fn execute_job<T, F, E>(&self, job: F) -> Result<T, JobError>
    where
        F: std::future::Future<Output = Result<T, E>> + Send,
        E: std::fmt::Display,
        T: Send,
    {
        // Acquire semaphore permit
        let _permit = self.semaphore.acquire().await
            .map_err(|_| JobError::TooManyJobs)?;
        
        // Increment job count
        self.current_jobs.fetch_add(1, Ordering::SeqCst);
        
        // Create timeout future
        let timeout_future = timeout(self.timeout, job);
        
        // Execute job with timeout
        let result = match timeout_future.await {
            Ok(Ok(result)) => Ok(result),
            Ok(Err(e)) => Err(JobError::ExecutionFailed(e.to_string())),
            Err(_) => Err(JobError::Timeout),
        };
        
        // Decrement job count
        self.current_jobs.fetch_sub(1, Ordering::SeqCst);
        
        result
    }
    
    /// Get current job count
    pub fn current_job_count(&self) -> usize {
        self.current_jobs.load(Ordering::SeqCst)
    }
    
    /// Get maximum concurrent jobs
    pub fn max_concurrent(&self) -> usize {
        self.max_concurrent
    }
}

impl WAFRules {
    /// Create new WAF rules
    pub fn new(max_requests_per_ip: u32, time_window: u64) -> Self {
        Self {
            max_requests_per_ip,
            time_window,
            blocked_ips: HashSet::new(),
            geo_restrictions: HashMap::new(),
            request_counters: HashMap::new(),
            suspicious_patterns: vec![
                "DROP TABLE".to_string(),
                "INSERT INTO".to_string(),
                "UNION SELECT".to_string(),
                "../".to_string(),
                "<script>".to_string(),
            ],
        }
    }
    
    /// Check if a request should be allowed
    pub fn check_request(&mut self, request: &Request) -> Result<bool, WAFError> {
        // Check if IP is blocked
        if self.blocked_ips.contains(&request.ip) {
            return Err(WAFError::BlockedIP);
        }
        
        // Check geographic restrictions
        if let Some(allowed) = self.geo_restrictions.get(&request.country) {
            if !allowed {
                return Err(WAFError::GeoBlocked);
            }
        }
        
        // Check for suspicious patterns
        if self.contains_suspicious_pattern(request) {
            return Err(WAFError::SuspiciousPattern);
        }
        
        // Check rate limits
        if self.check_rate_limit(&request.ip)? {
            return Err(WAFError::RateLimitExceeded);
        }
        
        Ok(true)
    }
    
    /// Check rate limit for an IP
    fn check_rate_limit(&mut self, ip: &str) -> Result<bool, WAFError> {
        let now = Instant::now();
        let entry = self.request_counters.entry(ip.to_string()).or_insert((0, now));
        let (_count, last_reset) = entry;
        
        // Reset counter if time window has passed
        if now.duration_since(*last_reset).as_secs() > self.time_window {
            *entry = (0, now);
        }
        
        // Increment counter
        entry.0 += 1;
        
        // Check if limit exceeded
        if entry.0 > self.max_requests_per_ip {
            Ok(true) // Rate limit exceeded
        } else {
            Ok(false) // Within limits
        }
    }
    
    /// Check for suspicious patterns in request
    fn contains_suspicious_pattern(&self, request: &Request) -> bool {
        let request_str = format!("{} {} {} {}", 
            request.path, 
            request.method, 
            request.user_agent, 
            request.api_key.as_deref().unwrap_or(""));
        
        for pattern in &self.suspicious_patterns {
            if request_str.contains(pattern) {
                return true;
            }
        }
        
        false
    }
    
    /// Block an IP address
    pub fn block_ip(&mut self, ip: &str) {
        self.blocked_ips.insert(ip.to_string());
    }
    
    /// Unblock an IP address
    pub fn unblock_ip(&mut self, ip: &str) {
        self.blocked_ips.remove(ip);
    }
    
    /// Set geographic restriction
    pub fn set_geo_restriction(&mut self, country: &str, allowed: bool) {
        self.geo_restrictions.insert(country.to_string(), allowed);
    }
    
    /// Add suspicious pattern
    pub fn add_suspicious_pattern(&mut self, pattern: &str) {
        self.suspicious_patterns.push(pattern.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    
    #[test]
    fn test_token_bucket() {
        let mut bucket = TokenBucket::new(10, 5); // 10 tokens, refill 5 per second
        
        // Consume all tokens
        assert!(bucket.consume(10));
        assert!(!bucket.consume(1)); // Should fail
        
        // Wait for refill (simulate)
        bucket.last_refill = Instant::now() - Duration::from_secs(2);
        
        // Should have refilled
        assert!(bucket.consume(5));
        assert_eq!(bucket.tokens(), 5);
    }
    
    #[test]
    fn test_rate_limiter() {
        let config = TokenBucketConfig {
            max_tokens: 5,
            refill_rate: 1,
        };
        let mut limiter = RateLimiter::new(config);
        
        // First 5 requests should be allowed
        for _ in 0..5 {
            assert!(limiter.is_allowed("user1"));
        }
        
        // 6th request should be denied
        assert!(!limiter.is_allowed("user1"));
    }
    
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
    }
    
    #[tokio::test]
    async fn test_job_guard() {
        let guard = JobGuard::new(2, Duration::from_secs(1));
        
        // Execute a simple job
        let result = guard.execute_job::<&str, _, anyhow::Error>(async {
            Ok("Job completed")
        }).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Job completed");
    }
    
    #[test]
    fn test_waf_rules() {
        let mut waf = WAFRules::new(10, 60); // 10 requests per minute
        
        let request = Request {
            ip: "192.168.1.1".to_string(),
            country: "US".to_string(),
            path: "/api/test".to_string(),
            method: "GET".to_string(),
            user_agent: "Mozilla/5.0".to_string(),
            api_key: None,
        };
        
        // Normal request should be allowed
        assert!(waf.check_request(&request).is_ok());
        
        // Block an IP
        waf.block_ip("192.168.1.1");
        
        // Request from blocked IP should be denied
        assert!(waf.check_request(&request).is_err());
    }
    
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
    }
}