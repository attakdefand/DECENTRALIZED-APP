//! API service
//!
//! This service provides RESTful APIs for querying off-chain data.

use anyhow::Result;
use axum::{
    extract::{Json, Query, State},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    routing::get,
    Router,
};
use axum::middleware::from_fn_with_state;
use std::net::SocketAddr;
use tracing::info;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// Import modules
mod api_middleware;
mod models;
mod contract;
mod contract_middleware;
mod malformed_field_middleware;
mod rate_limit_middleware;
mod auth_middleware;
mod allowlist_middleware;

use api_middleware::validate_payload;
use models::{Market, Order, Pool};
use models::pool::PoolRequest;
use models::pool::PoolResponse;
use models::order::OrderRequest;
use models::order::OrderResponse;
use models::market::MarketRequest;
use models::market::MarketResponse;

use contract_middleware::contract_validation_middleware;
use malformed_field_middleware::malformed_field_rejection_middleware;
use rate_limit_middleware::rate_limit_middleware;
use auth_middleware::auth_middleware;
use allowlist_middleware::allowlist_middleware;

// Import AppState and Metrics from the library
use api_service::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    init_tracing();

    info!("Starting API service");

    // Initialize state with all components
    let state = AppState::new();

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
        .layer(from_fn_with_state(state.clone(), track_metrics))
        // Add allowlist middleware (Service Contract Allowlist)
        .layer(from_fn_with_state(state.clone(), allowlist_middleware))
        // Add authentication middleware (Auth at Edge)
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        // Add rate limiting middleware
        .layer(from_fn_with_state(state.clone(), rate_limit_middleware))
        // Add contract validation middleware
        .layer(from_fn_with_state(state.clone(), contract_validation_middleware))
        // Add malformed field rejection middleware
        .layer(from_fn_with_state(state.clone(), malformed_field_rejection_middleware));

    // Run our app with hyper on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("API server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// Middleware to track request metrics
async fn track_metrics(
    State(state): State<AppState>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let start = std::time::Instant::now();
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

    state
        .metrics
        .request_durations
        .get_or_create(&labels)
        .observe(duration);

    // Track errors (4xx and 5xx status codes)
    let status_code: u16 = response.status().as_u16();
    if status_code >= 400 {
        let error_labels = vec![
            ("method".to_string(), method_str),
            ("path".to_string(), path_str),
            ("status".to_string(), status_str),
        ];
        state
            .metrics
            .request_errors
            .get_or_create(&error_labels)
            .inc();
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

async fn get_pools(Query(params): Query<PoolRequest>) -> Result<Json<PoolResponse>, (StatusCode, String)> {
    // Add a trace for this endpoint
    let span = tracing::info_span!("get_pools_endpoint");
    let _enter = span.enter();

    // Validate the request parameters
    if let Err(e) = validate_payload(&params) {
        return Err((StatusCode::BAD_REQUEST, e));
    }

    // Simulate some work
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    // Create sample pools
    let pools = vec![
        Pool {
            id: "pool1".to_string(),
            token_a: params.token_a.clone(),
            token_b: params.token_b.clone(),
            reserve_a: 1000.0,
            reserve_b: 2000.0,
        },
        Pool {
            id: "pool2".to_string(),
            token_a: params.token_a.clone(),
            token_b: params.token_b.clone(),
            reserve_a: 500.0,
            reserve_b: 1500.0,
        },
    ];

    Ok(Json(PoolResponse { pools }))
}

async fn get_orders(Query(params): Query<OrderRequest>) -> Result<Json<OrderResponse>, (StatusCode, String)> {
    // Add a trace for this endpoint
    let span = tracing::info_span!("get_orders_endpoint");
    let _enter = span.enter();

    // Validate the request parameters
    if let Err(e) = validate_payload(&params) {
        return Err((StatusCode::BAD_REQUEST, e));
    }

    // Simulate some work
    tokio::time::sleep(tokio::time::Duration::from_millis(15)).await;

    // Create sample orders
    let orders = vec![
        Order {
            id: "order1".to_string(),
            user: params.user.clone(),
            market: params.market.clone(),
            side: "buy".to_string(),
            price: 100.0,
            amount: 10.0,
        },
        Order {
            id: "order2".to_string(),
            user: params.user.clone(),
            market: params.market.clone(),
            side: "sell".to_string(),
            price: 99.0,
            amount: 5.0,
        },
    ];

    Ok(Json(OrderResponse { orders }))
}

async fn get_markets(Query(params): Query<MarketRequest>) -> Result<Json<MarketResponse>, (StatusCode, String)> {
    // Add a trace for this endpoint
    let span = tracing::info_span!("get_markets_endpoint");
    let _enter = span.enter();

    // Validate the request parameters
    if let Err(e) = validate_payload(&params) {
        return Err((StatusCode::BAD_REQUEST, e));
    }

    // Simulate some work
    tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;

    // Create sample markets
    let markets = vec![
        Market {
            id: "market1".to_string(),
            base_token: params.base_token.clone(),
            quote_token: params.quote_token.clone(),
            price: 100.0,
        },
        Market {
            id: "market2".to_string(),
            base_token: params.base_token.clone(),
            quote_token: params.quote_token.clone(),
            price: 99.5,
        },
    ];

    Ok(Json(MarketResponse { markets }))
}

// Handler for Prometheus metrics endpoint
async fn metrics_handler(state: State<AppState>) -> Result<String, (StatusCode, String)> {
    let registry = state.registry.lock().await;
    let mut buffer = String::new();

    prometheus_client::encoding::text::encode(&mut buffer, &registry)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(buffer)
}

// Initialize tracing
fn init_tracing() {
    // Simple tracing initialization
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();
}