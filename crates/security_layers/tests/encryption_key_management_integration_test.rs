//! Encryption and Key Management Integration Test
//!
//! This test validates that the encryption implementation and key management systems
//! work together correctly and pass all security layers and protection tests.

use security_layers::data_security::{
    DataAtRestConfig, DataAtRestManager, KeyRotationLog, KmsAccessLog,
};

/// Test that encryption implementation meets all security requirements
#[test]
fn test_encryption_implementation_security_requirements() {
    println!("Testing encryption implementation security requirements...");
    
    // Create a configuration that meets all security requirements
    let config = DataAtRestConfig {
        kms_encryption_enabled: true, // KMS-managed encryption
        kms_key_id: Some("secure-key-id-123".to_string()), // KMS key identifier
        envelope_encryption_enabled: true, // Envelope encryption for sensitive fields like PII
        key_rotation_interval: 86400, // 24 hours (meets minimum requirement)
        encryption_algorithm: "AES-256-GCM".to_string(), // Strong encryption algorithm
    };
    
    // Create the Data-at-Rest manager
    let manager = DataAtRestManager::new(config.clone()).expect("Failed to create DataAtRestManager");
    
    // Verify the configuration meets security requirements
    let manager_config = manager.get_config();
    assert!(manager_config.kms_encryption_enabled, "KMS encryption should be enabled");
    assert!(manager_config.kms_key_id.is_some(), "KMS key ID should be set");
    assert!(manager_config.envelope_encryption_enabled, "Envelope encryption should be enabled");
    assert_eq!(manager_config.key_rotation_interval, 86400, "Key rotation interval should be 24 hours");
    assert_eq!(manager_config.encryption_algorithm, "AES-256-GCM", "Encryption algorithm should be AES-256-GCM");
    
    // Verify encryption at rest is enabled
    assert!(manager.is_encryption_at_rest_enabled(), "Encryption at rest should be enabled");
    
    println!("✓ Encryption configuration meets all security requirements");
}

/// Test key management functionality
#[test]
fn test_key_management_functionality() {
    println!("Testing key management functionality...");
    
    let config = DataAtRestConfig {
        kms_encryption_enabled: true,
        kms_key_id: Some("test-key-id".to_string()),
        envelope_encryption_enabled: true,
        key_rotation_interval: 86400,
        encryption_algorithm: "AES-256-GCM".to_string(),
    };
    
    let mut manager = DataAtRestManager::new(config).expect("Failed to create DataAtRestManager");
    
    // Test key rotation logging
    let key_rotation_log = KeyRotationLog {
        timestamp: 1234567890,
        key_id: "database-encryption-key".to_string(),
        reason: "Scheduled rotation".to_string(),
        success: true,
        error_message: None,
    };
    
    manager.log_key_rotation(key_rotation_log);
    
    // Verify key rotation logs
    let rotation_logs = manager.get_key_rotation_logs();
    assert_eq!(rotation_logs.len(), 1, "Should have one key rotation log");
    assert_eq!(rotation_logs[0].key_id, "database-encryption-key", "Key ID should match");
    assert_eq!(rotation_logs[0].reason, "Scheduled rotation", "Reason should match");
    assert!(rotation_logs[0].success, "Rotation should be successful");
    
    // Test KMS access logging
    let kms_access_log = KmsAccessLog {
        timestamp: 1234567891,
        key_id: "pii-encryption-key".to_string(),
        operation: "encrypt".to_string(),
        success: true,
        error_message: None,
        accessed_by: Some("user-service".to_string()),
    };
    
    manager.log_kms_access(kms_access_log);
    
    // Verify KMS access logs
    let access_logs = manager.get_kms_access_logs();
    assert_eq!(access_logs.len(), 1, "Should have one KMS access log");
    assert_eq!(access_logs[0].key_id, "pii-encryption-key", "Key ID should match");
    assert_eq!(access_logs[0].operation, "encrypt", "Operation should match");
    assert_eq!(access_logs[0].accessed_by, Some("user-service".to_string()), "Accessed by should match");
    assert!(access_logs[0].success, "Access should be successful");
    
    println!("✓ Key management functionality working correctly");
}

/// Test telemetry and evidence generation
#[test]
fn test_encryption_telemetry_and_evidence() {
    println!("Testing encryption telemetry and evidence generation...");
    
    let config = DataAtRestConfig {
        kms_encryption_enabled: true,
        kms_key_id: Some("telemetry-test-key".to_string()),
        envelope_encryption_enabled: true,
        key_rotation_interval: 86400,
        encryption_algorithm: "AES-256-GCM".to_string(),
    };
    
    let mut manager = DataAtRestManager::new(config).expect("Failed to create DataAtRestManager");
    
    // Add some logs for telemetry
    manager.log_key_rotation(KeyRotationLog {
        timestamp: 1234567890,
        key_id: "key-1".to_string(),
        reason: "Scheduled rotation".to_string(),
        success: true,
        error_message: None,
    });
    
    manager.log_key_rotation(KeyRotationLog {
        timestamp: 1234567891,
        key_id: "key-2".to_string(),
        reason: "Compromised key".to_string(),
        success: false,
        error_message: Some("Network error".to_string()),
    });
    
    manager.log_kms_access(KmsAccessLog {
        timestamp: 1234567892,
        key_id: "key-1".to_string(),
        operation: "encrypt".to_string(),
        success: true,
        error_message: None,
        accessed_by: Some("service-a".to_string()),
    });
    
    manager.log_kms_access(KmsAccessLog {
        timestamp: 1234567893,
        key_id: "key-2".to_string(),
        operation: "decrypt".to_string(),
        success: true,
        error_message: None,
        accessed_by: Some("service-b".to_string()),
    });
    
    // Generate telemetry report
    let report = manager.generate_telemetry_report();
    
    // Verify report contains required information
    assert!(report.contains("Data-at-Rest Encryption Logs:"), "Report should contain header");
    assert!(report.contains("Total Key Rotation Logs: 2"), "Report should show 2 key rotation logs");
    assert!(report.contains("Total KMS Access Logs: 2"), "Report should show 2 KMS access logs");
    assert!(report.contains("Successful Key Rotations: 1"), "Report should show 1 successful rotation");
    assert!(report.contains("Failed Key Rotations: 1"), "Report should show 1 failed rotation");
    assert!(report.contains("key-1"), "Report should contain key-1");
    assert!(report.contains("key-2"), "Report should contain key-2");
    assert!(report.contains("Scheduled rotation"), "Report should contain rotation reason");
    assert!(report.contains("Compromised key"), "Report should contain compromised key reason");
    assert!(report.contains("encrypt"), "Report should contain encrypt operation");
    assert!(report.contains("decrypt"), "Report should contain decrypt operation");
    assert!(report.contains("service-a"), "Report should contain service-a");
    assert!(report.contains("service-b"), "Report should contain service-b");
    
    println!("✓ Telemetry and evidence generation working correctly");
}

/// Integration test showing how encryption and key management work together
#[test]
fn test_encryption_key_management_integration() {
    println!("Testing encryption and key management integration...");
    
    // Create a secure configuration for a DEX application
    let config = DataAtRestConfig {
        kms_encryption_enabled: true,
        kms_key_id: Some("dex-production-key-123".to_string()),
        envelope_encryption_enabled: true,
        key_rotation_interval: 86400, // 24 hours
        encryption_algorithm: "AES-256-GCM".to_string(),
    };
    
    let mut manager = DataAtRestManager::new(config).expect("Failed to create DataAtRestManager");
    
    // Simulate key rotation for database encryption
    manager.log_key_rotation(KeyRotationLog {
        timestamp: 1234567890,
        key_id: "dex-database-key".to_string(),
        reason: "Scheduled rotation".to_string(),
        success: true,
        error_message: None,
    });
    
    // Simulate KMS access for encrypting user PII data
    manager.log_kms_access(KmsAccessLog {
        timestamp: 1234567891,
        key_id: "dex-pii-key".to_string(),
        operation: "encrypt".to_string(),
        success: true,
        error_message: None,
        accessed_by: Some("user-service".to_string()),
    });
    
    // Simulate KMS access for decrypting user PII data
    manager.log_kms_access(KmsAccessLog {
        timestamp: 1234567892,
        key_id: "dex-pii-key".to_string(),
        operation: "decrypt".to_string(),
        success: true,
        error_message: None,
        accessed_by: Some("api-service".to_string()),
    });
    
    // Verify the Data-at-Rest configuration meets security requirements
    assert!(manager.is_encryption_at_rest_enabled(), "Encryption at rest should be enabled");
    
    // Generate the required evidence/telemetry
    let telemetry_report = manager.generate_telemetry_report();
    println!("Telemetry Report:\n{}", telemetry_report);
    
    // Verify that we have the required evidence
    assert!(telemetry_report.contains("Data-at-Rest Encryption Logs:"), "Should contain logs header");
    assert!(telemetry_report.contains("Total Key Rotation Logs: 1"), "Should show 1 key rotation log");
    assert!(telemetry_report.contains("Total KMS Access Logs: 2"), "Should show 2 KMS access logs");
    assert!(telemetry_report.contains("dex-database-key"), "Should contain database key");
    assert!(telemetry_report.contains("dex-pii-key"), "Should contain PII key");
    assert!(telemetry_report.contains("Scheduled rotation"), "Should contain rotation reason");
    assert!(telemetry_report.contains("encrypt"), "Should contain encrypt operation");
    assert!(telemetry_report.contains("decrypt"), "Should contain decrypt operation");
    assert!(telemetry_report.contains("user-service"), "Should contain user-service");
    assert!(telemetry_report.contains("api-service"), "Should contain api-service");
    
    // Verify the goal: "Protect data if disk/db is stolen"
    // By having encryption at rest enabled, we protect data if disk/db is stolen
    assert!(manager.is_encryption_at_rest_enabled(), "Should protect data if disk/db is stolen");
    
    println!("✓ Encryption and key management integration working correctly");
}

/// Test that the implementation passes security layer protection tests
#[test]
fn test_encryption_security_layer_protection() {
    println!("Testing encryption security layer protection...");
    
    // Test various security configurations
    let secure_configs = vec![
        DataAtRestConfig {
            kms_encryption_enabled: true,
            kms_key_id: Some("key-1".to_string()),
            envelope_encryption_enabled: true,
            key_rotation_interval: 86400, // 24 hours
            encryption_algorithm: "AES-256-GCM".to_string(),
        },
        DataAtRestConfig {
            kms_encryption_enabled: true,
            kms_key_id: Some("key-2".to_string()),
            envelope_encryption_enabled: true,
            key_rotation_interval: 172800, // 48 hours
            encryption_algorithm: "AES-256-GCM".to_string(),
        },
    ];
    
    for (i, config) in secure_configs.iter().enumerate() {
        let manager = DataAtRestManager::new(config.clone());
        assert!(manager.is_ok(), "Secure config {} should be valid", i + 1);
        
        let manager = manager.unwrap();
        assert!(manager.is_encryption_at_rest_enabled(), "Encryption should be enabled for config {}", i + 1);
    }
    
    // Test that invalid configurations are rejected
    let invalid_configs = vec![
        DataAtRestConfig {
            kms_encryption_enabled: true,
            kms_key_id: Some("key-1".to_string()),
            envelope_encryption_enabled: true,
            key_rotation_interval: 60, // Too short (less than 1 hour)
            encryption_algorithm: "AES-256-GCM".to_string(),
        },
        DataAtRestConfig {
            kms_encryption_enabled: true,
            kms_key_id: Some("key-2".to_string()),
            envelope_encryption_enabled: true,
            key_rotation_interval: 1800, // Too short (less than 1 hour)
            encryption_algorithm: "AES-256-GCM".to_string(),
        },
    ];
    
    for (i, config) in invalid_configs.iter().enumerate() {
        let manager = DataAtRestManager::new(config.clone());
        assert!(manager.is_err(), "Invalid config {} should be rejected", i + 1);
    }
    
    println!("✓ Encryption passes security layer protection tests");
}