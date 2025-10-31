# IAM RBAC Map

This document defines the Role-Based Access Control (RBAC) mappings for the DECENTRALIZED-APP project.

## Overview

The IAM RBAC map establishes the relationship between user roles, permissions, and resources within the system. This ensures that users have appropriate access based on their job functions while adhering to the principle of least privilege.

## Role Definitions

### Administrative Roles

#### System Administrator
- **Description**: Full system administration privileges
- **Scope**: All systems and applications
- **Permissions**:
  - System configuration and management
  - User account management
  - Security policy management
  - System monitoring and maintenance
  - Backup and recovery operations
- **Members**: 
  - alice.admin@company.com
  - bob.admin@company.com

#### Security Administrator
- **Description**: Security-focused administrative privileges
- **Scope**: Security systems and tools
- **Permissions**:
  - Security policy management
  - Access control management
  - Security monitoring and alerting
  - Incident response coordination
  - Vulnerability management
- **Members**: 
  - carol.security@company.com
  - david.security@company.com

#### Network Administrator
- **Description**: Network infrastructure administration
- **Scope**: Network systems and devices
- **Permissions**:
  - Network configuration management
  - Firewall rule management
  - Network monitoring and troubleshooting
  - DNS and DHCP management
  - Network security controls
- **Members**: 
  - eve.network@company.com
  - frank.network@company.com

### Development Roles

#### Lead Developer
- **Description**: Senior development team member with extended privileges
- **Scope**: Development environments and code repositories
- **Permissions**:
  - Code repository management
  - Branch protection rule configuration
  - CI/CD pipeline management
  - Development environment administration
  - Code review approvals
- **Members**: 
  - grace.lead@company.com
  - henry.lead@company.com

#### Developer
- **Description**: Standard development team member
- **Scope**: Development environments and assigned projects
- **Permissions**:
  - Code commit and push access
  - Pull request creation and review
  - Development environment access
  - Testing environment access
  - Debugging tool access
- **Members**: 
  - irene.dev@company.com
  - jack.dev@company.com
  - karen.dev@company.com
  - liam.dev@company.com

#### DevOps Engineer
- **Description**: Development operations specialist
- **Scope**: CI/CD systems and deployment environments
- **Permissions**:
  - CI/CD pipeline configuration
  - Deployment environment management
  - Infrastructure as code management
  - Monitoring system configuration
  - Log management
- **Members**: 
  - mia.devops@company.com
  - noah.devops@company.com

### Operations Roles

#### Operations Manager
- **Description**: Operations team lead
- **Scope**: Production and staging environments
- **Permissions**:
  - Production environment access
  - System performance monitoring
  - Capacity planning
  - Incident management
  - Change approval authority
- **Members**: 
  - olivia.ops@company.com

#### Site Reliability Engineer
- **Description**: SRE responsible for system reliability
- **Scope**: Production systems and infrastructure
- **Permissions**:
  - Production system monitoring
  - Automated remediation execution
  - Performance optimization
  - System scaling operations
  - Root cause analysis
- **Members**: 
  - peter.sre@company.com
  - quinn.sre@company.com

#### Support Engineer
- **Description**: Technical support specialist
- **Scope**: Customer support systems and tools
- **Permissions**:
  - Customer account information access
  - Support ticket management
  - Diagnostic tool access
  - Limited production system access
  - Customer communication tools
- **Members**: 
  - rachel.support@company.com
  - sam.support@company.com

### Business Roles

#### Product Manager
- **Description**: Product strategy and management
- **Scope**: Product analytics and planning tools
- **Permissions**:
  - Product analytics access
  - Roadmap planning tools
  - Customer feedback systems
  - Market research databases
  - Competitive analysis tools
- **Members**: 
  - tom.product@company.com
  - uma.product@company.com

#### Business Analyst
- **Description**: Business intelligence and analysis
- **Scope**: Business data and analytics systems
- **Permissions**:
  - Business intelligence tools
  - Data warehouse access
  - Reporting system access
  - Dashboard creation and management
  - Data visualization tools
- **Members**: 
  - victor.analyst@company.com
  - wendy.analyst@company.com

### Specialized Roles

#### Auditor
- **Description**: Internal or external auditor
- **Scope**: Audit systems and compliance tools
- **Permissions**:
  - Audit log access
  - Compliance reporting tools
  - System configuration review
  - Security policy review
  - Limited read-only access to production systems
- **Members**: 
  - External audit firm representatives
  - Internal audit team

#### Compliance Officer
- **Description**: Regulatory compliance specialist
- **Scope**: Compliance systems and documentation
- **Permissions**:
  - Compliance monitoring tools
  - Regulatory reporting systems
  - Policy management systems
  - Compliance audit tools
  - Risk assessment tools
- **Members**: 
  - xavier.compliance@company.com

## Permission Mappings

### System Access Permissions

| Role | Production | Staging | Development | Testing | Admin |
|------|------------|---------|-------------|---------|-------|
| System Administrator | Full | Full | Full | Full | Full |
| Security Administrator | Read/Write | Read/Write | Read/Write | Read/Write | Full |
| Network Administrator | Read/Write | Read/Write | Read/Write | Read | Full |
| Lead Developer | Read | Full | Full | Full | Limited |
| Developer | Read | Read | Full | Full | None |
| DevOps Engineer | Read/Write | Full | Full | Full | Limited |
| Operations Manager | Full | Full | Read | Read | Limited |
| Site Reliability Engineer | Full | Full | Read | Read | Limited |
| Support Engineer | Limited | Limited | None | None | None |
| Product Manager | Read | Read | None | None | None |
| Business Analyst | Read | Read | None | None | None |
| Auditor | Read Only | Read Only | Read Only | Read Only | Read Only |
| Compliance Officer | Read Only | Read Only | Read Only | Read Only | Read Only |

### Resource Permissions

#### Code Repository Access
- **System Administrator**: Full access to all repositories
- **Security Administrator**: Read access to all repositories, write to security policy repos
- **Lead Developer**: Full access to assigned project repositories
- **Developer**: Read/write access to assigned project repositories
- **DevOps Engineer**: Read/write access to CI/CD and infrastructure repositories
- **Others**: Read-only access to public repositories only

#### Database Access
- **System Administrator**: Full access to all databases
- **Security Administrator**: Read access to all databases, write to security audit tables
- **Lead Developer**: Read/write access to development and staging databases
- **Developer**: Read/write access to development databases, read-only to staging
- **DevOps Engineer**: Read/write access to all non-production databases
- **Support Engineer**: Limited read-only access to production databases for support purposes
- **Business Analyst**: Read-only access to analytics and reporting databases
- **Others**: No database access

#### Infrastructure Access
- **System Administrator**: Full access to all infrastructure
- **Security Administrator**: Read access to all infrastructure, write to security components
- **Network Administrator**: Full access to network infrastructure
- **DevOps Engineer**: Read/write access to cloud and container infrastructure
- **Site Reliability Engineer**: Read/write access to production infrastructure
- **Operations Manager**: Read/write access to production infrastructure
- **Others**: No infrastructure access

## Access Control Policies

### Authentication Requirements
- **All Roles**: Multi-factor authentication required
- **Administrative Roles**: Hardware security keys required
- **Specialized Roles**: Certificate-based authentication where applicable

### Session Management
- **Default Session Timeout**: 8 hours
- **Administrative Roles**: 2-hour timeout with re-authentication
- **Specialized Roles**: 4-hour timeout

### Access Review Schedule
- **Administrative Roles**: Monthly review
- **Standard Roles**: Quarterly review
- **Specialized Roles**: Semi-annual review

### Provisioning Process
1. Role request submission
2. Manager approval
3. Security team review
4. System access provisioning
5. Access confirmation notification

### Deprovisioning Process
1. Employment termination or role change notification
2. Immediate access suspension
3. Asset collection and verification
4. System access removal
5. Audit trail documentation

## Role Hierarchy

```
System Administrator
├── Security Administrator
├── Network Administrator
├── Lead Developer
│   └── Developer
├── DevOps Engineer
├── Operations Manager
│   └── Site Reliability Engineer
├── Support Engineer
├── Product Manager
├── Business Analyst
├── Auditor
└── Compliance Officer
```

## Role Assignment Matrix

| User | Primary Role | Secondary Roles | Assignment Date | Review Date |
|------|--------------|-----------------|-----------------|-------------|
| alice.admin@company.com | System Administrator | Security Administrator | 2025-01-15 | 2025-11-15 |
| bob.admin@company.com | System Administrator | Network Administrator | 2025-02-20 | 2025-12-20 |
| carol.security@company.com | Security Administrator | None | 2025-03-10 | 2025-12-10 |
| david.security@company.com | Security Administrator | Auditor | 2025-04-05 | 2026-01-05 |
| eve.network@company.com | Network Administrator | None | 2025-05-12 | 2026-02-12 |
| frank.network@company.com | Network Administrator | DevOps Engineer | 2025-06-18 | 2026-03-18 |
| grace.lead@company.com | Lead Developer | None | 2025-07-22 | 2026-04-22 |
| henry.lead@company.com | Lead Developer | Product Manager | 2025-08-30 | 2026-05-30 |
| irene.dev@company.com | Developer | None | 2025-09-15 | 2026-06-15 |
| jack.dev@company.com | Developer | Business Analyst | 2025-10-01 | 2026-07-01 |

## Policy Enforcement

### Automated Controls
- Identity and access management system
- Role-based access control enforcement
- Regular access review workflows
- Automated deprovisioning integration

### Manual Controls
- Manager approval for role assignments
- Security team validation of access requests
- Periodic access certification processes
- Exception management for special cases

## Compliance Mapping

### Regulatory Requirements
- **SOX**: Role segregation and access controls
- **GDPR**: Data access controls and privacy by design
- **PCI DSS**: Administrative access controls and monitoring
- **ISO 27001**: Access control policies and procedures

### Audit Requirements
- Role assignment documentation
- Access review records
- Exception management logs
- User access provisioning/deprovisioning records

## Review and Updates

### Review Schedule
- **Annual**: Complete RBAC map review
- **Quarterly**: Role assignment verification
- **Monthly**: Critical role access reviews

### Update Process
1. Change request submission
2. Business justification documentation
3. Risk assessment completion
4. Approval workflow execution
5. System updates implementation
6. Communication to affected users

## References

- [Policy Catalog](POLICY-CATALOG.md)
- [Exception Management](EXCEPTIONS.md)
- [Infrastructure Access Policy](../../infra/policies/OPA-Cedar/infrastructure_access.cedar)
- [Repository Access Policy](../../infra/policies/OPA-Cedar/repository_access.cedar)
- [Data Access Policy](../../infra/policies/OPA-Cedar/data_access.cedar)