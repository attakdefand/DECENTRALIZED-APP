//! Oracle integrity tests
//!
//! This file contains tests for oracle integrity mechanisms.

use decentralized_app_oracle::{
    OracleIntegrityTests, 
    PriceData, 
    current_timestamp
};

/// Test price manipulation detection
#[test]
fn test_price_manipulation_detection() {
    let tests = OracleIntegrityTests::new(0.05, 300, 80); // 5% max change, 5min max staleness, 80 min confidence
    
    let current = PriceData {
        pair: "ETH/USD".to_string(),
        price: 3000000000000000000000, // $3000
        timestamp: 1000,
        confidence: 95,
        oracle_provider: "Test".to_string(),
    };
    
    let previous = PriceData {
        pair: "ETH/USD".to_string(),
        price: 2000000000000000000000, // $2000
        timestamp: 900,
        confidence: 95,
        oracle_provider: "Test".to_string(),
    };
    
    // This should detect manipulation (50% change in 100 seconds)
    assert!(!tests.test_price_manipulation(&current, &previous));
}

/// Test normal price movement
#[test]
fn test_normal_price_movement() {
    let tests = OracleIntegrityTests::new(0.05, 300, 80);
    
    let current = PriceData {
        pair: "ETH/USD".to_string(),
        price: 3000000000000000000000, // $3000
        timestamp: 1000,
        confidence: 95,
        oracle_provider: "Test".to_string(),
    };
    
    let previous = PriceData {
        pair: "ETH/USD".to_string(),
        price: 2900000000000000000000, // $2900 (3.45% change)
        timestamp: 900,
        confidence: 95,
        oracle_provider: "Test".to_string(),
    };
    
    // This should pass (3.45% change in 100 seconds)
    assert!(tests.test_price_manipulation(&current, &previous));
}

/// Test data staleness
#[test]
fn test_data_staleness() {
    let tests = OracleIntegrityTests::new(0.05, 300, 80);
    
    let stale_price = PriceData {
        pair: "ETH/USD".to_string(),
        price: 3000000000000000000000,
        timestamp: current_timestamp() - 400, // 400 seconds old
        confidence: 95,
        oracle_provider: "Test".to_string(),
    };
    
    // This should detect staleness
    assert!(!tests.test_data_staleness(&stale_price));
    
    let fresh_price = PriceData {
        pair: "ETH/USD".to_string(),
        price: 3000000000000000000000,
        timestamp: current_timestamp() - 100, // 100 seconds old
        confidence: 95,
        oracle_provider: "Test".to_string(),
    };
    
    // This should pass freshness check
    assert!(tests.test_data_staleness(&fresh_price));
}

/// Test confidence level
#[test]
fn test_confidence_level() {
    let tests = OracleIntegrityTests::new(0.05, 300, 80);
    
    let low_confidence_price = PriceData {
        pair: "ETH/USD".to_string(),
        price: 3000000000000000000000,
        timestamp: current_timestamp(),
        confidence: 70, // Below threshold
        oracle_provider: "Test".to_string(),
    };
    
    // This should fail confidence check
    assert!(!tests.test_confidence(&low_confidence_price));
    
    let high_confidence_price = PriceData {
        pair: "ETH/USD".to_string(),
        price: 3000000000000000000000,
        timestamp: current_timestamp(),
        confidence: 90, // Above threshold
        oracle_provider: "Test".to_string(),
    };
    
    // This should pass confidence check
    assert!(tests.test_confidence(&high_confidence_price));
}

/// Test complete integrity check
#[test]
fn test_complete_integrity_check() {
    let tests = OracleIntegrityTests::new(0.05, 300, 80);
    
    let current = PriceData {
        pair: "ETH/USD".to_string(),
        price: 3000000000000000000000, // $3000
        timestamp: current_timestamp(),
        confidence: 95,
        oracle_provider: "Test".to_string(),
    };
    
    let previous = PriceData {
        pair: "ETH/USD".to_string(),
        price: 2900000000000000000000, // $2900
        timestamp: current_timestamp() - 100,
        confidence: 95,
        oracle_provider: "Test".to_string(),
    };
    
    // This should pass all checks
    let failures = tests.run_integrity_tests(&current, Some(&previous));
    assert!(failures.is_empty());
}

/// Test integrity check with failures
#[test]
fn test_integrity_check_with_failures() {
    let tests = OracleIntegrityTests::new(0.05, 300, 80);
    
    let current = PriceData {
        pair: "ETH/USD".to_string(),
        price: 3000000000000000000000, // $3000
        timestamp: current_timestamp() - 400, // Stale data
        confidence: 70, // Low confidence
        oracle_provider: "Test".to_string(),
    };
    
    let previous = PriceData {
        pair: "ETH/USD".to_string(),
        price: 2000000000000000000000, // $2000 (50% change)
        timestamp: current_timestamp() - 500,
        confidence: 95,
        oracle_provider: "Test".to_string(),
    };
    
    // This should fail multiple checks
    let failures = tests.run_integrity_tests(&current, Some(&previous));
    assert!(!failures.is_empty());
    assert!(failures.len() >= 2); // At least staleness and confidence failures
}

/// Performance test for integrity checks
#[test]
fn test_integrity_check_performance() {
    let tests = OracleIntegrityTests::new(0.05, 300, 80);
    
    let current = PriceData {
        pair: "ETH/USD".to_string(),
        price: 3000000000000000000000,
        timestamp: current_timestamp(),
        confidence: 95,
        oracle_provider: "Test".to_string(),
    };
    
    let previous = PriceData {
        pair: "ETH/USD".to_string(),
        price: 2900000000000000000000,
        timestamp: current_timestamp() - 100,
        confidence: 95,
        oracle_provider: "Test".to_string(),
    };
    
    // Run multiple integrity checks to test performance
    for _ in 0..1000 {
        let _ = tests.run_integrity_tests(&current, Some(&previous));
    }
}