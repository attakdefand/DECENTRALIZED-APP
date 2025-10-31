//! Specific Security Layer Features Validation Tests
//!
//! This module contains tests that specifically validate the features from lines 4-5 
//! of the web3_protection_layers.csv file:
//! 1. Layer 1, Exception Management, Risk Acceptance Workflow
//! 2. Layer 1, Audit & Assurance, Internal/External Audit Tracking

use security_layers::governance_policy::*;

/// Test that validates the specific features from lines 4-5 of web3_protection_layers.csv
#[test]
fn test_specific_security_layer_features() {
    println!("Testing specific security layer features from lines 4-5 of web3_protection_layers.csv...");
    
    // Test Layer 1, Exception Management, Risk Acceptance Workflow
    // "Exception register, owner+expiry, tracked in repo / ticket"
    // "Force accountability for any deviation"
    // "Open exceptions with expiry and sign-off"
    test_exception_management_features();
    
    // Test Layer 1, Audit & Assurance, Internal/External Audit Tracking
    // "Security audit issues labeled in tracker, remediation SLAs"
    // "Close gaps found by audit / pen test"
    // "% audit findings closed on time, PR links"
    test_audit_and_assurance_features();
    
    println!("All specific security layer features validated successfully!");
}

/// Test Layer 1, Exception Management, Risk Acceptance Workflow
/// Component/Mechanism: "Exception register, owner+expiry, tracked in repo / ticket"
/// Goal: "Force accountability for any deviation"
/// Evidence/Telemetry: "Open exceptions with expiry and sign-off"
fn test_exception_management_features() {
    println!("Testing Layer 1, Exception Management, Risk Acceptance Workflow...");
    
    // Create exception register
    let mut exception_register = ExceptionRegister::new();
    
    // Test exception registration with owner and expiry (tracked in repo / ticket)
    let future_expiry = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() + 86400; // 24 hours from now
        
    let risk_exception = RiskException {
        id: "exception-2023-001".to_string(),
        description: "Temporary exception for development environment access".to_string(),
        risk_owner: "dev-team-lead".to_string(), // Owner specified
        expiry_date: future_expiry, // Expiry date specified
        justification: "Required for urgent bug fix deployment".to_string(),
        approval_status: ExceptionStatus::Approved,
    };
    
    // Register exception (tracked in repo / ticket)
    assert!(exception_register.register_exception(risk_exception).is_ok());
    assert_eq!(exception_register.list_exceptions().len(), 1);
    
    // Test that exception has owner and expiry
    let registered_exception = exception_register.get_exception("exception-2023-001").unwrap();
    assert_eq!(registered_exception.risk_owner, "dev-team-lead");
    assert_eq!(registered_exception.expiry_date, future_expiry);
    
    // Test active exceptions (open exceptions with expiry)
    let active_exceptions = exception_register.get_active_exceptions();
    assert_eq!(active_exceptions.len(), 1);
    
    // Test exception expiry checking
    assert!(!exception_register.is_expired("exception-2023-001"));
    
    // Test Risk Acceptance Workflow
    let mut workflow = RiskAcceptanceWorkflow::new();
    let risk_exception2 = RiskException {
        id: "exception-2023-002".to_string(),
        description: "Another temporary exception".to_string(),
        risk_owner: "security-team".to_string(),
        expiry_date: future_expiry,
        justification: "Required for security testing".to_string(),
        approval_status: ExceptionStatus::Pending, // Pending approval
    };
    
    // Submit exception for approval (workflow)
    assert!(workflow.submit_exception(risk_exception2).is_ok());
    
    // Approve exception (sign-off)
    assert!(workflow.approve_exception("exception-2023-002", "approver").is_ok());
    
    // Verify approval status (accountability)
    // Since we can't directly access the exception, we'll verify through statistics
    let stats = workflow.get_exception_statistics();
    assert_eq!(stats.total_count, 1);
    assert_eq!(stats.approved_count, 1);
    
    // Test exceptions by owner (accountability)
    let owner_exceptions = workflow.get_exceptions_by_owner("security-team");
    assert_eq!(owner_exceptions.len(), 1);
    // Verify the exception was properly updated by checking its approval status
    assert_eq!(owner_exceptions[0].approval_status, ExceptionStatus::Approved);
    
    // Test expiring exceptions (telemetry)
    let expiring_exceptions = workflow.get_expiring_exceptions();
    assert!(!expiring_exceptions.is_empty()); // Should have exceptions expiring within 7 days
    
    println!("✓ Exception Management features validated");
}

/// Test Layer 1, Audit & Assurance, Internal/External Audit Tracking
/// Component/Mechanism: "Security audit issues labeled in tracker, remediation SLAs"
/// Goal: "Close gaps found by audit / pen test"
/// Evidence/Telemetry: "% audit findings closed on time, PR links"
fn test_audit_and_assurance_features() {
    println!("Testing Layer 1, Audit & Assurance, Internal/External Audit Tracking...");
    
    // Create audit tracker
    let mut audit_tracker = AuditTracker::new();
    
    // Test security audit issues labeled in tracker with remediation SLAs
    let future_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() + 86400; // 24 hours from now (SLA deadline)
        
    let audit_issue = AuditIssue {
        id: "audit-2023-001".to_string(),
        description: "Missing input validation in user profile update endpoint".to_string(),
        severity: AuditSeverity::High, // Labeled by severity
        finding_date: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        assigned_to: "backend-team".to_string(), // Assigned for remediation
        status: IssueStatus::Open,
        remediation_plan: "Implement strict input validation and sanitization".to_string(),
        sla_deadline: future_time, // SLA deadline for remediation
    };
    
    // Register audit issue (labeled in tracker)
    assert!(audit_tracker.register_issue(audit_issue).is_ok());
    assert_eq!(audit_tracker.list_issues().len(), 1);
    
    // Test issue retrieval and tracking
    let retrieved_issue = audit_tracker.get_issue("audit-2023-001").unwrap();
    assert_eq!(retrieved_issue.description, "Missing input validation in user profile update endpoint");
    assert_eq!(retrieved_issue.severity, AuditSeverity::High);
    assert_eq!(retrieved_issue.assigned_to, "backend-team");
    assert_eq!(retrieved_issue.sla_deadline, future_time);
    
    // Test open issues tracking (gaps found by audit)
    let open_issues = audit_tracker.get_open_issues();
    assert_eq!(open_issues.len(), 1);
    
    // Test issues by severity (labeled in tracker)
    let high_severity_issues = audit_tracker.get_issues_by_severity(AuditSeverity::High);
    assert_eq!(high_severity_issues.len(), 1);
    
    // Test issues by assignee (remediation tracking)
    let backend_team_issues = audit_tracker.get_issues_by_assignee("backend-team");
    assert_eq!(backend_team_issues.len(), 1);
    
    // Test high severity issues (pen test gaps)
    let critical_issues = audit_tracker.get_high_severity_issues();
    assert_eq!(critical_issues.len(), 1);
    
    // Test issue resolution (close gaps)
    assert!(audit_tracker.resolve_issue("audit-2023-001").is_ok());
    
    // Test audit statistics (evidence/telemetry)
    let stats = audit_tracker.get_audit_statistics();
    assert_eq!(stats.total_count, 1);
    assert_eq!(stats.resolved_count, 1);
    assert_eq!(stats.open_count, 0);
    
    // Test overdue issues tracking (closed on time)
    let overdue_issues = audit_tracker.get_overdue_issues();
    assert_eq!(overdue_issues.len(), 0); // No overdue issues since we resolved it on time
    
    // Add another issue to test percentage metrics
    let audit_issue2 = AuditIssue {
        id: "audit-2023-002".to_string(),
        description: "Weak password policy".to_string(),
        severity: AuditSeverity::Critical,
        finding_date: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        assigned_to: "security-team".to_string(),
        status: IssueStatus::Open,
        remediation_plan: "Implement password complexity requirements".to_string(),
        sla_deadline: future_time,
    };
    
    assert!(audit_tracker.register_issue(audit_issue2).is_ok());
    
    // Test updated statistics
    let updated_stats = audit_tracker.get_audit_statistics();
    assert_eq!(updated_stats.total_count, 2);
    assert_eq!(updated_stats.resolved_count, 1);
    assert_eq!(updated_stats.open_count, 1);
    
    // Calculate percentage of closed findings (telemetry)
    let closure_percentage = (updated_stats.resolved_count as f64 / updated_stats.total_count as f64) * 100.0;
    assert_eq!(closure_percentage, 50.0); // 1 out of 2 issues resolved
    
    println!("✓ Audit & Assurance features validated");
}