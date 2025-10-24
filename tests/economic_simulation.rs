//! Economic simulation tests for risk parameters
//!
//! This file contains tests that simulate various economic scenarios
//! to validate the risk management system.

use decentralized_app_core::risk::{RiskManager, EconomicScenario, ScenarioImpact};

/// Test bull market scenario
#[test]
fn test_bull_market_scenario() {
    let risk_manager = RiskManager::new().expect("Failed to create risk manager");
    
    // Find the bull market scenario
    let bull_market = risk_manager.scenarios.iter()
        .find(|s| s.name == "Bull Market")
        .expect("Bull market scenario not found");
    
    assert_eq!(bull_market.volatility, 0.02);
    assert_eq!(bull_market.liquidity_stress, 0.8);
    assert_eq!(bull_market.duration, 28800);
    assert!(matches!(bull_market.impact, ScenarioImpact::Low));
}

/// Test bear market scenario
#[test]
fn test_bear_market_scenario() {
    let risk_manager = RiskManager::new().expect("Failed to create risk manager");
    
    // Find the bear market scenario
    let bear_market = risk_manager.scenarios.iter()
        .find(|s| s.name == "Bear Market")
        .expect("Bear market scenario not found");
    
    assert_eq!(bear_market.volatility, 0.05);
    assert_eq!(bear_market.liquidity_stress, 0.6);
    assert_eq!(bear_market.duration, 28800);
    assert!(matches!(bear_market.impact, ScenarioImpact::Medium));
}

/// Test flash crash scenario
#[test]
fn test_flash_crash_scenario() {
    let risk_manager = RiskManager::new().expect("Failed to create risk manager");
    
    // Find the flash crash scenario
    let flash_crash = risk_manager.scenarios.iter()
        .find(|s| s.name == "Flash Crash")
        .expect("Flash crash scenario not found");
    
    assert_eq!(flash_crash.volatility, 0.15);
    assert_eq!(flash_crash.liquidity_stress, 0.3);
    assert_eq!(flash_crash.duration, 1440);
    assert!(matches!(flash_crash.impact, ScenarioImpact::High));
}

/// Test black swan scenario
#[test]
fn test_black_swan_scenario() {
    let risk_manager = RiskManager::new().expect("Failed to create risk manager");
    
    // Find the black swan scenario
    let black_swan = risk_manager.scenarios.iter()
        .find(|s| s.name == "Black Swan")
        .expect("Black swan scenario not found");
    
    assert_eq!(black_swan.volatility, 0.30);
    assert_eq!(black_swan.liquidity_stress, 0.1);
    assert_eq!(black_swan.duration, 720);
    assert!(matches!(black_swan.impact, ScenarioImpact::Critical));
}

/// Test risk parameter validation
#[test]
fn test_risk_parameter_validation() {
    let risk_manager = RiskManager::new().expect("Failed to create risk manager");
    
    // Test ETH collateral factor
    let eth_cf = risk_manager.get_collateral_factor("ETH").expect("ETH collateral factor not found");
    assert_eq!(eth_cf.factor, 80);
    assert_eq!(eth_cf.max_exposure, 1000000000000000000000000);
    
    // Test BTC liquidation threshold
    let btc_lt = risk_manager.get_liquidation_threshold("BTC").expect("BTC liquidation threshold not found");
    assert_eq!(btc_lt.threshold, 80);
    assert_eq!(btc_lt.penalty, 12);
    
    // Test USDC liquidation ratio
    let usdc_lr = risk_manager.get_liquidation_ratio("USDC/USD").expect("USDC/USD liquidation ratio not found");
    assert_eq!(usdc_lr.target_ratio, 0.95);
    assert_eq!(usdc_lr.max_ratio, 0.98);
}

/// Test fee distribution
#[test]
fn test_fee_distribution() {
    let risk_manager = RiskManager::new().expect("Failed to create risk manager");
    
    // Test fee distribution with 1000 tokens
    let distribution = risk_manager.distribute_fees(1000000000000000000000); // 1000 tokens
    
    assert_eq!(distribution.insurance, 300000000000000000000); // 300 tokens
    assert_eq!(distribution.treasury, 400000000000000000000); // 400 tokens
    assert_eq!(distribution.stakers, 200000000000000000000); // 200 tokens
    assert_eq!(distribution.development, 100000000000000000000); // 100 tokens
}

/// Test liquidation scenarios
#[test]
fn test_liquidation_scenarios() {
    let risk_manager = RiskManager::new().expect("Failed to create risk manager");
    
    // Test ETH liquidation
    let is_liquidatable = risk_manager.is_liquidatable("ETH", 100000000000000000000, 85000000000000000000);
    assert!(is_liquidatable); // 100 ETH collateral, 85 ETH debt (85% ratio)
    
    // Test BTC liquidation
    let is_liquidatable = risk_manager.is_liquidatable("BTC", 100000000000000000000, 80000000000000000000);
    assert!(is_liquidatable); // 100 BTC collateral, 80 BTC debt (80% ratio)
    
    // Test USDC liquidation
    let is_liquidatable = risk_manager.is_liquidatable("USDC", 1000000000000, 950000000000);
    assert!(is_liquidatable); // 1000 USDC collateral, 950 USDC debt (95% ratio)
}

/// Test risk monitoring
#[test]
fn test_risk_monitoring() {
    let risk_manager = RiskManager::new().expect("Failed to create risk manager");
    
    // Test risk alerts
    let alerts = risk_manager.check_risk_alerts(0.85, 0.15, 0.15);
    assert_eq!(alerts.len(), 3); // All three alerts should trigger
    
    // Test normal conditions (no alerts)
    let alerts = risk_manager.check_risk_alerts(0.50, 0.50, 0.05);
    assert_eq!(alerts.len(), 0); // No alerts should trigger
}

/// Test emergency procedures
#[test]
fn test_emergency_procedures() {
    let risk_manager = RiskManager::new().expect("Failed to create risk manager");
    
    // Test circuit breaker trigger
    let emergency_message = risk_manager.check_emergency(0.06, 0.90); // 6% loss, 90% recovery
    assert!(emergency_message.is_some());
    assert!(emergency_message.unwrap().contains("Circuit breaker"));
    
    // Test recovery condition
    let emergency_message = risk_manager.check_emergency(0.03, 0.96); // 3% loss, 96% recovery
    assert!(emergency_message.is_some());
    assert!(emergency_message.unwrap().contains("recover"));
}

/// Test insurance fund calculations
#[test]
fn test_insurance_fund_calculations() {
    let risk_manager = RiskManager::new().expect("Failed to create risk manager");
    
    // Test insurance coverage calculation
    let coverage = risk_manager.calculate_coverage(100000000000000000000); // 100 token loss
    assert_eq!(coverage, 80000000000000000000); // 80 tokens covered (80%)
    
    // Test maximum payout limit
    let coverage = risk_manager.calculate_coverage(2000000000000000000000); // 2000 token loss
    assert_eq!(coverage, 100000000000000000000000); // Max 100 tokens (not 1600)
}

/// Performance test for risk calculations
#[test]
fn test_risk_calculation_performance() {
    let risk_manager = RiskManager::new().expect("Failed to create risk manager");
    
    // Perform multiple risk calculations to test performance
    for _ in 0..1000 {
        let _ = risk_manager.distribute_fees(1000000000000000000000);
        let _ = risk_manager.calculate_coverage(100000000000000000000);
        let _ = risk_manager.is_liquidatable("ETH", 100000000000000000000, 85000000000000000000);
    }
}

/// Integration test for complete risk workflow
#[test]
fn test_complete_risk_workflow() {
    let risk_manager = RiskManager::new().expect("Failed to create risk manager");
    
    // Simulate a user position
    let collateral_value = 100000000000000000000; // 100 ETH
    let debt_value = 80000000000000000000; // 80 ETH (80% ratio)
    
    // Check if position is liquidatable
    let is_liquidatable = risk_manager.is_liquidatable("ETH", collateral_value, debt_value);
    assert!(is_liquidatable);
    
    // Calculate liquidation penalty
    if let Some(lt) = risk_manager.get_liquidation_threshold("ETH") {
        let penalty = lt.liquidation_penalty(debt_value);
        assert_eq!(penalty, 8000000000000000000); // 8 ETH penalty (10% of 80 ETH)
    }
    
    // Distribute fees from liquidation
    let fees = 10000000000000000000; // 10 ETH in fees
    let distribution = risk_manager.distribute_fees(fees);
    
    // Add to insurance fund
    let insurance_contribution = distribution.insurance;
    let coverage = risk_manager.calculate_coverage(50000000000000000000); // 50 ETH loss
    assert_eq!(coverage, 40000000000000000000); // 40 ETH covered
    
    // Check risk alerts
    let alerts = risk_manager.check_risk_alerts(0.80, 0.30, 0.08);
    assert!(!alerts.is_empty());
}