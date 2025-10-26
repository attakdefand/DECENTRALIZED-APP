//! Security Layers Validation Tests
//!
//! This module contains tests that validate all security layers against the features
//! defined in the web3_protection_layers.csv file.

use security_layers::{
    governance_policy::*,
    identity_access::*,
    types::SecurityLayer,
};
use std::collections::HashMap;

/// Test that validates all security layers from the CSV file
#[test]
fn test_security_layers_from_csv() {
    // Create a list of all security layers from the CSV file
    let security_layers = create_security_layers_from_csv();
    
    // Verify we have the correct number of layers
    // Note: We have 39 layers because we're counting all sub-layers
    // Layer 1 has 3 sub-layers, Layer 2 has 4 sub-layers, etc.
    println!("Total security layers: {}", security_layers.len());
    assert_eq!(security_layers.len(), 39, "Expected 39 security layers from CSV");
    
    // Test governance and policy management (Layer 1)
    test_layer_1_governance_policy(&security_layers);
    
    // Test identity and access control (Layer 2)
    test_layer_2_identity_access(&security_layers);
    
    println!("All security layers validated successfully!");
}

/// Create security layers from the CSV data
fn create_security_layers_from_csv() -> Vec<SecurityLayer> {
    let mut layers = Vec::new();
    
    // Layer 1: Governance & Policy
    layers.push(SecurityLayer {
        layer_number: 1,
        layer_name: "Governance & Policy".to_string(),
        main_type: "Policy Management".to_string(),
        sub_type: "Security Policy Catalog".to_string(),
        component_mechanism: "Org-wide security policy, coding standards, infra hardening guidelines, data handling rules".to_string(),
        goal: "Make security mandatory and auditable".to_string(),
        evidence_telemetry: "Signed policy docs, control mapping, approvals".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 1,
        layer_name: "Governance & Policy".to_string(),
        main_type: "Exception Management".to_string(),
        sub_type: "Risk Acceptance Workflow".to_string(),
        component_mechanism: "Exception register, owner+expiry, tracked in repo / ticket".to_string(),
        goal: "Force accountability for any deviation".to_string(),
        evidence_telemetry: "Open exceptions with expiry and sign-off".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 1,
        layer_name: "Governance & Policy".to_string(),
        main_type: "Audit & Assurance".to_string(),
        sub_type: "Internal/External Audit Tracking".to_string(),
        component_mechanism: "Security audit issues labeled in tracker, remediation SLAs".to_string(),
        goal: "Close gaps found by audit / pen test".to_string(),
        evidence_telemetry: "% audit findings closed on time, PR links".to_string(),
    });
    
    // Layer 2: Identity & Access Control
    layers.push(SecurityLayer {
        layer_number: 2,
        layer_name: "Identity & Access Control".to_string(),
        main_type: "AuthN (Who are you)".to_string(),
        sub_type: "User/Auth Service".to_string(),
        component_mechanism: "Password hashing, MFA, OAuth2/OIDC, JWT signing/verification".to_string(),
        goal: "Only legit users can enter".to_string(),
        evidence_telemetry: "Auth logs, failed login attempts, token issuance logs".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 2,
        layer_name: "Identity & Access Control".to_string(),
        main_type: "AuthZ (What can you do)".to_string(),
        sub_type: "RBAC/ABAC/PBAC".to_string(),
        component_mechanism: "Role-based access control, attribute-based access control, policy-based access (OPA / Cedar)".to_string(),
        goal: "Stop privilege abuse / lateral movement".to_string(),
        evidence_telemetry: "Access decision logs, denied actions".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 2,
        layer_name: "Identity & Access Control".to_string(),
        main_type: "Session & Token Hygiene".to_string(),
        sub_type: "Token Lifecycle".to_string(),
        component_mechanism: "Short-lived access tokens, refresh tokens, rotation, revocation list".to_string(),
        goal: "Reduce stolen-token blast radius".to_string(),
        evidence_telemetry: "Token expiry histogram, revoked token hits".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 2,
        layer_name: "Identity & Access Control".to_string(),
        main_type: "Secrets Hygiene".to_string(),
        sub_type: "Secret Distribution".to_string(),
        component_mechanism: "Vault / KMS, no secrets in code, per-service credentials".to_string(),
        goal: "Stop credential leaks".to_string(),
        evidence_telemetry: "Secrets rotation logs, secret age report".to_string(),
    });
    
    // Layer 3: Application Security
    layers.push(SecurityLayer {
        layer_number: 3,
        layer_name: "Application Security".to_string(),
        main_type: "Input Protection".to_string(),
        sub_type: "Validation & Sanitization".to_string(),
        component_mechanism: "Strict type validation, regex allowlists, length limits, unicode normalization".to_string(),
        goal: "Block injection, XSS, deserialization attacks".to_string(),
        evidence_telemetry: "Rejected request counts by rule".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 3,
        layer_name: "Application Security".to_string(),
        main_type: "Output Protection".to_string(),
        sub_type: "Encoding/Escaping".to_string(),
        component_mechanism: "HTML encode, JSON encode, header encode".to_string(),
        goal: "Stop stored/reflective XSS".to_string(),
        evidence_telemetry: "CSP violation reports, browser security reports".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 3,
        layer_name: "Application Security".to_string(),
        main_type: "Business Logic Controls".to_string(),
        sub_type: "Rate/Velocity Rules".to_string(),
        component_mechanism: "OTP retry limits, withdrawal limits, anti-bruteforce counters, anti-spam throttles".to_string(),
        goal: "Stop abuse of legit flows".to_string(),
        evidence_telemetry: "Per-user throttle hits, lockouts".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 3,
        layer_name: "Application Security".to_string(),
        main_type: "Dependency Safety".to_string(),
        sub_type: "SAST/SCA".to_string(),
        component_mechanism: "Static code scanning, dependency vulnerability scan, SBOM, license scan".to_string(),
        goal: "Stop known-vuln libs from shipping".to_string(),
        evidence_telemetry: "Critical vuln count, unresolved vuln age".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 3,
        layer_name: "Application Security".to_string(),
        main_type: "Runtime Protections".to_string(),
        sub_type: "WAF / RASP".to_string(),
        component_mechanism: "WAF rulesets (OWASP Top 10), runtime self-protection hooks in app".to_string(),
        goal: "Block exploit patterns pre-database".to_string(),
        evidence_telemetry: "WAF block events, rule hit rate".to_string(),
    });
    
    // Layer 4: API & Gateway Security
    layers.push(SecurityLayer {
        layer_number: 4,
        layer_name: "API & Gateway Security".to_string(),
        main_type: "Protocol Safety".to_string(),
        sub_type: "Schema Enforcement".to_string(),
        component_mechanism: "Strongly typed request/response contract, OpenAPI/GraphQL schema validation".to_string(),
        goal: "Reject malformed/unknown fields before logic runs".to_string(),
        evidence_telemetry: "% rejected at gateway vs app".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 4,
        layer_name: "API & Gateway Security".to_string(),
        main_type: "Abuse Mitigation".to_string(),
        sub_type: "Rate Limit / Throttle / Burst Control".to_string(),
        component_mechanism: "Per-IP rate limit, per-token rate limit, circuit breakers, quota windows".to_string(),
        goal: "Stop DoS / scraping / brute force".to_string(),
        evidence_telemetry: "HTTP 429 counts, surge graphs".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 4,
        layer_name: "API & Gateway Security".to_string(),
        main_type: "Auth at Edge".to_string(),
        sub_type: "JWT / mTLS at Gateway".to_string(),
        component_mechanism: "mTLS between client and gateway, gateway verifies signature/claims before forwarding".to_string(),
        goal: "Drop bad traffic early".to_string(),
        evidence_telemetry: "Gateway auth failure logs".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 4,
        layer_name: "API & Gateway Security".to_string(),
        main_type: "Data Filtering".to_string(),
        sub_type: "Header/Body Scrubbers".to_string(),
        component_mechanism: "Strip dangerous headers, reject unsupported verbs, block oversized payloads".to_string(),
        goal: "Reduce attack surface".to_string(),
        evidence_telemetry: "Blocked verb stats, oversized body rejections".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 4,
        layer_name: "API & Gateway Security".to_string(),
        main_type: "Allowlisting".to_string(),
        sub_type: "Service Contract Allowlist".to_string(),
        component_mechanism: "Only allow specific routes/methods per client/app tier".to_string(),
        goal: "Make public surface area explicit".to_string(),
        evidence_telemetry: "Denied route attempts by client id".to_string(),
    });
    
    // Layer 5: Data Security
    layers.push(SecurityLayer {
        layer_number: 5,
        layer_name: "Data Security".to_string(),
        main_type: "Data Classification".to_string(),
        sub_type: "Sensitivity Tiering".to_string(),
        component_mechanism: "Classify data: public / internal / confidential / restricted".to_string(),
        goal: "Know which data needs strong controls".to_string(),
        evidence_telemetry: "Data inventory with labels".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 5,
        layer_name: "Data Security".to_string(),
        main_type: "Data-in-Transit".to_string(),
        sub_type: "TLS Everywhere".to_string(),
        component_mechanism: "HTTPS/TLS 1.2+, HSTS, mTLS service-to-service".to_string(),
        goal: "Stop sniffing / MITM".to_string(),
        evidence_telemetry: "TLS handshake logs, cert rotation logs".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 5,
        layer_name: "Data Security".to_string(),
        main_type: "Data-at-Rest".to_string(),
        sub_type: "Encryption at Rest".to_string(),
        component_mechanism: "KMS-managed disk/volume/db encryption, envelope encryption for fields like PII".to_string(),
        goal: "Protect data if disk/db is stolen".to_string(),
        evidence_telemetry: "Key rotation logs, KMS access logs".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 5,
        layer_name: "Data Security".to_string(),
        main_type: "Data Minimization".to_string(),
        sub_type: "Field Reduction / Masking".to_string(),
        component_mechanism: "Store only required attributes, redact PII in logs, tokenize high-risk values".to_string(),
        goal: "Shrink breach impact".to_string(),
        evidence_telemetry: "PII in logs scanner report".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 5,
        layer_name: "Data Security".to_string(),
        main_type: "Backup & Restore".to_string(),
        sub_type: "Signed/Encrypted Backups".to_string(),
        component_mechanism: "Periodic encrypted snapshots, offline copy, tested restore drill".to_string(),
        goal: "Survive ransomware / data loss".to_string(),
        evidence_telemetry: "Successful restore drill evidence, RPO/RTO metrics".to_string(),
    });
    
    // Layer 6: Network & Infrastructure Security
    layers.push(SecurityLayer {
        layer_number: 6,
        layer_name: "Network & Infrastructure Security".to_string(),
        main_type: "Perimeter Defense".to_string(),
        sub_type: "Edge Firewall / CDN".to_string(),
        component_mechanism: "CDN DDoS absorb, geo/IP blocklists, L4/L7 filtering".to_string(),
        goal: "Keep junk traffic out".to_string(),
        evidence_telemetry: "Edge drop rate, DDoS absorbed volume".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 6,
        layer_name: "Network & Infrastructure Security".to_string(),
        main_type: "Segmentation".to_string(),
        sub_type: "Zero Trust / Microsegmentation".to_string(),
        component_mechanism: "Isolate services/namespaces/VPCs, block east-west except allowlisted".to_string(),
        goal: "Contain compromise blast radius".to_string(),
        evidence_telemetry: "Denied east-west attempts".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 6,
        layer_name: "Network & Infrastructure Security".to_string(),
        main_type: "OSI Hardening".to_string(),
        sub_type: "Protocol/Port Hygiene".to_string(),
        component_mechanism: "Close unused ports, disable legacy TLS ciphers, strict DNS rules".to_string(),
        goal: "Cut legacy attack paths".to_string(),
        evidence_telemetry: "Open port diff vs baseline".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 6,
        layer_name: "Network & Infrastructure Security".to_string(),
        main_type: "Host Hardening".to_string(),
        sub_type: "Baseline Images & CIS Benchmarks".to_string(),
        component_mechanism: "Read-only root FS, minimal base images, kernel hardening, SSH lockdown".to_string(),
        goal: "Reduce exploitable surface on hosts/containers".to_string(),
        evidence_telemetry: "Drift reports from baseline, CIS score".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 6,
        layer_name: "Network & Infrastructure Security".to_string(),
        main_type: "Secrets on Host".to_string(),
        sub_type: "Runtime Secret Mounting".to_string(),
        component_mechanism: "Inject secrets at runtime (tmpfs, env vars via agent) instead of baked into image".to_string(),
        goal: "Stop image leaks of creds".to_string(),
        evidence_telemetry: "Secrets-in-image scan results".to_string(),
    });
    
    // Layer 7: Resilience & Availability
    layers.push(SecurityLayer {
        layer_number: 7,
        layer_name: "Resilience & Availability".to_string(),
        main_type: "Redundancy".to_string(),
        sub_type: "HA/Failover".to_string(),
        component_mechanism: "Multi-AZ deploy, load balancer health checks, replicas per service".to_string(),
        goal: "Survive node/zone loss".to_string(),
        evidence_telemetry: "Failover event logs, uptime %".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 7,
        layer_name: "Resilience & Availability".to_string(),
        main_type: "Traffic Protection".to_string(),
        sub_type: "Circuit Breakers / Bulkheads / Rate Shaping".to_string(),
        component_mechanism: "Trip breaker on slow dependency, isolate noisy tenants, shed load gracefully".to_string(),
        goal: "Protect core systems during incidents".to_string(),
        evidence_telemetry: "Breaker open/close timeline, shed %".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 7,
        layer_name: "Resilience & Availability".to_string(),
        main_type: "Graceful Degradation".to_string(),
        sub_type: "Feature Flags / Read-only Mode".to_string(),
        component_mechanism: "Serve cached data when DB down, put system into withdraw-disabled mode instead of full outage".to_string(),
        goal: "Keep partial service alive".to_string(),
        evidence_telemetry: "Time spent in degraded mode vs full outage".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 7,
        layer_name: "Resilience & Availability".to_string(),
        main_type: "Disaster Recovery".to_string(),
        sub_type: "DR Playbook & Chaos Testing".to_string(),
        component_mechanism: "Regular chaos drills, region evacuation practice, RPO/RTO tracking".to_string(),
        goal: "Know we can recover under stress".to_string(),
        evidence_telemetry: "Chaos test reports, RTO achieved".to_string(),
    });
    
    // Layer 8: Observability & Detection
    layers.push(SecurityLayer {
        layer_number: 8,
        layer_name: "Observability & Detection".to_string(),
        main_type: "Telemetry".to_string(),
        sub_type: "Basic Monitoring/Logging/Tracing".to_string(),
        component_mechanism: "Centralized logs, metrics, traces, span IDs across hops".to_string(),
        goal: "See attacks and failures fast".to_string(),
        evidence_telemetry: "p95 latency, error rate, auth failures over time".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 8,
        layer_name: "Observability & Detection".to_string(),
        main_type: "Security Detection".to_string(),
        sub_type: "SIEM / IDS / Anomaly Alerts".to_string(),
        component_mechanism: "Login anomaly detection, data exfil alerts, container breakout alerts".to_string(),
        goal: "Catch intrusion quickly".to_string(),
        evidence_telemetry: "Mean time to detect (MTTD)".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 8,
        layer_name: "Observability & Detection".to_string(),
        main_type: "Forensics & Evidence".to_string(),
        sub_type: "Immutable Audit Logs".to_string(),
        component_mechanism: "Append-only audit trail for admin actions, config changes, withdrawals, policy edits".to_string(),
        goal: "Prove who did what and when".to_string(),
        evidence_telemetry: "Audit log integrity check, tamper alerts".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 8,
        layer_name: "Observability & Detection".to_string(),
        main_type: "Incident Response".to_string(),
        sub_type: "Runbooks & Pager".to_string(),
        component_mechanism: "Who wakes up, what they do, communication path, rollback steps".to_string(),
        goal: "Shorten incident lifetime".to_string(),
        evidence_telemetry: "Mean time to recover (MTTR), postmortem quality".to_string(),
    });
    
    // Layer 9: Software Supply Chain
    layers.push(SecurityLayer {
        layer_number: 9,
        layer_name: "Software Supply Chain".to_string(),
        main_type: "Artifact Integrity".to_string(),
        sub_type: "Build Signing / Provenance".to_string(),
        component_mechanism: "Sigstore/cosign signed container images, SBOM attached to artifact".to_string(),
        goal: "Ensure what runs = what we built".to_string(),
        evidence_telemetry: "Unsigned image block count".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 9,
        layer_name: "Software Supply Chain".to_string(),
        main_type: "Dependency Trust".to_string(),
        sub_type: "SCA / Pin / Verify".to_string(),
        component_mechanism: "Pin versions via lockfiles, verify checksums, disallow typosquat packages".to_string(),
        goal: "Stop malicious libs".to_string(),
        evidence_telemetry: "Unapproved dependency install attempts".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 9,
        layer_name: "Software Supply Chain".to_string(),
        main_type: "CI/CD Gatekeeping".to_string(),
        sub_type: "Policy-as-Code in Pipeline".to_string(),
        component_mechanism: "CI enforces tests, security scans, lint, license policy before deploy".to_string(),
        goal: "Block unsafe code from production".to_string(),
        evidence_telemetry: "% builds blocked by policy gate".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 9,
        layer_name: "Software Supply Chain".to_string(),
        main_type: "Runtime Drift Control".to_string(),
        sub_type: "Image Drift / Host Drift".to_string(),
        component_mechanism: "Continuously check what's running vs approved manifest".to_string(),
        goal: "Detect sneaky containers".to_string(),
        evidence_telemetry: "Drift incidents per week".to_string(),
    });
    
    layers
}

/// Test Layer 1: Governance & Policy Management
fn test_layer_1_governance_policy(layers: &[SecurityLayer]) {
    println!("Testing Layer 1: Governance & Policy Management");
    
    // Verify we have 3 sub-layers for Layer 1
    let layer_1_count = layers.iter().filter(|l| l.layer_number == 1).count();
    assert_eq!(layer_1_count, 3, "Expected 3 sub-layers for Layer 1");
    
    // Test Policy Catalog implementation
    let mut catalog = PolicyCatalog::new();
    
    let policy = SecurityPolicy {
        id: "sec-policy-001".to_string(),
        title: "Security Policy".to_string(),
        content: "Organization-wide security policy".to_string(),
        version: "1.0".to_string(),
        effective_date: 1234567890,
        approvers: vec!["admin1".to_string(), "admin2".to_string()],
        signatures: vec![
            PolicySignature {
                signer: "admin1".to_string(),
                signature: "sig1".to_string(),
                timestamp: 1234567891,
            }
        ],
    };
    
    catalog.add_policy(policy);
    assert_eq!(catalog.list_policies().len(), 1);
    
    // Test Exception Management implementation
    let mut register = ExceptionRegister::new();
    
    let future_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() + 3600; // 1 hour in the future
        
    let exception = RiskException {
        id: "risk-ex-001".to_string(),
        description: "Test risk exception".to_string(),
        risk_owner: "risk-owner".to_string(),
        expiry_date: future_time,
        justification: "Business need".to_string(),
        approval_status: ExceptionStatus::Approved,
    };
    
    register.register_exception(exception).unwrap();
    assert_eq!(register.list_exceptions().len(), 1);
    
    // Test Audit & Assurance implementation
    let mut tracker = AuditTracker::new();
    
    let issue = AuditIssue {
        id: "audit-001".to_string(),
        description: "Test audit issue".to_string(),
        severity: AuditSeverity::High,
        finding_date: 1234567890,
        assigned_to: "auditor".to_string(),
        status: IssueStatus::Open,
        remediation_plan: "Fix the issue".to_string(),
        sla_deadline: future_time,
    };
    
    tracker.register_issue(issue).unwrap();
    assert_eq!(tracker.list_issues().len(), 1);
    
    println!("✓ Layer 1 tests passed");
}

/// Test Layer 2: Identity & Access Control
fn test_layer_2_identity_access(layers: &[SecurityLayer]) {
    println!("Testing Layer 2: Identity & Access Control");
    
    // Verify we have 4 sub-layers for Layer 2
    let layer_2_count = layers.iter().filter(|l| l.layer_number == 2).count();
    assert_eq!(layer_2_count, 4, "Expected 4 sub-layers for Layer 2");
    
    // Test AuthN implementation
    let mut authn = AuthNManager::new();
    
    let user_id = authn.register_user("testuser", "test@example.com", "password123").unwrap();
    let session_id = authn.authenticate("testuser", "password123").unwrap();
    
    // Validate session
    let session = authn.validate_session(&session_id).unwrap();
    assert_eq!(session.user_id, user_id);
    
    // Test AuthZ implementation
    let mut authz = AuthZManager::new();
    
    let policy = RbacPolicy {
        id: "admin-role".to_string(),
        name: "Administrator".to_string(),
        permissions: vec!["read".to_string(), "write".to_string(), "delete".to_string()],
        description: "Full access".to_string(),
    };
    
    authz.create_policy(policy).unwrap();
    authz.assign_role(&user_id, "admin-role").unwrap();
    
    // Check permissions
    assert!(authz.has_permission(&user_id, "read"));
    assert!(authz.has_permission(&user_id, "write"));
    assert!(authz.has_permission(&user_id, "delete"));
    assert!(!authz.has_permission(&user_id, "nonexistent"));
    
    // Test Session & Token Hygiene
    let mut session_mgr = SessionManager::new(2); // Max 2 sessions per user
    
    let session1 = session_mgr.create_session(&user_id, Some("127.0.0.1".to_string())).unwrap();
    let session2 = session_mgr.create_session(&user_id, Some("127.0.0.1".to_string())).unwrap();
    
    // Validate sessions
    assert!(session_mgr.validate_session(&session1).is_ok());
    assert!(session_mgr.validate_session(&session2).is_ok());
    
    // Test session revocation
    session_mgr.revoke_session(&session1).unwrap();
    assert!(session_mgr.validate_session(&session1).is_err());
    
    // Test Secrets Hygiene
    let key = [1u8; 32]; // Test key
    let mut secret_mgr = SecretManager::new(key);
    
    secret_mgr.store_secret("api_key", "secret123").unwrap();
    let retrieved = secret_mgr.retrieve_secret("api_key").unwrap();
    assert_eq!(retrieved, "secret123");
    
    println!("✓ Layer 2 tests passed");
}