//! RPC Resilience Module
//!
//! This module implements RPC resilience measures including TLS/mTLS configuration,
//! RPC provider management, failover policies, and pinning strategies.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents an RPC provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcProvider {
    /// Provider name
    pub name: String,
    /// Provider endpoint URL
    pub endpoint: String,
    /// Provider region
    pub region: String,
    /// Provider priority (lower is higher priority)
    pub priority: u32,
    /// Whether the provider is active
    pub active: bool,
    /// Last health check timestamp
    pub last_check: u64,
    /// Provider health status
    pub health_status: ProviderHealth,
}

impl RpcProvider {
    /// Create a new RPC provider
    pub fn new(name: String, endpoint: String, region: String, priority: u32) -> Self {
        Self {
            name,
            endpoint,
            region,
            priority,
            active: true,
            last_check: 0,
            health_status: ProviderHealth::Healthy,
        }
    }
}

/// Represents TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// TLS version
    pub version: String,
    /// Whether to use mTLS
    pub use_mtls: bool,
    /// Certificate pins
    pub cert_pins: Vec<String>,
    /// Certificate rotation interval in seconds
    pub cert_rotation_interval: u64,
}

/// Represents provider health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProviderHealth {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Represents failover policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverPolicy {
    /// Maximum number of retries
    pub max_retries: u32,
    /// Timeout in milliseconds
    pub timeout_ms: u64,
    /// Retry delay in milliseconds
    pub retry_delay_ms: u64,
    /// Whether to use exponential backoff
    pub exponential_backoff: bool,
}

/// Represents circuit breaker state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CircuitBreakerState {
    Closed,
    Open,
    HalfOpen,
}

/// Represents a circuit breaker for RPC providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreaker {
    /// Circuit breaker state
    pub state: CircuitBreakerState,
    /// Failure count
    pub failure_count: u32,
    /// Last failure timestamp
    pub last_failure: u64,
    /// Timeout for open state in milliseconds
    pub timeout_ms: u64,
    /// Threshold for opening circuit breaker
    pub failure_threshold: u32,
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    pub fn new(failure_threshold: u32, timeout_ms: u64) -> Self {
        Self {
            state: CircuitBreakerState::Closed,
            failure_count: 0,
            last_failure: 0,
            timeout_ms,
            failure_threshold,
        }
    }

    /// Record a failure
    pub fn record_failure(&mut self, timestamp: u64) {
        self.failure_count += 1;
        self.last_failure = timestamp;
        
        if self.failure_count >= self.failure_threshold {
            self.state = CircuitBreakerState::Open;
        }
    }

    /// Reset the circuit breaker
    pub fn reset(&mut self) {
        self.state = CircuitBreakerState::Closed;
        self.failure_count = 0;
        self.last_failure = 0;
    }

    /// Check if the circuit breaker allows requests
    pub fn can_execute(&mut self, timestamp: u64) -> bool {
        match self.state {
            CircuitBreakerState::Closed => true,
            CircuitBreakerState::Open => {
                if timestamp - self.last_failure > self.timeout_ms {
                    self.state = CircuitBreakerState::HalfOpen;
                    true
                } else {
                    false
                }
            }
            CircuitBreakerState::HalfOpen => true,
        }
    }
}

/// Custom error type for RPC resilience operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RpcResilienceError {
    /// Provider not found
    ProviderNotFound(String),
    /// Provider unhealthy
    ProviderUnhealthy(String),
    /// Circuit breaker open
    CircuitBreakerOpen(String),
    /// TLS configuration error
    TlsError(String),
    /// Timeout error
    TimeoutError(String),
    /// Generic error
    GenericError(String),
}

impl std::fmt::Display for RpcResilienceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RpcResilienceError::ProviderNotFound(name) => write!(f, "Provider not found: {}", name),
            RpcResilienceError::ProviderUnhealthy(name) => write!(f, "Provider unhealthy: {}", name),
            RpcResilienceError::CircuitBreakerOpen(name) => write!(f, "Circuit breaker open: {}", name),
            RpcResilienceError::TlsError(msg) => write!(f, "TLS error: {}", msg),
            RpcResilienceError::TimeoutError(msg) => write!(f, "Timeout error: {}", msg),
            RpcResilienceError::GenericError(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for RpcResilienceError {}

/// Manages RPC resilience including provider selection, failover, and circuit breaking
pub struct RpcResilienceManager {
    /// TLS configuration
    pub tls_config: TlsConfig,
    /// RPC providers
    pub providers: HashMap<String, RpcProvider>,
    /// Circuit breakers for each provider
    pub circuit_breakers: HashMap<String, CircuitBreaker>,
    /// Failover policy
    pub failover_policy: FailoverPolicy,
}

impl RpcResilienceManager {
    /// Create a new RPC resilience manager
    pub fn new(tls_config: TlsConfig, failover_policy: FailoverPolicy) -> Self {
        Self {
            tls_config,
            providers: HashMap::new(),
            circuit_breakers: HashMap::new(),
            failover_policy,
        }
    }

    /// Add an RPC provider
    pub fn add_provider(&mut self, provider: RpcProvider) {
        let provider_name = provider.name.clone();
        self.providers.insert(provider_name.clone(), provider);
        self.circuit_breakers.insert(
            provider_name,
            CircuitBreaker::new(5, 60000), // Default: 5 failures, 60s timeout
        );
    }

    /// Remove an RPC provider
    pub fn remove_provider(&mut self, name: &str) -> Result<(), RpcResilienceError> {
        if self.providers.remove(name).is_some() {
            self.circuit_breakers.remove(name);
            Ok(())
        } else {
            Err(RpcResilienceError::ProviderNotFound(name.to_string()))
        }
    }

    /// Update provider health status
    pub fn update_provider_health(&mut self, name: &str, health: ProviderHealth) -> Result<(), RpcResilienceError> {
        let timestamp = self.current_timestamp();
        if let Some(provider) = self.providers.get_mut(name) {
            provider.health_status = health;
            provider.last_check = timestamp;
            Ok(())
        } else {
            Err(RpcResilienceError::ProviderNotFound(name.to_string()))
        }
    }

    /// Get current timestamp in seconds
    fn current_timestamp(&self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    /// Get healthy providers sorted by priority
    pub fn get_healthy_providers(&self) -> Vec<RpcProvider> {
        let timestamp = self.current_timestamp();
        let mut healthy_providers: Vec<RpcProvider> = self
            .providers
            .iter()
            .filter(|(_, provider)| {
                provider.active
                    && provider.health_status == ProviderHealth::Healthy
            })
            .map(|(_, provider)| provider.clone())
            .collect();
        
        // Sort by priority (lower is higher priority)
        healthy_providers.sort_by_key(|p| p.priority);
        healthy_providers
    }

    /// Check if a provider is healthy based on circuit breaker status
    fn is_provider_healthy(&mut self, name: &str, timestamp: u64) -> Result<bool, RpcResilienceError> {
        if let Some(cb) = self.circuit_breakers.get_mut(name) {
            Ok(cb.can_execute(timestamp))
        } else {
            Err(RpcResilienceError::ProviderNotFound(name.to_string()))
        }
    }

    /// Record a successful RPC call
    pub fn record_success(&mut self, provider_name: &str) {
        if let Some(cb) = self.circuit_breakers.get_mut(provider_name) {
            cb.reset();
        }
    }

    /// Record a failed RPC call
    pub fn record_failure(&mut self, provider_name: &str) {
        let timestamp = self.current_timestamp();
        if let Some(cb) = self.circuit_breakers.get_mut(provider_name) {
            cb.record_failure(timestamp);
        }
    }

    /// Execute an RPC call with resilience measures
    pub async fn execute_rpc_call<T, F, Fut>(&mut self, mut operation: F) -> Result<T, RpcResilienceError>
    where
        F: FnMut(&RpcProvider) -> Fut,
        Fut: std::future::Future<Output = Result<T, Box<dyn std::error::Error + Send + Sync>>>,
    {
        let timestamp = self.current_timestamp();
        let healthy_providers = self.get_healthy_providers();
        
        if healthy_providers.is_empty() {
            return Err(RpcResilienceError::GenericError("No healthy providers available".to_string()));
        }

        let mut last_error = None;
        
        // Try each provider according to failover policy
        for (attempt, provider) in healthy_providers.iter().enumerate() {
            // Check circuit breaker
            if !self.is_provider_healthy(&provider.name, timestamp).unwrap_or(false) {
                last_error = Some(RpcResilienceError::CircuitBreakerOpen(provider.name.clone()));
                continue;
            }

            // Execute with timeout
            let timeout_duration = std::time::Duration::from_millis(self.failover_policy.timeout_ms);
            let result = tokio::time::timeout(timeout_duration, operation(provider)).await;

            match result {
                Ok(Ok(response)) => {
                    self.record_success(&provider.name);
                    return Ok(response);
                }
                Ok(Err(_e)) => {
                    self.record_failure(&provider.name);
                    last_error = Some(RpcResilienceError::GenericError("RPC call failed".to_string()));
                }
                Err(_) => {
                    self.record_failure(&provider.name);
                    last_error = Some(RpcResilienceError::TimeoutError(provider.name.clone()));
                }
            }

            // If we've reached max retries, don't continue
            if attempt >= self.failover_policy.max_retries as usize {
                break;
            }

            // Wait before retry with exponential backoff if enabled
            let delay_ms = if self.failover_policy.exponential_backoff {
                self.failover_policy.retry_delay_ms * (2_u64.pow(attempt as u32))
            } else {
                self.failover_policy.retry_delay_ms
            };
            
            tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
        }

        Err(last_error.unwrap_or(RpcResilienceError::GenericError("All providers failed".to_string())))
    }
}