//! Retry service
//!
//! This module provides retry functionality with exponential backoff for the web UI.

use futures_util::Future;
use gloo_timers::future::TimeoutFuture;

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub base_delay: f64, // in milliseconds
    pub max_delay: f64,  // in milliseconds
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: 100.0,
            max_delay: 5000.0,
            backoff_multiplier: 2.0,
        }
    }
}

/// Calculate delay for current attempt
fn calculate_delay(config: &RetryConfig, attempt: u32) -> f64 {
    let delay = config.base_delay * config.backoff_multiplier.powf(attempt as f64 - 1.0);
    delay.min(config.max_delay)
}

/// Retry a future operation with exponential backoff
pub async fn retry_future<T, E, F, Fut>(
    config: &RetryConfig,
    mut operation: F,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    let mut attempt = 0;
    
    loop {
        attempt += 1;
        
        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                if attempt >= config.max_attempts {
                    return Err(error);
                }
                
                // Calculate delay and wait
                let delay = calculate_delay(config, attempt);
                TimeoutFuture::new(delay as u32).await;
            }
        }
    }
}

/// Retry service
pub struct RetryService {
    config: RetryConfig,
}

impl RetryService {
    /// Create a new retry service with default configuration
    pub fn new() -> Self {
        Self {
            config: RetryConfig::default(),
        }
    }

    /// Create a new retry service with custom configuration
    pub fn with_config(config: RetryConfig) -> Self {
        Self { config }
    }

    /// Retry an operation with the service's configuration
    pub async fn retry<T, E, F, Fut>(&self, operation: F) -> Result<T, E>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<T, E>>,
        E: std::fmt::Debug,
    {
        retry_future(&self.config, operation).await
    }
}

impl Default for RetryService {
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
    fn test_retry_service_creation() {
        let retry = RetryService::new();
        assert!(true); // Just test that it can be created
        
        let config = RetryConfig::default();
        let retry_with_config = RetryService::with_config(config);
        assert!(true); // Just test that it can be created
    }

    #[wasm_bindgen_test]
    async fn test_delay_calculation() {
        let config = RetryConfig::default();
        let delay = calculate_delay(&config, 1);
        assert_eq!(delay, 100.0);
        
        let delay = calculate_delay(&config, 2);
        assert_eq!(delay, 200.0);
        
        let delay = calculate_delay(&config, 3);
        assert_eq!(delay, 400.0);
    }
}