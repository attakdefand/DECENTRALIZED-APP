//! Data Minimization Usage Example
//!
//! This example demonstrates how to use the Data Minimization implementation
//! for a decentralized exchange application.

use security_layers::data_security::{
    DataMinimizationConfig, DataMinimizationManager,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Data Minimization Usage Example");
    println!("==============================");
    
    // Create a Data Minimization configuration for a DEX application
    let config = DataMinimizationConfig {
        // Store only required attributes to minimize data exposure
        store_only_required: true,
        
        // Redact PII in logs to prevent sensitive information exposure
        redact_pii_in_logs: true,
        
        // Tokenize high-risk values to protect them even if data is breached
        tokenize_high_risk_values: true,
        
        // Define PII patterns that should be redacted from logs
        pii_patterns: vec![
            "email".to_string(),
            "phone".to_string(),
            "wallet_address".to_string(),
            "ssn".to_string(),
        ],
        
        // Define high-risk patterns that should be tokenized
        high_risk_patterns: vec![
            "password".to_string(),
            "private_key".to_string(),
            "secret".to_string(),
            "api_key".to_string(),
        ],
    };
    
    // Create a Data Minimization manager with the configuration
    let mut manager = DataMinimizationManager::new(config);
    
    println!("✓ Data Minimization manager created with configuration");
    
    // Example 1: Redacting PII from logs
    println!("\n1. Redacting PII from logs:");
    
    let log_entries = vec![
        "User login: email=user@example.com, wallet=0x742d35Cc6634C0532925a3b8D91D0b6bCf8fA1E2",
        "Transaction processed for user with phone=123-456-7890",
        "SSN verification completed: ssn=123-45-6789",
        "Normal system log entry without PII",
    ];
    
    for (i, log_entry) in log_entries.iter().enumerate() {
        let redacted_log = manager.redact_pii(log_entry);
        println!("   Original log {}: {}", i + 1, log_entry);
        println!("   Redacted log {}:  {}", i + 1, redacted_log);
    }
    
    // Example 2: Tokenizing high-risk values
    println!("\n2. Tokenizing high-risk values:");
    
    let high_risk_values = vec![
        "super_secret_password_123",
        "private_key_abc123def456",
        "api_key_xyz789",
    ];
    
    for (i, value) in high_risk_values.iter().enumerate() {
        let tokenized_value = manager.tokenize_value(value);
        println!("   Original value {}: {}", i + 1, value);
        println!("   Tokenized value {}: {}", i + 1, tokenized_value);
    }
    
    // Example 3: Scanning logs for PII to generate evidence/telemetry
    println!("\n3. Scanning logs for PII (generating evidence/telemetry):");
    
    let logs_to_scan = vec![
        "User registered with email=user@example.com and phone=123-456-7890".to_string(),
        "Private key accessed: private_key_abc123def456".to_string(),
        "API key used: api_key_xyz789".to_string(),
        "Normal system log entry".to_string(),
        "User wallet address: 0x742d35Cc6634C0532925a3b8D91D0b6bCf8fA1E2".to_string(),
    ];
    
    let scanner_report = manager.scan_logs_for_pii(&logs_to_scan);
    
    println!("   Scanner report generated:");
    println!("   - Timestamp: {}", scanner_report.timestamp);
    println!("   - PII instances found: {}", scanner_report.pii_instances_found);
    println!("   - High-risk values found: {}", scanner_report.high_risk_values_found);
    println!("   - Success: {}", scanner_report.success);
    
    // Store the scanner report for evidence/telemetry
    manager.scan_logs_for_pii(&logs_to_scan);
    
    // Example 4: Generating telemetry report (required evidence)
    println!("\n4. Generating telemetry report (required evidence):");
    
    let telemetry_report = manager.generate_telemetry_report();
    println!("   Telemetry Report:");
    println!("{}", telemetry_report);
    
    // Example 5: Verifying that Data Minimization is properly configured
    println!("5. Verifying Data Minimization configuration:");
    
    let is_enabled = manager.is_data_minimization_enabled();
    println!("   Data Minimization enabled: {}", is_enabled);
    
    if is_enabled {
        println!("   ✓ All Data Minimization mechanisms are enabled");
        println!("   ✓ Goal achieved: Shrink breach impact");
        println!("   ✓ Evidence/Telemetry provided: PII in logs scanner report");
    } else {
        println!("   ✗ Data Minimization is not fully enabled");
    }
    
    // Example 6: Updating configuration
    println!("\n6. Updating configuration:");
    
    let updated_config = DataMinimizationConfig {
        store_only_required: true,
        redact_pii_in_logs: true,
        tokenize_high_risk_values: true,
        pii_patterns: vec![
            "email".to_string(),
            "phone".to_string(),
            "wallet".to_string(), // Updated pattern
        ],
        high_risk_patterns: vec![
            "password".to_string(),
            "private_key".to_string(),
            "secret".to_string(),
            "token".to_string(), // Added new pattern
        ],
    };
    
    manager.update_config(updated_config);
    println!("   Configuration updated successfully");
    
    // Verify the updated configuration
    let config = manager.get_config();
    println!("   Updated PII patterns: {:?}", config.pii_patterns);
    println!("   Updated high-risk patterns: {:?}", config.high_risk_patterns);
    
    println!("\nData Minimization Usage Example completed successfully!");
    
    Ok(())
}