//! API service
//!
//! This service provides RESTful APIs for querying off-chain data.

use anyhow::Result;
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};
use core::logging;
use std::net::SocketAddr;
use tracing::info;

// Add Prometheus metrics imports
use prometheus_client::{
    encoding::text::encode,
    metrics::{counter::Counter, family::Family, histogram::Histogram},
    registry::Registry,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Instant;

// Add OpenTelemetry imports
use opentelemetry::{
    global,
    trace::{TraceContextExt, Tracer},
    Context, KeyValue,
};
use opentelemetry_sdk::{trace::TracerProvider, Resource};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// Create a struct to hold our metrics
#[derive(Clone)]
struct Metrics {
    request_durations: Family<Vec<(String, String)>, Histogram>,
    request_errors: Family<Vec<(String, String)>, Counter>,
    total_requests: Counter,
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
    
    info!("Starting API service");
    
    // Initialize metrics
    let mut registry = Registry::default();
    let request_durations: Family<Vec<(String, String)>, Histogram> = Family::new_with_constructor(|| {
        Histogram::new(
            [
                0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
            ]
            .into_iter(),
        )
    });
    
    let request_errors: Family<Vec<(String, String)>, Counter> = Family::new_with_constructor(Counter::default);
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
    };
    
    let state = AppState {
        metrics: metrics.clone(),
        registry: Arc::new(Mutex::new(registry)),
    };
    
    // Build our application with routes
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/api/v1/pools", get(get_pools))
        .route("/api/v1/orders", get(get_orders))
        .route("/api/v1/markets", get(get_markets))
        .route("/metrics", get(metrics_handler))
        .with_state(state.clone())
        // Add middleware to track request durations
        .layer(middleware::from_fn_with_state(
            state.clone(),
            track_metrics,
        ));
    
    // In a real implementation, we would:
    // 1. Connect to the database
    // 2. Set up connection pools
    // 3. Add middleware for tracing, CORS, etc.
    // 4. Start the HTTP server
    
    // Run our app with hyper on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("API server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

// Initialize OpenTelemetry tracing
fn init_tracing() {
    // Create a Jaeger tracer
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("api-service")
        .install_simple()
        .expect("Failed to install OpenTelemetry tracer");
    
    // Create a tracing subscriber
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();
}

// Middleware to track request metrics
async fn track_metrics(
    State(state): State<AppState>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    
    // Create a span for this request
    let span = tracing::info_span!(
        "http_request",
        method = %method,
        path = %path,
    );
    let _enter = span.enter();
    
    state.metrics.total_requests.inc();
    
    let response = next.run(request).await;
    
    let duration = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();
    
    // Clone values for reuse
    let method_str = method.to_string();
    let path_str = path.clone();
    let status_str = status.clone();
    
    // Record the metric
    let labels = vec![
        ("method".to_string(), method_str.clone()),
        ("path".to_string(), path_str.clone()),
        ("status".to_string(), status_str.clone()),
    ];
    
    state.metrics.request_durations.get_or_create(&labels).observe(duration);
    
    // Track errors (4xx and 5xx status codes)
    let status_code: u16 = response.status().as_u16();
    if status_code >= 400 {
        let error_labels = vec![
            ("method".to_string(), method_str),
            ("path".to_string(), path_str),
            ("status".to_string(), status_str),
        ];
        state.metrics.request_errors.get_or_create(&error_labels).inc();
    }
    
    response
}

// Basic route handlers
async fn root() -> &'static str {
    // Add a trace for this endpoint
    let span = tracing::info_span!("root_endpoint");
    let _enter = span.enter();
    
    "Decentralized Application API Service"
}

async fn health_check() -> &'static str {
    // Add a trace for this endpoint
    let span = tracing::info_span!("health_check_endpoint");
    let _enter = span.enter();
    
    "OK"
}

async fn get_pools() -> &'static str {
    // Add a trace for this endpoint
    let span = tracing::info_span!("get_pools_endpoint");
    let _enter = span.enter();
    
    // Simulate some work
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    
    "Pool data would be returned here"
}

async fn get_orders() -> &'static str {
    // Add a trace for this endpoint
    let span = tracing::info_span!("get_orders_endpoint");
    let _enter = span.enter();
    
    // Simulate some work
    tokio::time::sleep(tokio::time::Duration::from_millis(15)).await;
    
    "Order data would be returned here"
}

async fn get_markets() -> &'static str {
    // Add a trace for this endpoint
    let span = tracing::info_span!("get_markets_endpoint");
    let _enter = span.enter();
    
    // Simulate some work
    tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
    
    "Market data would be returned here"
}

// Handler for Prometheus metrics endpoint
async fn metrics_handler(
    state: State<AppState>,
) -> Result<String, (StatusCode, String)> {
    let registry = state.registry.lock().await;
    let mut buffer = String::new();
    
    encode(&mut buffer, &registry)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(buffer)
}