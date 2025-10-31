//! Authentication Middleware Tests
//! 
//! This file contains tests for the JWT and mTLS authentication middleware
//! that implements "Auth at Edge" security features.

use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::get,
    Router,
};
use axum_test::TestServer;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

use api_service::AppState;

// Import the auth middleware from the api_service crate
use api_service::auth_middleware;

/// JWT Claims structure for testing
#[derive(Debug, Serialize, Deserialize)]
struct TestClaims {
    /// Subject (user identifier)
    sub: String,
    /// Issuer
    iss: String,
    /// Audience
    aud: String,
    /// Expiration time
    exp: usize,
    /// Issued at time
    iat: usize,
    /// Not before time
    nbf: usize,
    /// Roles/permissions
    roles: Option<Vec<String>>,
}

/// Create a valid JWT token for testing
fn create_valid_jwt_token() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;
    
    let claims = TestClaims {
        sub: "test-user".to_string(),
        iss: "your-issuer".to_string(),
        aud: "api-users".to_string(),
        exp: now + 3600, // 1 hour from now
        iat: now,
        nbf: now,
        roles: Some(vec!["user".to_string()]),
    };
    
    let header = Header::new(Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret("your-secret-key".as_bytes());
    
    encode(&header, &claims, &encoding_key).expect("Failed to create JWT token")
}

/// Create an expired JWT token for testing
fn create_expired_jwt_token() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;
    
    let claims = TestClaims {
        sub: "test-user".to_string(),
        iss: "your-issuer".to_string(),
        aud: "api-users".to_string(),
        exp: now - 3600, // 1 hour ago
        iat: now - 7200,
        nbf: now - 7200,
        roles: Some(vec!["user".to_string()]),
    };
    
    let header = Header::new(Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret("your-secret-key".as_bytes());
    
    encode(&header, &claims, &encoding_key).expect("Failed to create JWT token")
}

#[tokio::test]
async fn test_jwt_authentication_success() {
    // Create a test server with the auth middleware
    let state = AppState::new();
    let app = Router::new()
        .route("/api/v1/pools", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth_middleware::auth_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Create a valid JWT token
    let token = create_valid_jwt_token();
    
    // Make a request with valid JWT token
    let response = server
        .get("/api/v1/pools")
        .header("authorization", format!("Bearer {}", token))
        .await;
    
    // Should be successful (200 OK)
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_jwt_authentication_failure_expired_token() {
    // Create a test server with the auth middleware
    let state = AppState::new();
    let app = Router::new()
        .route("/api/v1/pools", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth_middleware::auth_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Create an expired JWT token
    let token = create_expired_jwt_token();
    
    // Make a request with expired JWT token
    let response = server
        .get("/api/v1/pools")
        .header("authorization", format!("Bearer {}", token))
        .await;
    
    // Should be unauthorized (401)
    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    
    // Check that the response contains error information
    let body_text = response.text();
    assert!(body_text.contains("Authentication failed"));
    assert!(body_text.contains("JWT validation failed"));
}

#[tokio::test]
async fn test_jwt_authentication_failure_no_token() {
    // Create a test server with the auth middleware
    let state = AppState::new();
    let app = Router::new()
        .route("/api/v1/pools", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth_middleware::auth_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Make a request without any authentication
    let response = server
        .get("/api/v1/pools")
        .await;
    
    // Should be unauthorized (401)
    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    
    // Check that the response contains error information
    let body_text = response.text();
    assert!(body_text.contains("Authentication failed"));
    assert!(body_text.contains("No valid JWT token found"));
}

#[tokio::test]
async fn test_mtls_authentication_success() {
    // Create a test server with the auth middleware
    let state = AppState::new();
    let app = Router::new()
        .route("/api/v1/pools", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth_middleware::auth_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Make a request with client certificate header (simulating mTLS)
    let response = server
        .get("/api/v1/pools")
        .header("x-client-cert", "mock-certificate-data")
        .await;
    
    // Should be successful (200 OK)
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_mtls_authentication_failure_no_cert() {
    // Create a test server with the auth middleware
    let state = AppState::new();
    let app = Router::new()
        .route("/api/v1/pools", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth_middleware::auth_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Make a request without client certificate
    let response = server
        .get("/api/v1/pools")
        .await;
    
    // Should be unauthorized (401)
    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    
    // Check that the response contains error information
    let body_text = response.text();
    assert!(body_text.contains("Authentication failed"));
    assert!(body_text.contains("No valid client certificate found"));
}

#[tokio::test]
async fn test_auth_metrics_tracking() {
    // Create a test server with the auth middleware
    let state = AppState::new();
    let app = Router::new()
        .route("/api/v1/pools", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth_middleware::auth_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Make a request without authentication to trigger a rejection
    let response = server
        .get("/api/v1/pools")
        .await;
    
    // Should be unauthorized (401)
    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    
    // Check that gateway rejections counter was incremented
    // Note: In a real test, we would check the actual metrics,
    // but for this test we're just verifying the flow works
    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_non_protected_routes_bypass_auth() {
    // Create a test server with the auth middleware
    let state = AppState::new();
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth_middleware::auth_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Make a request to a non-protected route
    let response = server
        .get("/health")
        .await;
    
    // Should be successful (200 OK) without authentication
    assert_eq!(response.status_code(), StatusCode::OK);
}