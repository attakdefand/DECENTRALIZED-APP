//! Orders service
//!
//! This module provides functions for interacting with orders.

use serde::{Deserialize, Serialize};

use crate::services::api::ApiClient;

/// Order side enum
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Order type enum
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Market,
    Limit,
    Stop,
}

/// Order status enum
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Open,
    Filled,
    Cancelled,
}

/// Order data structure
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Order {
    pub id: String,
    pub pair: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub price: f64,
    pub amount: f64,
    pub filled: f64,
    pub status: OrderStatus,
}

/// Create order request
#[derive(Serialize)]
pub struct CreateOrderRequest {
    pub pair: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub price: f64,
    pub amount: f64,
}

/// Create order response
#[derive(Deserialize)]
pub struct CreateOrderResponse {
    pub order_id: String,
    pub status: OrderStatus,
}

/// Orders service
pub struct OrdersService {
    client: ApiClient,
}

impl OrdersService {
    /// Create a new orders service
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    /// Get all orders
    pub async fn get_orders(&self) -> Result<Vec<Order>, anyhow::Error> {
        self.client.get("/orders").await
    }

    /// Get a specific order by ID
    pub async fn get_order(&self, id: &str) -> Result<Order, anyhow::Error> {
        self.client.get(&format!("/orders/{}", id)).await
    }

    /// Create a new order
    pub async fn create_order(&self, request: CreateOrderRequest) -> Result<CreateOrderResponse, anyhow::Error> {
        self.client.post("/orders", &request).await
    }

    /// Cancel an order
    pub async fn cancel_order(&self, id: &str) -> Result<(), anyhow::Error> {
        self.client.delete(&format!("/orders/{}", id)).await
    }
}