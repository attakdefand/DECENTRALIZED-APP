//! Rate Limiting Tests
//! 
//! This file contains tests for the rate limiting, throttling, and burst control features

#[cfg(test)]
mod tests {
    use api_service::rate_limit_middleware::{
        RateLimitConfig, RequestRateLimitEntry, CircuitBreaker, CircuitState
    };
    use std::time::Duration;

    #[test]
    fn test_rate_limit_entry_reset() {
        let mut entry = RequestRateLimitEntry::new();
        
        // Test that we can create an entry
        assert_eq!(entry.count, 0);
        
        // Test reset functionality through public methods
        entry.reset_if_needed(Duration::from_secs(60));
        assert_eq!(entry.count, 0);
    }

    #[test]
    fn test_circuit_breaker_states() {
        let mut circuit_breaker = CircuitBreaker::new();
        
        // Test initial state
        assert_eq!(circuit_breaker.state, CircuitState::Closed);
        assert!(circuit_breaker.allow_request());
        
        // Test failure recording
        circuit_breaker.record_failure();
        assert_eq!(circuit_breaker.failure_count, 1);
        
        // Test success recording
        circuit_breaker.record_success();
        assert_eq!(circuit_breaker.failure_count, 0);
        assert_eq!(circuit_breaker.state, CircuitState::Closed);
    }

    #[test]
    fn test_rate_limit_config_default() {
        let config = RateLimitConfig::default();
        assert_eq!(config.max_requests_per_ip, 100);
        assert_eq!(config.max_requests_per_token, 1000);
        assert_eq!(config.window_duration, Duration::from_secs(60));
        assert_eq!(config.burst_size, 10);
        assert_eq!(config.surge_threshold, 50);
    }
}