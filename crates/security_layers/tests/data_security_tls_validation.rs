//! Data Security TLS Validation Tests
//!
//! This module contains tests that validate the Data-in-Transit TLS functionality
//! as defined in the web3_protection_layers.csv file for Layer 5.

use security_layers::data_security::{
    TlsConfig, HstsConfig, MtlsConfig, TlsManager, TlsHandshakeLog, CertRotationLog,
};

/// Test TLS configuration creation and validation
#[test]
fn test_tls_config_creation_and_validation() {
    // Test default configuration
    let config = TlsConfig::new();
    assert_eq!(config.min_version, "1.2");
    assert!(config.enforce_https);
    assert!(config.hsts_config.enabled);
    assert!(config.mtls_config.enabled);
    assert_eq!(config.cert_rotation_interval, 86400);
    
    // Test validation
    assert!(config.validate().is_ok());
    
    // Test invalid TLS version
    let mut invalid_config = config.clone();
    invalid_config.min_version = "1.0".to_string();
    assert!(invalid_config.validate().is_err());
    
    // Test invalid certificate rotation interval
    let mut invalid_config2 = config.clone();
    invalid_config2.cert_rotation_interval = 60; // Less than 1 hour
    assert!(invalid_config2.validate().is_err());
}

/// Test HSTS configuration
#[test]
fn test_hsts_config() {
    let hsts_config = HstsConfig {
        enabled: true,
        max_age: 31536000, // 1 year
        include_subdomains: true,
        preload: false,
    };
    
    assert!(hsts_config.enabled);
    assert_eq!(hsts_config.max_age, 31536000);
    assert!(hsts_config.include_subdomains);
    assert!(!hsts_config.preload);
}

/// Test mTLS configuration
#[test]
fn test_mtls_config() {
    let mtls_config = MtlsConfig {
        enabled: true,
        ca_cert: Some("ca-cert.pem".to_string()),
        crl: Some("crl.pem".to_string()),
        verification_mode: "strict".to_string(),
    };
    
    assert!(mtls_config.enabled);
    assert_eq!(mtls_config.ca_cert, Some("ca-cert.pem".to_string()));
    assert_eq!(mtls_config.crl, Some("crl.pem".to_string()));
    assert_eq!(mtls_config.verification_mode, "strict");
}

/// Test TLS manager creation and configuration
#[test]
fn test_tls_manager() {
    let config = TlsConfig::new();
    let manager = TlsManager::new(config).unwrap();
    
    // Test configuration access
    let config = manager.get_config();
    assert_eq!(config.min_version, "1.2");
    assert!(config.enforce_https);
    
    // Test TLS everywhere check
    assert!(manager.is_tls_everywhere_enabled());
}

/// Test TLS manager configuration updates
#[test]
fn test_tls_manager_config_updates() {
    let config = TlsConfig::new();
    let mut manager = TlsManager::new(config).unwrap();
    
    // Test updating configuration
    let new_config = TlsConfig {
        min_version: "1.3".to_string(),
        enforce_https: true,
        hsts_config: HstsConfig {
            enabled: true,
            max_age: 31536000,
            include_subdomains: true,
            preload: false,
        },
        mtls_config: MtlsConfig {
            enabled: true,
            ca_cert: Some("new-ca.pem".to_string()),
            crl: None,
            verification_mode: "strict".to_string(),
        },
        cert_rotation_interval: 43200, // 12 hours
    };
    
    assert!(manager.update_config(new_config.clone()).is_ok());
    
    let updated_config = manager.get_config();
    assert_eq!(updated_config.min_version, "1.3");
    assert_eq!(updated_config.cert_rotation_interval, 43200);
    assert_eq!(updated_config.mtls_config.ca_cert, Some("new-ca.pem".to_string()));
}

/// Test TLS handshake logging
#[test]
fn test_tls_handshake_logging() {
    let config = TlsConfig::new();
    let mut manager = TlsManager::new(config).unwrap();
    
    // Test logging handshakes
    let handshake_log = TlsHandshakeLog {
        timestamp: 1234567890,
        client_ip: "192.168.1.100".to_string(),
        server_name: "api.example.com".to_string(),
        tls_version: "1.3".to_string(),
        cipher_suite: "TLS_AES_256_GCM_SHA384".to_string(),
        success: true,
        error_message: None,
    };
    
    manager.log_handshake(handshake_log.clone());
    
    // Test retrieving logs
    let logs = manager.get_handshake_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].timestamp, 1234567890);
    assert_eq!(logs[0].client_ip, "192.168.1.100");
    assert_eq!(logs[0].server_name, "api.example.com");
    assert_eq!(logs[0].tls_version, "1.3");
    assert!(logs[0].success);
    assert_eq!(logs[0].error_message, None);
    
    // Test logging a failed handshake
    let failed_handshake_log = TlsHandshakeLog {
        timestamp: 1234567891,
        client_ip: "192.168.1.101".to_string(),
        server_name: "api.example.com".to_string(),
        tls_version: "1.2".to_string(),
        cipher_suite: "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384".to_string(),
        success: false,
        error_message: Some("Certificate verification failed".to_string()),
    };
    
    manager.log_handshake(failed_handshake_log.clone());
    
    // Test retrieving logs
    let logs = manager.get_handshake_logs();
    assert_eq!(logs.len(), 2);
    assert!(!logs[1].success);
    assert_eq!(logs[1].error_message, Some("Certificate verification failed".to_string()));
}

/// Test certificate rotation logging
#[test]
fn test_cert_rotation_logging() {
    let config = TlsConfig::new();
    let mut manager = TlsManager::new(config).unwrap();
    
    // Test logging certificate rotation
    let rotation_log = CertRotationLog {
        timestamp: 1234567890,
        cert_id: "api-cert-001".to_string(),
        reason: "Scheduled rotation".to_string(),
        success: true,
        error_message: None,
    };
    
    manager.log_cert_rotation(rotation_log.clone());
    
    // Test retrieving logs
    let logs = manager.get_cert_rotation_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0].timestamp, 1234567890);
    assert_eq!(logs[0].cert_id, "api-cert-001");
    assert_eq!(logs[0].reason, "Scheduled rotation");
    assert!(logs[0].success);
    assert_eq!(logs[0].error_message, None);
    
    // Test logging a failed rotation
    let failed_rotation_log = CertRotationLog {
        timestamp: 1234567891,
        cert_id: "api-cert-002".to_string(),
        reason: "Manual rotation".to_string(),
        success: false,
        error_message: Some("Key generation failed".to_string()),
    };
    
    manager.log_cert_rotation(failed_rotation_log.clone());
    
    // Test retrieving logs
    let logs = manager.get_cert_rotation_logs();
    assert_eq!(logs.len(), 2);
    assert!(!logs[1].success);
    assert_eq!(logs[1].error_message, Some("Key generation failed".to_string()));
}

/// Test telemetry report generation
#[test]
fn test_telemetry_report_generation() {
    let config = TlsConfig::new();
    let mut manager = TlsManager::new(config).unwrap();
    
    // Add some handshake logs
    manager.log_handshake(TlsHandshakeLog {
        timestamp: 1234567890,
        client_ip: "192.168.1.100".to_string(),
        server_name: "api.example.com".to_string(),
        tls_version: "1.3".to_string(),
        cipher_suite: "TLS_AES_256_GCM_SHA384".to_string(),
        success: true,
        error_message: None,
    });
    
    manager.log_handshake(TlsHandshakeLog {
        timestamp: 1234567891,
        client_ip: "192.168.1.101".to_string(),
        server_name: "api.example.com".to_string(),
        tls_version: "1.2".to_string(),
        cipher_suite: "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384".to_string(),
        success: false,
        error_message: Some("Certificate verification failed".to_string()),
    });
    
    // Add some certificate rotation logs
    manager.log_cert_rotation(CertRotationLog {
        timestamp: 1234567892,
        cert_id: "api-cert-001".to_string(),
        reason: "Scheduled rotation".to_string(),
        success: true,
        error_message: None,
    });
    
    manager.log_cert_rotation(CertRotationLog {
        timestamp: 1234567893,
        cert_id: "api-cert-002".to_string(),
        reason: "Manual rotation".to_string(),
        success: false,
        error_message: Some("Key generation failed".to_string()),
    });
    
    // Generate telemetry report
    let report = manager.generate_telemetry_report();
    
    // Verify report contains expected information
    assert!(report.contains("TLS Handshake and Certificate Rotation Logs:"));
    assert!(report.contains("Total Handshake Logs: 2"));
    assert!(report.contains("Successful Handshakes: 1"));
    assert!(report.contains("Failed Handshakes: 1"));
    assert!(report.contains("Total Certificate Rotation Logs: 2"));
    assert!(report.contains("Successful Rotations: 1"));
    assert!(report.contains("Failed Rotations: 1"));
    assert!(report.contains("192.168.1.100"));
    assert!(report.contains("192.168.1.101"));
    assert!(report.contains("api-cert-001"));
    assert!(report.contains("api-cert-002"));
    assert!(report.contains("Certificate verification failed"));
    assert!(report.contains("Key generation failed"));
}

/// Test the specific requirement from the CSV: "HTTPS/TLS 1.2+, HSTS, mTLS service-to-service"
#[test]
fn test_csv_requirement_tls_everywhere() {
    // Create a TLS configuration that meets the CSV requirements
    let config = TlsConfig {
        min_version: "1.2".to_string(), // TLS 1.2+
        enforce_https: true, // HTTPS
        hsts_config: HstsConfig {
            enabled: true, // HSTS
            max_age: 31536000,
            include_subdomains: true,
            preload: false,
        },
        mtls_config: MtlsConfig {
            enabled: true, // mTLS
            ca_cert: Some("ca.pem".to_string()),
            crl: None,
            verification_mode: "strict".to_string(),
        },
        cert_rotation_interval: 86400,
    };
    
    let manager = TlsManager::new(config).unwrap();
    
    // Verify the configuration meets the requirements
    let config = manager.get_config();
    assert!(config.min_version == "1.2" || config.min_version == "1.3"); // TLS 1.2+
    assert!(config.enforce_https); // HTTPS
    assert!(config.hsts_config.enabled); // HSTS
    assert!(config.mtls_config.enabled); // mTLS
    
    // Verify TLS everywhere is enabled
    assert!(manager.is_tls_everywhere_enabled());
}

/// Test the specific requirement from the CSV: "TLS handshake logs, cert rotation logs"
#[test]
fn test_csv_requirement_telemetry() {
    let config = TlsConfig::new();
    let mut manager = TlsManager::new(config).unwrap();
    
    // Add logs to meet the evidence/telemetry requirement
    manager.log_handshake(TlsHandshakeLog {
        timestamp: 1234567890,
        client_ip: "10.0.0.1".to_string(),
        server_name: "service-a.internal".to_string(),
        tls_version: "1.3".to_string(),
        cipher_suite: "TLS_AES_256_GCM_SHA384".to_string(),
        success: true,
        error_message: None,
    });
    
    manager.log_cert_rotation(CertRotationLog {
        timestamp: 1234567891,
        cert_id: "service-a-cert".to_string(),
        reason: "Scheduled rotation".to_string(),
        success: true,
        error_message: None,
    });
    
    // Generate the required evidence/telemetry
    let report = manager.generate_telemetry_report();
    
    // Verify the evidence/telemetry requirement is met
    assert!(report.contains("TLS Handshake and Certificate Rotation Logs:"));
    assert!(report.contains("Total Handshake Logs: 1"));
    assert!(report.contains("Total Certificate Rotation Logs: 1"));
    assert!(report.contains("10.0.0.1"));
    assert!(report.contains("service-a.internal"));
    assert!(report.contains("service-a-cert"));
    assert!(report.contains("Scheduled rotation"));
}

/// Integration test showing how the TLS system works with the overall security layers
#[test]
fn test_tls_integration() {
    // Create a TLS configuration for a DEX application
    let config = TlsConfig {
        min_version: "1.3".to_string(), // Use the latest TLS version
        enforce_https: true,
        hsts_config: HstsConfig {
            enabled: true,
            max_age: 31536000, // 1 year
            include_subdomains: true,
            preload: false,
        },
        mtls_config: MtlsConfig {
            enabled: true,
            ca_cert: Some("dex-ca.pem".to_string()),
            crl: Some("dex-crl.pem".to_string()),
            verification_mode: "strict".to_string(),
        },
        cert_rotation_interval: 86400, // 24 hours
    };
    
    let mut manager = TlsManager::new(config).unwrap();
    
    // Simulate service-to-service communication with mTLS
    manager.log_handshake(TlsHandshakeLog {
        timestamp: 1234567890,
        client_ip: "10.0.1.10".to_string(), // API service
        server_name: "indexer.dex.internal".to_string(), // Indexer service
        tls_version: "1.3".to_string(),
        cipher_suite: "TLS_AES_256_GCM_SHA384".to_string(),
        success: true,
        error_message: None,
    });
    
    manager.log_handshake(TlsHandshakeLog {
        timestamp: 1234567891,
        client_ip: "10.0.2.20".to_string(), // Keeper service
        server_name: "api.dex.internal".to_string(), // API service
        tls_version: "1.3".to_string(),
        cipher_suite: "TLS_AES_256_GCM_SHA384".to_string(),
        success: true,
        error_message: None,
    });
    
    // Simulate certificate rotation
    manager.log_cert_rotation(CertRotationLog {
        timestamp: 1234567892,
        cert_id: "api-service-cert".to_string(),
        reason: "Scheduled rotation".to_string(),
        success: true,
        error_message: None,
    });
    
    // Verify the TLS configuration meets security requirements
    assert!(manager.is_tls_everywhere_enabled());
    
    // Generate the required evidence/telemetry
    let telemetry_report = manager.generate_telemetry_report();
    println!("Telemetry Report:\n{}", telemetry_report);
    
    // Verify that we have the required evidence
    assert!(telemetry_report.contains("TLS Handshake and Certificate Rotation Logs:"));
    assert!(telemetry_report.contains("Total Handshake Logs: 2"));
    assert!(telemetry_report.contains("Total Certificate Rotation Logs: 1"));
    assert!(telemetry_report.contains("indexer.dex.internal"));
    assert!(telemetry_report.contains("api.dex.internal"));
    assert!(telemetry_report.contains("api-service-cert"));
    
    // Verify the goal: "Stop sniffing / MITM"
    // By having TLS everywhere enabled with mTLS, we prevent sniffing and MITM attacks
    assert!(manager.is_tls_everywhere_enabled());
}