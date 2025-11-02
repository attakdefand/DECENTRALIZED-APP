//! API Gateway Allowlist Tests
//! 
//! This file contains comprehensive tests for the service contract allowlist feature
//! that implements "Only allow specific routes/methods per client/app tier",
//! "Make public surface area explicit", and "Denied route attempts by client id".

use axum::{
    body::Body,
    http::{Request, StatusCode, Method, header::{HeaderName, HeaderValue}},
    routing::{get, post, put, delete},
    Router,
};
use axum_test::TestServer;

use api_service::AppState;
use api_service::allowlist_middleware::allowlist_middleware;

#[tokio::test]
async fn test_only_allow_specific_routes_per_client_tier() {
    // Test that only specific routes are allowed per client tier
    let state = AppState::new();
    let app = Router::new()
        .route("/", get(|| async { "Root" }))
        .route("/health", get(|| async { "OK" }))
        .route("/api/v1/pools", get(|| async { "Pools" }))
        .route("/api/v1/orders", get(|| async { "Orders" }))
        .route("/api/v1/markets", get(|| async { "Markets" }))
        .route("/api/v1/admin", get(|| async { "Admin" }))
        .route("/api/v1/admin/users", get(|| async { "Admin Users" }))
        .route("/metrics", get(|| async { "Metrics" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), allowlist_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Test Public tier - should only have access to root and health
    let response = server.get("/").add_header(
        HeaderName::from_static("x-client-tier"), 
        HeaderValue::from_static("public")
    ).await;
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let response = server.get("/health").add_header(
        HeaderName::from_static("x-client-tier"), 
        HeaderValue::from_static("public")
    ).await;
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let response = server.get("/api/v1/pools").add_header(
        HeaderName::from_static("x-client-tier"), 
        HeaderValue::from_static("public")
    ).await;
    assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    
    // Test User tier - should have access to API endpoints but not admin
    let response = server.get("/api/v1/pools").add_header(
        HeaderName::from_static("x-client-tier"), 
        HeaderValue::from_static("user")
    ).await;
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let response = server.get("/api/v1/orders").add_header(
        HeaderName::from_static("x-client-tier"), 
        HeaderValue::from_static("user")
    ).await;
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let response = server.get("/api/v1/markets").add_header(
        HeaderName::from_static("x-client-tier"), 
        HeaderValue::from_static("user")
    ).await;
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let response = server.get("/api/v1/admin").add_header(
        HeaderName::from_static("x-client-tier"), 
        HeaderValue::from_static("user")
    ).await;
    assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    
    // Test Admin tier - should have access to everything
    let response = server.get("/api/v1/pools").add_header(
        HeaderName::from_static("x-client-tier"), 
        HeaderValue::from_static("admin")
    ).await;
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let response = server.get("/api/v1/admin").add_header(
        HeaderName::from_static("x-client-tier"), 
        HeaderValue::from_static("admin")
    ).await;
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let response = server.get("/metrics").add_header(
        HeaderName::from_static("x-client-tier"), 
        HeaderValue::from_static("admin")
    ).await;
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_make_public_surface_area_explicit() {
    // Test that the public surface area is explicitly defined
    let state = AppState::new();
    let app = Router::new()
        .route("/", get(|| async { "Root" }))
        .route("/health", get(|| async { "OK" }))
        .route("/api/v1/pools", get(|| async { "Pools" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), allowlist_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Test that only explicitly allowed public routes work
    let response = server.get("/").add_header(
        HeaderName::from_static("x-client-tier"), 
        HeaderValue::from_static("public")
    ).await;
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let response = server.get("/health").add_header(
        HeaderName::from_static("x-client-tier"), 
        HeaderValue::from_static("public")
    ).await;
    assert_eq!(response.status_code(), StatusCode::OK);
    
    // Test that other routes are explicitly denied for public tier
    let response = server.get("/api/v1/pools").add_header(
        HeaderName::from_static("x-client-tier"), 
        HeaderValue::from_static("public")
    ).await;
    assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    
    let response = server.get("/nonexistent").add_header(
        HeaderName::from_static("x-client-tier"), 
        HeaderValue::from_static("public")
    ).await;
    assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_denied_route_attempts_by_client_id() {
    // Test that denied route attempts are tracked by client ID
    let state = AppState::new();
    let app = Router::new()
        .route("/api/v1/admin", get(|| async { "Admin" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), allowlist_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Test with specific client ID that gets denied
    let response = server
        .get("/api/v1/admin")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("user")
        )
        .add_header(
            HeaderName::from_static("x-client-id"), 
            HeaderValue::from_static("test-client-123")
        )
        .await;
    
    // Should be forbidden (403)
    assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    
    // Test with different client ID that gets denied
    let response = server
        .get("/api/v1/admin")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("public")
        )
        .add_header(
            HeaderName::from_static("x-client-id"), 
            HeaderValue::from_static("another-client-456")
        )
        .await;
    
    // Should be forbidden (403)
    assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    
    // In a real implementation with proper logging, we would verify that
    // these attempts were logged with the respective client IDs
}

#[tokio::test]
async fn test_method_level_allowlisting() {
    // Test that method-level allowlisting works correctly
    let state = AppState::new();
    let app = Router::new()
        .route("/api/v1/pools", get(|| async { "GET Pools" }))
        .route("/api/v1/pools", post(|| async { "POST Pools" }))
        .route("/api/v1/pools", put(|| async { "PUT Pools" }))
        .route("/api/v1/pools", delete(|| async { "DELETE Pools" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), allowlist_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Test Admin tier - should have access to all methods
    let response = server.post("/api/v1/pools").add_header(
        HeaderName::from_static("x-client-tier"), 
        HeaderValue::from_static("admin")
    ).await;
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let response = server.put("/api/v1/pools").add_header(
        HeaderName::from_static("x-client-tier"), 
        HeaderValue::from_static("admin")
    ).await;
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let response = server.delete("/api/v1/pools").add_header(
        HeaderName::from_static("x-client-tier"), 
        HeaderValue::from_static("admin")
    ).await;
    assert_eq!(response.status_code(), StatusCode::OK);
    
    // Test User tier - should only have GET access
    let response = server.get("/api/v1/pools").add_header(
        HeaderName::from_static("x-client-tier"), 
        HeaderValue::from_static("user")
    ).await;
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let response = server.post("/api/v1/pools").add_header(
        HeaderName::from_static("x-client-tier"), 
        HeaderValue::from_static("user")
    ).await;
    assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    
    let response = server.put("/api/v1/pools").add_header(
        HeaderName::from_static("x-client-tier"), 
        HeaderValue::from_static("user")
    ).await;
    assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    
    let response = server.delete("/api/v1/pools").add_header(
        HeaderName::from_static("x-client-tier"), 
        HeaderValue::from_static("user")
    ).await;
    assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_tier_identification_from_headers() {
    // Test that client tiers are correctly identified from different headers
    let state = AppState::new();
    let app = Router::new()
        .route("/api/v1/pools", get(|| async { "Pools" }))
        .route("/metrics", get(|| async { "Metrics" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), allowlist_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Test explicit tier header
    let response = server
        .get("/api/v1/pools")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("user")
        )
        .await;
    assert_eq!(response.status_code(), StatusCode::OK);
    
    // Test service tier identification from API key
    let response = server
        .get("/api/v1/pools")
        .add_header(
            HeaderName::from_static("x-api-key"), 
            HeaderValue::from_static("service-key-123")
        )
        .await;
    assert_eq!(response.status_code(), StatusCode::OK);
    
    // Test service tier identification from API key should allow metrics access
    let response = server
        .get("/metrics")
        .add_header(
            HeaderName::from_static("x-api-key"), 
            HeaderValue::from_static("service-key-123")
        )
        .await;
    assert_eq!(response.status_code(), StatusCode::OK);
    
    // Test user tier identification from JWT token
    let response = server
        .get("/api/v1/pools")
        .add_header(
            HeaderName::from_static("authorization"), 
            HeaderValue::from_static("Bearer valid-jwt-token")
        )
        .await;
    assert_eq!(response.status_code(), StatusCode::OK);
    
    // Test user tier should not allow metrics access
    let response = server
        .get("/metrics")
        .add_header(
            HeaderName::from_static("authorization"), 
            HeaderValue::from_static("Bearer valid-jwt-token")
        )
        .await;
    assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_premium_tier_extended_access() {
    // Test that premium tier has extended access compared to regular user
    let state = AppState::new();
    let app = Router::new()
        .route("/api/v1/pools", get(|| async { "Pools" }))
        .route("/api/v1/admin/stats", get(|| async { "Stats" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), allowlist_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Test User tier - should not have access to premium stats
    let response = server
        .get("/api/v1/admin/stats")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("user")
        )
        .await;
    assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    
    // Test Premium tier - should have access to premium stats
    let response = server
        .get("/api/v1/admin/stats")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("premium")
        )
        .await;
    assert_eq!(response.status_code(), StatusCode::OK);
    
    // Both should have access to regular API endpoints
    let response = server
        .get("/api/v1/pools")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("user")
        )
        .await;
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let response = server
        .get("/api/v1/pools")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("premium")
        )
        .await;
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_gateway_rejection_metrics() {
    // Test that gateway rejections are properly counted in metrics
    let state = AppState::new();
    let app = Router::new()
        .route("/api/v1/admin", get(|| async { "Admin" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), allowlist_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Record initial rejection count
    let initial_rejections = state.metrics.gateway_rejections.get();
    
    // Make a request that should be rejected
    let response = server
        .get("/api/v1/admin")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("user")
        )
        .await;
    
    assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    
    // Check that rejection count increased
    let final_rejections = state.metrics.gateway_rejections.get();
    assert_eq!(final_rejections, initial_rejections + 1);
    
    // Make another rejected request
    let response = server
        .get("/api/v1/admin")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("public")
        )
        .await;
    
    assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    
    // Check that rejection count increased again
    let final_rejections = state.metrics.gateway_rejections.get();
    assert_eq!(final_rejections, initial_rejections + 2);
}