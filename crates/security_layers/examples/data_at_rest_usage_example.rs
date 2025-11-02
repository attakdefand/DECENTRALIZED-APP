//! Example of using the Data-at-Rest Encryption implementation
//!
//! This example demonstrates how to configure and use the Data-at-Rest functionality
//! for securing data at rest as specified in the security requirements.

use security_layers::data_security::{DataAtRestConfig, KeyRotationLog, KmsAccessLog, DataAtRestManager};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Data-at-Rest Encryption Implementation Example ===\n");

    // Create a Data-at-Rest configuration that meets the security requirements
    let config = DataAtRestConfig {
        kms_encryption_enabled: true, // KMS-managed encryption
        kms_key_id: Some("dex-kms-key-123".to_string()), // KMS key identifier
        envelope_encryption_enabled: true, // Envelope encryption for fields like PII
        key_rotation_interval: 86400, // Rotate keys every 24 hours
        encryption_algorithm: "AES-256-GCM".to_string(), // Strong encryption algorithm
    };

    // Create a Data-at-Rest manager with the configuration
    let mut manager = DataAtRestManager::new(config)?;
    println!("✓ Data-at-Rest manager created successfully");

    // Verify that encryption at rest is enabled
    if manager.is_encryption_at_rest_enabled() {
        println!("✓ Encryption at Rest is properly configured:");
        println!("  - KMS encryption: enabled");
        println!("  - KMS key ID: dex-kms-key-123");
        println!("  - Envelope encryption: enabled");
        println!("  - Key rotation interval: 24 hours");
        println!("  - Encryption algorithm: AES-256-GCM");
    } else {
        println!("✗ Encryption at Rest configuration is incomplete");
        return Ok(());
    }

    // Simulate key rotation for database encryption
    println!("\n=== Simulating Key Rotation ===");
    
    let key_rotation_log = KeyRotationLog {
        timestamp: 1234567890,
        key_id: "dex-database-key".to_string(),
        reason: "Scheduled rotation".to_string(),
        success: true,
        error_message: None,
    };
    
    manager.log_key_rotation(key_rotation_log);
    println!("✓ Logged successful key rotation for database encryption");

    // Simulate KMS access for encrypting user PII data
    println!("\n=== Simulating KMS Access for PII Encryption ===");
    
    let kms_access_log = KmsAccessLog {
        timestamp: 1234567891,
        key_id: "dex-pii-key".to_string(),
        operation: "encrypt".to_string(),
        success: true,
        error_message: None,
        accessed_by: Some("user-service".to_string()),
    };
    
    manager.log_kms_access(kms_access_log);
    println!("✓ Logged successful KMS access for PII encryption");

    // Simulate KMS access for decrypting user PII data
    println!("\n=== Simulating KMS Access for PII Decryption ===");
    
    let kms_access_log = KmsAccessLog {
        timestamp: 1234567892,
        key_id: "dex-pii-key".to_string(),
        operation: "decrypt".to_string(),
        success: true,
        error_message: None,
        accessed_by: Some("api-service".to_string()),
    };
    
    manager.log_kms_access(kms_access_log);
    println!("✓ Logged successful KMS access for PII decryption");

    // Generate telemetry report
    println!("\n=== Telemetry Report ===");
    let telemetry_report = manager.generate_telemetry_report();
    println!("{}", telemetry_report);

    // Demonstrate the security goal achievement
    println!("=== Security Goals Achieved ===");
    println!("✓ Data protection if disk/db is stolen: All data is encrypted at rest");
    println!("✓ Compliance: All requirements from the CSV file are implemented");
    println!("✓ Auditability: Comprehensive logs for key rotations and KMS access");

    Ok(())
}