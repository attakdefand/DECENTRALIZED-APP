use axum::{
    extract::Request,
    http::{header, HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use serde_json::json;
use validator::Validate;
use chrono::Utc;

/// Middleware to validate request payloads against schema
pub async fn validation_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Check if content type is JSON
    if let Some(content_type) = headers.get(header::CONTENT_TYPE) {
        if content_type.to_str().unwrap_or("").contains("application/json") {
            // Here we would normally validate the request body against the schema
            // For now, we'll just pass through
            // In a full implementation, we would deserialize and validate the payload
        }
    }
    
    Ok(next.run(request).await)
}

/// Validate a struct that implements Validate trait
pub fn validate_payload<T: Validate>(payload: &T) -> Result<(), String> {
    match payload.validate() {
        Ok(_) => Ok(()),
        Err(errors) => {
            let error_messages: Vec<String> = errors
                .field_errors()
                .iter()
                .map(|(field, errs)| {
                    format!(
                        "{}: {}",
                        field,
                        errs.iter()
                            .map(|e| e.message.clone().unwrap_or_default())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                })
                .collect();
            Err(error_messages.join("; "))
        }
    }
}

/// Create error response for validation failures
pub fn validation_error_response(error_message: &str) -> Response {
    let error_json = json!({
        "error": "Validation failed",
        "message": error_message,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    let body = serde_json::to_string(&error_json).unwrap_or_else(|_| "{}".to_string());
    
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header(header::CONTENT_TYPE, "application/json")
        .body(axum::body::Body::from(body))
        .unwrap()
}