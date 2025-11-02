use axum::{
    http::{HeaderMap, StatusCode},
    response::Response,
};
use serde_json::{json, Value};
use chrono::Utc;

/// Contract validation result
#[derive(Debug)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
}

/// Validate request against contract
pub fn validate_request_contract(
    headers: &HeaderMap,
    body: &str,
    endpoint: &str,
) -> ValidationResult {
    let mut errors = Vec::new();
    
    // Check content type
    if let Some(content_type) = headers.get("content-type") {
        if content_type.to_str().unwrap_or("").contains("application/json") {
            // Parse JSON body
            match serde_json::from_str::<Value>(body) {
                Ok(json_value) => {
                    // Validate against schema based on endpoint
                    match endpoint {
                        "/api/v1/pools" => validate_pool_contract(&json_value, &mut errors),
                        "/api/v1/orders" => validate_order_contract(&json_value, &mut errors),
                        "/api/v1/markets" => validate_market_contract(&json_value, &mut errors),
                        _ => errors.push(format!("Unknown endpoint: {}", endpoint)),
                    }
                }
                Err(e) => {
                    errors.push(format!("Invalid JSON: {}", e));
                }
            }
        } else {
            errors.push("Unsupported content type".to_string());
        }
    } else {
        errors.push("Missing content-type header".to_string());
    }
    
    ValidationResult {
        is_valid: errors.is_empty(),
        errors,
    }
}

/// Validate pool contract
fn validate_pool_contract(value: &Value, errors: &mut Vec<String>) {
    if let Some(obj) = value.as_object() {
        // Check required fields
        let required_fields = ["id", "tokenA", "tokenB", "reserveA", "reserveB"];
        for field in &required_fields {
            if !obj.contains_key(&field.to_string()) {
                errors.push(format!("Missing required field: {}", field));
            }
        }
        
        // Validate field types
        if let Some(id) = obj.get("id") {
            if !id.is_string() {
                errors.push("Field 'id' must be a string".to_string());
            }
        }
        
        if let Some(token_a) = obj.get("tokenA") {
            if !token_a.is_string() {
                errors.push("Field 'tokenA' must be a string".to_string());
            }
        }
        
        if let Some(token_b) = obj.get("tokenB") {
            if !token_b.is_string() {
                errors.push("Field 'tokenB' must be a string".to_string());
            }
        }
        
        if let Some(reserve_a) = obj.get("reserveA") {
            if !reserve_a.is_number() {
                errors.push("Field 'reserveA' must be a number".to_string());
            }
        }
        
        if let Some(reserve_b) = obj.get("reserveB") {
            if !reserve_b.is_number() {
                errors.push("Field 'reserveB' must be a number".to_string());
            }
        }
    } else {
        errors.push("Request body must be a JSON object".to_string());
    }
}

/// Validate order contract
fn validate_order_contract(value: &Value, errors: &mut Vec<String>) {
    if let Some(obj) = value.as_object() {
        // Check required fields
        let required_fields = ["id", "user", "market", "side", "price", "amount"];
        for field in &required_fields {
            if !obj.contains_key(&field.to_string()) {
                errors.push(format!("Missing required field: {}", field));
            }
        }
        
        // Validate field types
        if let Some(id) = obj.get("id") {
            if !id.is_string() {
                errors.push("Field 'id' must be a string".to_string());
            }
        }
        
        if let Some(user) = obj.get("user") {
            if !user.is_string() {
                errors.push("Field 'user' must be a string".to_string());
            }
        }
        
        if let Some(market) = obj.get("market") {
            if !market.is_string() {
                errors.push("Field 'market' must be a string".to_string());
            }
        }
        
        if let Some(side) = obj.get("side") {
            if let Some(side_str) = side.as_str() {
                if side_str != "buy" && side_str != "sell" {
                    errors.push("Field 'side' must be 'buy' or 'sell'".to_string());
                }
            } else {
                errors.push("Field 'side' must be a string".to_string());
            }
        }
        
        if let Some(price) = obj.get("price") {
            if !price.is_number() {
                errors.push("Field 'price' must be a number".to_string());
            }
        }
        
        if let Some(amount) = obj.get("amount") {
            if !amount.is_number() {
                errors.push("Field 'amount' must be a number".to_string());
            }
        }
    } else {
        errors.push("Request body must be a JSON object".to_string());
    }
}

/// Validate market contract
fn validate_market_contract(value: &Value, errors: &mut Vec<String>) {
    if let Some(obj) = value.as_object() {
        // Check required fields
        let required_fields = ["id", "baseToken", "quoteToken", "price"];
        for field in &required_fields {
            if !obj.contains_key(&field.to_string()) {
                errors.push(format!("Missing required field: {}", field));
            }
        }
        
        // Validate field types
        if let Some(id) = obj.get("id") {
            if !id.is_string() {
                errors.push("Field 'id' must be a string".to_string());
            }
        }
        
        if let Some(base_token) = obj.get("baseToken") {
            if !base_token.is_string() {
                errors.push("Field 'baseToken' must be a string".to_string());
            }
        }
        
        if let Some(quote_token) = obj.get("quoteToken") {
            if !quote_token.is_string() {
                errors.push("Field 'quoteToken' must be a string".to_string());
            }
        }
        
        if let Some(price) = obj.get("price") {
            if !price.is_number() {
                errors.push("Field 'price' must be a number".to_string());
            }
        }
    } else {
        errors.push("Request body must be a JSON object".to_string());
    }
}

/// Create contract validation error response
pub fn contract_validation_error_response(errors: Vec<String>) -> Response {
    let error_json = json!({
        "error": "Contract validation failed",
        "message": "The request does not conform to the expected contract",
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