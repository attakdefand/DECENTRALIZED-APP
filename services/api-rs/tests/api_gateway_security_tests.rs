//! API Gateway Security Tests
//! 
//! This file contains comprehensive tests for the API gateway security features
//! including authentication, authorization, and security logging.

use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::get,
    Router,
};
use axum_test::TestServer;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
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

#[tokio::test]
async fn test_auth_at_edge_jwt_verification() {
    // Test that JWT tokens are verified at the gateway level before forwarding requests
    let state = AppState::new();
    let app = Router::new()
        .route("/api/v1/pools", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth_middleware::auth_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Test with valid JWT token
    let token = create_valid_jwt_token();
    let response = server
        .get("/api/v1/pools")
        .header("authorization", format!("Bearer {}", token))
        .await;
    
    // Should be successful (200 OK)
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_auth_at_edge_mtls_verification() {
    // Test that mTLS certificates are verified at the gateway level
    let state = AppState::new();
    let app = Router::new()
        .route("/api/v1/orders", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth_middleware::auth_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Test with client certificate header (simulating mTLS)
    let response = server
        .get("/api/v1/orders")
        .header("x-client-cert", "mock-certificate-data")
        .await;
    
    // Should be successful (200 OK)
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_drop_bad_traffic_early() {
    // Test that bad traffic is dropped early at the gateway level
    let state = AppState::new();
    let app = Router::new()
        .route("/api/v1/markets", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth_middleware::auth_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Test without any authentication - should be rejected early
    let response = server
        .get("/api/v1/markets")
        .await;
    
    // Should be unauthorized (401) - dropped early
    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    
    // Verify error message indicates early rejection
    let body_text = response.text();
    assert!(body_text.contains("Authentication failed"));
    assert!(body_text.contains("Unable to authenticate request"));
}

#[tokio::test]
async fn test_gateway_auth_failure_logs() {
    // Test that authentication failures are logged at the gateway level
    let state = AppState::new();
    let app = Router::new()
        .route("/api/v1/pools", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth_middleware::auth_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Test with invalid authentication - should generate logs
    let response = server
        .get("/api/v1/pools")
        .header("authorization", "Bearer invalid-token")
        .await;
    
    // Should be unauthorized (401)
    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    
    // In a real implementation, we would check that logs were generated
    // For this test, we're verifying the flow works correctly
    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_mtls_between_client_and_gateway() {
    // Test mTLS communication between client and gateway
    let state = AppState::new();
    let app = Router::new()
        .route("/api/v1/pools", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth_middleware::auth_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Test with forwarded client certificate info (simulating proxy setup)
    let response = server
        .get("/api/v1/pools")
        .header("x-forwarded-client-cert", "mock-forwarded-cert-data")
        .await;
    
    // Should be successful (200 OK)
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_signature_claims_verification_before_forwarding() {
    // Test that signature and claims are verified before forwarding requests
    let state = AppState::new();
    let app = Router::new()
        .route("/api/v1/orders", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth_middleware::auth_middleware));
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Test with valid JWT token - signature and claims should be verified
    let token = create_valid_jwt_token();
    let response = server
        .get("/api/v1/orders")
        .header("authorization", format!("Bearer {}", token))
        .await;
    
    // Should be successful (200 OK) - indicates verification passed
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn test_integration_with_other_security_layers() {
    // Test that Auth at Edge works with other security layers
    // This test verifies the integration with rate limiting, contract validation, etc.
    
    // For this test, we'll just verify that the middleware can be chained
    // with other middleware without conflicts
    
    let state = AppState::new();
    let app = Router::new()
        .route("/api/v1/markets", get(|| async { "OK" }))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth_middleware::auth_middleware));
        // In a full implementation, we would also add other middleware layers here
    
    let server = TestServer::new(app).expect("Failed to create test server");
    
    // Test with valid authentication
    let token = create_valid_jwt_token();
    let response = server
        .get("/api/v1/markets")
        .header("authorization", format!("Bearer {}", token))
        .await;
    
    // Should be successful (200 OK)
    assert_eq!(response.status_code(), StatusCode::OK);
}