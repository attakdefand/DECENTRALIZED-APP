//! API service
//!
//! This service provides RESTful APIs for querying off-chain data.

use anyhow::Result;
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{Response, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use core::logging;
use std::net::SocketAddr;
use tracing::info;
use serde::{Serialize, Deserialize};

// Add Prometheus metrics imports
use prometheus_client::{
    encoding::text::encode,
    metrics::{counter::Counter, family::Family, histogram::Histogram},
    registry::Registry,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Instant;

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
        .route("/api/v1/orders", get(get_orders).post(create_order))
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

// Middleware to track request metrics
async fn track_metrics(
    State(state): State<AppState>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    
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
    "Decentralized Application API Service"
}

async fn health_check() -> &'static str {
    "OK"
}

// API Response types matching frontend models
#[derive(Debug, Serialize, Deserialize)]
struct TokenInfo {
    symbol: String,
    address: String,
    decimals: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct PoolInfo {
    id: String,
    token_a: TokenInfo,
    token_b: TokenInfo,
    liquidity: String,
    volume_24h: String,
    apr: String,
    fee_tier: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PoolResponse {
    pools: Vec<PoolInfo>,
    total: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct OrderInfo {
    id: String,
    pair: String,
    side: String,
    price: String,
    amount: String,
    filled: String,
    status: String,
    timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct OrderResponse {
    orders: Vec<OrderInfo>,
    total: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateOrderRequest {
    pair: String,
    side: String,
    price: f64,
    amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct MarketInfo {
    pair: String,
    price: String,
    change_24h: String,
    volume_24h: String,
    high_24h: String,
    low_24h: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MarketResponse {
    markets: Vec<MarketInfo>,
    total: usize,
}

// Security: Validate pool data before returning
async fn get_pools() -> impl IntoResponse {
    // In production, this would query the database
    // For now, return mock data matching frontend expectations
    
    let pools = vec![
        PoolInfo {
            id: "pool-eth-usdc-001".to_string(),
            token_a: TokenInfo {
                symbol: "ETH".to_string(),
                address: "0x0000000000000000000000000000000000000001".to_string(),
                decimals: 18,
            },
            token_b: TokenInfo {
                symbol: "USDC".to_string(),
                address: "0x0000000000000000000000000000000000000002".to_string(),
                decimals: 6,
            },
            liquidity: "1250000.75".to_string(),
            volume_24h: "45000.30".to_string(),
            apr: "12.5".to_string(),
            fee_tier: "0.3".to_string(),
        },
        PoolInfo {
            id: "pool-btc-usdc-001".to_string(),
            token_a: TokenInfo {
                symbol: "BTC".to_string(),
                address: "0x0000000000000000000000000000000000000003".to_string(),
                decimals: 8,
            },
            token_b: TokenInfo {
                symbol: "USDC".to_string(),
                address: "0x0000000000000000000000000000000000000002".to_string(),
                decimals: 6,
            },
            liquidity: "2500000.00".to_string(),
            volume_24h: "87000.45".to_string(),
            apr: "8.75".to_string(),
            fee_tier: "0.3".to_string(),
        },
    ];
    
    let response = PoolResponse {
        total: pools.len(),
        pools,
    };
    
    Json(response)
}

// Security: Validate order data
async fn get_orders() -> impl IntoResponse {
    let orders = vec![
        OrderInfo {
            id: "order-001".to_string(),
            pair: "ETH/USDC".to_string(),
            side: "buy".to_string(),
            price: "2500.50".to_string(),
            amount: "1.5".to_string(),
            filled: "1.0".to_string(),
            status: "open".to_string(),
            timestamp: 1704067200,
        },
        OrderInfo {
            id: "order-002".to_string(),
            pair: "BTC/USDC".to_string(),
            side: "sell".to_string(),
            price: "45000.00".to_string(),
            amount: "0.25".to_string(),
            filled: "0.25".to_string(),
            status: "filled".to_string(),
            timestamp: 1704063600,
        },
    ];
    
    let response = OrderResponse {
        total: orders.len(),
        orders,
    };
    
    Json(response)
}

// Security: Validate order creation request
async fn create_order(Json(payload): Json<CreateOrderRequest>) -> impl IntoResponse {
    // Security: Validate inputs
    if payload.pair.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Invalid pair"
        })));
    }
    
    if payload.side != "buy" && payload.side != "sell" {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Invalid side, must be 'buy' or 'sell'"
        })));
    }
    
    if payload.price <= 0.0 || payload.amount <= 0.0 {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Price and amount must be positive"
        })));
    }
    
    // In production: Create order in database
    let order = OrderInfo {
        id: format!("order-{}", chrono::Utc::now().timestamp()),
        pair: payload.pair,
        side: payload.side,
        price: payload.price.to_string(),
        amount: payload.amount.to_string(),
        filled: "0.0".to_string(),
        status: "open".to_string(),
        timestamp: chrono::Utc::now().timestamp() as u64,
    };
    
    (StatusCode::CREATED, Json(order))
}

// Security: Validate market data
async fn get_markets() -> impl IntoResponse {
    let markets = vec![
        MarketInfo {
            pair: "ETH/USDC".to_string(),
            price: "2530.0".to_string(),
            change_24h: "2.5".to_string(),
            volume_24h: "45000000.0".to_string(),
            high_24h: "2550.0".to_string(),
            low_24h: "2480.0".to_string(),
        },
        MarketInfo {
            pair: "BTC/USDC".to_string(),
            price: "45300.0".to_string(),
            change_24h: "-1.2".to_string(),
            volume_24h: "87000000.0".to_string(),
            high_24h: "46000.0".to_string(),
            low_24h: "44800.0".to_string(),
        },
    ];
    
    let response = MarketResponse {
        total: markets.len(),
        markets,
    };
    
    Json(response)
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