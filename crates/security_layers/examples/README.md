# Data Security Implementation Examples

This directory contains examples demonstrating how to use the Data Security implementations for securing data in transit and at rest.

## Running the Examples

To run the TLS usage example:

```bash
cd d:\DECENTRALIZED-APP\crates\security_layers
cargo run --example tls_usage_example
```

To run the Data-at-Rest usage example:

```bash
cd d:\DECENTRALIZED-APP\crates\security_layers
cargo run --example data_at_rest_usage_example
```

To run the Data Minimization usage example:

```bash
cd d:\DECENTRALIZED-APP\crates\security_layers
cargo run --example data_minimization_usage_example
```

To run the Backup & Restore usage example:

```bash
cd d:\DECENTRALIZED-APP\crates\security_layers
cargo run --example backup_restore_usage_example
```

## Example Overview

### TLS Usage Example ([tls_usage_example.rs](file:///d:/DECENTRALIZED-APP/crates/security_layers/examples/tls_usage_example.rs))
Demonstrates:
1. **Configuration**: How to set up TLS with HTTPS enforcement, HSTS, and mTLS
2. **Validation**: How to validate TLS configuration meets security requirements
3. **Usage**: How to use the TlsManager to handle TLS operations
4. **Logging**: How to log TLS handshakes and certificate rotations
5. **Telemetry**: How to generate reports for audit and monitoring

### Data-at-Rest Usage Example ([data_at_rest_usage_example.rs](file:///d:/DECENTRALIZED-APP/crates/security_layers/examples/data_at_rest_usage_example.rs))
Demonstrates:
1. **Configuration**: How to set up Data-at-Rest encryption with KMS integration
2. **Validation**: How to validate Data-at-Rest configuration meets security requirements
3. **Usage**: How to use the DataAtRestManager to handle encryption operations
4. **Logging**: How to log key rotations and KMS accesses
5. **Telemetry**: How to generate reports for audit and monitoring

### Data Minimization Usage Example ([data_minimization_usage_example.rs](file:///d:/DECENTRALIZED-APP/crates/security_layers/examples/data_minimization_usage_example.rs))
Demonstrates:
1. **Configuration**: How to set up Data Minimization with PII redaction and tokenization
2. **Validation**: How to validate Data Minimization configuration meets security requirements
3. **Usage**: How to use the DataMinimizationManager to handle data minimization operations
4. **Logging**: How to redact PII from logs and tokenize high-risk values
5. **Telemetry**: How to scan logs for PII and generate reports for audit and monitoring

### Backup & Restore Usage Example ([backup_restore_usage_example.rs](file:///d:/DECENTRALIZED-APP/crates/security_layers/examples/backup_restore_usage_example.rs))
Demonstrates:
1. **Configuration**: How to set up Backup & Restore with periodic snapshots and offline copies
2. **Validation**: How to validate Backup & Restore configuration meets security requirements
3. **Usage**: How to use the BackupRestoreManager to handle backup and restore operations
4. **Logging**: How to create snapshots, offline copies, and perform restore drills
5. **Telemetry**: How to generate reports with RPO/RTO metrics for audit and monitoring

## Key Features Demonstrated

### TLS (Data-in-Transit)
- TLS 1.2+ enforcement
- HTTPS mandatory for all connections
- HTTP Strict Transport Security (HSTS)
- Mutual TLS (mTLS) for service-to-service authentication
- Certificate rotation management
- Comprehensive logging for audit trails
- Telemetry reporting

### Data-at-Rest Encryption
- KMS-managed encryption for disk/volume/database
- Envelope encryption for sensitive fields like PII
- Key rotation management
- KMS access control and monitoring
- Comprehensive logging for audit trails
- Telemetry reporting

### Data Minimization
- Store only required attributes to minimize data exposure
- Redact PII in logs to prevent sensitive information exposure
- Tokenize high-risk values to protect them even if data is breached
- Scan logs for PII to generate evidence/telemetry
- Comprehensive reporting for audit trails

### Backup & Restore
- Periodic encrypted snapshots to ensure recent backups exist
- Offline copies to protect against ransomware
- Tested restore drills to ensure actual restore capability
- Comprehensive metrics collection (RPO/RTO)
- Telemetry reporting for compliance

## Security Goals Achieved

### TLS (Data-in-Transit)
- **Prevention of Sniffing**: All data in transit is encrypted
- **Prevention of MITM Attacks**: Mutual authentication ensures only trusted parties communicate
- **Compliance**: Implementation meets all requirements from the security specification

### Data-at-Rest Encryption
- **Data Protection**: All data at rest is encrypted using strong encryption algorithms
- **Key Management**: Secure key management through KMS integration
- **Compliance**: Implementation meets all requirements from the security specification

### Data Minimization
- **Breach Impact Reduction**: Minimize the amount of data stored and exposed
- **PII Protection**: Prevent sensitive information from being exposed in logs
- **High-Risk Value Protection**: Protect critical values through tokenization
- **Compliance**: Implementation meets all requirements from the security specification

### Backup & Restore
- **Ransomware Protection**: Offline copies protect against ransomware attacks
- **Data Loss Prevention**: Periodic snapshots ensure recent backups exist
- **Recovery Assurance**: Tested restore drills ensure actual restore capability
- **Compliance**: Implementation meets all requirements from the security specification