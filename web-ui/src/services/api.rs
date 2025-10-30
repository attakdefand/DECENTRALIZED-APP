//! Base API service
//!
//! This module provides the base API client for making HTTP requests.

use gloo_net::http::Request;
use serde::{de::DeserializeOwned, Serialize};
use gloo_net::websocket::{futures::WebSocket};
use gloo_utils::errors::JsError;

/// API client
pub struct ApiClient {
    base_url: String,
}

impl ApiClient {
    /// Create a new API client
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    /// Get the base URL
    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }

    /// Make a GET request
    pub async fn get<T>(&self, endpoint: &str) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let resp = Request::get(&url).send().await?;
        let json = resp.json::<T>().await?;
        Ok(json)
    }

    /// Make a POST request
    pub async fn post<T, U>(&self, endpoint: &str, data: &T) -> Result<U, anyhow::Error>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let resp = Request::post(&url)
            .json(data)?
            .send()
            .await?;
        let json = resp.json::<U>().await?;
        Ok(json)
    }

    /// Make a PUT request
    pub async fn put<T, U>(&self, endpoint: &str, data: &T) -> Result<U, anyhow::Error>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let resp = Request::put(&url)
            .json(data)?
            .send()
            .await?;
        let json = resp.json::<U>().await?;
        Ok(json)
    }

    /// Make a DELETE request
    pub async fn delete<T>(&self, endpoint: &str) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        let resp = Request::delete(&url).send().await?;
        let json = resp.json::<T>().await?;
        Ok(json)
    }

    /// Create a WebSocket connection for real-time updates
    pub fn connect_websocket(&self, endpoint: &str) -> Result<WebSocket, JsError> {
        let ws_url = format!("ws://{}/{}", 
            self.base_url.strip_prefix("http://").unwrap_or(&self.base_url), 
            endpoint.strip_prefix("/").unwrap_or(endpoint));
        WebSocket::open(&ws_url)
    }
}

/// Create a default API client
pub fn create_client() -> ApiClient {
    // In a real app, this would be configurable
    ApiClient::new("http://localhost:3000/api/v1".to_string())
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