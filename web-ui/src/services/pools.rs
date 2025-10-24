//! Pools service
//!
//! This module provides functions for interacting with liquidity pools.

use serde::{Deserialize, Serialize};

use crate::services::api::ApiClient;

/// Pool data structure
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Pool {
    pub id: String,
    pub token_a: String,
    pub token_b: String,
    pub liquidity: f64,
    pub volume_24h: f64,
    pub apr: f64,
}

/// Create pool request
#[derive(Serialize)]
pub struct CreatePoolRequest {
    pub token_a: String,
    pub token_b: String,
    pub amount_a: f64,
    pub amount_b: f64,
}

/// Create pool response
#[derive(Deserialize)]
pub struct CreatePoolResponse {
    pub pool_id: String,
    pub lp_tokens: f64,
}

/// Pools service
pub struct PoolsService {
    client: ApiClient,
}

impl PoolsService {
    /// Create a new pools service
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    /// Get all pools
    pub async fn get_pools(&self) -> Result<Vec<Pool>, anyhow::Error> {
        self.client.get("/pools").await
    }

    /// Get a specific pool by ID
    pub async fn get_pool(&self, id: &str) -> Result<Pool, anyhow::Error> {
        self.client.get(&format!("/pools/{}", id)).await
    }

    /// Create a new pool
    pub async fn create_pool(
        &self,
        request: CreatePoolRequest,
    ) -> Result<CreatePoolResponse, anyhow::Error> {
        self.client.post("/pools", &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_pool_struct_creation() {
        let pool = Pool {
            id: "pool-1".to_string(),
            token_a: "ETH".to_string(),
            token_b: "USDC".to_string(),
            liquidity: 1000000.0,
            volume_24h: 50000.0,
            apr: 0.15,
        };

        assert_eq!(pool.id, "pool-1");
        assert_eq!(pool.token_a, "ETH");
        assert_eq!(pool.token_b, "USDC");
        assert_eq!(pool.liquidity, 1000000.0);
        assert_eq!(pool.volume_24h, 50000.0);
        assert_eq!(pool.apr, 0.15);
    }

    #[wasm_bindgen_test]
    fn test_pools_service_creation() {
        let client = ApiClient::new("http://localhost:3000/api/v1".to_string());
        let pools_service = PoolsService::new(client);

        // This is a simple test to ensure the service can be created
        assert!(true);
    }
}
