use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use serde_json::json;
use chrono::Utc;

use crate::{contract::{validate_request_contract, contract_validation_error_response}, AppState};

/// Contract validation middleware
/// 
/// This middleware validates incoming requests against predefined contracts
/// and rejects malformed requests before they reach the application logic.
pub async fn contract_validation_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, Response> {
    // Get the request path
    let path = request.uri().path().to_string();
    
    // Only validate specific endpoints
    let endpoints_to_validate = [
        "/api/v1/pools",
        "/api/v1/orders", 
        "/api/v1/markets"
    ];
    
    if endpoints_to_validate.contains(&path.as_str()) {
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
        
        // Validate the contract
        let validation_result = validate_request_contract(&headers, &body_str, &path);
        
        if !validation_result.is_valid {
            // Track gateway rejection metric
            state.metrics.gateway_rejections.inc();
            return Err(contract_validation_error_response(validation_result.errors));
        }
        
        // Recreate the request with the body
        let new_request = Request::from_parts(parts, axum::body::Body::from(body_str));
        
        // Continue with the request
        Ok(next.run(new_request).await)
    } else {
        // Continue with the request without validation
        Ok(next.run(request).await)
    }
}