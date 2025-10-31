# Key Rotation Runbook

This document provides procedures for rotating cryptographic keys used in the DECENTRALIZED-APP infrastructure.

## Overview

Key rotation is a critical security practice that involves periodically replacing cryptographic keys to reduce the impact of a potential key compromise. This runbook outlines the procedures for rotating various types of keys used in the system.

## Key Rotation Principles

### Rotation Frequency
- **High-value keys**: Every 90 days
- **Standard keys**: Every 180 days
- **Long-term keys**: Every 365 days
- **Emergency rotation**: As needed for security incidents

### Rotation Methods
- **Automatic rotation**: For keys managed by cloud services
- **Manual rotation**: For keys requiring manual intervention
- **Re-encryption rotation**: For data encryption keys requiring data re-encryption

### Key States
- **Active**: Currently in use for encryption/decryption
- **Standby**: Available for decryption of old data
- **Deprecated**: No longer in use, pending deletion
- **Destroyed**: Securely deleted

## Key Types and Rotation Procedures

### 1. Infrastructure Keys

#### TLS Certificates
**Rotation Frequency**: Every 90 days or as certificates approach expiration

**Procedure**:
1. Generate new certificate signing request (CSR)
2. Submit CSR to certificate authority
3. Install new certificate on servers/load balancers
4. Validate certificate installation
5. Update certificate references in applications
6. Monitor for certificate-related errors
7. Revoke old certificate after grace period

**Validation**:
- Certificate expiration monitoring
- SSL/TLS handshake testing
- Application functionality testing

#### SSH Keys
**Rotation Frequency**: Every 180 days for service accounts, annually for user accounts

**Procedure**:
1. Generate new SSH key pair
2. Distribute public key to authorized systems
3. Update SSH configuration files
4. Test SSH connectivity with new keys
5. Remove old public keys after validation
6. Update key inventory records

**Validation**:
- SSH connectivity testing
- Key fingerprint verification
- Access log monitoring

### 2. Application Keys

#### API Keys
**Rotation Frequency**: Every 180 days or when compromised

**Procedure**:
1. Generate new API key
2. Update application configuration with new key
3. Deploy updated configuration to all environments
4. Validate API functionality with new key
5. Disable old API key after grace period
6. Remove old key from configuration

**Validation**:
- API call success rate monitoring
- Error rate analysis
- Application functionality testing

#### Database Credentials
**Rotation Frequency**: Every 90 days

**Procedure**:
1. Generate new database user and credentials
2. Grant same permissions as old user
3. Update application configuration with new credentials
4. Deploy updated configuration to all environments
5. Validate database connectivity and functionality
6. Revoke old database user after grace period
7. Drop old database user

**Validation**:
- Database connection testing
- Query execution validation
- Application functionality testing

### 3. Encryption Keys

#### Data Encryption Keys (DEKs)
**Rotation Frequency**: Every 180 days

**Procedure**:
1. Generate new data encryption key
2. Re-encrypt existing data with new key
3. Update key management system with new key
4. Validate data decryption with new key
5. Mark old key as deprecated
6. Schedule old key for destruction

**Validation**:
- Data decryption testing
- Integrity verification
- Performance monitoring

#### Key Encryption Keys (KEKs)
**Rotation Frequency**: Every 365 days

**Procedure**:
1. Generate new key encryption key
2. Re-encrypt data encryption keys with new KEK
3. Update key management system
4. Validate decryption of data with new KEK chain
5. Mark old KEK as deprecated
6. Schedule old KEK for destruction

**Validation**:
- Key decryption testing
- Data accessibility verification
- Key management system audit

### 4. Service Account Keys

#### Cloud Service Account Keys
**Rotation Frequency**: Every 90 days

**Procedure**:
1. Create new service account key
2. Update applications with new key
3. Deploy updated configuration
4. Validate service functionality
5. Disable old service account key
6. Delete old key after grace period

**Validation**:
- Service availability monitoring
- Error rate analysis
- Log analysis for authentication failures

#### Application Service Account Keys
**Rotation Frequency**: Every 180 days

**Procedure**:
1. Generate new service account credentials
2. Update service configuration
3. Deploy updated configuration
4. Validate service functionality
5. Disable old service account
6. Remove old credentials

**Validation**:
- Service health checks
- Functionality testing
- Authentication log monitoring

## Key Management System

### Key Storage
- **Hardware Security Modules (HSMs)**: For high-value keys
- **Cloud KMS**: For cloud-managed keys
- **Encrypted Key Files**: For application-managed keys

### Key Lifecycle
1. **Generation**: Secure key generation with proper entropy
2. **Activation**: Key becomes active for use
3. **Usage**: Key used for cryptographic operations
4. **Rotation**: Key replaced with new version
5. **Deactivation**: Key no longer used for new operations
6. **Destruction**: Secure deletion of key material

### Key Inventory
- **Key ID**: Unique identifier for each key
- **Key Type**: Algorithm and key size
- **Creation Date**: When key was generated
- **Activation Date**: When key became active
- **Rotation Date**: When key was rotated
- **Destruction Date**: When key was destroyed
- **Owner**: Team responsible for key
- **Purpose**: Intended use of key

## Rotation Automation

### Automated Rotation
- **Cloud Services**: Use native key rotation features
- **Kubernetes Secrets**: Use external secrets operators
- **CI/CD Pipelines**: Integrate key rotation into deployment pipelines

### Monitoring and Alerting
- **Key Expiration**: Alerts for upcoming key expirations
- **Rotation Failures**: Alerts for failed rotation attempts
- **Unauthorized Access**: Alerts for suspicious key access

### Backup and Recovery
- **Key Backups**: Secure backup of key material
- **Disaster Recovery**: Procedures for key recovery
- **Business Continuity**: Key availability during outages

## Emergency Procedures

### Compromised Keys
1. **Immediate Isolation**: Disable compromised key
2. **Impact Assessment**: Determine scope of compromise
3. **Emergency Rotation**: Generate and deploy new keys
4. **System Audit**: Review logs for unauthorized access
5. **Incident Reporting**: Document security incident
6. **Communication**: Notify affected parties

### Failed Rotations
1. **Rollback**: Revert to previous key version
2. **Root Cause Analysis**: Determine cause of failure
3. **Fix Implementation**: Address underlying issue
4. **Retry Rotation**: Attempt rotation again
5. **Monitoring**: Enhanced monitoring during retry

## Testing and Validation

### Pre-Rotation Testing
- **Dry Run**: Test rotation procedure in non-production
- **Impact Assessment**: Evaluate potential service disruption
- **Rollback Plan**: Ensure rollback procedures are documented

### Post-Rotation Validation
- **Functionality Testing**: Verify all services work correctly
- **Security Testing**: Confirm security controls are intact
- **Performance Testing**: Ensure no performance degradation

### Regular Drills
- **Quarterly Drills**: Practice key rotation procedures
- **Annual Review**: Review and update rotation procedures
- **Lessons Learned**: Document and share rotation experiences

## Roles and Responsibilities

### Key Owners
- **Responsibility**: Overall key management responsibility
- **Tasks**: 
  - Schedule key rotations
  - Coordinate rotation activities
  - Ensure proper validation
  - Maintain key inventory

### Security Team
- **Responsibility**: Security aspects of key management
- **Tasks**:
  - Review key rotation procedures
  - Monitor for key-related security events
  - Investigate key compromises
  - Approve emergency rotations

### Operations Team
- **Responsibility**: Operational execution of key rotations
- **Tasks**:
  - Execute key rotation procedures
  - Monitor service impact
  - Troubleshoot rotation issues
  - Maintain key management systems

### Development Teams
- **Responsibility**: Application-level key management
- **Tasks**:
  - Update applications for key rotations
  - Implement key rotation in code
  - Test application with new keys
  - Document key usage in applications

## Audit and Compliance

### Audit Requirements
- **Rotation Logs**: Detailed logs of all key rotations
- **Validation Records**: Records of post-rotation validation
- **Incident Reports**: Documentation of key-related incidents
- **Compliance Reports**: Reports for regulatory compliance

### Compliance Mapping
- **SOX**: Key management controls for financial systems
- **PCI DSS**: Key management for cardholder data
- **HIPAA**: Key management for protected health information
- **GDPR**: Key management for personal data protection

## Tools and References

### Key Management Tools
- **AWS KMS**: For AWS-managed keys
- **Azure Key Vault**: For Azure-managed keys
- **Google Cloud KMS**: For GCP-managed keys
- **HashiCorp Vault**: For application-managed secrets
- **OpenSSL**: For manual key operations

### Monitoring Tools
- **Cloud Monitoring**: For cloud service key monitoring
- **SIEM**: For security event monitoring
- **APM Tools**: For application performance monitoring
- **Log Aggregation**: For centralized log analysis

## References

- [MPC/HSM Policy](../security/mpc-hsm-policy.md)
- [Multisig Addresses](../security/multisig-addresses.md)
- [Policy Registry](../../infra/policies/policy-registry.md)
- [Infrastructure Access Policy](../../infra/policies/OPA-Cedar/infrastructure_access.cedar)