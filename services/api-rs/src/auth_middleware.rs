//! Authentication Middleware
//! 
//! This middleware implements JWT and mTLS authentication at the gateway level
//! to drop bad traffic early and provide gateway auth failure logs.

use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use serde_json::json;
use chrono::Utc;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use std::collections::HashSet;

use crate::AppState;

/// JWT Claims structure
#[derive(Debug, serde::Deserialize)]
struct Claims {
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

/// Authentication result
#[derive(Debug)]
struct AuthResult {
    /// Whether authentication was successful
    is_valid: bool,
    /// Error messages if authentication failed
    errors: Vec<String>,
    /// Authenticated user ID if successful
    user_id: Option<String>,
}

/// Authentication middleware
/// 
/// This middleware performs JWT and mTLS authentication at the gateway level
/// to drop bad traffic early and provide gateway auth failure logs.
pub async fn auth_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, Response> {
    // Check if this path requires authentication
    let path = request.uri().path().to_string();
    let endpoints_requiring_auth = [
        "/api/v1/pools",
        "/api/v1/orders", 
        "/api/v1/markets"
    ];
    
    if endpoints_requiring_auth.contains(&path.as_str()) {
        // Try JWT authentication first
        let jwt_result = validate_jwt(&headers);
        
        if !jwt_result.is_valid {
            // Try mTLS authentication as fallback
            let mtls_result = validate_mtls(&headers, &request);
            
            if !mtls_result.is_valid {
                // Both authentication methods failed
                // Track gateway rejection metric
                state.metrics.gateway_rejections.inc();
                
                // Log authentication failure
                log_auth_failure(&jwt_result.errors, &mtls_result.errors, &request);
                
                return Err(create_auth_error_response(&jwt_result.errors, &mtls_result.errors));
            }
        }
    }
    
    // Continue with the request
    Ok(next.run(request).await)
}

/// Validate JWT token from Authorization header
fn validate_jwt(headers: &HeaderMap) -> AuthResult {
    // Try to get the Authorization header
    if let Some(auth_header) = headers.get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                
                // Validate JWT token
                match validate_jwt_token(token) {
                    Ok(claims) => {
                        return AuthResult {
                            is_valid: true,
                            errors: vec![],
                            user_id: Some(claims.sub),
                        };
                    }
                    Err(e) => {
                        return AuthResult {
                            is_valid: false,
                            errors: vec![format!("JWT validation failed: {}", e)],
                            user_id: None,
                        };
                    }
                }
            }
        }
    }
    
    // No JWT token found
    AuthResult {
        is_valid: false,
        errors: vec!["No valid JWT token found".to_string()],
        user_id: None,
    }
}

/// Validate JWT token
fn validate_jwt_token(token: &str) -> Result<Claims, String> {
    // Decode the header to get the algorithm
    let header = decode_header(token)
        .map_err(|e| format!("Failed to decode JWT header: {}", e))?;
    
    // For demo purposes, we'll use a simple secret
    // In production, this should be properly configured
    let secret = "your-secret-key";
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    
    // Set up validation
    let mut validation = Validation::new(
        header.alg
    );
    validation.set_audience(&["api-users"]);
    validation.set_issuer(&["your-issuer"]);
    
    // Decode and validate the token
    let token_data = decode::<Claims>(token, &decoding_key, &validation)
        .map_err(|e| format!("JWT validation error: {}", e))?;
    
    Ok(token_data.claims)
}

/// Validate mTLS authentication
fn validate_mtls(headers: &HeaderMap, request: &Request) -> AuthResult {
    // Check for client certificate information
    // In a real implementation, this would check the TLS certificate presented by the client
    // For now, we'll simulate this by checking for specific headers that would be set by a proxy
    
    // Check for client certificate header (would be set by a reverse proxy)
    if let Some(cert_header) = headers.get("x-client-cert") {
        if let Ok(cert_str) = cert_header.to_str() {
            // In a real implementation, we would validate the certificate here
            // For demo purposes, we'll just check if it exists
            if !cert_str.is_empty() {
                return AuthResult {
                    is_valid: true,
                    errors: vec![],
                    user_id: Some("mtls-user".to_string()),
                };
            }
        }
    }
    
    // Check for forwarded client certificate info
    if let Some(cert_info) = headers.get("x-forwarded-client-cert") {
        if let Ok(cert_info_str) = cert_info.to_str() {
            // In a real implementation, we would validate the certificate chain here
            // For demo purposes, we'll just check if it exists
            if !cert_info_str.is_empty() {
                return AuthResult {
                    is_valid: true,
                    errors: vec![],
                    user_id: Some("mtls-user".to_string()),
                };
            }
        }
    }
    
    AuthResult {
        is_valid: false,
        errors: vec!["No valid client certificate found".to_string()],
        user_id: None,
    }
}

/// Log authentication failure
fn log_auth_failure(jwt_errors: &[String], mtls_errors: &[String], request: &Request) {
    let timestamp = Utc::now().to_rfc3339();
    let method = request.method().to_string();
    let path = request.uri().path().to_string();
    
    // Collect all errors
    let mut all_errors = Vec::new();
    all_errors.extend_from_slice(jwt_errors);
    all_errors.extend_from_slice(mtls_errors);
    
    // Log the authentication failure
    tracing::warn!(
        timestamp = %timestamp,
        method = %method,
        path = %path,
        auth_errors = ?all_errors,
        "Gateway authentication failure"
    );
}

/// Create authentication error response
fn create_auth_error_response(jwt_errors: &[String], mtls_errors: &[String]) -> Response {
    let timestamp = Utc::now().to_rfc3339();
    
    // Collect all errors
    let mut all_errors = Vec::new();
    all_errors.extend_from_slice(jwt_errors);
    all_errors.extend_from_slice(mtls_errors);
    
    let error_json = json!({
        "error": "Authentication failed",
        "message": "Unable to authenticate request",
        "details": all_errors,
        "timestamp": timestamp
    });
    
    let body = serde_json::to_string(&error_json).unwrap_or_else(|_| "{}".to_string());
    
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .header("content-type", "application/json")
        .header("www-authenticate", "Bearer realm=\"api\"")
        .body(axum::body::Body::from(body))
        .unwrap()
}