//! Allowlist Middleware Tests
//! 
//! This file contains tests for the service contract allowlist middleware
//! that implements route/method allowlisting per client tier.

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
async fn test_public_tier_access_allowed_routes() {
    // Create a test server with the allowlist middleware
    let state = AppState::new();
    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/health", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), allowlist_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Test public tier accessing allowed routes
    let response = server
        .get("/")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("public")
        )
        .await;
    
    // Should be successful (200 OK)
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let response = server
        .get("/health")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("public")
        )
        .await;
    
    // Should be successful (200 OK)
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_public_tier_access_denied_routes() {
    // Create a test server with the allowlist middleware
    let state = AppState::new();
    let app = Router::new()
        .route("/api/v1/pools", get(|| async { "OK" }))
        .route("/api/v1/orders", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), allowlist_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Test public tier accessing restricted routes
    let response = server
        .get("/api/v1/pools")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("public")
        )
        .await;
    
    // Should be forbidden (403)
    assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
    
    let response = server
        .get("/api/v1/orders")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("public")
        )
        .await;
    
    // Should be forbidden (403)
    assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_user_tier_access_allowed_routes() {
    // Create a test server with the allowlist middleware
    let state = AppState::new();
    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/health", get(|| async { "OK" }))
        .route("/api/v1/pools", get(|| async { "OK" }))
        .route("/api/v1/orders", get(|| async { "OK" }))
        .route("/api/v1/markets", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), allowlist_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Test user tier accessing allowed routes
    let response = server
        .get("/api/v1/pools")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("user")
        )
        .await;
    
    // Should be successful (200 OK)
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let response = server
        .get("/api/v1/orders")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("user")
        )
        .await;
    
    // Should be successful (200 OK)
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let response = server
        .get("/api/v1/markets")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("user")
        )
        .await;
    
    // Should be successful (200 OK)
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_user_tier_access_denied_routes() {
    // Create a test server with the allowlist middleware
    let state = AppState::new();
    let app = Router::new()
        .route("/api/v1/admin", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), allowlist_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Test user tier accessing admin routes
    let response = server
        .get("/api/v1/admin")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("user")
        )
        .await;
    
    // Should be forbidden (403)
    assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_admin_tier_access_all_routes() {
    // Create a test server with the allowlist middleware
    let state = AppState::new();
    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/health", get(|| async { "OK" }))
        .route("/api/v1/pools", get(|| async { "OK" }))
        .route("/api/v1/pools", post(|| async { "OK" }))
        .route("/api/v1/pools", put(|| async { "OK" }))
        .route("/api/v1/pools", delete(|| async { "OK" }))
        .route("/api/v1/admin", get(|| async { "OK" }))
        .route("/api/v1/admin", post(|| async { "OK" }))
        .route("/metrics", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), allowlist_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Test admin tier accessing all routes
    let response = server
        .get("/api/v1/pools")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("admin")
        )
        .await;
    
    // Should be successful (200 OK)
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let response = server
        .post("/api/v1/pools")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("admin")
        )
        .await;
    
    // Should be successful (200 OK)
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let response = server
        .put("/api/v1/pools")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("admin")
        )
        .await;
    
    // Should be successful (200 OK)
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let response = server
        .delete("/api/v1/pools")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("admin")
        )
        .await;
    
    // Should be successful (200 OK)
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let response = server
        .get("/api/v1/admin")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("admin")
        )
        .await;
    
    // Should be successful (200 OK)
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let response = server
        .get("/metrics")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("admin")
        )
        .await;
    
    // Should be successful (200 OK)
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_service_tier_access_allowed_routes() {
    // Create a test server with the allowlist middleware
    let state = AppState::new();
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/api/v1/pools", get(|| async { "OK" }))
        .route("/api/v1/pools", post(|| async { "OK" }))
        .route("/api/v1/pools", put(|| async { "OK" }))
        .route("/api/v1/pools", delete(|| async { "OK" }))
        .route("/metrics", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), allowlist_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Test service tier accessing allowed routes
    let response = server
        .get("/api/v1/pools")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("service")
        )
        .add_header(
            HeaderName::from_static("x-api-key"), 
            HeaderValue::from_static("service-key")
        )
        .await;
    
    // Should be successful (200 OK)
    assert_eq!(response.status_code(), StatusCode::OK);
    
    let response = server
        .post("/api/v1/pools")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("service")
        )
        .add_header(
            HeaderName::from_static("x-api-key"), 
            HeaderValue::from_static("service-key")
        )
        .await;
    
    // Should be successful (200 OK)
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_method_allowlisting() {
    // Create a test server with the allowlist middleware
    let state = AppState::new();
    let app = Router::new()
        .route("/api/v1/pools", get(|| async { "OK" }))
        .route("/api/v1/pools", post(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), allowlist_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Test user tier accessing allowed GET method
    let response = server
        .get("/api/v1/pools")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("user")
        )
        .await;
    
    // Should be successful (200 OK)
    assert_eq!(response.status_code(), StatusCode::OK);
    
    // Test user tier accessing forbidden POST method
    let response = server
        .post("/api/v1/pools")
        .add_header(
            HeaderName::from_static("x-client-tier"), 
            HeaderValue::from_static("user")
        )
        .await;
    
    // Should be forbidden (403)
    assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_client_id_tracking() {
    // Create a test server with the allowlist middleware
    let state = AppState::new();
    let app = Router::new()
        .route("/api/v1/admin", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), allowlist_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Test with specific client ID
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
    
    // In a real implementation, we would check that the client ID was logged
    // For this test, we're just verifying the flow works
    assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_default_to_public_tier() {
    // Create a test server with the allowlist middleware
    let state = AppState::new();
    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/health", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), allowlist_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Test without specifying client tier (should default to public)
    let response = server
        .get("/")
        .await;
    
    // Should be successful (200 OK) - public routes are allowed
    assert_eq!(response.status_code(), StatusCode::OK);
    
    // Test without specifying client tier on restricted route
    let response = server
        .get("/api/v1/pools")
        .await;
    
    // Should be forbidden (403) - not a public route
    assert_eq!(response.status_code(), StatusCode::FORBIDDEN);
}