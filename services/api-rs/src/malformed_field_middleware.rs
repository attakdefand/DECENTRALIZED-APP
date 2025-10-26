//! Malformed field rejection middleware
//! 
//! This middleware rejects requests with malformed or unknown fields
//! before they reach the application logic.

use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use serde_json::{json, Value};
use std::collections::HashSet;
use chrono::Utc;

use crate::AppState;

/// Malformed field rejection middleware
/// 
/// This middleware rejects requests with malformed or unknown fields
/// and tracks rejection metrics.
pub async fn malformed_field_rejection_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, Response> {
    // Check if content type is JSON
    if let Some(content_type) = headers.get("content-type") {
        if content_type.to_str().unwrap_or("").contains("application/json") {
            // Get the request path
            let path = request.uri().path().to_string();
            
            // Convert the request back to a new request with the same body
            let (parts, body) = request.into_parts();
            let body_bytes = axum::body::to_bytes(body, 1024 * 1024).await
                .map_err(|_| {
                    let error_json = json!({
                        "error": "Failed to read request body",
                        "timestamp": Utc::now().to_rfc3339()
                    });
                    let body = serde_json::to_string(&error_json).unwrap_or_else(|_| "{}".to_string());
                    Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .header("content-type", "application/json")
                        .body(axum::body::Body::from(body))
                        .unwrap()
                })?;
            
            // Convert body to string
            let body_str = String::from_utf8(body_bytes.to_vec())
                .map_err(|_| {
                    let error_json = json!({
                        "error": "Invalid UTF-8 in request body",
                        "timestamp": Utc::now().to_rfc3339()
                    });
                    let body = serde_json::to_string(&error_json).unwrap_or_else(|_| "{}".to_string());
                    Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .header("content-type", "application/json")
                        .body(axum::body::Body::from(body))
                        .unwrap()
                })?;
            
            // Parse JSON body
            if !body_str.is_empty() {
                match serde_json::from_str::<Value>(&body_str) {
                    Ok(json_value) => {
                        // Validate fields based on endpoint
                        let validation_result = validate_known_fields(&json_value, &path);
                        
                        if !validation_result.is_valid {
                            // Track gateway rejection metric
                            state.metrics.gateway_rejections.inc();
                            
                            return Err(create_malformed_field_error_response(
                                validation_result.errors,
                                "Request contains unknown or malformed fields"
                            ));
                        }
                    }
                    Err(e) => {
                        return Err(create_malformed_field_error_response(
                            vec![e.to_string()],
                            "Invalid JSON in request body"
                        ));
                    }
                }
            }
            
            // Recreate the request with the body
            let new_request = Request::from_parts(parts, axum::body::Body::from(body_str));
            
            // Continue with the request
            Ok(next.run(new_request).await)
        } else {
            // For non-JSON content, recreate the original request and continue
            let (parts, body) = request.into_parts();
            let new_request = Request::from_parts(parts, body);
            Ok(next.run(new_request).await)
        }
    } else {
        // Continue with the request without validation if no content-type header
        Ok(next.run(request).await)
    }
}

/// Validation result for field validation
struct FieldValidationResult {
    is_valid: bool,
    errors: Vec<String>,
}

/// Validate that all fields in the JSON are known for the given endpoint
fn validate_known_fields(value: &Value, endpoint: &str) -> FieldValidationResult {
    let mut errors = Vec::new();
    
    if let Some(obj) = value.as_object() {
        let allowed_fields: HashSet<&str> = match endpoint {
            "/api/v1/pools" => [
                "id", "tokenA", "tokenB", "reserveA", "reserveB"
            ].iter().cloned().collect(),
            "/api/v1/orders" => [
                "id", "user", "market", "side", "price", "amount"
            ].iter().cloned().collect(),
            "/api/v1/markets" => [
                "id", "baseToken", "quoteToken", "price"
            ].iter().cloned().collect(),
            _ => HashSet::new(),
        };
        
        // Check for unknown fields
        for (field, _) in obj {
            if !allowed_fields.contains(field.as_str()) {
                errors.push(format!("Unknown field: {}", field));
            }
        }
    }
    
    FieldValidationResult {
        is_valid: errors.is_empty(),
        errors,
    }
}

/// Create error response for malformed field rejections
fn create_malformed_field_error_response(errors: Vec<String>, message: &str) -> Response {
    let error_json = json!({
        "error": "Malformed field rejection",
        "message": message,
        "details": errors,
        "timestamp": Utc::now().to_rfc3339()
    });
    
    let body = serde_json::to_string(&error_json).unwrap_or_else(|_| "{}".to_string());
    
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header("content-type", "application/json")
        .body(axum::body::Body::from(body))
        .unwrap()
}