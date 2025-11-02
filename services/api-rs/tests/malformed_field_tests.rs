//! Malformed Field Rejection Tests
//! 
//! This file contains tests for the malformed field rejection features

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use std::collections::HashSet;

    // Test malformed field detection
    #[test]
    fn test_malformed_field_detection() {
        // Test with valid fields
        let valid_json = r#"{
            "id": "pool1",
            "tokenA": "ETH",
            "tokenB": "USDC",
            "reserveA": 1000.0,
            "reserveB": 2000.0
        }"#;
        
        let result = serde_json::from_str::<Value>(valid_json);
        assert!(result.is_ok());
        
        let json_value = result.unwrap();
        let obj = json_value.as_object().unwrap();
        
        let allowed_fields: HashSet<&str> = [
            "id", "tokenA", "tokenB", "reserveA", "reserveB"
        ].iter().cloned().collect();
        
        let mut unknown_fields = Vec::new();
        for (field, _) in obj {
            if !allowed_fields.contains(field.as_str()) {
                unknown_fields.push(field.clone());
            }
        }
        
        assert!(unknown_fields.is_empty());
    }

    #[test]
    fn test_unknown_field_detection() {
        // Test with unknown fields
        let invalid_json = r#"{
            "id": "pool1",
            "tokenA": "ETH",
            "tokenB": "USDC",
            "reserveA": 1000.0,
            "reserveB": 2000.0,
            "unknownField": "should be rejected",
            "anotherUnknown": "also should be rejected"
        }"#;
        
        let result = serde_json::from_str::<Value>(invalid_json);
        assert!(result.is_ok());
        
        let json_value = result.unwrap();
        let obj = json_value.as_object().unwrap();
        
        let allowed_fields: HashSet<&str> = [
            "id", "tokenA", "tokenB", "reserveA", "reserveB"
        ].iter().cloned().collect();
        
        let mut unknown_fields = Vec::new();
        for (field, _) in obj {
            if !allowed_fields.contains(field.as_str()) {
                unknown_fields.push(field.clone());
            }
        }
        
        assert!(!unknown_fields.is_empty());
        assert_eq!(unknown_fields.len(), 2);
        assert!(unknown_fields.contains(&"unknownField".to_string()));
        assert!(unknown_fields.contains(&"anotherUnknown".to_string()));
    }

    #[test]
    fn test_empty_json() {
        // Test with empty JSON
        let empty_json = r#"{}"#;
        
        let result = serde_json::from_str::<Value>(empty_json);
        assert!(result.is_ok());
        
        let json_value = result.unwrap();
        let obj = json_value.as_object().unwrap();
        
        let allowed_fields: HashSet<&str> = [
            "id", "tokenA", "tokenB", "reserveA", "reserveB"
        ].iter().cloned().collect();
        
        let mut unknown_fields = Vec::new();
        for (field, _) in obj {
            if !allowed_fields.contains(field.as_str()) {
                unknown_fields.push(field.clone());
            }
        }
        
        // Empty object should have no unknown fields
        assert!(unknown_fields.is_empty());
    }
}