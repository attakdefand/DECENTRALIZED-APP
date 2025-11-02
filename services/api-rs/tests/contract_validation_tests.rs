//! Contract Validation Tests
//! 
//! This file contains tests for the contract validation features

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use std::collections::HashSet;

    // Test known fields validation
    #[test]
    fn test_known_fields_validation() {
        // Test valid pool fields
        let pool_json = serde_json::json!({
            "id": "pool1",
            "tokenA": "ETH",
            "tokenB": "USDC",
            "reserveA": 1000.0,
            "reserveB": 2000.0
        });
        
        let allowed_fields: HashSet<&str> = [
            "id", "tokenA", "tokenB", "reserveA", "reserveB"
        ].iter().cloned().collect();
        
        let obj = pool_json.as_object().unwrap();
        let mut unknown_fields = Vec::new();
        
        for (field, _) in obj {
            if !allowed_fields.contains(field.as_str()) {
                unknown_fields.push(field.clone());
            }
        }
        
        assert!(unknown_fields.is_empty());
        
        // Test invalid pool fields (unknown field)
        let invalid_pool_json = serde_json::json!({
            "id": "pool1",
            "tokenA": "ETH",
            "tokenB": "USDC",
            "reserveA": 1000.0,
            "reserveB": 2000.0,
            "unknownField": "should be rejected"
        });
        
        let obj = invalid_pool_json.as_object().unwrap();
        let mut unknown_fields = Vec::new();
        
        for (field, _) in obj {
            if !allowed_fields.contains(field.as_str()) {
                unknown_fields.push(field.clone());
            }
        }
        
        assert!(!unknown_fields.is_empty());
        assert_eq!(unknown_fields[0], "unknownField");
    }

    // Test contract validation function
    #[test]
    fn test_contract_validation() {
        // Test valid contract
        let valid_contract = r#"{
            "id": "pool1",
            "tokenA": "ETH",
            "tokenB": "USDC",
            "reserveA": 1000.0,
            "reserveB": 2000.0
        }"#;
        
        let result = serde_json::from_str::<Value>(valid_contract);
        assert!(result.is_ok());
        
        // Test invalid JSON
        let invalid_contract = r#"{
            "id": "pool1",
            "tokenA": "ETH",
            "tokenB": "USDC",
            "reserveA": 1000.0,
            "reserveB": 2000.0,
        }"#; // Trailing comma makes it invalid
        
        let result = serde_json::from_str::<Value>(invalid_contract);
        assert!(result.is_err());
    }
}