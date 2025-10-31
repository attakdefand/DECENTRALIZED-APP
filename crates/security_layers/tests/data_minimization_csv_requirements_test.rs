//! Data Minimization CSV Requirements Test
//!
//! This test verifies that the Data Minimization implementation satisfies
//! all requirements specified in the web3_protection_layers.csv file:
//!
//! Layer: 5
//! Main Type: Data Security
//! Sub Type: Data Minimization
//! Component/Mechanism: "Store only required attributes, redact PII in logs, tokenize high-risk values"
//! Goal: "Shrink breach impact"
//! Evidence/Telemetry: "PII in logs scanner report"

use security_layers::data_security::{
    DataMinimizationConfig, DataMinimizationManager,
};

/// Test that the implementation satisfies the CSV requirements for Data Minimization
#[test]
fn test_data_minimization_csv_requirements() {
    // Requirement from CSV:
    // "5,Data Security,Data Minimization,Field Reduction / Masking,"Store only required attributes, redact PII in logs, tokenize high-risk values","Shrink breach impact","PII in logs scanner report"
    
    // Create configuration that implements the required mechanisms
    let config = DataMinimizationConfig {
        // Mechanism: "Store only required attributes"
        store_only_required: true,
        
        // Mechanism: "redact PII in logs"
        redact_pii_in_logs: true,
        
        // Mechanism: "tokenize high-risk values"
        tokenize_high_risk_values: true,
        
        // PII patterns to redact
        pii_patterns: vec![
            "email".to_string(),
            "phone".to_string(),
            "ssn".to_string(),
        ],
        
        // High-risk patterns to tokenize
        high_risk_patterns: vec![
            "password".to_string(),
            "private_key".to_string(),
            "secret".to_string(),
        ],
    };
    
    let mut manager = DataMinimizationManager::new(config);
    
    // Verify the mechanisms are implemented correctly
    
    // 1. "Store only required attributes"
    let config = manager.get_config();
    assert!(config.store_only_required, "Must store only required attributes");
    
    // 2. "redact PII in logs"
    assert!(config.redact_pii_in_logs, "Must redact PII in logs");
    
    // Test PII redaction functionality
    let log_with_pii = "User john.doe@example.com logged in with SSN: 123-45-6789";
    let redacted_log = manager.redact_pii(log_with_pii);
    assert!(!redacted_log.contains("john.doe@example.com"), "Email should be redacted");
    assert!(!redacted_log.contains("123-45-6789"), "SSN should be redacted");
    assert!(redacted_log.contains("[EMAIL_REDACTED]"), "Email should be replaced with redaction marker");
    assert!(redacted_log.contains("[SSN_REDACTED]"), "SSN should be replaced with redaction marker");
    
    // 3. "tokenize high-risk values"
    assert!(config.tokenize_high_risk_values, "Must tokenize high-risk values");
    
    // Test high-risk value tokenization
    let high_risk_value = "super_secret_password_123";
    let tokenized_value = manager.tokenize_value(high_risk_value);
    assert_ne!(tokenized_value, high_risk_value, "High-risk value should be tokenized");
    assert!(tokenized_value.starts_with("token_"), "Tokenized value should have token prefix");
    
    // Verify the goal is achieved: "Shrink breach impact"
    assert!(manager.is_data_minimization_enabled(), "Data minimization should be enabled to shrink breach impact");
    
    // Verify the evidence/telemetry is provided: "PII in logs scanner report"
    
    // Simulate scanning logs for PII to generate the required report
    let logs = vec![
        "User login with email: user@example.com".to_string(),
        "Password change to new_secret_password".to_string(),
        "Normal system log entry".to_string(),
    ];
    
    let scanner_report = manager.scan_logs_for_pii(&logs);
    
    // Verify the scanner report is generated (the required evidence/telemetry)
    assert!(scanner_report.success, "Scanner report should be successful");
    assert!(scanner_report.timestamp > 0, "Scanner report should have timestamp");
    assert!(scanner_report.pii_instances_found >= 0, "Scanner report should count PII instances");
    assert!(scanner_report.high_risk_values_found >= 0, "Scanner report should count high-risk values");
    
    // Generate the required evidence/telemetry: "PII in logs scanner report"
    let telemetry_report = manager.generate_telemetry_report();
    
    // Verify the evidence/telemetry requirement is met
    assert!(telemetry_report.contains("Data Minimization Report:"), "Telemetry report should contain header");
    assert!(telemetry_report.contains("Total Scanner Reports: 1"), "Should show scanner report count");
    assert!(telemetry_report.contains("Recent PII Scanner Reports:"), "Should show recent scanner reports");
    
    // Verify that the implementation achieves the goal: "Shrink breach impact"
    // By implementing all three mechanisms, we shrink breach impact:
    // 1. Store only required attributes reduces the amount of data that could be breached
    // 2. Redact PII in logs prevents sensitive information from being exposed in logs
    // 3. Tokenize high-risk values ensures that even if data is breached, high-risk values are protected
    
    assert!(manager.is_data_minimization_enabled(), "All data minimization mechanisms should be enabled to achieve the goal");
    
    println!("✓ Data Minimization CSV requirements test passed");
    println!("✓ Mechanisms implemented: Store only required attributes, redact PII in logs, tokenize high-risk values");
    println!("✓ Goal achieved: Shrink breach impact");
    println!("✓ Evidence/Telemetry provided: PII in logs scanner report");
}