//! Rate Limiting Middleware
//! 
//! This middleware implements rate limiting, throttling, and burst control
//! to prevent DoS, scraping, and brute force attacks.

use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use serde_json::json;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use chrono::Utc;

use crate::AppState;

/// Rate limiting configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum requests per IP per time window
    pub max_requests_per_ip: u32,
    /// Maximum requests per token per time window
    pub max_requests_per_token: u32,
    /// Time window for rate limiting
    pub window_duration: Duration,
    /// Burst size (additional requests allowed in burst)
    pub burst_size: u32,
    /// Threshold for triggering surge detection
    pub surge_threshold: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests_per_ip: 100,     // 100 requests per window per IP
            max_requests_per_token: 1000, // 1000 requests per window per token
            window_duration: Duration::from_secs(60), // 1 minute window
            burst_size: 10,               // Allow 10 burst requests
            surge_threshold: 50,          // Trigger surge detection at 50 requests
        }
    }
}

/// Rate limit entry for tracking requests
#[derive(Debug, Clone)]
pub struct RequestRateLimitEntry {
    /// Count of requests in current window
    pub count: u32,
    /// Timestamp of last reset
    pub last_reset: Instant,
    /// Burst tokens available
    pub burst_tokens: u32,
    /// Surge detection counter
    pub surge_count: u32,
}

impl RequestRateLimitEntry {
    pub fn new() -> Self {
        Self {
            count: 0,
            last_reset: Instant::now(),
            burst_tokens: 0,
            surge_count: 0,
        }
    }
    
    /// Reset the counter if the window has expired
    pub fn reset_if_needed(&mut self, window_duration: Duration) {
        if self.last_reset.elapsed() >= window_duration {
            self.count = 0;
            self.last_reset = Instant::now();
            self.burst_tokens = 0; // Reset burst tokens on window reset
            self.surge_count = 0;  // Reset surge counter
        }
    }
    
    /// Check if this request triggers a surge
    pub fn is_surge(&mut self, threshold: u32) -> bool {
        self.surge_count += 1;
        self.surge_count >= threshold
    }
}

// Helper function to create a new rate limit entry
fn create_rate_limit_entry() -> RequestRateLimitEntry {
    RequestRateLimitEntry {
        count: 0,
        last_reset: Instant::now(),
        burst_tokens: 0,
        surge_count: 0,
    }
}

/// Rate limiting state
#[derive(Debug, Clone)]
pub struct RateLimitState {
    /// Rate limiting configuration
    config: RateLimitConfig,
    /// Per-IP rate limit tracking
    pub ip_limits: Arc<Mutex<HashMap<IpAddr, RequestRateLimitEntry>>>,
    /// Per-token rate limit tracking
    pub token_limits: Arc<Mutex<HashMap<String, RequestRateLimitEntry>>>,
    /// Circuit breaker state
    pub circuit_breaker: Arc<Mutex<CircuitBreaker>>,
    /// Surge detection metrics
    pub surge_counter: Arc<Mutex<u32>>,
}

impl RateLimitState {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            ip_limits: Arc::new(Mutex::new(HashMap::new())),
            token_limits: Arc::new(Mutex::new(HashMap::new())),
            circuit_breaker: Arc::new(Mutex::new(CircuitBreaker::new())),
            surge_counter: Arc::new(Mutex::new(0)),
        }
    }
    
    /// Get a copy of the config (for use in middleware)
    pub fn get_config(&self) -> RateLimitConfig {
        self.config.clone()
    }
    
    /// Get config values for IP rate limiting
    pub fn get_ip_config(&self) -> (u32, Duration, u32) {
        (self.config.max_requests_per_ip, self.config.window_duration, self.config.burst_size)
    }
    
    /// Get config values for token rate limiting
    pub fn get_token_config(&self) -> (u32, Duration, u32) {
        (self.config.max_requests_per_token, self.config.window_duration, self.config.burst_size)
    }
    
    /// Get surge threshold
    pub fn get_surge_threshold(&self) -> u32 {
        self.config.surge_threshold
    }
}

/// Circuit breaker for service protection
#[derive(Debug)]
pub struct CircuitBreaker {
    /// Current state of the circuit breaker
    pub state: CircuitState,
    /// Failure count
    pub failure_count: u32,
    /// Last failure time
    pub last_failure: Option<Instant>,
    /// Threshold for opening the circuit
    pub failure_threshold: u32,
    /// Timeout before allowing test requests
    pub timeout: Duration,
    /// Success count in half-open state
    pub success_count: u32,
    /// Required successes to close circuit in half-open state
    pub required_successes: u32,
}

/// Circuit breaker states
#[derive(Debug, Clone, PartialEq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

impl CircuitBreaker {
    pub fn new() -> Self {
        Self {
            state: CircuitState::Closed,
            failure_count: 0,
            last_failure: None,
            failure_threshold: 5, // Open after 5 failures
            timeout: Duration::from_secs(30), // 30 second timeout
            success_count: 0,
            required_successes: 3, // Need 3 successes to close in half-open
        }
    }
    
    /// Record a failure
    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure = Some(Instant::now());
        
        if self.failure_count >= self.failure_threshold {
            self.state = CircuitState::Open;
            self.success_count = 0; // Reset success count when opening
        }
    }
    
    /// Record a success
    pub fn record_success(&mut self) {
        match self.state {
            CircuitState::Closed => {
                // In closed state, just reset failure count
                self.failure_count = 0;
                self.last_failure = None;
            }
            CircuitState::HalfOpen => {
                // In half-open state, count successes
                self.success_count += 1;
                if self.success_count >= self.required_successes {
                    // Enough successes, close the circuit
                    self.state = CircuitState::Closed;
                    self.failure_count = 0;
                    self.last_failure = None;
                }
            }
            CircuitState::Open => {
                // Should not receive success in open state, but if we do,
                // transition to half-open
                self.state = CircuitState::HalfOpen;
                self.success_count = 1;
            }
        }
    }
    
    /// Check if request should be allowed
    pub fn allow_request(&mut self) -> bool {
        match self.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                // Check if timeout has expired
                if let Some(last_failure) = self.last_failure {
                    if last_failure.elapsed() >= self.timeout {
                        self.state = CircuitState::HalfOpen;
                        self.success_count = 0;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            CircuitState::HalfOpen => {
                // Allow test requests in half-open state
                true
            }
        }
    }
    
    /// Report result of a request through the circuit breaker
    pub fn report_result(&mut self, success: bool) {
        if success {
            self.record_success();
        } else {
            self.record_failure();
        }
    }
}

/// Rate limiting middleware
/// 
/// This middleware implements rate limiting, throttling, and burst control
/// to prevent DoS, scraping, and brute force attacks.
pub async fn rate_limit_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, Response> {
    // Extract IP address from request
    let ip = extract_ip_from_request(&request);
    
    // Extract token from headers (if present)
    let token = extract_token_from_headers(&headers);
    
    // Check circuit breaker
    {
        let mut circuit_breaker = state.rate_limit_state.circuit_breaker.lock().await;
        if !circuit_breaker.allow_request() {
            // Track circuit breaker rejection in metrics
            state.metrics.gateway_rejections.inc();
            
            return Err(create_rate_limit_error_response(
                "Service temporarily unavailable due to circuit breaker",
                StatusCode::SERVICE_UNAVAILABLE,
            ));
        }
    }
    
    // Check IP rate limit
    if let Some(ip) = ip {
        // Get config values
        let (max_requests, window_duration, burst_size) = state.rate_limit_state.get_ip_config();
        let surge_threshold = state.rate_limit_state.get_surge_threshold();
        
        let mut ip_limits = state.rate_limit_state.ip_limits.lock().await;
        // Use a different approach to avoid type conflicts
        let needs_insert = !ip_limits.contains_key(&ip);
        
        if needs_insert {
            // Create entry using the new function
            let entry = RequestRateLimitEntry::new();
            ip_limits.insert(ip, entry);
        }
        
        // Get mutable reference to entry
        let entry = ip_limits.get_mut(&ip).unwrap();
        entry.reset_if_needed(window_duration);
        
        // Check for surge activity
        if entry.is_surge(surge_threshold) {
            let mut surge_counter = state.rate_limit_state.surge_counter.lock().await;
            *surge_counter += 1;
            
            // Log surge event (in a real implementation, this would go to logs/metrics)
            println!("SURGE DETECTED: IP {} triggered surge #{}", ip, *surge_counter);
        }
        
        // Check burst tokens first
        if entry.burst_tokens < burst_size {
            entry.burst_tokens += 1;
        } else if entry.count >= max_requests {
            // Track rate limit rejection in metrics
            state.metrics.gateway_rejections.inc();
            
            return Err(create_rate_limit_error_response(
                "Rate limit exceeded for IP address",
                StatusCode::TOO_MANY_REQUESTS,
            ));
        }
        
        entry.count += 1;
    }
    
    // Check token rate limit
    if let Some(token) = &token {
        // Get config values
        let (max_requests, window_duration, burst_size) = state.rate_limit_state.get_token_config();
        
        let mut token_limits = state.rate_limit_state.token_limits.lock().await;
        // Use a different approach to avoid type conflicts
        let needs_insert = !token_limits.contains_key(token);
        
        if needs_insert {
            // Create entry using the new function
            let entry = RequestRateLimitEntry::new();
            token_limits.insert(token.clone(), entry);
        }
        
        // Get mutable reference to entry
        let entry = token_limits.get_mut(token).unwrap();
        entry.reset_if_needed(window_duration);
        
        // Check burst tokens first
        if entry.burst_tokens < burst_size {
            entry.burst_tokens += 1;
        } else if entry.count >= max_requests {
            // Track rate limit rejection in metrics
            state.metrics.gateway_rejections.inc();
            
            return Err(create_rate_limit_error_response(
                "Rate limit exceeded for token",
                StatusCode::TOO_MANY_REQUESTS,
            ));
        }
        
        entry.count += 1;
    }
    
    // Continue with the request
    let response = next.run(request).await;
    
    // Update circuit breaker based on response
    {
        let mut circuit_breaker = state.rate_limit_state.circuit_breaker.lock().await;
        let success = response.status().is_success();
        circuit_breaker.report_result(success);
    }
    
    Ok(response)
}

/// Extract IP address from request
fn extract_ip_from_request(request: &Request) -> Option<IpAddr> {
    // Try to get IP from forwarded headers first
    if let Some(forwarded) = request.headers().get("x-forwarded-for") {
        if let Ok(forwarded_str) = forwarded.to_str() {
            if let Some(first_ip) = forwarded_str.split(',').next() {
                if let Ok(ip) = first_ip.trim().parse::<IpAddr>() {
                    return Some(ip);
                }
            }
        }
    }
    
    // Try to get IP from x-real-ip header
    if let Some(real_ip) = request.headers().get("x-real-ip") {
        if let Ok(real_ip_str) = real_ip.to_str() {
            if let Ok(ip) = real_ip_str.parse::<IpAddr>() {
                return Some(ip);
            }
        }
    }
    
    // Fall back to connection info
    request
        .extensions()
        .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
        .map(|connect_info| connect_info.0.ip())
}

/// Extract token from headers
fn extract_token_from_headers(headers: &HeaderMap) -> Option<String> {
    // Try to get token from authorization header
    if let Some(auth) = headers.get("authorization") {
        if let Ok(auth_str) = auth.to_str() {
            if auth_str.starts_with("Bearer ") {
                return Some(auth_str[7..].to_string());
            }
        }
    }
    
    // Try to get token from x-api-token header
    if let Some(token) = headers.get("x-api-token") {
        if let Ok(token_str) = token.to_str() {
            return Some(token_str.to_string());
        }
    }
    
    None
}

/// Create error response for rate limiting
fn create_rate_limit_error_response(message: &str, status: StatusCode) -> Response {
    let error_json = json!({
        "error": "Rate limit exceeded",
        "message": message,
        "timestamp": Utc::now().to_rfc3339()
    });
    
    let body = serde_json::to_string(&error_json).unwrap_or_else(|_| "{}".to_string());
    
    Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .header("retry-after", "60") // Suggest retry after 60 seconds
        .body(axum::body::Body::from(body))
        .unwrap()
}