//! API service library
//!
//! This crate provides the core functionality for the API service.

// Re-export dependencies
pub use axum;
pub use serde;
pub use serde_json;
pub use tokio;
pub use chrono;

// Export modules
pub mod api_middleware;
pub mod contract;
pub mod contract_middleware;
pub mod malformed_field_middleware;
pub mod models;
pub mod rate_limit_middleware;
pub mod auth_middleware;
pub mod allowlist_middleware;
pub mod websocket;
pub mod database;

use prometheus_client::{
    metrics::{counter::Counter, family::Family, histogram::Histogram},
    registry::Registry,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use rate_limit_middleware::{RateLimitState, RateLimitConfig};

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    /// Metrics collection
    pub metrics: Metrics,
    /// Prometheus registry
    pub registry: Arc<Mutex<Registry>>,
    /// Rate limiting state
    pub rate_limit_state: RateLimitState,
}

/// Metrics collection
#[derive(Clone)]
pub struct Metrics {
    /// Request duration histograms
    pub request_durations: Family<Vec<(String, String)>, Histogram>,
    /// Request error counters
    pub request_errors: Family<Vec<(String, String)>, Counter>,
    /// Total request counter
    pub total_requests: Counter,
    /// Gateway level rejections counter
    pub gateway_rejections: Counter,
    /// Application level rejections counter
    pub app_rejections: Counter,
}

impl AppState {
    /// Create a new AppState with default configuration
    pub fn new() -> Self {
        // Initialize metrics
        let mut registry = Registry::default();
        let request_durations: Family<Vec<(String, String)>, Histogram> =
            Family::new_with_constructor(|| {
                Histogram::new(
                    [
                        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
                    ]
                    .into_iter(),
                )
            });

        let request_errors: Family<Vec<(String, String)>, Counter> =
            Family::new_with_constructor(Counter::default);
        let total_requests = Counter::default();

        registry.register(
            "http_request_duration_seconds",
            "HTTP request latencies in seconds",
            request_durations.clone(),
        );

        registry.register(
            "http_request_errors_total",
            "Total number of HTTP request errors",
            request_errors.clone(),
        );

        registry.register(
            "http_requests_total",
            "Total number of HTTP requests",
            total_requests.clone(),
        );

        let metrics = Metrics {
            request_durations,
            request_errors,
            total_requests,
            gateway_rejections: Counter::default(),
            app_rejections: Counter::default(),
        };

        registry.register(
            "gateway_rejections_total",
            "Total number of requests rejected at gateway level",
            metrics.gateway_rejections.clone(),
        );

        registry.register(
            "app_rejections_total",
            "Total number of requests rejected at application level",
            metrics.app_rejections.clone(),
        );

        let rate_limit_state = RateLimitState::new(RateLimitConfig::default());

        Self {
            metrics: metrics.clone(),
            registry: Arc::new(Mutex::new(registry)),
            rate_limit_state,
        }
    }
}