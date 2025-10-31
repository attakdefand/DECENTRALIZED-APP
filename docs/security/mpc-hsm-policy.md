# MPC/HSM Policy

This document defines the policies and procedures for using Multi-Party Computation (MPC) and Hardware Security Modules (HSMs) for cryptographic key management.

## Overview

This policy establishes requirements for the use of MPC and HSM technologies to protect high-value cryptographic keys and perform sensitive cryptographic operations within the DECENTRALIZED-APP infrastructure.

## Scope

This policy applies to all systems and applications that require protection of high-value cryptographic keys, including:
- Private keys for blockchain transactions
- Encryption keys for sensitive data
- Authentication keys for critical systems
- Code signing keys for software releases

## Policy Statements

### 1. Technology Selection

#### HSM Usage
- **Requirement**: All high-value cryptographic keys MUST be stored in HSMs
- **Rationale**: HSMs provide hardware-level protection against key extraction
- **Exceptions**: Only approved by the Security Officer for specific use cases

#### MPC Usage
- **Requirement**: Multi-signature operations SHOULD use MPC where feasible
- **Rationale**: MPC provides distributed trust and eliminates single points of failure
- **Implementation**: Use threshold signatures for key operations

### 2. Key Management

#### Key Generation
- **HSM Keys**: All keys MUST be generated within the HSM
- **MPC Keys**: Key shares MUST be generated using secure MPC protocols
- **Export**: Private keys MUST never be exported in plaintext form

#### Key Storage
- **HSM Storage**: All private keys MUST be stored in HSM secure memory
- **MPC Storage**: Key shares MUST be distributed across multiple locations
- **Backup**: All keys MUST have secure backup procedures

#### Key Usage
- **Cryptographic Operations**: All operations MUST be performed within the HSM/MPC environment
- **Access Control**: Key usage MUST be logged and monitored
- **Rate Limiting**: Key usage MAY be rate-limited to prevent abuse

### 3. Access Control

#### Authentication
- **HSM Access**: Access to HSMs MUST require multi-factor authentication
- **MPC Access**: Access to MPC nodes MUST require strong authentication
- **Role-Based Access**: Access MUST be granted based on job function

#### Authorization
- **Least Privilege**: Users MUST have minimum required privileges
- **Segregation of Duties**: No single user SHOULD have complete control
- **Approval Workflows**: Critical operations MUST require approval

#### Session Management
- **Timeout**: Sessions MUST timeout after periods of inactivity
- **Audit**: All access MUST be logged for audit purposes
- **Monitoring**: Suspicious access patterns MUST be detected and alerted

### 4. Operations Security

#### Physical Security
- **HSM Location**: HSMs MUST be located in physically secure environments
- **Access Control**: Physical access MUST be restricted and logged
- **Environmental Controls**: Appropriate environmental controls MUST be maintained

#### Network Security
- **Network Isolation**: HSM networks SHOULD be isolated from general networks
- **Encryption**: All communications MUST be encrypted
- **Firewall Rules**: Access MUST be restricted through firewall rules

#### Software Security
- **Patch Management**: HSM software MUST be kept up to date
- **Configuration Management**: HSM configurations MUST be documented and controlled
- **Vulnerability Management**: Regular vulnerability assessments MUST be performed

### 5. Monitoring and Auditing

#### Activity Logging
- **Comprehensive Logging**: All key operations MUST be logged
- **Log Protection**: Logs MUST be protected from tampering
- **Retention**: Logs MUST be retained for compliance purposes

#### Alerting
- **Real-time Alerts**: Critical operations MUST generate real-time alerts
- **Anomaly Detection**: Unusual patterns MUST be detected and reported
- **Incident Response**: Security events MUST trigger incident response procedures

#### Regular Audits
- **Internal Audits**: Regular internal audits MUST be conducted
- **External Audits**: Periodic external audits MUST be performed
- **Compliance Verification**: Compliance with this policy MUST be verified

### 6. Incident Response

#### Compromise Detection
- **Monitoring**: Continuous monitoring for signs of compromise
- **Indicators**: Defined indicators of key compromise
- **Reporting**: Clear procedures for reporting suspected compromises

#### Response Procedures
- **Immediate Isolation**: Compromised systems MUST be isolated immediately
- **Impact Assessment**: Scope of compromise MUST be determined
- **Key Revocation**: Compromised keys MUST be revoked
- **Key Regeneration**: New keys MUST be generated following this policy

#### Recovery
- **Backup Restoration**: Secure backups MUST be available for recovery
- **System Rebuilding**: Compromised systems MUST be rebuilt from trusted sources
- **Validation**: Systems MUST be validated before returning to service

## Implementation Requirements

### HSM Implementation

#### Hardware Selection
- **FIPS 140-2 Level 3**: HSMs MUST meet FIPS 140-2 Level 3 certification
- **Tamper Resistance**: HSMs MUST provide physical tamper resistance
- **Key Backup**: HSMs MUST support secure key backup and recovery

#### Configuration
- **Default Settings**: Default configurations MUST be reviewed and hardened
- **Administrative Access**: Administrative access MUST be strictly controlled
- **Operational Procedures**: Standard operating procedures MUST be documented

#### Integration
- **API Usage**: HSM APIs MUST be used securely
- **Application Integration**: Applications MUST integrate with HSMs properly
- **Performance Considerations**: Performance impacts MUST be managed

### MPC Implementation

#### Protocol Selection
- **Secure Protocols**: Only cryptographically secure MPC protocols MUST be used
- **Implementation Review**: MPC implementations MUST be reviewed by security experts
- **Library Selection**: Only well-vetted MPC libraries SHOULD be used

#### Node Management
- **Node Distribution**: MPC nodes MUST be distributed across different locations
- **Node Security**: Each MPC node MUST be secured according to this policy
- **Communication Security**: Inter-node communications MUST be secured

#### Key Share Management
- **Share Distribution**: Key shares MUST be distributed securely
- **Share Recovery**: Procedures for share recovery MUST be established
- **Share Revocation**: Compromised shares MUST be revocable

## Compliance Requirements

### Regulatory Compliance
- **SOX**: Compliance with financial reporting requirements
- **PCI DSS**: Compliance with payment card industry standards
- **GDPR**: Compliance with data protection regulations
- **HIPAA**: Compliance with healthcare information privacy rules

### Industry Standards
- **NIST**: Adherence to NIST cryptographic standards
- **ISO 27001**: Compliance with information security management standards
- **OWASP**: Following OWASP cryptographic best practices

### Internal Standards
- **Security Policies**: Alignment with organizational security policies
- **Risk Management**: Integration with enterprise risk management
- **Audit Requirements**: Meeting internal audit requirements

## Roles and Responsibilities

### Security Officer
- **Policy Oversight**: Overall responsibility for this policy
- **Compliance Monitoring**: Ensuring compliance with this policy
- **Incident Management**: Managing security incidents related to HSM/MPC

### System Administrators
- **HSM Management**: Day-to-day management of HSM systems
- **MPC Node Management**: Management of MPC nodes
- **Access Control**: Implementation of access control measures

### Application Developers
- **Secure Integration**: Integrating applications with HSM/MPC securely
- **Key Usage**: Using cryptographic keys according to this policy
- **Error Handling**: Properly handling cryptographic errors

### Auditors
- **Policy Review**: Regular review of this policy
- **Compliance Assessment**: Assessing compliance with this policy
- **Recommendations**: Providing recommendations for improvement

## Training and Awareness

### Required Training
- **HSM Operations**: Training on HSM operation and management
- **MPC Concepts**: Understanding of MPC principles and operation
- **Security Best Practices**: General cryptographic security best practices

### Awareness Programs
- **Regular Updates**: Keeping staff informed of policy changes
- **Security News**: Sharing relevant security information
- **Lessons Learned**: Communicating lessons from incidents

## Review and Updates

### Review Schedule
- **Annual Review**: This policy MUST be reviewed annually
- **Trigger Events**: Review MUST occur after security incidents
- **Regulatory Changes**: Review MUST occur when regulations change

### Update Process
- **Change Request**: Formal process for proposing changes
- **Stakeholder Review**: Review by relevant stakeholders
- **Approval**: Approval by designated authority
- **Communication**: Communication of changes to affected parties

## References

- [Key Rotation Runbook](../runbooks/key-rotation.md)
- [Multisig Addresses](multisig-addresses.md)
- [Policy Registry](../../infra/policies/policy-registry.md)
- [NIST Cryptographic Standards](https://csrc.nist.gov/publications/detail/fips/140/2/final)
- [FIPS 140-2 Validation](https://csrc.nist.gov/projects/cryptographic-module-validation-program)