//! Data Privacy PII Minimization Validation Tests
//!
//! This module contains tests that validate the PII minimization functionality
//! for privacy protection as part of the data security implementation.

use security_layers::data_security::{
    DataMinimizationConfig, DataMinimizationManager,
};

/// Test PII minimization with various data types
#[test]
fn test_pii_minimization_with_various_data_types() {
    let config = DataMinimizationConfig {
        store_only_required: true,
        redact_pii_in_logs: true,
        tokenize_high_risk_values: true,
        pii_patterns: vec![
            "email".to_string(),
            "phone".to_string(),
            "ssn".to_string(),
            "credit_card".to_string(),
            "address".to_string(),
        ],
        high_risk_patterns: vec![
            "password".to_string(),
            "private_key".to_string(),
            "secret".to_string(),
            "api_key".to_string(),
        ],
    };

    let mut manager = DataMinimizationManager::new(config);

    // Test PII redaction for various data types
    let test_logs = vec![
        "User login: email=user@example.com, phone=123-456-7890".to_string(),
        "SSN verification: ssn=123-45-6789".to_string(),
        "Payment processed with credit_card=4111-1111-1111-1111".to_string(),
        "Shipping to address=123 Main St, Anytown, USA".to_string(),
        "Authentication with password=super_secret_password_123".to_string(),
        "API access with api_key=sk-abc123def456".to_string(),
        "Wallet access with private_key=0x1234567890abcdef".to_string(),
    ];

    // Process each log entry
    for log in &test_logs {
        let redacted_log = manager.redact_pii(log);
        println!("Original: {}", log);
        println!("Redacted: {}", redacted_log);
    }

    // Test specific redaction cases
    let email_log = "User login: user@example.com";
    let redacted_email = manager.redact_pii(email_log);
    assert!(redacted_email.contains("[EMAIL_REDACTED]"));
    assert!(!redacted_email.contains("user@example.com"));

    let phone_log = "Call customer at 123-456-7890";
    let redacted_phone = manager.redact_pii(phone_log);
    assert!(redacted_phone.contains("[PHONE_REDACTED]"));
    assert!(!redacted_phone.contains("123-456-7890"));

    let ssn_log = "Verify SSN: 123-45-6789";
    let redacted_ssn = manager.redact_pii(ssn_log);
    assert!(redacted_ssn.contains("[SSN_REDACTED]"));
    assert!(!redacted_ssn.contains("123-45-6789"));

    // Test high-risk value tokenization
    let high_risk_values = vec![
        "super_secret_password_123",
        "sk-abc123def456",
        "0x1234567890abcdef",
        "secret_api_key_value",
    ];

    for value in &high_risk_values {
        let tokenized_value = manager.tokenize_value(value);
        assert_ne!(*value, tokenized_value);
        assert!(tokenized_value.starts_with("token_"));
        println!("Original: {}", value);
        println!("Tokenized: {}", tokenized_value);
    }

    // Scan logs for PII to generate reports
    let scanner_report = manager.scan_logs_for_pii(&test_logs);
    assert!(scanner_report.success);
    assert!(scanner_report.pii_instances_found > 0);
    assert!(scanner_report.high_risk_values_found > 0);

    // Verify data minimization is enabled
    assert!(manager.is_data_minimization_enabled());

    println!("PII minimization test passed with {} PII instances and {} high-risk values found", 
             scanner_report.pii_instances_found, scanner_report.high_risk_values_found);
}

/// Test PII minimization edge cases
#[test]
fn test_pii_minimization_edge_cases() {
    let config = DataMinimizationConfig {
        store_only_required: true,
        redact_pii_in_logs: true,
        tokenize_high_risk_values: true,
        pii_patterns: vec![
            "email".to_string(),
            "phone".to_string(),
        ],
        high_risk_patterns: vec![
            "password".to_string(),
            "secret".to_string(),
        ],
    };

    let manager = DataMinimizationManager::new(config);

    // Test edge cases for PII redaction
    let edge_case_logs = vec![
        // Empty log
        "".to_string(),
        // Log without PII
        "System started successfully".to_string(),
        // Log with partial matches
        "User emailaddress attempted login".to_string(),
        // Log with multiple PII instances
        "User user@example.com with phone 123-456-7890 and backup phone 098-765-4321".to_string(),
        // Log with special characters
        "Contact: user+tag@example.com, phone: 123.456.7890".to_string(),
    ];

    for log in &edge_case_logs {
        let redacted_log = manager.redact_pii(log);
        // Should not panic and should return a string
        assert!(!redacted_log.is_empty() || log.is_empty());
        println!("Edge case - Original: '{}', Redacted: '{}'", log, redacted_log);
    }

    // Test edge cases for tokenization
    let edge_case_values = vec![
        "",  // Empty string
        "a", // Single character
        "password", // Exact match to pattern
        "mypassword123", // Contains pattern
    ];

    for value in &edge_case_values {
        let tokenized_value = manager.tokenize_value(value);
        // Should not panic
        println!("Edge case - Original: '{}', Tokenized: '{}'", value, tokenized_value);
    }

    println!("PII minimization edge cases test passed");
}

/// Test PII minimization configuration scenarios
#[test]
fn test_pii_minimization_configuration_scenarios() {
    // Test with all features disabled
    let disabled_config = DataMinimizationConfig {
        store_only_required: false,
        redact_pii_in_logs: false,
        tokenize_high_risk_values: false,
        pii_patterns: vec![],
        high_risk_patterns: vec![],
    };

    let disabled_manager = DataMinimizationManager::new(disabled_config);
    assert!(!disabled_manager.is_data_minimization_enabled());

    let test_log = "User user@example.com logged in with password=secret123";
    let redacted_log = disabled_manager.redact_pii(test_log);
    assert_eq!(test_log, redacted_log); // Should be unchanged

    let tokenized_value = disabled_manager.tokenize_value("secret123");
    assert_eq!("secret123", tokenized_value); // Should be unchanged

    // Test with only PII redaction enabled
    let pii_only_config = DataMinimizationConfig {
        store_only_required: false,
        redact_pii_in_logs: true,
        tokenize_high_risk_values: false,
        pii_patterns: vec!["email".to_string()],
        high_risk_patterns: vec![],
    };

    let pii_only_manager = DataMinimizationManager::new(pii_only_config);
    assert!(!pii_only_manager.is_data_minimization_enabled()); // Should be false because not all features are enabled

    let redacted_log = pii_only_manager.redact_pii(test_log);
    assert!(redacted_log.contains("[EMAIL_REDACTED]"));
    assert!(redacted_log.contains("password=secret123")); // Should not be tokenized

    let tokenized_value = pii_only_manager.tokenize_value("secret123");
    assert_eq!("secret123", tokenized_value); // Should be unchanged

    // Test with only tokenization enabled
    let token_only_config = DataMinimizationConfig {
        store_only_required: false,
        redact_pii_in_logs: false,
        tokenize_high_risk_values: true,
        pii_patterns: vec![],
        high_risk_patterns: vec!["password".to_string()],
    };

    let token_only_manager = DataMinimizationManager::new(token_only_config);
    assert!(!token_only_manager.is_data_minimization_enabled()); // Should be false because not all features are enabled

    let redacted_log = token_only_manager.redact_pii(test_log);
    assert_eq!(test_log, redacted_log); // Should be unchanged

    let tokenized_value = token_only_manager.tokenize_value("secret123");
    assert_ne!("secret123", tokenized_value); // Should be tokenized
    assert!(tokenized_value.starts_with("token_"));

    // Test with all features enabled
    let full_config = DataMinimizationConfig {
        store_only_required: true,
        redact_pii_in_logs: true,
        tokenize_high_risk_values: true,
        pii_patterns: vec!["email".to_string()],
        high_risk_patterns: vec!["password".to_string()],
    };

    let full_manager = DataMinimizationManager::new(full_config);
    assert!(full_manager.is_data_minimization_enabled()); // Should be true because all features are enabled

    let redacted_log = full_manager.redact_pii(test_log);
    assert!(redacted_log.contains("[EMAIL_REDACTED]"));
    assert!(!redacted_log.contains("user@example.com"));

    let tokenized_value = full_manager.tokenize_value("secret123");
    assert_ne!("secret123", tokenized_value);
    assert!(tokenized_value.starts_with("token_"));

    println!("PII minimization configuration scenarios test passed");
}

/// Test PII scanner report functionality
#[test]
fn test_pii_scanner_report_functionality() {
    let config = DataMinimizationConfig {
        store_only_required: true,
        redact_pii_in_logs: true,
        tokenize_high_risk_values: true,
        pii_patterns: vec!["email".to_string(), "phone".to_string()],
        high_risk_patterns: vec!["password".to_string(), "secret".to_string()],
    };

    let mut manager = DataMinimizationManager::new(config);

    // Generate multiple scanner reports
    let test_logs_batches = vec![
        vec![
            "User login: email=user1@example.com".to_string(),
            "Password change: password=new_secret_1".to_string(),
        ],
        vec![
            "User login: email=user2@example.com, phone=123-456-7890".to_string(),
            "API key used: secret=api_key_123".to_string(),
        ],
        vec![
            "System log without PII".to_string(),
        ],
    ];

    // Generate reports for each batch
    for (i, logs) in test_logs_batches.iter().enumerate() {
        let report = manager.scan_logs_for_pii(logs);
        assert!(report.success);
        println!("Batch {}: Found {} PII instances and {} high-risk values", 
                 i + 1, report.pii_instances_found, report.high_risk_values_found);
    }

    // Check that reports are stored
    let reports = manager.get_scanner_reports();
    assert_eq!(reports.len(), 3);
    
    // Check report details
    assert!(reports[0].timestamp > 0);
    assert!(reports[0].success);
    
    // Generate telemetry report
    let telemetry_report = manager.generate_telemetry_report();
    assert!(telemetry_report.contains("Data Minimization Report:"));
    assert!(telemetry_report.contains("Total Scanner Reports: 3"));
    assert!(telemetry_report.contains("Store only required attributes: true"));
    assert!(telemetry_report.contains("Redact PII in logs: true"));
    assert!(telemetry_report.contains("Tokenize high-risk values: true"));

    // Test that old reports are cleaned up (keep only last 1000)
    for _ in 0..1005 {
        let logs = vec!["Test log".to_string()];
        manager.scan_logs_for_pii(&logs);
    }
    
    let reports = manager.get_scanner_reports();
    assert!(reports.len() <= 1000); // Should not exceed 1000 reports

    println!("PII scanner report functionality test passed");
}