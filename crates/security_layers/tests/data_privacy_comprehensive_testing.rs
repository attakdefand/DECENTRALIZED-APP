//! Comprehensive Data Privacy Testing
//!
//! This module contains comprehensive tests for data privacy functionality,
//! including PII minimization, encryption, and protection mechanisms.

use core::data_protection::{FieldEncryption, PiiMap, DsrErasureManager, PiiField, PiiClassification, DsrRequest, DsrRequestType, DsrRequestStatus};
use security_layers::data_security::{DataMinimizationConfig, DataMinimizationManager};
use std::collections::HashSet;

/// Test comprehensive privacy implementation with PII minimization
#[test]
fn test_comprehensive_privacy_implementation() {
    println!("Starting comprehensive privacy implementation test");
    
    // 1. Test Field Encryption (part of privacy protection)
    let key = [1u8; 32]; // Test key
    let encryption = FieldEncryption::new(key, "privacy-test-key".to_string());
    
    let plaintext = b"Sensitive user data that requires privacy protection";
    let encrypted = encryption.encrypt(plaintext).expect("Encryption should succeed");
    let decrypted = encryption.decrypt(&encrypted).expect("Decryption should succeed");
    
    assert_eq!(plaintext, decrypted.as_slice());
    assert_ne!(plaintext, encrypted.as_slice()); // Ensure data is actually encrypted
    
    println!("✓ Field encryption/decryption working correctly");
    
    // 2. Test PII Map and Classification (part of PII minimization)
    let mut pii_map = PiiMap::new();
    
    // Add various PII fields with different classifications
    let pii_fields = vec![
        PiiField {
            name: "user_email".to_string(),
            description: "User email address".to_string(),
            classification: PiiClassification::Confidential,
            storage_location: "users table".to_string(),
            retention_period: 365,
            legal_basis: "Contract performance".to_string(),
        },
        PiiField {
            name: "user_ssn".to_string(),
            description: "User social security number".to_string(),
            classification: PiiClassification::HighlyConfidential,
            storage_location: "secure users table".to_string(),
            retention_period: 730,
            legal_basis: "Legal obligation".to_string(),
        },
        PiiField {
            name: "user_phone".to_string(),
            description: "User phone number".to_string(),
            classification: PiiClassification::Confidential,
            storage_location: "users table".to_string(),
            retention_period: 365,
            legal_basis: "Legitimate interest".to_string(),
        },
    ];
    
    for field in &pii_fields {
        pii_map.add_field(field.clone());
    }
    
    // Verify PII fields were added correctly
    assert!(pii_map.has_field("user_email"));
    assert!(pii_map.has_field("user_ssn"));
    assert!(pii_map.has_field("user_phone"));
    assert!(!pii_map.has_field("nonexistent_field"));
    
    // Test classification filtering
    let confidential_fields = pii_map.get_fields_by_classification(PiiClassification::Confidential);
    assert_eq!(confidential_fields.len(), 2); // email and phone
    
    let highly_confidential_fields = pii_map.get_fields_by_classification(PiiClassification::HighlyConfidential);
    assert_eq!(highly_confidential_fields.len(), 1); // SSN
    
    println!("✓ PII mapping and classification working correctly");
    
    // 3. Test Data Minimization (core privacy requirement)
    let minimization_config = DataMinimizationConfig {
        store_only_required: true,
        redact_pii_in_logs: true,
        tokenize_high_risk_values: true,
        pii_patterns: vec![
            "email".to_string(),
            "phone".to_string(),
            "ssn".to_string(),
        ],
        high_risk_patterns: vec![
            "password".to_string(),
            "private_key".to_string(),
            "secret".to_string(),
        ],
    };
    
    let mut minimization_manager = DataMinimizationManager::new(minimization_config);
    
    // Test PII redaction
    let log_with_pii = "User user@example.com logged in with phone 123-456-7890 and SSN 123-45-6789";
    let redacted_log = minimization_manager.redact_pii(log_with_pii);
    
    assert!(!redacted_log.contains("user@example.com"));
    assert!(!redacted_log.contains("123-456-7890"));
    assert!(!redacted_log.contains("123-45-6789"));
    assert!(redacted_log.contains("[EMAIL_REDACTED]"));
    assert!(redacted_log.contains("[PHONE_REDACTED]"));
    assert!(redacted_log.contains("[SSN_REDACTED]"));
    
    println!("✓ PII redaction working correctly");
    
    // Test high-risk value tokenization
    let high_risk_values = vec![
        "super_secret_password_123",
        "private_key_0x1234567890abcdef",
        "secret_api_key_value",
    ];
    
    for value in &high_risk_values {
        let tokenized = minimization_manager.tokenize_value(value);
        assert_ne!(*value, tokenized);
        assert!(tokenized.starts_with("token_"));
    }
    
    println!("✓ High-risk value tokenization working correctly");
    
    // Test logs scanning for PII
    let test_logs = vec![
        "User login: email=user@example.com".to_string(),
        "Password change: password=new_secret_123".to_string(),
        "Normal system log without PII".to_string(),
        "User phone update: phone=098-765-4321".to_string(),
    ];
    
    let scanner_report = minimization_manager.scan_logs_for_pii(&test_logs);
    assert!(scanner_report.success);
    assert!(scanner_report.pii_instances_found > 0);
    assert!(scanner_report.high_risk_values_found > 0);
    
    println!("✓ PII scanning working correctly, found {} PII instances and {} high-risk values", 
             scanner_report.pii_instances_found, scanner_report.high_risk_values_found);
    
    // Test telemetry report generation
    let telemetry_report = minimization_manager.generate_telemetry_report();
    assert!(telemetry_report.contains("Data Minimization Report:"));
    assert!(telemetry_report.contains("Store only required attributes: true"));
    assert!(telemetry_report.contains("Redact PII in logs: true"));
    assert!(telemetry_report.contains("Tokenize high-risk values: true"));
    
    println!("✓ Telemetry report generation working correctly");
    
    // 4. Test DSR (Data Subject Request) functionality
    let mut dsr_manager = DsrErasureManager::new(pii_map);
    
    // Create DSR request for PII erasure
    let mut affected_fields = HashSet::new();
    affected_fields.insert("user_email".to_string());
    affected_fields.insert("user_phone".to_string());
    
    let dsr_request = DsrRequest {
        id: "privacy-test-001".to_string(),
        request_type: DsrRequestType::Erasure,
        requester_id: "user-privacy-test".to_string(),
        timestamp: 1234567890,
        status: DsrRequestStatus::Pending,
        affected_fields,
        resolution_notes: None,
    };
    
    // Submit and process DSR request
    assert!(dsr_manager.submit_request(dsr_request).is_ok());
    assert!(dsr_manager.process_erasure("privacy-test-001").is_ok());
    
    // Verify request status
    let status = dsr_manager.get_request_status("privacy-test-001").unwrap();
    assert_eq!(status, &DsrRequestStatus::Completed);
    
    // Verify audit logging
    let audit_log = dsr_manager.get_audit_log();
    assert!(!audit_log.is_empty());
    assert!(audit_log.iter().any(|log| log.contains("privacy-test-001")));
    assert!(audit_log.iter().any(|log| log.contains("Erasure completed")));
    
    println!("✓ DSR erasure functionality working correctly");
    
    // 5. Verify overall privacy compliance
    assert!(minimization_manager.is_data_minimization_enabled());
    
    println!("✓ All privacy mechanisms working together correctly");
    println!("Comprehensive privacy implementation test passed!");
}

/// Test privacy boundary conditions and edge cases
#[test]
fn test_privacy_boundary_conditions() {
    println!("Starting privacy boundary conditions test");
    
    // Test with empty PII map
    let empty_pii_map = PiiMap::new();
    let mut empty_dsr_manager = DsrErasureManager::new(empty_pii_map);
    
    // Test DSR with empty map
    let empty_request = DsrRequest {
        id: "empty-test-001".to_string(),
        request_type: DsrRequestType::Erasure,
        requester_id: "user-empty-test".to_string(),
        timestamp: 1234567890,
        status: DsrRequestStatus::Pending,
        affected_fields: HashSet::new(),
        resolution_notes: None,
    };
    
    assert!(empty_dsr_manager.submit_request(empty_request).is_ok());
    // Processing should succeed even with no fields to erase
    assert!(empty_dsr_manager.process_erasure("empty-test-001").is_ok());
    
    // Test data minimization with empty patterns
    let empty_config = DataMinimizationConfig {
        store_only_required: true,
        redact_pii_in_logs: true,
        tokenize_high_risk_values: true,
        pii_patterns: vec![],
        high_risk_patterns: vec![],
    };
    
    let empty_manager = DataMinimizationManager::new(empty_config);
    let log_without_pii = "Normal system log without any PII";
    let result = empty_manager.redact_pii(log_without_pii);
    assert_eq!(log_without_pii, result); // Should be unchanged
    
    // Test with minimal data
    let minimal_config = DataMinimizationConfig {
        store_only_required: false, // Disable one feature
        redact_pii_in_logs: true,
        tokenize_high_risk_values: true,
        pii_patterns: vec!["test".to_string()],
        high_risk_patterns: vec!["secret".to_string()],
    };
    
    let mut minimal_manager = DataMinimizationManager::new(minimal_config);
    assert!(!minimal_manager.is_data_minimization_enabled()); // Should be false because not all features are enabled
    
    // Enable all features
    minimal_manager.update_config(DataMinimizationConfig {
        store_only_required: true,
        redact_pii_in_logs: true,
        tokenize_high_risk_values: true,
        pii_patterns: vec!["test".to_string()],
        high_risk_patterns: vec!["secret".to_string()],
    });
    
    assert!(minimal_manager.is_data_minimization_enabled()); // Should be true now
    
    println!("✓ Privacy boundary conditions test passed");
}

/// Test privacy integration with security layers
#[test]
fn test_privacy_security_layers_integration() {
    println!("Starting privacy security layers integration test");
    
    // Test that privacy mechanisms work with security layers framework
    let privacy_config = DataMinimizationConfig {
        store_only_required: true,
        redact_pii_in_logs: true,
        tokenize_high_risk_values: true,
        pii_patterns: vec![
            "email".to_string(),
            "phone".to_string(),
            "wallet".to_string(),
        ],
        high_risk_patterns: vec![
            "password".to_string(),
            "private_key".to_string(),
            "secret".to_string(),
        ],
    };
    
    let mut privacy_manager = DataMinimizationManager::new(privacy_config);
    
    // Simulate a realistic privacy scenario for a decentralized application
    let app_logs = vec![
        "User 0x742d35Cc6634C0532925a3b8D91D0b6bCf8fA1E2 connected with email user@example.com".to_string(),
        "Transaction processed for wallet 0x742d35Cc6634C0532925a3b8D91D0b6bCf8fA1E2, phone 123-456-7890".to_string(),
        "Authentication successful with password super_secret_wallet_password_123".to_string(),
        "Private key 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef accessed".to_string(),
        "API call with secret api_key_789xyz".to_string(),
        "Normal system operation log".to_string(),
    ];
    
    // Process logs through privacy mechanisms
    let mut processed_logs = Vec::new();
    for log in &app_logs {
        let processed_log = privacy_manager.redact_pii(log);
        processed_logs.push(processed_log);
    }
    
    // Verify PII was redacted
    for log in &processed_logs {
        // Should not contain original PII that matches regex patterns
        assert!(!log.contains("user@example.com"));
        assert!(!log.contains("123-456-7890"));
        assert!(!log.contains("123-45-6789"));
        
        // Should contain redaction markers for regex patterns
        if log.contains("email") || log.contains("user@") {
            assert!(log.contains("[EMAIL_REDACTED]"));
        }
        if log.contains("phone") || log.contains("123-456-7890") {
            assert!(log.contains("[PHONE_REDACTED]"));
        }
        if log.contains("SSN") || log.contains("123-45-6789") {
            assert!(log.contains("[SSN_REDACTED]"));
        }
    }
    
    // Tokenize high-risk values
    let high_risk_values = vec![
        "super_secret_wallet_password_123",
        "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
        "api_key_789xyz",
    ];
    
    let mut tokenized_values = Vec::new();
    for value in &high_risk_values {
        let tokenized = privacy_manager.tokenize_value(value);
        assert_ne!(*value, tokenized);
        assert!(tokenized.starts_with("token_"));
        tokenized_values.push(tokenized);
    }
    
    // Scan logs for PII to generate compliance reports
    let scanner_report = privacy_manager.scan_logs_for_pii(&app_logs);
    assert!(scanner_report.success);
    assert!(scanner_report.pii_instances_found > 0);
    assert!(scanner_report.high_risk_values_found > 0);
    
    // Generate compliance telemetry
    let telemetry_report = privacy_manager.generate_telemetry_report();
    assert!(telemetry_report.contains("Data Minimization Report:"));
    assert!(telemetry_report.contains("Total Scanner Reports: 1"));
    assert!(telemetry_report.contains("Recent PII Scanner Reports:"));
    
    // Verify privacy goal achievement: "Shrink breach impact"
    assert!(privacy_manager.is_data_minimization_enabled());
    
    println!("✓ Privacy mechanisms successfully reduced data exposure");
    println!("✓ PII redaction prevented sensitive data exposure in logs");
    println!("✓ High-risk value tokenization protects critical secrets");
    println!("✓ Compliance reporting provides required telemetry");
    println!("✓ Privacy goal 'Shrink breach impact' achieved");
    
    println!("Privacy security layers integration test passed!");
}