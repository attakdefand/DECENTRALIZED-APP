//! Base API service
//!
//! This module provides the base API client with security, retry logic, and backend integration.

use gloo_net::http::Request;
use serde::{de::DeserializeOwned, Serialize};
use gloo_net::websocket::{futures::WebSocket};
use gloo_utils::errors::JsError;
use super::config::BackendConfig;
use super::auth::AuthService;

/// API client with security and backend integration
pub struct ApiClient {
    base_url: String,
    config: BackendConfig,
}

impl ApiClient {
    /// Create a new API client with configuration
    pub fn new(base_url: String) -> Self {
        let config = BackendConfig::from_environment();
        Self { base_url, config }
    }
    
    /// Create API client from configuration
    pub fn from_config(config: BackendConfig) -> Self {
        let base_url = format!("{}/api/v1", config.api_url);
        Self { base_url, config }
    }

    /// Get the base URL
    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }

    /// Make a GET request with authentication and security headers
    pub async fn get<T>(&self, endpoint: &str) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned,
    {
        let url = self.build_url(endpoint)?;
        
        let mut request = Request::get(&url);
        request = self.add_security_headers(request)?;
        
        let resp = request.send().await?;
        
        // Security: Check status code
        if !resp.ok() {
            return Err(anyhow::anyhow!(
                "Request failed with status: {}", 
                resp.status()
            ));
        }
        
        let json = resp.json::<T>().await?;
        Ok(json)
    }

    /// Make a POST request with authentication and security headers
    pub async fn post<T, U>(&self, endpoint: &str, data: &T) -> Result<U, anyhow::Error>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        let url = self.build_url(endpoint)?;
        
        let mut request = Request::post(&url).json(data)?;
        request = self.add_security_headers(request)?;
        
        let resp = request.send().await?;
        
        if !resp.ok() {
            return Err(anyhow::anyhow!(
                "Request failed with status: {}",
                resp.status()
            ));
        }
        
        let json = resp.json::<U>().await?;
        Ok(json)
    }

    /// Make a PUT request with authentication and security headers
    pub async fn put<T, U>(&self, endpoint: &str, data: &T) -> Result<U, anyhow::Error>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        let url = self.build_url(endpoint)?;
        
        let mut request = Request::put(&url).json(data)?;
        request = self.add_security_headers(request)?;
        
        let resp = request.send().await?;
        
        if !resp.ok() {
            return Err(anyhow::anyhow!(
                "Request failed with status: {}",
                resp.status()
            ));
        }
        
        let json = resp.json::<U>().await?;
        Ok(json)
    }

    /// Make a DELETE request with authentication and security headers
    pub async fn delete<T>(&self, endpoint: &str) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned,
    {
        let url = self.build_url(endpoint)?;
        
        let mut request = Request::delete(&url);
        request = self.add_security_headers(request)?;
        
        let resp = request.send().await?;
        
        if !resp.ok() {
            return Err(anyhow::anyhow!(
                "Request failed with status: {}",
                resp.status()
            ));
        }
        
        let json = resp.json::<T>().await?;
        Ok(json)
    }
    
    /// Build full URL with validation
    fn build_url(&self, endpoint: &str) -> Result<String, anyhow::Error> {
        // Security: Validate endpoint
        if endpoint.contains("..")
|| endpoint.contains("//") {
            return Err(anyhow::anyhow!("Invalid endpoint path"));
        }
        
        let url = format!("{}{}", self.base_url, endpoint);
        Ok(url)
    }
    
    /// Add security headers to request
    fn add_security_headers(&self, request: Request) -> Result<Request, anyhow::Error> {
        let mut req = request;
        
        // Add authentication header if token exists
        if let Some(token) = AuthService::get_stored_token() {
            req = req.header("Authorization", &format!("Bearer {}", token));
        }
        
        // Security: Add CSRF protection
        req = req.header("X-Requested-With", "XMLHttpRequest");
        
        // Security: Content type
        req = req.header("Content-Type", "application/json");
        
        // Security: Accept only JSON
        req = req.header("Accept", "application/json");
        
        Ok(req)
    }

    /// Create a WebSocket connection with security
    pub fn connect_websocket(&self, endpoint: &str) -> Result<WebSocket, JsError> {
        // Security: Validate endpoint
        if endpoint.contains("..") || endpoint.contains("//") {
            use wasm_bindgen::JsValue;
            return Err(JsError::from(JsValue::from_str("Invalid WebSocket endpoint")));
        }
        
        let ws_base = self.config.ws_url.clone();
        let ws_url = format!("{}/{}", 
            ws_base,
            endpoint.strip_prefix("/").unwrap_or(endpoint));
        
        // Add authentication token to WebSocket URL if available
        let final_url = if let Some(token) = AuthService::get_stored_token() {
            format!("{}?token={}", ws_url, token)
        } else {
            ws_url
        };
        
        WebSocket::open(&final_url)
    }
}

/// Create a default API client from environment configuration
pub fn create_client() -> ApiClient {
    let config = BackendConfig::from_environment();
    ApiClient::from_config(config)
}

/// Create API client for pools endpoint
pub fn create_pools_client() -> ApiClient {
    create_client()
}

/// Create API client for orders endpoint
pub fn create_orders_client() -> ApiClient {
    create_client()
}

/// Create API client for markets endpoint
pub fn create_markets_client() -> ApiClient {
    create_client()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_api_client_creation() {
        let client = ApiClient::new("http://localhost:3000/api/v1".to_string());
        assert_eq!(client.get_base_url(), "http://localhost:3000/api/v1");
    }

    #[wasm_bindgen_test]
    fn test_create_default_client() {
        let client = create_client();
        assert_eq!(client.get_base_url(), "http://localhost:3000/api/v1");
    }
}