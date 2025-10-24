//! Off-chain indexer service
//!
//! This service streams blockchain events and materializes them into queryable database tables.

use anyhow::Result;
use core::logging;
use tracing::info;

// Add Prometheus metrics imports
use prometheus_client::{
    encoding::text::encode,
    metrics::{gauge::Gauge},
    registry::Registry,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::net::SocketAddr;
use axum::{
    extract::State,
    http::StatusCode,
    routing::get,
    Router,
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// Create a struct to hold our metrics
#[derive(Clone)]
struct Metrics {
    indexer_lag: Gauge,
}

// Create a struct to hold our application state
#[derive(Clone)]
struct AppState {
    metrics: Metrics,
    registry: Arc<Mutex<Registry>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize OpenTelemetry tracing
    init_tracing();
    
    // Initialize logging
    logging::init();
    
    info!("Starting off-chain indexer service");
    
    // Initialize metrics
    let mut registry = Registry::default();
    let indexer_lag = Gauge::default();
    
    registry.register(
        "indexer_lag_blocks",
        "Current indexer lag in blocks",
        indexer_lag.clone(),
    );
    
    let metrics = Metrics {
        indexer_lag,
    };
    
    let state = AppState {
        metrics: metrics.clone(),
        registry: Arc::new(Mutex::new(registry)),
    };
    
    // Start a simple HTTP server for metrics endpoint
    let metrics_app = Router::new()
        .route("/metrics", get(metrics_handler))
        .with_state(state.clone());
    
    let metrics_addr = SocketAddr::from(([127, 0, 0, 1], 9091)); // Use port 9091 for metrics
    info!("Metrics server listening on {}", metrics_addr);
    
    tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(metrics_addr).await.unwrap();
        axum::serve(listener, metrics_app).await.unwrap();
    });
    
    // In a real implementation, we would:
    // 1. Connect to the database
    // 2. Connect to the blockchain RPC
    // 3. Stream events from the blockchain
    // 4. Process and store events in the database
    // 5. Materialize read models from events
    
    // Example database connection (would need actual DB URL)
    // let pool = PgPool::connect("postgresql://user:password@localhost/database").await?;
    
    info!("Indexer service initialized successfully");
    
    // Simulate indexer work with metrics updates
    let mut block_count = 0;
    loop {
        // Create a span for this indexing cycle
        let span = tracing::info_span!("indexing_cycle", block_number = block_count);
        let _enter = span.enter();
        
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        block_count += 1;
        info!("Indexer service running... Block: {}", block_count);
        
        // Simulate updating the indexer lag metric
        // In a real implementation, this would be calculated based on:
        // - Current blockchain head block number
        // - Last indexed block number
        // - Lag = head - indexed
        state.metrics.indexer_lag.set(block_count % 5); // Simulate varying lag
    }
}

// Initialize tracing
fn init_tracing() {
    // Simple tracing initialization
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

// Handler for Prometheus metrics endpoint
async fn metrics_handler(
    State(state): State<AppState>,
) -> Result<String, (StatusCode, String)> {
    let registry = state.registry.lock().await;
    let mut buffer = String::new();
    
    encode(&mut buffer, &registry)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(buffer)
}