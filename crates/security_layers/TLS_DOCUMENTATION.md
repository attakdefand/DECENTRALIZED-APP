# Data-in-Transit with TLS Everywhere

## Overview

This document describes the implementation of Data-in-Transit protection with TLS Everywhere as part of Layer 5: Data Security in the Web3 Protection Layers framework. This implementation satisfies the requirements specified in the `web3_protection_layers.csv` file:

- **Component / Mechanism**: "HTTPS/TLS 1.2+, HSTS, mTLS service-to-service"
- **Goal**: "Stop sniffing / MITM"
- **Evidence / Telemetry**: "TLS handshake logs, cert rotation logs"

## Implementation Details

### TLS Configuration

The implementation provides comprehensive TLS configuration capabilities:

1. **Minimum TLS Version** - Supports TLS 1.2 and TLS 1.3
2. **HTTPS Enforcement** - Ensures all communication uses HTTPS
3. **HSTS (HTTP Strict Transport Security)** - Prevents protocol downgrade attacks
4. **mTLS (Mutual TLS)** - Secures service-to-service communication
5. **Certificate Rotation** - Automated certificate management

### Core Components

#### TlsConfig Struct

The `TlsConfig` struct represents the TLS configuration:

```rust
pub struct TlsConfig {
    /// Minimum TLS version (e.g., "1.2", "1.3")
    pub min_version: String,
    /// Whether to enforce HTTPS
    pub enforce_https: bool,
    /// HSTS configuration
    pub hsts_config: HstsConfig,
    /// mTLS configuration for service-to-service communication
    pub mtls_config: MtlsConfig,
    /// Certificate rotation interval in seconds
    pub cert_rotation_interval: u64,
}
```

#### HstsConfig Struct

The `HstsConfig` struct represents HTTP Strict Transport Security configuration:

```rust
pub struct HstsConfig {
    /// Whether HSTS is enabled
    pub enabled: bool,
    /// Max age in seconds
    pub max_age: u64,
    /// Whether to include subdomains
    pub include_subdomains: bool,
    /// Whether to set the preload flag
    pub preload: bool,
}
```

#### MtlsConfig Struct

The `MtlsConfig` struct represents Mutual TLS configuration:

```rust
pub struct MtlsConfig {
    /// Whether mTLS is enabled
    pub enabled: bool,
    /// Certificate authority for client certificates
    pub ca_cert: Option<String>,
    /// Certificate revocation list
    pub crl: Option<String>,
    /// Certificate verification mode
    pub verification_mode: String, // "strict", "relaxed", etc.
}
```

#### TlsManager

The `TlsManager` provides a high-level interface for TLS operations and telemetry:

```rust
pub struct TlsManager {
    /// TLS configuration
    config: TlsConfig,
    /// TLS handshake logs
    handshake_logs: Vec<TlsHandshakeLog>,
    /// Certificate rotation logs
    cert_rotation_logs: Vec<CertRotationLog>,
}
```

## Usage Examples

### Basic Usage

```rust
use security_layers::data_security::{
    TlsConfig, HstsConfig, MtlsConfig, TlsManager,
};

// Create a TLS configuration that meets security requirements
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
        ca_cert: Some("ca.pem".to_string()),
        crl: None,
        verification_mode: "strict".to_string(),
    },
    cert_rotation_interval: 86400, // Rotate certificates every 24 hours
};

// Create a TLS manager
let mut manager = TlsManager::new(config).expect("Failed to create TLS manager");

// Verify TLS everywhere is enabled
if manager.is_tls_everywhere_enabled() {
    println!("TLS Everywhere is properly configured");
}
```

### Logging TLS Operations

```rust
// Log a successful TLS handshake
manager.log_handshake(TlsHandshakeLog {
    timestamp: 1234567890,
    client_ip: "10.0.1.10".to_string(),
    server_name: "service-b.internal".to_string(),
    tls_version: "1.3".to_string(),
    cipher_suite: "TLS_AES_256_GCM_SHA384".to_string(),
    success: true,
    error_message: None,
});

// Log a certificate rotation
manager.log_cert_rotation(CertRotationLog {
    timestamp: 1234567891,
    cert_id: "service-b-cert".to_string(),
    reason: "Scheduled rotation".to_string(),
    success: true,
    error_message: None,
});

// Generate telemetry report
let report = manager.generate_telemetry_report();
println!("{}", report);
```

## Integration with Security Layers

This implementation integrates with the broader security layers framework and satisfies the validation requirements in `security_layers_validation.rs`:

```rust
// Test that validates the data-in-transit layer from the CSV file
fn test_layer_5_data_in_transit(layers: &[SecurityLayer]) {
    // Find the Data-in-Transit layer
    let data_in_transit_layer = layers.iter().find(|l| 
        l.layer_number == 5 && 
        l.main_type == "Data-in-Transit" && 
        l.sub_type == "TLS Everywhere"
    ).expect("Data-in-Transit layer should exist");
    
    // Verify the layer properties match the CSV
    assert_eq!(data_in_transit_layer.component_mechanism, "HTTPS/TLS 1.2+, HSTS, mTLS service-to-service");
    assert_eq!(data_in_transit_layer.goal, "Stop sniffing / MITM");
    assert_eq!(data_in_transit_layer.evidence_telemetry, "TLS handshake logs, cert rotation logs");
    
    // Test the actual implementation...
}
```

## Testing

The implementation includes comprehensive unit tests that validate all functionality:

1. `test_tls_config_creation_and_validation` - Tests TLS configuration creation and validation
2. `test_hsts_config` - Tests HSTS configuration
3. `test_mtls_config` - Tests mTLS configuration
4. `test_tls_manager` - Tests TLS manager creation and configuration
5. `test_tls_handshake_logging` - Tests TLS handshake logging
6. `test_cert_rotation_logging` - Tests certificate rotation logging
7. `test_telemetry_report_generation` - Tests telemetry report generation
8. `test_csv_requirement_tls_everywhere` - Tests the specific CSV requirements
9. `test_csv_requirement_telemetry` - Tests the evidence/telemetry requirements
10. `test_tls_integration` - Integration tests

All tests can be run with:

```bash
cargo test -p security_layers --test data_security_tls_validation
```

## Evidence and Telemetry

The implementation provides the required "TLS handshake logs, cert rotation logs" evidence through the `generate_telemetry_report()` method, which produces output like:

```
TLS Handshake and Certificate Rotation Logs:
Total Handshake Logs: 2
Successful Handshakes: 2
Failed Handshakes: 0
Total Certificate Rotation Logs: 1
Successful Rotations: 1
Failed Rotations: 0

Recent TLS Handshake Logs:
  1234567890 - 10.0.1.10:service-b.dex.internal - TLS 1.3 - SUCCESS - N/A
  1234567891 - 10.0.2.20:service-a.dex.internal - TLS 1.3 - SUCCESS - N/A

Recent Certificate Rotation Logs:
  1234567893 - service-a-cert - Scheduled rotation - SUCCESS - N/A
```

This telemetry clearly shows TLS operations, enabling security teams to monitor and verify that data in transit is properly protected.

## Security Benefits

1. **Encryption in Transit** - All data is encrypted during transmission
2. **Prevention of Eavesdropping** - TLS prevents network sniffing attacks
3. **Man-in-the-Middle Protection** - Certificate validation prevents MITM attacks
4. **Service Authentication** - mTLS ensures only authorized services can communicate
5. **Audit and Compliance** - Comprehensive logging for regulatory compliance
6. **Certificate Management** - Automated rotation reduces security risks

## Future Enhancements

Potential future enhancements could include:

1. **Advanced Cipher Suite Management** - Dynamic cipher suite selection based on security requirements
2. **OCSP Stapling** - Online Certificate Status Protocol support
3. **Certificate Pinning** - Additional certificate validation mechanisms
4. **Integration with KMS** - Key management service integration for certificate storage
5. **Export Functionality** - Compliance reporting and audit trail generation