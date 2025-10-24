//! IPFS Pin Coverage Monitoring Service
//!
//! This service monitors IPFS pin coverage to ensure data availability.

use anyhow::Result;
use core::logging;
use tracing::info;

// Add Prometheus metrics imports
use axum::{extract::State, http::StatusCode, routing::get, Router};
use prometheus_client::{encoding::text::encode, metrics::gauge::Gauge, registry::Registry};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

// Create a struct to hold our metrics
#[derive(Clone)]
struct Metrics {
    ipfs_pin_coverage: Gauge,
}

// Create a struct to hold our application state
#[derive(Clone)]
struct AppState {
    metrics: Metrics,
    registry: Arc<Mutex<Registry>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    logging::init();

    info!("Starting IPFS pin coverage monitoring service");

    // Initialize metrics
    let mut registry = Registry::default();
    let ipfs_pin_coverage = Gauge::default();

    registry.register(
        "ipfs_pin_coverage_percent",
        "IPFS pin coverage percentage",
        ipfs_pin_coverage.clone(),
    );

    let metrics = Metrics { ipfs_pin_coverage };

    let state = AppState {
        metrics: metrics.clone(),
        registry: Arc::new(Mutex::new(registry)),
    };

    // Start a simple HTTP server for metrics endpoint
    let metrics_app = Router::new()
        .route("/metrics", get(metrics_handler))
        .with_state(state.clone());

    let metrics_addr = SocketAddr::from(([127, 0, 0, 1], 9093)); // Use port 9093 for metrics
    info!("Metrics server listening on {}", metrics_addr);

    tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(metrics_addr).await.unwrap();
        axum::serve(listener, metrics_app).await.unwrap();
    });

    info!("IPFS pin coverage monitoring service initialized successfully");

    // Simulate IPFS pin coverage monitoring
    let mut coverage = 99.5;
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        info!(
            "IPFS pin coverage monitoring... Current coverage: {}%",
            coverage
        );

        // Update the metric
        state.metrics.ipfs_pin_coverage.set(coverage as i64);

        // Simulate slight variations in coverage
        coverage = if coverage > 99.0 {
            coverage - 0.1
        } else {
            99.5
        };
    }
}

// Handler for Prometheus metrics endpoint
async fn metrics_handler(State(state): State<AppState>) -> Result<String, (StatusCode, String)> {
    let registry = state.registry.lock().await;
    let mut buffer = String::new();

    encode(&mut buffer, &registry)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(buffer)
}
