//! Allowlist Middleware
//! 
//! This middleware implements service contract allowlisting to only allow specific 
//! routes/methods per client/app tier, making the public surface area explicit 
//! and tracking denied route attempts by client ID.

use axum::{
    extract::{Request, State},
    http::{HeaderMap, Method, StatusCode},
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use chrono::Utc;

use crate::AppState;

/// Client tier representing different levels of access
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClientTier {
    /// Public client with limited access
    Public,
    /// Authenticated user with standard access
    User,
    /// Premium user with extended access
    Premium,
    /// Admin user with full access
    Admin,
    /// Service-to-service communication
    Service,
}

/// Serializable representation of HTTP methods for service contracts
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SerializableMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    PATCH,
    TRACE,
}

impl From<&Method> for SerializableMethod {
    fn from(method: &Method) -> Self {
        match method {
            &Method::GET => SerializableMethod::GET,
            &Method::POST => SerializableMethod::POST,
            &Method::PUT => SerializableMethod::PUT,
            &Method::DELETE => SerializableMethod::DELETE,
            &Method::HEAD => SerializableMethod::HEAD,
            &Method::OPTIONS => SerializableMethod::OPTIONS,
            &Method::CONNECT => SerializableMethod::CONNECT,
            &Method::PATCH => SerializableMethod::PATCH,
            &Method::TRACE => SerializableMethod::TRACE,
            _ => {
                // For custom methods, we'll use the string representation
                // This is a simplification for the demo
                SerializableMethod::GET
            }
        }
    }
}

impl From<SerializableMethod> for Method {
    fn from(serializable: SerializableMethod) -> Self {
        match serializable {
            SerializableMethod::GET => Method::GET,
            SerializableMethod::POST => Method::POST,
            SerializableMethod::PUT => Method::PUT,
            SerializableMethod::DELETE => Method::DELETE,
            SerializableMethod::HEAD => Method::HEAD,
            SerializableMethod::OPTIONS => Method::OPTIONS,
            SerializableMethod::CONNECT => Method::CONNECT,
            SerializableMethod::PATCH => Method::PATCH,
            SerializableMethod::TRACE => Method::TRACE,
        }
    }
}

/// Service contract defining allowed routes and methods for a client tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceContract {
    /// Client tier this contract applies to
    pub tier: ClientTier,
    /// Allowed routes and their permitted methods
    pub allowed_routes: HashMap<String, Vec<SerializableMethod>>,
    /// Description of this contract
    pub description: String,
}

/// Allowlist result
#[derive(Debug)]
struct AllowlistResult {
    /// Whether the request is allowed
    is_allowed: bool,
    /// Reason for denial if not allowed
    reason: Option<String>,
    /// Client tier if identified
    client_tier: Option<ClientTier>,
}

/// Allowlist middleware
/// 
/// This middleware implements service contract allowlisting to only allow specific 
/// routes/methods per client/app tier, making the public surface area explicit 
/// and tracking denied route attempts by client ID.
pub async fn allowlist_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, Response> {
    // Get request details
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    
    // Identify client tier
    let client_info = identify_client_tier(&headers);
    
    // Check if request is allowed
    let allowlist_result = is_request_allowed(&method, &path, &client_info.tier);
    
    if !allowlist_result.is_allowed {
        // Track denied request in metrics
        state.metrics.gateway_rejections.inc();
        
        // Log denied attempt with client ID
        log_denied_attempt(&method, &path, &client_info, &allowlist_result);
        
        // Return forbidden response
        return Err(create_forbidden_response(&allowlist_result));
    }
    
    // Continue with the request
    Ok(next.run(request).await)
}

/// Identify client tier based on headers
fn identify_client_tier(headers: &HeaderMap) -> ClientInfo {
    // Check for client tier header
    if let Some(tier_header) = headers.get("x-client-tier") {
        if let Ok(tier_str) = tier_header.to_str() {
            match tier_str.to_lowercase().as_str() {
                "public" => return ClientInfo { tier: ClientTier::Public, id: get_client_id(headers) },
                "user" => return ClientInfo { tier: ClientTier::User, id: get_client_id(headers) },
                "premium" => return ClientInfo { tier: ClientTier::Premium, id: get_client_id(headers) },
                "admin" => return ClientInfo { tier: ClientTier::Admin, id: get_client_id(headers) },
                "service" => return ClientInfo { tier: ClientTier::Service, id: get_client_id(headers) },
                _ => {}
            }
        }
    }
    
    // Check for API key which might indicate a service
    if headers.contains_key("x-api-key") {
        return ClientInfo { tier: ClientTier::Service, id: get_client_id(headers) };
    }
    
    // Check for JWT token which indicates an authenticated user
    if let Some(auth_header) = headers.get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                return ClientInfo { tier: ClientTier::User, id: get_client_id(headers) };
            }
        }
    }
    
    // Default to public tier
    ClientInfo { tier: ClientTier::Public, id: get_client_id(headers) }
}

/// Get client ID from headers
fn get_client_id(headers: &HeaderMap) -> String {
    // Try to get client ID from header
    if let Some(client_id_header) = headers.get("x-client-id") {
        if let Ok(client_id) = client_id_header.to_str() {
            return client_id.to_string();
        }
    }
    
    // Try to get user ID from JWT token (simplified)
    if let Some(auth_header) = headers.get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                // In a real implementation, we would decode the JWT to get the user ID
                // For now, we'll use a placeholder
                return "authenticated-user".to_string();
            }
        }
    }
    
    // Default to anonymous
    "anonymous".to_string()
}

/// Client information
#[derive(Debug)]
struct ClientInfo {
    /// Client tier
    tier: ClientTier,
    /// Client ID
    id: String,
}

/// Check if a request is allowed based on service contracts
fn is_request_allowed(method: &Method, path: &str, client_tier: &ClientTier) -> AllowlistResult {
    // Get service contracts for this tier
    let contracts = get_service_contracts(client_tier);
    
    // Convert method to serializable method for comparison
    let serializable_method = SerializableMethod::from(method);
    
    // Check if the path is allowed
    for contract in contracts {
        if let Some(allowed_methods) = contract.allowed_routes.get(path) {
            // Check if the method is allowed
            if allowed_methods.contains(&serializable_method) {
                return AllowlistResult {
                    is_allowed: true,
                    reason: None,
                    client_tier: Some(client_tier.clone()),
                };
            } else {
                return AllowlistResult {
                    is_allowed: false,
                    reason: Some(format!("Method {} not allowed for path {} in tier {:?}", method, path, client_tier)),
                    client_tier: Some(client_tier.clone()),
                };
            }
        }
    }
    
    // Path not found in allowlist
    AllowlistResult {
        is_allowed: false,
        reason: Some(format!("Path {} not allowed for tier {:?}", path, client_tier)),
        client_tier: Some(client_tier.clone()),
    }
}

/// Get service contracts for a client tier
fn get_service_contracts(client_tier: &ClientTier) -> Vec<ServiceContract> {
    match client_tier {
        ClientTier::Public => vec![ServiceContract {
            tier: ClientTier::Public,
            allowed_routes: {
                let mut routes = HashMap::new();
                routes.insert("/health".to_string(), vec![SerializableMethod::GET]);
                routes.insert("/".to_string(), vec![SerializableMethod::GET]);
                routes
            },
            description: "Public access contract with minimal endpoints".to_string(),
        }],
        ClientTier::User => vec![ServiceContract {
            tier: ClientTier::User,
            allowed_routes: {
                let mut routes = HashMap::new();
                routes.insert("/health".to_string(), vec![SerializableMethod::GET]);
                routes.insert("/".to_string(), vec![SerializableMethod::GET]);
                routes.insert("/api/v1/pools".to_string(), vec![SerializableMethod::GET]);
                routes.insert("/api/v1/orders".to_string(), vec![SerializableMethod::GET]);
                routes.insert("/api/v1/markets".to_string(), vec![SerializableMethod::GET]);
                routes
            },
            description: "Standard user access contract".to_string(),
        }],
        ClientTier::Premium => vec![ServiceContract {
            tier: ClientTier::Premium,
            allowed_routes: {
                let mut routes = HashMap::new();
                routes.insert("/health".to_string(), vec![SerializableMethod::GET]);
                routes.insert("/".to_string(), vec![SerializableMethod::GET]);
                routes.insert("/api/v1/pools".to_string(), vec![SerializableMethod::GET]);
                routes.insert("/api/v1/orders".to_string(), vec![SerializableMethod::GET]);
                routes.insert("/api/v1/markets".to_string(), vec![SerializableMethod::GET]);
                routes.insert("/api/v1/admin/stats".to_string(), vec![SerializableMethod::GET]);
                routes
            },
            description: "Premium user access contract with extended endpoints".to_string(),
        }],
        ClientTier::Admin => vec![ServiceContract {
            tier: ClientTier::Admin,
            allowed_routes: {
                let mut routes = HashMap::new();
                routes.insert("/health".to_string(), vec![SerializableMethod::GET]);
                routes.insert("/".to_string(), vec![SerializableMethod::GET]);
                routes.insert("/api/v1/pools".to_string(), vec![SerializableMethod::GET, SerializableMethod::POST, SerializableMethod::PUT, SerializableMethod::DELETE]);
                routes.insert("/api/v1/orders".to_string(), vec![SerializableMethod::GET, SerializableMethod::POST, SerializableMethod::PUT, SerializableMethod::DELETE]);
                routes.insert("/api/v1/markets".to_string(), vec![SerializableMethod::GET, SerializableMethod::POST, SerializableMethod::PUT, SerializableMethod::DELETE]);
                routes.insert("/api/v1/admin".to_string(), vec![SerializableMethod::GET, SerializableMethod::POST, SerializableMethod::PUT, SerializableMethod::DELETE]);
                routes.insert("/metrics".to_string(), vec![SerializableMethod::GET]);
                routes
            },
            description: "Administrator access contract with full access".to_string(),
        }],
        ClientTier::Service => vec![ServiceContract {
            tier: ClientTier::Service,
            allowed_routes: {
                let mut routes = HashMap::new();
                routes.insert("/health".to_string(), vec![SerializableMethod::GET]);
                routes.insert("/api/v1/pools".to_string(), vec![SerializableMethod::GET, SerializableMethod::POST, SerializableMethod::PUT, SerializableMethod::DELETE]);
                routes.insert("/api/v1/orders".to_string(), vec![SerializableMethod::GET, SerializableMethod::POST, SerializableMethod::PUT, SerializableMethod::DELETE]);
                routes.insert("/api/v1/markets".to_string(), vec![SerializableMethod::GET, SerializableMethod::POST, SerializableMethod::PUT, SerializableMethod::DELETE]);
                routes.insert("/metrics".to_string(), vec![SerializableMethod::GET]);
                routes
            },
            description: "Service-to-service access contract".to_string(),
        }],
    }
}

/// Log denied attempt
fn log_denied_attempt(method: &Method, path: &str, client_info: &ClientInfo, result: &AllowlistResult) {
    let timestamp = Utc::now().to_rfc3339();
    
    tracing::warn!(
        timestamp = %timestamp,
        client_id = %client_info.id,
        client_tier = ?client_info.tier,
        method = %method,
        path = %path,
        reason = ?result.reason,
        "Route access denied by allowlist"
    );
}

/// Create forbidden response
fn create_forbidden_response(result: &AllowlistResult) -> Response {
    let timestamp = Utc::now().to_rfc3339();
    
    let error_json = json!({
        "error": "Access denied",
        "message": "This route or method is not allowed for your client tier",
        "reason": result.reason,
        "timestamp": timestamp
    });
    
    let body = serde_json::to_string(&error_json).unwrap_or_else(|_| "{}".to_string());
    
    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .header("content-type", "application/json")
        .body(axum::body::Body::from(body))
        .unwrap()
}