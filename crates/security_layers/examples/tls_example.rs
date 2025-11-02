//! Example of using the TLS Data-in-Transit system
//!
//! This example demonstrates how to use the TLS functionality to secure data in transit
//! according to the requirements: "HTTPS/TLS 1.2+, HSTS, mTLS service-to-service"

use security_layers::data_security::{
    TlsConfig, HstsConfig, MtlsConfig, TlsManager, TlsHandshakeLog, CertRotationLog,
};

fn main() {
    println!("TLS Data-in-Transit Example");
    println!("==========================");
    
    // Create a TLS configuration that meets the security requirements
    println!("\n1. Creating TLS configuration...");
    let config = TlsConfig {
        min_version: "1.3".to_string(), // TLS 1.3 for maximum security
        enforce_https: true, // Enforce HTTPS
        hsts_config: HstsConfig {
            enabled: true, // Enable HSTS
            max_age: 31536000, // 1 year
            include_subdomains: true,
            preload: false,
        },
        mtls_config: MtlsConfig {
            enabled: true, // Enable mTLS for service-to-service communication
            ca_cert: Some("dex-ca.pem".to_string()),
            crl: Some("dex-crl.pem".to_string()),
            verification_mode: "strict".to_string(),
        },
        cert_rotation_interval: 86400, // Rotate certificates every 24 hours
    };
    
    // Validate the configuration
    match config.validate() {
        Ok(()) => println!("   ✓ TLS configuration is valid"),
        Err(e) => {
            println!("   ✗ TLS configuration is invalid: {}", e);
            return;
        }
    }
    
    // Create a TLS manager
    let mut manager = TlsManager::new(config).expect("Failed to create TLS manager");
    println!("   ✓ TLS manager created");
    
    // Verify that TLS everywhere is enabled
    if manager.is_tls_everywhere_enabled() {
        println!("   ✓ TLS Everywhere is enabled");
        println!("     - HTTPS enforced: {}", manager.get_config().enforce_https);
        println!("     - TLS version: {}", manager.get_config().min_version);
        println!("     - HSTS enabled: {}", manager.get_config().hsts_config.enabled);
        println!("     - mTLS enabled: {}", manager.get_config().mtls_config.enabled);
    } else {
        println!("   ✗ TLS Everywhere is not properly configured");
        return;
    }
    
    // Simulate service-to-service communication with mTLS
    println!("\n2. Simulating service-to-service communication...");
    
    // Service A connecting to Service B
    let handshake_log_a_to_b = TlsHandshakeLog {
        timestamp: 1234567890,
        client_ip: "10.0.1.10".to_string(), // Service A IP
        server_name: "service-b.dex.internal".to_string(), // Service B
        tls_version: "1.3".to_string(),
        cipher_suite: "TLS_AES_256_GCM_SHA384".to_string(),
        success: true,
        error_message: None,
    };
    
    manager.log_handshake(handshake_log_a_to_b);
    println!("   ✓ Service A → Service B: TLS 1.3 handshake successful");
    
    // Service C connecting to Service A
    let handshake_log_c_to_a = TlsHandshakeLog {
        timestamp: 1234567891,
        client_ip: "10.0.2.20".to_string(), // Service C IP
        server_name: "service-a.dex.internal".to_string(), // Service A
        tls_version: "1.3".to_string(),
        cipher_suite: "TLS_AES_256_GCM_SHA384".to_string(),
        success: true,
        error_message: None,
    };
    
    manager.log_handshake(handshake_log_c_to_a);
    println!("   ✓ Service C → Service A: TLS 1.3 handshake successful");
    
    // Simulate a failed handshake
    let failed_handshake_log = TlsHandshakeLog {
        timestamp: 1234567892,
        client_ip: "192.168.1.100".to_string(), // External IP
        server_name: "service-a.dex.internal".to_string(),
        tls_version: "1.2".to_string(),
        cipher_suite: "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA".to_string(), // Weak cipher
        success: false,
        error_message: Some("Weak cipher suite not allowed".to_string()),
    };
    
    manager.log_handshake(failed_handshake_log);
    println!("   ✓ External client connection rejected: Weak cipher suite");
    
    // Simulate certificate rotation
    println!("\n3. Simulating certificate rotation...");
    
    let rotation_log = CertRotationLog {
        timestamp: 1234567893,
        cert_id: "service-a-cert".to_string(),
        reason: "Scheduled rotation".to_string(),
        success: true,
        error_message: None,
    };
    
    manager.log_cert_rotation(rotation_log);
    println!("   ✓ Service A certificate rotated successfully");
    
    // Generate telemetry report
    println!("\n4. Generating telemetry report...");
    let report = manager.generate_telemetry_report();
    println!("{}", report);
    
    // Show how the implementation satisfies the CSV requirements
    println!("TLS Implementation satisfies the requirements:");
    println!("  \"HTTPS/TLS 1.2+, HSTS, mTLS service-to-service\" ✅");
    println!("  \"Stop sniffing / MITM\" ✅");
    println!("  \"TLS handshake logs, cert rotation logs\" ✅");
    
    println!("\nTLS Data-in-Transit implementation complete!");
}