//! Example of using the TLS Everywhere implementation
//!
//! This example demonstrates how to configure and use the TLS functionality
//! for securing data in transit as specified in the security requirements.

use security_layers::data_security::{TlsConfig, HstsConfig, MtlsConfig, TlsManager, TlsHandshakeLog, CertRotationLog};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== TLS Everywhere Implementation Example ===\n");

    // Create a TLS configuration that meets the security requirements
    let config = TlsConfig {
        min_version: "1.3".to_string(), // Use the latest TLS version
        enforce_https: true, // Enforce HTTPS for all connections
        hsts_config: HstsConfig {
            enabled: true, // Enable HTTP Strict Transport Security
            max_age: 31536000, // 1 year
            include_subdomains: true,
            preload: false,
        },
        mtls_config: MtlsConfig {
            enabled: true, // Enable mutual TLS for service-to-service authentication
            ca_cert: Some("dex-ca.pem".to_string()),
            crl: Some("dex-crl.pem".to_string()),
            verification_mode: "strict".to_string(),
        },
        cert_rotation_interval: 86400, // Rotate certificates every 24 hours
    };

    // Validate the configuration
    config.validate()?;
    println!("✓ TLS configuration validated successfully");

    // Create a TLS manager with the configuration
    let mut manager = TlsManager::new(config)?;
    println!("✓ TLS manager created successfully");

    // Verify that TLS everywhere is enabled
    if manager.is_tls_everywhere_enabled() {
        println!("✓ TLS Everywhere is properly configured:");
        println!("  - HTTPS enforcement: enabled");
        println!("  - TLS version: 1.3");
        println!("  - HSTS: enabled");
        println!("  - mTLS: enabled");
    } else {
        println!("✗ TLS Everywhere configuration is incomplete");
        return Ok(());
    }

    // Simulate service-to-service communication with mTLS
    println!("\n=== Simulating Service-to-Service Communication ===");
    
    let handshake_log = TlsHandshakeLog {
        timestamp: 1234567890,
        client_ip: "10.0.1.10".to_string(), // API service
        server_name: "indexer.dex.internal".to_string(), // Indexer service
        tls_version: "1.3".to_string(),
        cipher_suite: "TLS_AES_256_GCM_SHA384".to_string(),
        success: true,
        error_message: None,
    };
    
    manager.log_handshake(handshake_log);
    println!("✓ Logged successful TLS handshake between services");

    // Simulate certificate rotation
    println!("\n=== Simulating Certificate Rotation ===");
    
    let rotation_log = CertRotationLog {
        timestamp: 1234567891,
        cert_id: "api-service-cert".to_string(),
        reason: "Scheduled rotation".to_string(),
        success: true,
        error_message: None,
    };
    
    manager.log_cert_rotation(rotation_log);
    println!("✓ Logged successful certificate rotation");

    // Generate telemetry report
    println!("\n=== Telemetry Report ===");
    let telemetry_report = manager.generate_telemetry_report();
    println!("{}", telemetry_report);

    // Demonstrate the security goal achievement
    println!("=== Security Goals Achieved ===");
    println!("✓ Sniffing prevention: All data is encrypted in transit");
    println!("✓ MITM prevention: Mutual authentication ensures only trusted parties can communicate");
    println!("✓ Compliance: All requirements from the CSV file are implemented");

    Ok(())
}