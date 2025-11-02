//! Tests for Proof of Reserves validation and treasury management features

use core::risk::{RiskManager, ProofOfReservesReport, TreasuryMetrics};
use std::collections::HashMap;

/// Test Proof of Reserves report creation and validation
#[test]
fn test_proof_of_reserves_report() {
    let mut reserves = HashMap::new();
    reserves.insert("ETH".to_string(), 100000000000000000000);
    reserves.insert("USDC".to_string(), 2000000000000);
    
    let mut report = ProofOfReservesReport::new(
        reserves.clone(),
        500000.0,
        "validator_signature".to_string(),
        "report_hash".to_string()
    );
    
    // Test initial state
    assert_eq!(report.total_value_usd, 500000.0);
    assert_eq!(report.validator_signature, "validator_signature");
    assert_eq!(report.report_hash, "report_hash");
    
    // Test freshness update
    report.update_freshness();
    // Freshness should be very small since we just created it
    assert!(report.freshness_hours < 1.0);
    
    // Test is_fresh
    assert!(report.is_fresh());
}

/// Test treasury metrics tracking
#[test]
fn test_treasury_metrics() {
    let mut metrics = TreasuryMetrics::new();
    
    // Test initial state
    assert_eq!(metrics.limit_breach_count, 0);
    assert!(metrics.por_freshness_hours >= 0.0);
    assert!(metrics.is_within_limits());
    
    // Test update with fresh report
    metrics.update(12.5, 0);
    assert_eq!(metrics.por_freshness_hours, 12.5);
    assert_eq!(metrics.limit_breach_count, 0);
    assert!(metrics.is_within_limits());
    
    // Test update with limit breach
    metrics.update(12.5, 1);
    assert_eq!(metrics.limit_breach_count, 1);
    assert!(!metrics.is_within_limits()); // Should fail due to breach count > 0
    
    // Test with stale report
    metrics.update(25.0, 0);
    assert!(!metrics.is_within_limits()); // Should fail due to freshness >= 24
}

/// Test RiskManager Proof of Reserves features
#[test]
fn test_risk_manager_por_features() {
    let mut risk_manager = RiskManager::new().unwrap();
    
    // Test adding a POR report
    let mut reserves = HashMap::new();
    reserves.insert("ETH".to_string(), 100000000000000000000);
    let report = ProofOfReservesReport::new(
        reserves,
        500000.0,
        "validator_signature".to_string(),
        "report_hash".to_string()
    );
    
    risk_manager.add_por_report(report);
    assert_eq!(risk_manager.por_reports.len(), 1);
    
    // Test validation with fresh report
    let issues = risk_manager.validate_por_reports();
    // Should be empty since the report is fresh
    assert!(issues.is_empty());
    
    // Test recording a limit breach
    risk_manager.record_limit_breach();
    assert_eq!(risk_manager.treasury_metrics.limit_breach_count, 1);
    
    // Test evidence generation
    let por_evidence = risk_manager.generate_por_evidence();
    assert!(por_evidence.contains("Proof of Reserves Report Evidence"));
    
    let breach_evidence = risk_manager.generate_limit_breach_evidence();
    assert!(breach_evidence.contains("Limit Breach Evidence"));
}

/// Test evidence generation
#[test]
fn test_evidence_generation() {
    let mut reserves = HashMap::new();
    reserves.insert("ETH".to_string(), 100000000000000000000);
    reserves.insert("USDC".to_string(), 2000000000000);
    
    let mut report = ProofOfReservesReport::new(
        reserves,
        500000.0,
        "validator_signature".to_string(),
        "report_hash".to_string()
    );
    
    // Test evidence content
    let evidence_content = format!(
        "{{\
        \"reserves_data\": {:?},\
        \"total_value_usd\": {},\
        \"validator_signature\": \"{}\",\
        \"timestamp\": {}\
        }}",
        report.reserves,
        report.total_value_usd,
        report.validator_signature,
        report.timestamp
    );
    
    assert!(evidence_content.contains("ETH"));
    assert!(evidence_content.contains("USDC"));
    assert!(evidence_content.contains("500000"));
    assert!(evidence_content.contains("validator_signature"));
}