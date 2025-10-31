//! Markets service
//!
//! This module provides functions for interacting with market data.

use serde::{Deserialize, Serialize};

use crate::services::api::ApiClient;

/// Market data structure
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Market {
    pub pair: String,
    pub price: f64,
    pub change_24h: f64,
    pub volume_24h: f64,
    pub high_24h: f64,
    pub low_24h: f64,
}

/// Chart data point
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChartDataPoint {
    pub timestamp: u64,
    pub price: f64,
}

/// Markets service
pub struct MarketsService {
    client: ApiClient,
}

impl MarketsService {
    /// Create a new markets service
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    /// Get all markets
    pub async fn get_markets(&self) -> Result<Vec<Market>, anyhow::Error> {
        self.client.get("/markets").await
    }

    /// Get a specific market by pair
    pub async fn get_market(&self, pair: &str) -> Result<Market, anyhow::Error> {
        self.client.get(&format!("/markets/{}", pair)).await
    }

    /// Get chart data for a market
    pub async fn get_chart_data(&self, pair: &str) -> Result<Vec<ChartDataPoint>, anyhow::Error> {
        self.client.get(&format!("/markets/{}/chart", pair)).await
    }
}