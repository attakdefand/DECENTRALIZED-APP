//! Schema Validation Tests
//! 
//! This file contains tests for the schema validation features

#[cfg(test)]
mod tests {
    use api_service::models::pool::Pool;
    use api_service::models::order::Order;
    use api_service::models::market::Market;
    use validator::Validate;

    #[test]
    fn test_pool_validation() {
        // Test valid pool
        let valid_pool = Pool {
            id: "pool1".to_string(),
            token_a: "ETH".to_string(),
            token_b: "USDC".to_string(),
            reserve_a: 1000.0,
            reserve_b: 2000.0,
        };
        
        assert!(valid_pool.validate().is_ok());
        
        // Test invalid pool (negative reserve)
        let invalid_pool = Pool {
            id: "pool1".to_string(),
            token_a: "ETH".to_string(),
            token_b: "USDC".to_string(),
            reserve_a: -1000.0,
            reserve_b: 2000.0,
        };
        
        assert!(invalid_pool.validate().is_err());
    }

    #[test]
    fn test_order_validation() {
        // Test valid order
        let valid_order = Order {
            id: "order1".to_string(),
            user: "user1".to_string(),
            market: "ETH-USDC".to_string(),
            side: "buy".to_string(),
            price: 100.0,
            amount: 10.0,
        };
        
        assert!(valid_order.validate().is_ok());
        
        // Test invalid order (invalid side)
        let invalid_order = Order {
            id: "order1".to_string(),
            user: "user1".to_string(),
            market: "ETH-USDC".to_string(),
            side: "invalid".to_string(),
            price: 100.0,
            amount: 10.0,
        };
        
        assert!(invalid_order.validate().is_err());
    }

    #[test]
    fn test_market_validation() {
        // Test valid market
        let valid_market = Market {
            id: "market1".to_string(),
            base_token: "ETH".to_string(),
            quote_token: "USDC".to_string(),
            price: 100.0,
        };
        
        assert!(valid_market.validate().is_ok());
        
        // Test invalid market (negative price)
        let invalid_market = Market {
            id: "market1".to_string(),
            base_token: "ETH".to_string(),
            quote_token: "USDC".to_string(),
            price: -100.0,
        };
        
        assert!(invalid_market.validate().is_err());
    }
}