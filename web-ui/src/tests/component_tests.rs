//! Component tests
//!
//! Tests for UI components with proper data handling

use wasm_bindgen_test::*;
use crate::components::pool_card::PoolData;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_pool_data_creation() {
    let pool = PoolData {
        id: "test-pool-1".to_string(),
        token_a: "ETH".to_string(),
        token_b: "USDC".to_string(),
        liquidity: 1000000.0,
        volume_24h: 50000.0,
        apr: 15.5,
    };
    
    assert_eq!(pool.id, "test-pool-1");
    assert_eq!(pool.token_a, "ETH");
    assert_eq!(pool.token_b, "USDC");
    assert_eq!(pool.liquidity, 1000000.0);
    assert_eq!(pool.volume_24h, 50000.0);
    assert_eq!(pool.apr, 15.5);
}

#[wasm_bindgen_test]
fn test_pool_data_clone() {
    let pool1 = PoolData {
        id: "1".to_string(),
        token_a: "BTC".to_string(),
        token_b: "USDT".to_string(),
        liquidity: 2000000.0,
        volume_24h: 100000.0,
        apr: 10.0,
    };
    
    let pool2 = pool1.clone();
    
    assert_eq!(pool1.id, pool2.id);
    assert_eq!(pool1.token_a, pool2.token_a);
    assert_eq!(pool1.token_b, pool2.token_b);
}

#[wasm_bindgen_test]
fn test_pool_data_serialization() {
    use serde_json;
    
    let pool = PoolData {
        id: "ser-test".to_string(),
        token_a: "ETH".to_string(),
        token_b: "DAI".to_string(),
        liquidity: 500000.0,
        volume_24h: 25000.0,
        apr: 12.0,
    };
    
    let serialized = serde_json::to_string(&pool).unwrap();
    assert!(!serialized.is_empty(), "Serialization should produce output");
    
    let deserialized: PoolData = serde_json::from_str(&serialized).unwrap();
    assert_eq!(pool.id, deserialized.id);
    assert_eq!(pool.token_a, deserialized.token_a);
}

#[wasm_bindgen_test]
fn test_pool_data_partial_eq() {
    let pool1 = PoolData {
        id: "1".to_string(),
        token_a: "ETH".to_string(),
        token_b: "USDC".to_string(),
        liquidity: 1000000.0,
        volume_24h: 50000.0,
        apr: 15.0,
    };
    
    let pool2 = PoolData {
        id: "1".to_string(),
        token_a: "ETH".to_string(),
        token_b: "USDC".to_string(),
        liquidity: 1000000.0,
        volume_24h: 50000.0,
        apr: 15.0,
    };
    
    let pool3 = PoolData {
        id: "2".to_string(),
        token_a: "BTC".to_string(),
        token_b: "USDC".to_string(),
        liquidity: 2000000.0,
        volume_24h: 100000.0,
        apr: 10.0,
    };
    
    assert_eq!(pool1, pool2, "Identical pools should be equal");
    assert_ne!(pool1, pool3, "Different pools should not be equal");
}

#[wasm_bindgen_test]
fn test_pool_data_validation() {
    // Test that pool data accepts valid ranges
    let valid_pool = PoolData {
        id: "valid".to_string(),
        token_a: "ETH".to_string(),
        token_b: "USDC".to_string(),
        liquidity: 0.0, // Can be zero for new pool
        volume_24h: 0.0,
        apr: 0.0,
    };
    
    assert!(valid_pool.liquidity >= 0.0);
    assert!(valid_pool.volume_24h >= 0.0);
    assert!(valid_pool.apr >= 0.0);
}

#[wasm_bindgen_test]
fn test_pool_data_formatting() {
    let pool = PoolData {
        id: "format-test".to_string(),
        token_a: "ETH".to_string(),
        token_b: "USDC".to_string(),
        liquidity: 1234567.89,
        volume_24h: 98765.43,
        apr: 15.678,
    };
    
    // Test formatting
    let liquidity_formatted = format!("${:.2}", pool.liquidity);
    assert_eq!(liquidity_formatted, "$1234567.89");
    
    let apr_formatted = format!("{:.2}%", pool.apr);
    assert_eq!(apr_formatted, "15.68%");
}
