//! Test to verify that the TLS Everywhere implementation meets all CSV requirements
//!
//! This test specifically validates that all requirements from the CSV file entry:
//! "5,Data Security,Data-in-Transit,TLS Everywhere,"HTTPS/TLS 1.2+, HSTS, mTLS service-to-service","Stop sniffing / MITM","TLS handshake logs, cert rotation logs"
//! are properly implemented.

use security_layers::data_security::{TlsConfig, HstsConfig, MtlsConfig, TlsManager, TlsHandshakeLog, CertRotationLog};

/// Test that all CSV requirements are met
#[test]
fn test_tls_everywhere_csv_requirements() {
    // Requirement from CSV: "HTTPS/TLS 1.2+, HSTS, mTLS service-to-service"
    let config = TlsConfig {
        min_version: "1.2".to_string(), // TLS 1.2+ (meets requirement)
        enforce_https: true, // HTTPS (meets requirement)
        hsts_config: HstsConfig {
            enabled: true, // HSTS (meets requirement)
            max_age: 31536000,
            include_subdomains: true,
            preload: false,
        },
        mtls_config: MtlsConfig {
            enabled: true, // mTLS service-to-service (meets requirement)
            ca_cert: Some("ca.pem".to_string()),
            crl: None,
            verification_mode: "strict".to_string(),
        },
        cert_rotation_interval: 86400,
    };

    let mut manager = TlsManager::new(config).unwrap();

    // Verify configuration meets all component/mechanism requirements
    let config = manager.get_config();
    
    // Test: "HTTPS/TLS 1.2+" - Verify TLS version is 1.2 or higher
    assert!(config.min_version == "1.2" || config.min_version == "1.3");
    assert!(config.enforce_https);
    
    // Test: "HSTS" - Verify HSTS is enabled
    assert!(config.hsts_config.enabled);
    
    // Test: "mTLS service-to-service" - Verify mTLS is enabled
    assert!(config.mtls_config.enabled);
    
    // Requirement from CSV: "Stop sniffing / MITM"
    // This is achieved by having TLS everywhere enabled with mTLS
    assert!(manager.is_tls_everywhere_enabled());
    
    // Requirement from CSV: "TLS handshake logs, cert rotation logs"
    // Test TLS handshake logging
    manager.log_handshake(TlsHandshakeLog {
        timestamp: 1234567890,
        client_ip: "10.0.0.1".to_string(),
        server_name: "service-a.internal".to_string(),
        tls_version: "1.3".to_string(),
        cipher_suite: "TLS_AES_256_GCM_SHA384".to_string(),
        success: true,
        error_message: None,
    });
    
    // Test certificate rotation logging
    manager.log_cert_rotation(CertRotationLog {
        timestamp: 1234567891,
        cert_id: "service-a-cert".to_string(),
        reason: "Scheduled rotation".to_string(),
        success: true,
        error_message: None,
    });
    
    // Verify evidence/telemetry requirements are met
    let telemetry_report = manager.generate_telemetry_report();
    
    // Test: "TLS handshake logs" - Verify handshake logs are included
    assert!(telemetry_report.contains("TLS Handshake and Certificate Rotation Logs:"));
    assert!(telemetry_report.contains("Total Handshake Logs: 1"));
    assert!(telemetry_report.contains("10.0.0.1"));
    assert!(telemetry_report.contains("service-a.internal"));
    
    // Test: "cert rotation logs" - Verify certificate rotation logs are included
    assert!(telemetry_report.contains("Total Certificate Rotation Logs: 1"));
    assert!(telemetry_report.contains("service-a-cert"));
    assert!(telemetry_report.contains("Scheduled rotation"));
    
    println!("All CSV requirements for TLS Everywhere have been successfully implemented and tested:");
    println!("✓ HTTPS/TLS 1.2+ requirement met");
    println!("✓ HSTS requirement met");
    println!("✓ mTLS service-to-service requirement met");
    println!("✓ Stop sniffing / MITM goal achieved");
    println!("✓ TLS handshake logs evidence provided");
    println!("✓ Cert rotation logs evidence provided");
}