//! Security Layers Simulation
//!
//! This binary runs simulations of all security layers to verify their functionality.

use security_layers::{
    governance_policy::*,
    identity_access::*,
    types::SecurityLayer,
};
use std::collections::HashMap;

fn main() {
    println!("Running Security Layers Simulation...");
    
    // Test governance and policy management
    test_governance_policy();
    
    // Test identity and access control
    test_identity_access();
    
    println!("All security layer simulations completed successfully!");
}

fn test_governance_policy() {
    println!("\n--- Testing Governance & Policy Management ---");
    
    // Test policy catalog
    let mut catalog = PolicyCatalog::new();
    
    let policy = SecurityPolicy {
        id: "sec-001".to_string(),
        title: "Security Policy".to_string(),
        content: "Security policy content".to_string(),
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
    println!("✓ Policy catalog created and policy added");
    
    // Test exception register
    let mut register = ExceptionRegister::new();
    
    let future_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() + 3600; // 1 hour in the future
        
    let exception = RiskException {
        id: "ex-001".to_string(),
        description: "Test exception".to_string(),
        risk_owner: "risk-owner".to_string(),
        expiry_date: future_time,
        justification: "Business need".to_string(),
        approval_status: ExceptionStatus::Approved,
    };
    
    register.register_exception(exception).unwrap();
    println!("✓ Exception register created and exception added");
    
    // Test audit tracker
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
    println!("✓ Audit tracker created and issue added");
}

fn test_identity_access() {
    println!("\n--- Testing Identity & Access Control ---");
    
    // Test authentication manager
    let mut authn = AuthNManager::new();
    
    let user_id = authn.register_user("testuser", "test@example.com", "password123").unwrap();
    println!("✓ User registered with ID: {}", user_id);
    
    let session_id = authn.authenticate("testuser", "password123").unwrap();
    println!("✓ User authenticated with session ID: {}", session_id);
    
    // Test authorization manager
    let mut authz = AuthZManager::new();
    
    let policy = RbacPolicy {
        id: "admin".to_string(),
        name: "Administrator".to_string(),
        permissions: vec!["read".to_string(), "write".to_string(), "delete".to_string()],
        description: "Full access".to_string(),
    };
    
    authz.create_policy(policy).unwrap();
    authz.assign_role(&user_id, "admin").unwrap();
    println!("✓ Role-based access control policy created and assigned");
    
    // Test session manager
    let mut session_mgr = SessionManager::new(2); // Max 2 sessions per user
    
    // Create sessions
    let session1 = session_mgr.create_session(&user_id, Some("192.168.1.100".to_string())).unwrap();
    let _session2 = session_mgr.create_session(&user_id, Some("192.168.1.101".to_string())).unwrap();

    println!("✓ Session manager created and sessions established");
    
    // Test secret manager
    let key = [1u8; 32]; // Test key
    let mut secret_mgr = SecretManager::new(key);
    
    secret_mgr.store_secret("api_key", "secret123").unwrap();
    let retrieved = secret_mgr.retrieve_secret("api_key").unwrap();
    assert_eq!(retrieved, "secret123");
    println!("✓ Secret manager created and secret stored/retrieved");
}