//! Integration Tests for Security Layers
//!
//! This module contains integration tests that demonstrate how all security layers
//! work together in a realistic scenario.

use security_layers::{
    governance_policy::*,
    identity_access::*,
};

/// Integration test demonstrating a complete security workflow
#[test]
fn test_complete_security_workflow() {
    println!("Running complete security workflow integration test...");
    
    // Step 1: Governance & Policy Management
    println!("Step 1: Setting up governance and policy management");
    
    // Create policy catalog and add security policy
    let mut policy_catalog = PolicyCatalog::new();
    
    let security_policy = SecurityPolicy {
        id: "org-security-policy-v1".to_string(),
        title: "Organization Security Policy".to_string(),
        content: "All employees must follow security best practices".to_string(),
        version: "1.0".to_string(),
        effective_date: 1234567890,
        approvers: vec!["cto@example.com".to_string(), "ciso@example.com".to_string()],
        signatures: vec![
            PolicySignature {
                signer: "cto@example.com".to_string(),
                signature: "cto-signature-123".to_string(),
                timestamp: 1234567891,
            }
        ],
    };
    
    policy_catalog.add_policy(security_policy);
    assert_eq!(policy_catalog.list_policies().len(), 1);
    println!("✓ Security policy added to catalog");
    
    // Register a risk exception
    let mut exception_register = ExceptionRegister::new();
    
    let future_expiry = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() + 86400; // 24 hours from now
        
    let risk_exception = RiskException {
        id: "exception-2023-001".to_string(),
        description: "Temporary exception for development environment access".to_string(),
        risk_owner: "dev-team-lead".to_string(),
        expiry_date: future_expiry,
        justification: "Required for urgent bug fix deployment".to_string(),
        approval_status: ExceptionStatus::Approved,
    };
    
    exception_register.register_exception(risk_exception).unwrap();
    assert_eq!(exception_register.list_exceptions().len(), 1);
    println!("✓ Risk exception registered");
    
    // Track an audit issue
    let mut audit_tracker = AuditTracker::new();
    
    let audit_issue = AuditIssue {
        id: "audit-2023-001".to_string(),
        description: "Missing input validation in user profile update endpoint".to_string(),
        severity: AuditSeverity::High,
        finding_date: 1234567892,
        assigned_to: "backend-team".to_string(),
        status: IssueStatus::Open,
        remediation_plan: "Implement strict input validation and sanitization".to_string(),
        sla_deadline: future_expiry,
    };
    
    audit_tracker.register_issue(audit_issue).unwrap();
    assert_eq!(audit_tracker.list_issues().len(), 1);
    assert_eq!(audit_tracker.get_open_issues().len(), 1);
    println!("✓ Audit issue tracked");
    
    // Step 2: Identity & Access Control
    println!("Step 2: Setting up identity and access control");
    
    // Register a new user
    let mut authn_manager = AuthNManager::new();
    
    let user_id = authn_manager.register_user(
        "alice.developer", 
        "alice.developer@example.com", 
        "securePassword123!"
    ).unwrap();
    
    println!("✓ User registered with ID: {}", user_id);
    
    // Authenticate the user
    let session_id = authn_manager.authenticate(
        "alice.developer", 
        "securePassword123!"
    ).unwrap();
    
    println!("✓ User authenticated with session: {}", session_id);
    
    // Validate the session
    let session = authn_manager.validate_session(&session_id).unwrap();
    assert_eq!(session.user_id, user_id);
    println!("✓ Session validated successfully");
    
    // Set up authorization
    let mut authz_manager = AuthZManager::new();
    
    // Create roles
    let developer_policy = RbacPolicy {
        id: "developer".to_string(),
        name: "Developer".to_string(),
        permissions: vec![
            "code.read".to_string(),
            "code.write".to_string(),
            "test.execute".to_string(),
        ],
        description: "Standard developer permissions".to_string(),
    };
    
    let admin_policy = RbacPolicy {
        id: "admin".to_string(),
        name: "Administrator".to_string(),
        permissions: vec![
            "code.read".to_string(),
            "code.write".to_string(),
            "code.delete".to_string(),
            "user.manage".to_string(),
            "system.configure".to_string(),
        ],
        description: "Full administrative permissions".to_string(),
    };
    
    authz_manager.create_policy(developer_policy).unwrap();
    authz_manager.create_policy(admin_policy).unwrap();
    println!("✓ RBAC policies created");
    
    // Assign roles to user
    authz_manager.assign_role(&user_id, "developer").unwrap();
    println!("✓ Developer role assigned to user");
    
    // Check permissions
    assert!(authz_manager.has_permission(&user_id, "code.read"));
    assert!(authz_manager.has_permission(&user_id, "code.write"));
    assert!(authz_manager.has_permission(&user_id, "test.execute"));
    assert!(!authz_manager.has_permission(&user_id, "user.manage"));
    println!("✓ Permission checks validated");
    
    // Manage sessions
    let mut session_manager = SessionManager::new(3); // Max 3 sessions per user
    
    let session1 = session_manager.create_session(
        &user_id, 
        Some("192.168.1.100".to_string())
    ).unwrap();
    
    let session2 = session_manager.create_session(
        &user_id, 
        Some("192.168.1.101".to_string())
    ).unwrap();
    
    // Validate sessions
    assert!(session_manager.validate_session(&session1).is_ok());
    assert!(session_manager.validate_session(&session2).is_ok());
    println!("✓ Multiple sessions created and validated");
    
    // Manage secrets
    let encryption_key = [1u8; 32]; // In practice, this should be a securely generated key
    let mut secret_manager = SecretManager::new(encryption_key);
    
    // Store API credentials
    secret_manager.store_secret(
        "github-api-token", 
        "ghp_secret_token_for_github_api"
    ).unwrap();
    
    secret_manager.store_secret(
        "database-password", 
        "super_secret_database_password"
    ).unwrap();
    
    // Retrieve secrets
    let github_token = secret_manager.retrieve_secret("github-api-token").unwrap();
    let db_password = secret_manager.retrieve_secret("database-password").unwrap();
    
    assert_eq!(github_token, "ghp_secret_token_for_github_api");
    assert_eq!(db_password, "super_secret_database_password");
    println!("✓ Secrets securely stored and retrieved");
    
    // Step 3: Demonstrate security layer integration
    println!("Step 3: Demonstrating security layer integration");
    
    // Check that our user can only access what they're authorized to
    let can_read_code = authz_manager.has_permission(&user_id, "code.read");
    let can_manage_users = authz_manager.has_permission(&user_id, "user.manage");
    
    assert!(can_read_code);
    assert!(!can_manage_users);
    println!("✓ Access control properly enforced");
    
    // Verify that our risk exception is still valid
    assert!(!exception_register.is_expired("exception-2023-001"));
    println!("✓ Risk exception is still valid");
    
    // Check that our audit issue is still open
    assert_eq!(audit_tracker.get_open_issues().len(), 1);
    println!("✓ Audit issue tracking working correctly");
    
    // Verify session management
    let user_sessions = session_manager.get_user_sessions(&user_id);
    assert_eq!(user_sessions.len(), 2);
    println!("✓ Session management working correctly");
    
    // Revoke a session
    session_manager.revoke_session(&session1).unwrap();
    assert!(session_manager.validate_session(&session1).is_err());
    assert!(session_manager.validate_session(&session2).is_ok());
    println!("✓ Session revocation working correctly");
    
    println!("Complete security workflow integration test passed!");
}