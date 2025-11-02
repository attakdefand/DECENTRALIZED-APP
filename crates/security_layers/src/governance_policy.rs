//! Governance & Policy Management
//!
//! This module implements security layers 1: Governance & Policy Management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Policy catalog containing all organizational security policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyCatalog {
    policies: HashMap<String, SecurityPolicy>,
    last_updated: u64,
    /// Audit trail of all policy changes
    audit_log: Vec<String>,
}

/// Security policy document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    pub id: String,
    pub title: String,
    pub content: String,
    pub version: String,
    pub effective_date: u64,
    pub approvers: Vec<String>,
    pub signatures: Vec<PolicySignature>,
}

/// Policy signature for audit trail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicySignature {
    pub signer: String,
    pub signature: String,
    pub timestamp: u64,
}

/// Commit signature verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitSignatureVerification {
    pub commit_hash: String,
    pub is_signed: bool,
    pub signer: Option<String>,
    pub verification_timestamp: u64,
}

/// Repository governance settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryGovernanceSettings {
    pub require_signed_commits: bool,
    pub min_signed_commits_percentage: f64,
    pub require_ci_checks: bool,
    pub allowed_signers: Vec<String>,
    pub block_on_open_high_risks: bool,
}

/// Policy audit event for tracking changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyAuditEvent {
    pub policy_id: String,
    pub action: PolicyAction,
    pub timestamp: u64,
    pub actor: String,
    pub details: String,
}

/// Policy action types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyAction {
    Created,
    Updated,
    Deleted,
    Signed,
    Revoked,
}

/// Exception register for tracking risk acceptances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExceptionRegister {
    exceptions: HashMap<String, RiskException>,
}

/// Risk exception record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskException {
    pub id: String,
    pub description: String,
    pub risk_owner: String,
    pub expiry_date: u64,
    pub justification: String,
    pub approval_status: ExceptionStatus,
}

/// Exception approval status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExceptionStatus {
    Pending,
    Approved,
    Rejected,
    Expired,
}

/// Exception statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExceptionStatistics {
    pub total_count: usize,
    pub approved_count: usize,
    pub pending_count: usize,
    pub rejected_count: usize,
    pub expired_count: usize,
}

/// Risk acceptance workflow manager
#[derive(Debug)]
pub struct RiskAcceptanceWorkflow {
    register: ExceptionRegister,
}

/// Audit and assurance tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditTracker {
    issues: HashMap<String, AuditIssue>,
}

/// Audit issue from internal/external audits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditIssue {
    pub id: String,
    pub description: String,
    pub severity: AuditSeverity,
    pub finding_date: u64,
    pub assigned_to: String,
    pub status: IssueStatus,
    pub remediation_plan: String,
    pub sla_deadline: u64,
}

/// Audit severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Issue status tracking
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IssueStatus {
    Open,
    InProgress,
    Resolved,
    Closed,
    Overdue,
}

/// Audit statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditStatistics {
    pub total_count: usize,
    pub open_count: usize,
    pub in_progress_count: usize,
    pub resolved_count: usize,
    pub closed_count: usize,
    pub overdue_count: usize,
}

impl PolicyCatalog {
    /// Create a new policy catalog
    pub fn new() -> Self {
        Self {
            policies: HashMap::new(),
            last_updated: 0,
            audit_log: Vec::new(),
        }
    }

    /// Verify commit signatures for governance compliance
    pub fn verify_commit_signatures(&self, commits: &[CommitSignatureVerification], settings: &RepositoryGovernanceSettings) -> Result<bool, String> {
        if commits.is_empty() {
            return Ok(true); // No commits to verify
        }

        let mut signed_count = 0;
        for commit in commits {
            if commit.is_signed {
                // Check if signer is in allowed list (if specified)
                if !settings.allowed_signers.is_empty() {
                    if let Some(signer) = &commit.signer {
                        if settings.allowed_signers.contains(signer) {
                            signed_count += 1;
                        }
                    }
                } else {
                    // No specific allowed signers, count all signed commits
                    signed_count += 1;
                }
            }
        }

        let signed_percentage = (signed_count as f64 / commits.len() as f64) * 100.0;
        
        // Check if signed commit percentage meets requirements
        if settings.require_signed_commits && signed_percentage < settings.min_signed_commits_percentage {
            return Ok(false);
        }

        Ok(true)
    }

    /// Check if merge should be blocked based on governance rules
    pub fn should_block_merge(&self, commits: &[CommitSignatureVerification], settings: &RepositoryGovernanceSettings, ci_checks_pass: bool) -> bool {
        // Block if CI checks are required but failing
        if settings.require_ci_checks && !ci_checks_pass {
            return true;
        }

        // Check commit signature requirements
        match self.verify_commit_signatures(commits, settings) {
            Ok(valid) => !valid,
            Err(_) => true, // Block on verification errors
        }
    }

    /// Add a policy to the catalog
    pub fn add_policy(&mut self, policy: SecurityPolicy) {
        let policy_id = policy.id.clone();
        self.policies.insert(policy_id.clone(), policy);
        self.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.audit_log.push(format!("Policy {} added", policy_id));
    }

    /// Update a policy in the catalog
    pub fn update_policy(&mut self, policy_id: &str, updated_policy: SecurityPolicy) -> Result<(), String> {
        if !self.policies.contains_key(policy_id) {
            return Err("Policy not found".to_string());
        }
        self.policies.insert(policy_id.to_string(), updated_policy);
        self.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.audit_log.push(format!("Policy {} updated", policy_id));
        Ok(())
    }

    /// Remove a policy from the catalog
    pub fn remove_policy(&mut self, policy_id: &str) -> Result<(), String> {
        if !self.policies.contains_key(policy_id) {
            return Err("Policy not found".to_string());
        }
        self.policies.remove(policy_id);
        self.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.audit_log.push(format!("Policy {} removed", policy_id));
        Ok(())
    }

    /// Get a policy by ID
    pub fn get_policy(&self, policy_id: &str) -> Option<&SecurityPolicy> {
        self.policies.get(policy_id)
    }

    /// List all policies
    pub fn list_policies(&self) -> Vec<&SecurityPolicy> {
        self.policies.values().collect()
    }

    /// Check if a policy is signed by all required approvers
    pub fn is_policy_signed(&self, policy_id: &str, required_approvers: &[String]) -> bool {
        let policy = match self.get_policy(policy_id) {
            Some(p) => p,
            None => return false,
        };

        // Check if all required approvers have signed
        for approver in required_approvers {
            let mut found = false;
            for signature in &policy.signatures {
                if &signature.signer == approver {
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }
        }
        true
    }

    /// Sign an existing policy with an additional signature
    pub fn sign_policy(&mut self, policy_id: &str, signature: PolicySignature) -> Result<(), String> {
        let policy = self.policies.get_mut(policy_id).ok_or("Policy not found")?;
        
        // Check if the signer has already signed
        for existing_signature in &policy.signatures {
            if existing_signature.signer == signature.signer {
                return Err("Policy already signed by this signer".to_string());
            }
        }
        
        // Check if the signer is an approver
        if !policy.approvers.contains(&signature.signer) {
            return Err("Unauthorized signer".to_string());
        }
        
        policy.signatures.push(signature);
        self.audit_log.push(format!("Policy {} signed", policy_id));
        Ok(())
    }

    /// Get audit log
    pub fn get_audit_log(&self) -> &Vec<String> {
        &self.audit_log
    }
}

impl Default for PolicyCatalog {
    fn default() -> Self {
        Self::new()
    }
}

impl ExceptionRegister {
    /// Create a new exception register
    pub fn new() -> Self {
        Self {
            exceptions: HashMap::new(),
        }
    }

    /// Register a new exception
    pub fn register_exception(&mut self, exception: RiskException) -> Result<(), String> {
        self.exceptions.insert(exception.id.clone(), exception);
        Ok(())
    }

    /// List all exceptions
    pub fn list_exceptions(&self) -> Vec<RiskException> {
        self.exceptions.values().cloned().collect()
    }

    /// Get an exception by ID
    pub fn get_exception(&self, id: &str) -> Option<&RiskException> {
        self.exceptions.get(id)
    }

    /// Get a mutable reference to an exception by ID
    pub fn get_exception_mut(&mut self, id: &str) -> Option<&mut RiskException> {
        self.exceptions.get_mut(id)
    }

    /// Approve an exception
    pub fn approve_exception(&mut self, exception_id: &str) -> Result<(), String> {
        let exception = self.get_exception_mut(exception_id).ok_or("Exception not found")?;
        exception.approval_status = ExceptionStatus::Approved;
        Ok(())
    }

    /// Get active (non-expired) exceptions
    pub fn get_active_exceptions(&self) -> Vec<RiskException> {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.exceptions
            .values()
            .filter(|e| e.expiry_date > current_time && e.approval_status != ExceptionStatus::Expired)
            .cloned()
            .collect()
    }

    /// Check if an exception is expired
    pub fn is_expired(&self, id: &str) -> bool {
        if let Some(exception) = self.exceptions.get(id) {
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            exception.expiry_date <= current_time
        } else {
            true // If not found, consider it expired
        }
    }

    /// Get exceptions by owner
    pub fn get_exceptions_by_owner(&self, owner: &str) -> Vec<RiskException> {
        self.exceptions
            .values()
            .filter(|e| e.risk_owner == owner)
            .cloned()
            .collect()
    }

    /// Get expiring exceptions (within 7 days)
    pub fn get_expiring_exceptions(&self) -> Vec<RiskException> {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let week_from_now = current_time + (7 * 24 * 60 * 60); // 7 days in seconds
        self.exceptions
            .values()
            .filter(|e| e.expiry_date <= week_from_now && e.expiry_date > current_time)
            .cloned()
            .collect()
    }

    /// Get exception statistics
    pub fn get_statistics(&self) -> ExceptionStatistics {
        let mut total_count = 0;
        let mut approved_count = 0;
        let mut pending_count = 0;
        let mut rejected_count = 0;
        let mut expired_count = 0;
        for exception in self.exceptions.values() {
            total_count += 1;
            match exception.approval_status {
                ExceptionStatus::Approved => approved_count += 1,
                ExceptionStatus::Pending => pending_count += 1,
                ExceptionStatus::Rejected => rejected_count += 1,
                ExceptionStatus::Expired => expired_count += 1,
            }
        }
        ExceptionStatistics {
            total_count,
            approved_count,
            pending_count,
            rejected_count,
            expired_count,
        }
    }
}

impl Default for ExceptionRegister {
    fn default() -> Self {
        Self::new()
    }
}

impl RiskAcceptanceWorkflow {
    /// Create a new risk acceptance workflow
    pub fn new() -> Self {
        Self {
            register: ExceptionRegister::new(),
        }
    }

    /// Submit an exception for approval
    pub fn submit_exception(&mut self, exception: RiskException) -> Result<(), String> {
        self.register.register_exception(exception)
    }

    /// Approve an exception
    pub fn approve_exception(&mut self, exception_id: &str, _approver: &str) -> Result<(), String> {
        self.register.approve_exception(exception_id)
    }

    /// Get exception statistics
    pub fn get_exception_statistics(&self) -> ExceptionStatistics {
        self.register.get_statistics()
    }

    /// Get exceptions by owner
    pub fn get_exceptions_by_owner(&self, owner: &str) -> Vec<RiskException> {
        self.register.get_exceptions_by_owner(owner)
    }

    /// Get expiring exceptions
    pub fn get_expiring_exceptions(&self) -> Vec<RiskException> {
        self.register.get_expiring_exceptions()
    }
}

impl Default for RiskAcceptanceWorkflow {
    fn default() -> Self {
        Self::new()
    }
}

impl AuditTracker {
    /// Create a new audit tracker
    pub fn new() -> Self {
        Self {
            issues: HashMap::new(),
        }
    }

    /// Register a new audit issue
    pub fn register_issue(&mut self, issue: AuditIssue) -> Result<(), String> {
        self.issues.insert(issue.id.clone(), issue);
        Ok(())
    }

    /// List all issues
    pub fn list_issues(&self) -> Vec<AuditIssue> {
        self.issues.values().cloned().collect()
    }

    /// Get an issue by ID
    pub fn get_issue(&self, id: &str) -> Option<&AuditIssue> {
        self.issues.get(id)
    }

    /// Get open issues
    pub fn get_open_issues(&self) -> Vec<AuditIssue> {
        self.issues
            .values()
            .filter(|i| i.status == IssueStatus::Open)
            .cloned()
            .collect()
    }

    /// Get issues by severity
    pub fn get_issues_by_severity(&self, severity: AuditSeverity) -> Vec<AuditIssue> {
        self.issues
            .values()
            .filter(|i| i.severity == severity)
            .cloned()
            .collect()
    }

    /// Get issues by assignee
    pub fn get_issues_by_assignee(&self, assignee: &str) -> Vec<AuditIssue> {
        self.issues
            .values()
            .filter(|i| i.assigned_to == assignee)
            .cloned()
            .collect()
    }

    /// Get high severity issues
    pub fn get_high_severity_issues(&self) -> Vec<AuditIssue> {
        self.issues
            .values()
            .filter(|i| i.severity == AuditSeverity::High || i.severity == AuditSeverity::Critical)
            .cloned()
            .collect()
    }

    /// Resolve an issue
    pub fn resolve_issue(&mut self, issue_id: &str) -> Result<(), String> {
        if let Some(issue) = self.issues.get_mut(issue_id) {
            issue.status = IssueStatus::Resolved;
            Ok(())
        } else {
            Err("Issue not found".to_string())
        }
    }

    /// Update issue status
    pub fn update_issue_status(&mut self, issue_id: &str, status: IssueStatus) -> Result<(), String> {
        if let Some(issue) = self.issues.get_mut(issue_id) {
            issue.status = status;
            Ok(())
        } else {
            Err("Issue not found".to_string())
        }
    }

    /// Close an issue
    pub fn close_issue(&mut self, issue_id: &str) -> Result<(), String> {
        if let Some(issue) = self.issues.get_mut(issue_id) {
            issue.status = IssueStatus::Closed;
            Ok(())
        } else {
            Err("Issue not found".to_string())
        }
    }

    /// Get overdue issues
    pub fn get_overdue_issues(&self) -> Vec<AuditIssue> {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.issues
            .values()
            .filter(|i| i.sla_deadline < current_time && i.status != IssueStatus::Closed && i.status != IssueStatus::Resolved)
            .cloned()
            .collect()
    }

    /// Get audit statistics
    pub fn get_audit_statistics(&self) -> AuditStatistics {
        let mut total_count = 0;
        let mut open_count = 0;
        let mut in_progress_count = 0;
        let mut resolved_count = 0;
        let mut closed_count = 0;
        let mut overdue_count = 0;
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        for issue in self.issues.values() {
            total_count += 1;
            match issue.status {
                IssueStatus::Open => open_count += 1,
                IssueStatus::InProgress => in_progress_count += 1,
                IssueStatus::Resolved => resolved_count += 1,
                IssueStatus::Closed => closed_count += 1,
                IssueStatus::Overdue => overdue_count += 1,
            }
            // Check if issue is overdue
            if issue.sla_deadline < current_time && issue.status != IssueStatus::Closed && issue.status != IssueStatus::Resolved {
                overdue_count += 1;
            }
        }
        AuditStatistics {
            total_count,
            open_count,
            in_progress_count,
            resolved_count,
            closed_count,
            overdue_count,
        }
    }
}

impl Default for AuditTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_catalog() {
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
        
        // Test policy retrieval
        let retrieved = catalog.get_policy("sec-001").unwrap();
        assert_eq!(retrieved.title, "Security Policy");
        
        // Test policy signing check
        assert!(!catalog.is_policy_signed("sec-001", &["admin1".to_string(), "admin2".to_string()]));
        assert!(catalog.is_policy_signed("sec-001", &["admin1".to_string()]));
        
        // Test audit log
        assert!(!catalog.get_audit_log().is_empty());
    }
    
    #[test]
    fn test_policy_catalog_update_and_remove() {
        let mut catalog = PolicyCatalog::new();
        
        let policy = SecurityPolicy {
            id: "sec-002".to_string(),
            title: "Initial Policy".to_string(),
            content: "Initial content".to_string(),
            version: "1.0".to_string(),
            effective_date: 1234567890,
            approvers: vec!["admin1".to_string()],
            signatures: vec![],
        };
        
        catalog.add_policy(policy);
        
        // Test policy update
        let updated_policy = SecurityPolicy {
            id: "sec-002".to_string(),
            title: "Updated Policy".to_string(),
            content: "Updated content".to_string(),
            version: "1.1".to_string(),
            effective_date: 1234567890,
            approvers: vec!["admin1".to_string()],
            signatures: vec![],
        };
        
        assert!(catalog.update_policy("sec-002", updated_policy).is_ok());
        
        let retrieved = catalog.get_policy("sec-002").unwrap();
        assert_eq!(retrieved.title, "Updated Policy");
        
        // Test policy removal
        assert!(catalog.remove_policy("sec-002").is_ok());
        assert!(catalog.get_policy("sec-002").is_none());
    }
    
    #[test]
    fn test_policy_signing() {
        let mut catalog = PolicyCatalog::new();
        
        let policy = SecurityPolicy {
            id: "sec-003".to_string(),
            title: "Signable Policy".to_string(),
            content: "Content to be signed".to_string(),
            version: "1.0".to_string(),
            effective_date: 1234567890,
            approvers: vec!["admin1".to_string(), "admin2".to_string()],
            signatures: vec![],
        };
        
        catalog.add_policy(policy);
        
        // Test valid signature
        let signature = PolicySignature {
            signer: "admin1".to_string(),
            signature: "valid_signature".to_string(),
            timestamp: 1234567891,
        };
        
        assert!(catalog.sign_policy("sec-003", signature).is_ok());
        
        // Test duplicate signature rejection
        let duplicate_signature = PolicySignature {
            signer: "admin1".to_string(),
            signature: "another_signature".to_string(),
            timestamp: 1234567892,
        };
        
        assert!(catalog.sign_policy("sec-003", duplicate_signature).is_err());
        
        // Test unauthorized signer rejection
        let unauthorized_signature = PolicySignature {
            signer: "unauthorized_user".to_string(),
            signature: "unauthorized_signature".to_string(),
            timestamp: 1234567893,
        };
        
        assert!(catalog.sign_policy("sec-003", unauthorized_signature).is_err());
    }

    #[test]
    fn test_exception_register() {
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
        
        // Test exception retrieval
        let retrieved = register.get_exception("ex-001").unwrap();
        assert_eq!(retrieved.description, "Test exception");
        
        // Test active exceptions
        let active = register.get_active_exceptions();
        assert_eq!(active.len(), 1);
    }

    #[test]
    fn test_risk_acceptance_workflow() {
        let mut workflow = RiskAcceptanceWorkflow::new();
        
        let future_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() + 3600; // 1 hour in the future
            
        let exception = RiskException {
            id: "ex-002".to_string(),
            description: "Test exception".to_string(),
            risk_owner: "risk-owner".to_string(),
            expiry_date: future_time,
            justification: "Business need".to_string(),
            approval_status: ExceptionStatus::Pending,
        };
        
        workflow.submit_exception(exception).unwrap();
        
        // Test approval
        workflow.approve_exception("ex-002", "approver").unwrap();
        
        let approved = workflow.register.get_exception("ex-002").unwrap();
        assert_eq!(approved.approval_status, ExceptionStatus::Approved);
    }
    
    #[test]
    fn test_risk_acceptance_workflow_statistics() {
        let mut workflow = RiskAcceptanceWorkflow::new();
        
        let future_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() + 3600; // 1 hour in the future
            
        // Add approved exception
        let approved_exception = RiskException {
            id: "ex-003".to_string(),
            description: "Approved exception".to_string(),
            risk_owner: "owner1".to_string(),
            expiry_date: future_time,
            justification: "Approved justification".to_string(),
            approval_status: ExceptionStatus::Approved,
        };
        
        workflow.submit_exception(approved_exception).unwrap();
        
        // Add pending exception
        let pending_exception = RiskException {
            id: "ex-004".to_string(),
            description: "Pending exception".to_string(),
            risk_owner: "owner2".to_string(),
            expiry_date: future_time,
            justification: "Pending justification".to_string(),
            approval_status: ExceptionStatus::Pending,
        };
        
        workflow.submit_exception(pending_exception).unwrap();
        
        // Test statistics
        let stats = workflow.get_exception_statistics();
        assert_eq!(stats.total_count, 2);
        assert_eq!(stats.approved_count, 1);
        assert_eq!(stats.pending_count, 1);
        assert_eq!(stats.rejected_count, 0);
        assert_eq!(stats.expired_count, 0);
        
        // Test exceptions by owner
        let owner_exceptions = workflow.get_exceptions_by_owner("owner1");
        assert_eq!(owner_exceptions.len(), 1);
    }
    
    #[test]
    fn test_risk_exception_expiry() {
        let mut workflow = RiskAcceptanceWorkflow::new();
        
        let past_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - 3600; // 1 hour in the past
            
        let expired_exception = RiskException {
            id: "ex-005".to_string(),
            description: "Expired exception".to_string(),
            risk_owner: "owner3".to_string(),
            expiry_date: past_time,
            justification: "Expired justification".to_string(),
            approval_status: ExceptionStatus::Approved,
        };
        
        workflow.submit_exception(expired_exception).unwrap();
        
        // Test that exception is expired
        assert!(workflow.register.is_expired("ex-005"));
    }

    #[test]
    fn test_audit_tracker() {
        let mut tracker = AuditTracker::new();
        
        let future_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() + 3600; // 1 hour in the future
            
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
        
        // Test issue retrieval
        let retrieved = tracker.get_issue("audit-001").unwrap();
        assert_eq!(retrieved.description, "Test audit issue");
        
        // Test status update
        tracker.update_issue_status("audit-001", IssueStatus::InProgress).unwrap();
        
        let updated = tracker.get_issue("audit-001").unwrap();
        assert_eq!(updated.status, IssueStatus::InProgress);
    }
    
    #[test]
    fn test_audit_tracker_statistics_and_filtering() {
        let mut tracker = AuditTracker::new();
        
        let future_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() + 3600; // 1 hour in the future
            
        // Add open issue
        let open_issue = AuditIssue {
            id: "audit-002".to_string(),
            description: "Open issue".to_string(),
            severity: AuditSeverity::Medium,
            finding_date: 1234567890,
            assigned_to: "auditor1".to_string(),
            status: IssueStatus::Open,
            remediation_plan: "Fix soon".to_string(),
            sla_deadline: future_time,
        };
        
        tracker.register_issue(open_issue).unwrap();
        
        // Add high severity issue
        let high_severity_issue = AuditIssue {
            id: "audit-003".to_string(),
            description: "Critical issue".to_string(),
            severity: AuditSeverity::Critical,
            finding_date: 1234567891,
            assigned_to: "auditor2".to_string(),
            status: IssueStatus::Open,
            remediation_plan: "Fix immediately".to_string(),
            sla_deadline: future_time,
        };
        
        tracker.register_issue(high_severity_issue).unwrap();
        
        // Add resolved issue
        let resolved_issue = AuditIssue {
            id: "audit-004".to_string(),
            description: "Resolved issue".to_string(),
            severity: AuditSeverity::Low,
            finding_date: 1234567892,
            assigned_to: "auditor1".to_string(),
            status: IssueStatus::Resolved,
            remediation_plan: "Already fixed".to_string(),
            sla_deadline: future_time,
        };
        
        tracker.register_issue(resolved_issue).unwrap();
        
        // Test statistics
        let stats = tracker.get_audit_statistics();
        assert_eq!(stats.total_count, 3);
        assert_eq!(stats.open_count, 2);
        assert_eq!(stats.resolved_count, 1);
        assert_eq!(stats.overdue_count, 0);
        
        // Test filtering by severity
        let high_severity_issues = tracker.get_high_severity_issues();
        assert_eq!(high_severity_issues.len(), 1);
        
        // Test filtering by assignee
        let auditor1_issues = tracker.get_issues_by_assignee("auditor1");
        assert_eq!(auditor1_issues.len(), 2);
        
        // Test issue resolution
        assert!(tracker.resolve_issue("audit-002").is_ok());
        let resolved = tracker.get_issue("audit-002").unwrap();
        assert_eq!(resolved.status, IssueStatus::Resolved);
        
        // Test issue closure
        assert!(tracker.close_issue("audit-003").is_ok());
        let closed = tracker.get_issue("audit-003").unwrap();
        assert_eq!(closed.status, IssueStatus::Closed);
    }

    #[test]
    fn test_commit_signature_verification() {
        let catalog = PolicyCatalog::new();
        
        let commits = vec![
            CommitSignatureVerification {
                commit_hash: "abc123".to_string(),
                is_signed: true,
                signer: Some("authorized-user".to_string()),
                verification_timestamp: 1234567890,
            },
            CommitSignatureVerification {
                commit_hash: "def456".to_string(),
                is_signed: true,
                signer: Some("authorized-user".to_string()),
                verification_timestamp: 1234567891,
            },
            CommitSignatureVerification {
                commit_hash: "ghi789".to_string(),
                is_signed: false,
                signer: None,
                verification_timestamp: 1234567892,
            },
        ];
        
        let settings = RepositoryGovernanceSettings {
            require_signed_commits: true,
            min_signed_commits_percentage: 100.0,
            require_ci_checks: true,
            allowed_signers: vec!["authorized-user".to_string()],
            block_on_open_high_risks: true,
        };
        
        // Test that verification fails with insufficient signed commits
        assert!(!catalog.verify_commit_signatures(&commits, &settings).unwrap());
        
        // Test with relaxed requirements
        let relaxed_settings = RepositoryGovernanceSettings {
            require_signed_commits: true,
            min_signed_commits_percentage: 60.0,
            require_ci_checks: true,
            allowed_signers: vec!["authorized-user".to_string()],
            block_on_open_high_risks: true,
        };
        
        // Should pass with 66% signed commits
        assert!(catalog.verify_commit_signatures(&commits, &relaxed_settings).unwrap());
        
        // Test merge blocking
        assert!(catalog.should_block_merge(&commits, &settings, true));
        assert!(!catalog.should_block_merge(&commits, &relaxed_settings, true));
        
        // Test CI check failure
        assert!(catalog.should_block_merge(&commits, &relaxed_settings, false));
    }
}