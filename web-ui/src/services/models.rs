//! API data models
//!
//! This module contains data structures for backend API responses

use serde::{Deserialize, Serialize};

/// Pool response from API
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PoolResponse {
    pub pools: Vec<PoolInfo>,
    pub total: usize,
}

/// Pool information from backend
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PoolInfo {
    pub id: String,
    pub token_a: TokenInfo,
    pub token_b: TokenInfo,
    pub liquidity: String,
    pub volume_24h: String,
    pub apr: String,
    pub fee_tier: String,
}

/// Token information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TokenInfo {
    pub symbol: String,
    pub address: String,
    pub decimals: u8,
}

/// Order response from API
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderResponse {
    pub orders: Vec<OrderInfo>,
    pub total: usize,
}

/// Order information from backend
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderInfo {
    pub id: String,
    pub pair: String,
    pub side: String,
    pub price: String,
    pub amount: String,
    pub filled: String,
    pub status: String,
    pub timestamp: u64,
}

/// Market response from API
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MarketResponse {
    pub markets: Vec<MarketInfo>,
    pub total: usize,
}

/// Market information from backend
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MarketInfo {
    pub pair: String,
    pub price: String,
    pub change_24h: String,
    pub volume_24h: String,
    pub high_24h: String,
    pub low_24h: String,
}

/// Health check response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime: u64,
}

/// Error response from API
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub code: u16,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    
    #[test]
    fn test_pool_response_serialization() {
        let pool_response = PoolResponse {
            pools: vec![PoolInfo {
                id: "1".to_string(),
                token_a: TokenInfo {
                    symbol: "ETH".to_string(),
                    address: "0x123".to_string(),
                    decimals: 18,
                },
                token_b: TokenInfo {
                    symbol: "USDC".to_string(),
                    address: "0x456".to_string(),
                    decimals: 6,
                },
                liquidity: "1000000".to_string(),
                volume_24h: "50000".to_string(),
                apr: "15.5".to_string(),
                fee_tier: "0.3".to_string(),
            }],
            total: 1,
        };
        
        let json = serde_json::to_string(&pool_response).unwrap();
        let deserialized: PoolResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(pool_response, deserialized);
    }
    
    #[test]
    fn test_order_response_serialization() {
        let order_response = OrderResponse {
            orders: vec![OrderInfo {
                id: "order1".to_string(),
                pair: "ETH/USDC".to_string(),
                side: "buy".to_string(),
                price: "2500.50".to_string(),
                amount: "1.5".to_string(),
                filled: "0.5".to_string(),
                status: "open".to_string(),
                timestamp: 1234567890,
            }],
            total: 1,
        };
        
        let json = serde_json::to_string(&order_response).unwrap();
        let deserialized: OrderResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(order_response, deserialized);
    }
    
    #[test]
    fn test_market_response_serialization() {
        let market_response = MarketResponse {
            markets: vec![MarketInfo {
                pair: "ETH/USDC".to_string(),
                price: "2500.00".to_string(),
                change_24h: "2.5".to_string(),
                volume_24h: "1000000".to_string(),
                high_24h: "2550.00".to_string(),
                low_24h: "2450.00".to_string(),
            }],
            total: 1,
        };
        
        let json = serde_json::to_string(&market_response).unwrap();
        let deserialized: MarketResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(market_response, deserialized);
    }
    
    #[test]
    fn test_health_response() {
        let health = HealthResponse {
            status: "OK".to_string(),
            version: "1.0.0".to_string(),
            uptime: 3600,
        };
        
        let json = serde_json::to_string(&health).unwrap();
        assert!(json.contains("OK"));
    }
    
    #[test]
    fn test_error_response() {
        let error = ErrorResponse {
            error: "BadRequest".to_string(),
            message: "Invalid parameter".to_string(),
            code: 400,
        };
        
        assert_eq!(error.code, 400);
        assert!(error.message.contains("Invalid"));
    }
}
