# Key Rotation Runbook

## Overview
This document provides procedures and guidelines for rotating cryptographic keys used in the decentralized application infrastructure. Proper key rotation is essential for maintaining security and compliance.

## Key Rotation Principles
1. **Regular Rotation**: Keys should be rotated on a regular schedule to limit exposure
2. **Compromise Response**: Keys must be immediately rotated if compromise is suspected
3. **Minimal Privilege**: Only authorized personnel should have access to perform key rotation
4. **Audit Trail**: All key rotation activities must be logged and auditable
5. **Backward Compatibility**: Rotation should maintain service availability during transition

## Key Types and Rotation Procedures

### TLS Certificates
**Rotation Frequency**: Every 90 days or as dictated by certificate authority
**Procedure**:
1. Generate new certificate signing request (CSR)
2. Submit CSR to certificate authority
3. Install new certificate on all affected services
4. Restart services to load new certificate
5. Verify certificate installation and functionality
6. Update certificate inventory
**Validation**: 
- Certificate expiration monitoring
- Service connectivity testing
- TLS handshake verification

### SSH Keys
**Rotation Frequency**: Every 180 days or upon personnel changes
**Procedure**:
1. Generate new SSH key pair
2. Distribute public key to authorized systems
3. Update SSH configuration files
4. Test SSH connectivity with new keys
5. Revoke old public keys from systems
6. Update key inventory
**Validation**:
- SSH connectivity testing
- Key fingerprint verification
- Access log review

### API Keys
**Rotation Frequency**: Every 90 days or as needed for security incidents
**Procedure**:
1. Generate new API key in service provider console
2. Update application configuration with new key
3. Deploy updated configuration to all environments
4. Test API functionality with new key
5. Revoke old API key after grace period
6. Update secrets management system
**Validation**:
- API functionality testing
- Service monitoring
- Access log verification

### Database Credentials
**Rotation Frequency**: Every 180 days or upon security incidents
**Procedure**:
1. Generate new database user credentials
2. Update application configuration with new credentials
3. Deploy updated configuration to all environments
4. Test database connectivity with new credentials
5. Change credentials for existing database user
6. Update secrets management system
**Validation**:
- Database connectivity testing
- Query execution verification
- Application functionality testing

### Data Encryption Keys (DEKs)
**Rotation Frequency**: Every 365 days or as dictated by compliance requirements
**Procedure**:
1. Generate new data encryption key
2. Re-encrypt existing data with new key
3. Update key encryption key (KEK) to protect new DEK
4. Verify data integrity after re-encryption
5. Update key management system
6. Schedule old key for archival
**Validation**:
- Data decryption testing
- Integrity verification
- Key management system synchronization

### Key Encryption Keys (KEKs)
**Rotation Frequency**: Every 365 days or as dictated by compliance requirements
**Procedure**:
1. Generate new key encryption key
2. Re-encrypt all data encryption keys with new KEK
3. Update key management system with new KEK
4. Verify successful re-encryption of all DEKs
5. Test decryption of sample data
6. Schedule old KEK for archival
**Validation**:
- Key encryption/decryption testing
- Data accessibility verification
- System performance monitoring

### Cloud Service Account Keys
**Rotation Frequency**: Every 90 days or upon personnel changes
**Procedure**:
1. Generate new service account key in cloud provider console
2. Update infrastructure as code with new key
3. Deploy updated configuration to all environments
4. Test cloud service connectivity with new key
5. Revoke old service account key after grace period
6. Update secrets management system
**Validation**:
- Cloud service connectivity testing
- Infrastructure provisioning verification
- Access log review

### Application Service Account Keys
**Rotation Frequency**: Every 180 days or upon security incidents
**Procedure**:
1. Generate new service account key in identity provider
2. Update application configuration with new key
3. Deploy updated configuration to all environments
4. Test service account authentication
5. Revoke old service account key after grace period
6. Update secrets management system
**Validation**:
- Authentication testing
- Service functionality verification
- Access log review

## Key Management System

### Key States
1. **Active**: Currently in use for encryption/decryption operations
2. **Standby**: Available for use but not currently active
3. **Compromised**: Suspected or confirmed security breach
4. **Expired**: Past rotation schedule but not yet revoked
5. **Revoked**: No longer valid for any operations
6. **Archived**: Retained for compliance but not in active use

### Key Inventory
All keys must be tracked in the key inventory system with:
- Key identifier
- Key type
- Creation date
- Expiration date
- Current state
- Associated services
- Rotation history
- Ownership information

## Rotation Automation

### Automated Rotation
The following key types support automated rotation:
- TLS certificates (via Let's Encrypt or similar)
- SSH keys (via infrastructure as code)
- API keys (via service provider APIs)
- Database credentials (via database management tools)

### Manual Rotation
The following key types require manual rotation:
- Data encryption keys
- Key encryption keys
- Cloud service account keys
- Application service account keys

### Rotation Scheduling
Key rotation tasks are scheduled through:
- Calendar reminders for manual rotations
- Automated jobs for supported key types
- Incident response procedures for compromise
- Compliance-driven schedules

## Emergency Procedures

### Compromised Keys
If key compromise is suspected or confirmed:
1. Immediately revoke the compromised key
2. Generate new key following standard procedures
3. Deploy new key to all affected systems
4. Test functionality with new key
5. Investigate scope of potential exposure
6. Document incident and remediation actions
7. Notify relevant stakeholders
8. Update incident response documentation

### Failed Rotations
If key rotation fails:
1. Assess impact on services and security
2. Attempt manual rotation if automated process failed
3. Implement temporary mitigation if necessary
4. Engage vendor support if needed
5. Document failure and resolution
6. Review and improve rotation procedures
7. Schedule follow-up rotation if needed

## Testing and Validation

### Pre-Rotation Testing
Before rotating any key:
1. Verify backup systems are functional
2. Confirm rollback procedures are available
3. Test new key in isolated environment
4. Validate service dependencies
5. Schedule maintenance window if needed

### Post-Rotation Validation
After key rotation:
1. Verify all services are functioning normally
2. Confirm new key is being used for operations
3. Monitor for authentication or encryption errors
4. Validate data accessibility
5. Update documentation and inventory
6. Close rotation task in tracking system

### Regular Testing Schedule
- Monthly: Verify automated rotation systems
- Quarterly: Test manual rotation procedures
- Annually: Full key rotation drill exercise
- As needed: Post-incident procedure review

## Roles and Responsibilities

### Key Management Team
- Primary responsibility for key rotation execution
- Maintains key inventory and rotation schedules
- Responds to key compromise incidents
- Documents procedures and maintains runbook

### System Administrators
- Execute key rotation procedures for their systems
- Validate successful rotation completion
- Report rotation issues or failures
- Maintain system-specific rotation documentation

### Security Team
- Monitor for key compromise indicators
- Review key rotation compliance
- Investigate security incidents related to keys
- Provide guidance on key management best practices

### Compliance Team
- Ensure key rotation meets regulatory requirements
- Conduct periodic key management audits
- Review and approve key rotation policies
- Report on key management compliance metrics

## Documentation and Communication

### Rotation Records
Each key rotation must be documented with:
- Date and time of rotation
- Personnel performing rotation
- Services affected
- Issues encountered
- Resolution actions taken
- Validation results

### Communication Plan
Key rotation activities requiring communication:
- Service-affecting rotations: Notify service owners
- Security-critical rotations: Notify security team
- Compliance-mandated rotations: Notify compliance team
- Emergency rotations: Notify incident response team

## Continuous Improvement

### Lessons Learned
After each key rotation:
- Document any issues or challenges encountered
- Identify opportunities for process improvement
- Update procedures based on lessons learned
- Share knowledge with team members

### Technology Updates
Regularly evaluate:
- New key management technologies
- Improved automation capabilities
- Enhanced security controls
- Industry best practices
- Compliance requirement changes

### Training and Awareness
- Regular training on key rotation procedures
- Updates on new technologies or processes
- Security awareness related to key management
- Compliance requirement education