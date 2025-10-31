//! Data Security Encryption at Rest Validation Tests
//!
//! This module contains tests that validate the Data-at-Rest encryption functionality
//! as defined in the web3_protection_layers.csv file for Layer 5.

use security_layers::data_security::{
    DataAtRestConfig, KeyRotationLog, KmsAccessLog, DataAtRestManager,
};

/// Test DataAtRestConfig creation and validation
#[test]
fn test_data_at_rest_config_creation_and_validation() {
    // Test default configuration
    let config = DataAtRestConfig {
        kms_encryption_enabled: true,
        kms_key_id: Some("test-key-id".to_string()),
        envelope_encryption_enabled: true,
        key_rotation_interval: 86400, // 24 hours
        encryption_algorithm: "AES-256-GCM".to_string(),
    };
    
    assert!(config.kms_encryption_enabled);
    assert_eq!(config.kms_key_id, Some("test-key-id".to_string()));
    assert!(config.envelope_encryption_enabled);
    assert_eq!(config.key_rotation_interval, 86400);
    assert_eq!(config.encryption_algorithm, "AES-256-GCM");
    
    // Test DataAtRestManager creation with valid config
    let manager = DataAtRestManager::new(config.clone()).unwrap();
    
    // Test configuration access
    let manager_config = manager.get_config();
    assert_eq!(manager_config.kms_key_id, Some("test-key-id".to_string()));
    
    // Test invalid configuration (key rotation interval too short)
    let invalid_config = DataAtRestConfig {
        key_rotation_interval: 60, // Less than 1 hour
        ..config.clone()
    };
    
    assert!(DataAtRestManager::new(invalid_config).is_err());
}

/// Test KeyRotationLog functionality
#[test]
fn test_key_rotation_log() {
    let log = KeyRotationLog {
        timestamp: 1234567890,
        key_id: "test-key-001".to_string(),
        reason: "Scheduled rotation".to_string(),
        success: true,
        error_message: None,
    };
    
    assert_eq!(log.timestamp, 1234567890);
    assert_eq!(log.key_id, "test-key-001");
    assert_eq!(log.reason, "Scheduled rotation");
    assert!(log.success);
    assert_eq!(log.error_message, None);
}

/// Test KmsAccessLog functionality
#[test]
fn test_kms_access_log() {
    let log = KmsAccessLog {
        timestamp: 1234567890,
        key_id: "test-key-001".to_string(),
        operation: "encrypt".to_string(),
        success: true,
        error_message: None,
        accessed_by: Some("user@example.com".to_string()),
    };
    
    assert_eq!(log.timestamp, 1234567890);
    assert_eq!(log.key_id, "test-key-001");
    assert_eq!(log.operation, "encrypt");
    assert!(log.success);
    assert_eq!(log.error_message, None);
    assert_eq!(log.accessed_by, Some("user@example.com".to_string()));
}

/// Test DataAtRestManager functionality
#[test]
fn test_data_at_rest_manager() {
    let config = DataAtRestConfig {
        kms_encryption_enabled: true,
        kms_key_id: Some("test-key-id".to_string()),
        envelope_encryption_enabled: true,
        key_rotation_interval: 86400,
        encryption_algorithm: "AES-256-GCM".to_string(),
    };
    
    let mut manager = DataAtRestManager::new(config).unwrap();
    
    // Test configuration access
    let config = manager.get_config();
    assert!(config.kms_encryption_enabled);
    assert_eq!(config.kms_key_id, Some("test-key-id".to_string()));
    assert!(config.envelope_encryption_enabled);
    
    // Test configuration update
    let new_config = DataAtRestConfig {
        kms_key_id: Some("new-key-id".to_string()),
        ..config.clone()
    };
    
    assert!(manager.update_config(new_config.clone()).is_ok());
    
    let updated_config = manager.get_config();
    assert_eq!(updated_config.kms_key_id, Some("new-key-id".to_string()));
    
    // Test invalid configuration update
    let invalid_config = DataAtRestConfig {
        key_rotation_interval: 60, // Less than 1 hour
        ..new_config.clone()
    };
    
    assert!(manager.update_config(invalid_config).is_err());
    
    // Test logging key rotations
    manager.log_key_rotation(KeyRotationLog {
        timestamp: 1234567890,
        key_id: "test-key-001".to_string(),
        reason: "Scheduled rotation".to_string(),
        success: true,
        error_message: None,
    });
    
    // Test retrieving key rotation logs
    let logs = manager.get_key_rotation_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].key_id, "test-key-001");
    assert_eq!(logs[0].reason, "Scheduled rotation");
    
    // Test logging KMS access
    manager.log_kms_access(KmsAccessLog {
        timestamp: 1234567891,
        key_id: "test-key-001".to_string(),
        operation: "encrypt".to_string(),
        success: true,
        error_message: None,
        accessed_by: Some("service-a".to_string()),
    });
    
    // Test retrieving KMS access logs
    let logs = manager.get_kms_access_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].key_id, "test-key-001");
    assert_eq!(logs[0].operation, "encrypt");
    assert_eq!(logs[0].accessed_by, Some("service-a".to_string()));
    
    // Test telemetry report generation
    let report = manager.generate_telemetry_report();
    assert!(report.contains("Data-at-Rest Encryption Logs:"));
    assert!(report.contains("Total Key Rotation Logs: 1"));
    assert!(report.contains("Total KMS Access Logs: 1"));
    assert!(report.contains("test-key-001"));
    assert!(report.contains("Scheduled rotation"));
    assert!(report.contains("encrypt"));
    assert!(report.contains("service-a"));
}

/// Test the specific requirement from the CSV: "KMS-managed disk/volume/db encryption, envelope encryption for fields like PII"
#[test]
fn test_csv_requirement_encryption_mechanisms() {
    let config = DataAtRestConfig {
        kms_encryption_enabled: true, // KMS-managed encryption
        kms_key_id: Some("kms-key-123".to_string()), // KMS key identifier
        envelope_encryption_enabled: true, // Envelope encryption for sensitive fields
        key_rotation_interval: 86400, // 24 hours
        encryption_algorithm: "AES-256-GCM".to_string(), // Strong encryption algorithm
    };
    
    let manager = DataAtRestManager::new(config).unwrap();
    
    // Verify the configuration meets the requirements
    let config = manager.get_config();
    assert!(config.kms_encryption_enabled); // KMS-managed encryption
    assert!(config.kms_key_id.is_some()); // KMS key identifier
    assert!(config.envelope_encryption_enabled); // Envelope encryption for fields like PII
    
    // Verify encryption at rest is enabled
    assert!(manager.is_encryption_at_rest_enabled());
}

/// Test the specific requirement from the CSV: "Key rotation logs, KMS access logs"
#[test]
fn test_csv_requirement_telemetry() {
    let config = DataAtRestConfig {
        kms_encryption_enabled: true,
        kms_key_id: Some("test-key-id".to_string()),
        envelope_encryption_enabled: true,
        key_rotation_interval: 86400,
        encryption_algorithm: "AES-256-GCM".to_string(),
    };
    
    let mut manager = DataAtRestManager::new(config).unwrap();
    
    // Add logs to meet the evidence/telemetry requirement
    manager.log_key_rotation(KeyRotationLog {
        timestamp: 1234567890,
        key_id: "database-encryption-key".to_string(),
        reason: "Scheduled rotation".to_string(),
        success: true,
        error_message: None,
    });
    
    manager.log_kms_access(KmsAccessLog {
        timestamp: 1234567891,
        key_id: "database-encryption-key".to_string(),
        operation: "decrypt".to_string(),
        success: true,
        error_message: None,
        accessed_by: Some("database-service".to_string()),
    });
    
    // Generate the required evidence/telemetry
    let report = manager.generate_telemetry_report();
    
    // Verify the evidence/telemetry requirement is met
    assert!(report.contains("Data-at-Rest Encryption Logs:"));
    assert!(report.contains("Total Key Rotation Logs: 1"));
    assert!(report.contains("Total KMS Access Logs: 1"));
    assert!(report.contains("database-encryption-key"));
    assert!(report.contains("Scheduled rotation"));
    assert!(report.contains("decrypt"));
    assert!(report.contains("database-service"));
}

/// Integration test showing how the Data-at-Rest encryption system works
#[test]
fn test_data_at_rest_integration() {
    // Create a Data-at-Rest configuration for a DEX application
    let config = DataAtRestConfig {
        kms_encryption_enabled: true,
        kms_key_id: Some("dex-kms-key-123".to_string()),
        envelope_encryption_enabled: true,
        key_rotation_interval: 86400, // 24 hours
        encryption_algorithm: "AES-256-GCM".to_string(),
    };
    
    let mut manager = DataAtRestManager::new(config).unwrap();
    
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
    assert!(manager.is_encryption_at_rest_enabled());
    
    // Generate the required evidence/telemetry
    let telemetry_report = manager.generate_telemetry_report();
    println!("Telemetry Report:\n{}", telemetry_report);
    
    // Verify that we have the required evidence
    assert!(telemetry_report.contains("Data-at-Rest Encryption Logs:"));
    assert!(telemetry_report.contains("Total Key Rotation Logs: 1"));
    assert!(telemetry_report.contains("Total KMS Access Logs: 2"));
    assert!(telemetry_report.contains("dex-database-key"));
    assert!(telemetry_report.contains("dex-pii-key"));
    assert!(telemetry_report.contains("Scheduled rotation"));
    assert!(telemetry_report.contains("encrypt"));
    assert!(telemetry_report.contains("decrypt"));
    assert!(telemetry_report.contains("user-service"));
    assert!(telemetry_report.contains("api-service"));
    
    // Verify the goal: "Protect data if disk/db is stolen"
    // By having encryption at rest enabled, we protect data if disk/db is stolen
    assert!(manager.is_encryption_at_rest_enabled());
}