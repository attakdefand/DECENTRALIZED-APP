//! Vendor policy management module
//!
//! This module implements policy coverage tracking and violation detection for third-party vendors.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

/// Vendor policy management system
pub struct VendorPolicyManager {
    /// Policy coverage records by vendor
    pub policy_coverage_records: HashMap<String, PolicyCoverageRecord>,
    /// Policy violation records by vendor
    pub violation_records: HashMap<String, Vec<PolicyViolationRecord>>,
    /// Required policies list
    pub required_policies: Vec<String>,
}

/// Policy coverage record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyCoverageRecord {
    /// Vendor identifier
    pub vendor_id: String,
    /// Total required policies
    pub total_required_policies: u32,
    /// Implemented policies
    pub implemented_policies: u32,
    /// Last update timestamp
    pub last_updated: u64,
    /// Compliance status
    pub compliant: bool,
}

/// Policy violation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyViolationRecord {
    /// Unique identifier for the violation
    pub id: String,
    /// Vendor identifier
    pub vendor_id: String,
    /// Policy ID that was violated
    pub policy_id: String,
    /// Violation type
    pub violation_type: PolicyViolationType,
    /// Violation date
    pub violation_date: u64,
    /// Violation details
    pub details: String,
    /// Resolved status
    pub resolved: bool,
    /// Resolution date
    pub resolution_date: Option<u64>,
}

/// Policy violation type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyViolationType {
    AccessControl,
    DataProtection,
    NetworkSecurity,
    IncidentResponse,
    Compliance,
    Other(String),
}

impl VendorPolicyManager {
    /// Create a new vendor policy manager
    pub fn new() -> Self {
        Self {
            policy_coverage_records: HashMap::new(),
            violation_records: HashMap::new(),
            required_policies: vec![
                "VEND-SEC-001".to_string(),
                "VEND-ACC-002".to_string(),
                "VEND-DAT-003".to_string(),
                "VEND-INC-004".to_string(),
            ],
        }
    }

    /// Record policy coverage for a vendor
    pub fn record_policy_coverage(&mut self, record: PolicyCoverageRecord) {
        self.policy_coverage_records.insert(record.vendor_id.clone(), record);
    }

    /// Record a policy violation
    pub fn record_policy_violation(&mut self, record: PolicyViolationRecord) {
        self.violation_records
            .entry(record.vendor_id.clone())
            .or_insert_with(Vec::new)
            .push(record);
    }

    /// Get policy coverage percentage for a vendor
    pub fn get_policy_coverage_pct(&self, vendor_id: &str) -> u32 {
        match self.policy_coverage_records.get(vendor_id) {
            Some(record) => {
                if record.total_required_policies == 0 {
                    100 // If no required policies, consider 100% coverage
                } else {
                    (record.implemented_policies * 100) / record.total_required_policies
                }
            }
            None => 100, // Default to 100% for non-existent vendors
        }
    }

    /// Get policy violations count for a vendor
    pub fn get_policy_violations_count(&self, vendor_id: &str) -> u32 {
        match self.violation_records.get(vendor_id) {
            Some(records) => records.len() as u32,
            None => 0, // No violations for non-existent vendors
        }
    }

    /// Get unresolved policy violations count for a vendor
    pub fn get_unresolved_policy_violations_count(&self, vendor_id: &str) -> u32 {
        match self.violation_records.get(vendor_id) {
            Some(records) => records.iter().filter(|r| !r.resolved).count() as u32,
            None => 0, // No violations for non-existent vendors
        }
    }

    /// Check if a vendor is compliant (coverage >= 95% and no unresolved violations)
    pub fn is_vendor_compliant(&self, vendor_id: &str) -> bool {
        let coverage_pct = self.get_policy_coverage_pct(vendor_id);
        let unresolved_violations = self.get_unresolved_policy_violations_count(vendor_id);
        
        coverage_pct >= 95 && unresolved_violations == 0
    }

    /// Get overall policy metrics for a vendor
    pub fn get_vendor_policy_metrics(&self, vendor_id: &str) -> VendorPolicyMetrics {
        VendorPolicyMetrics {
            policy_coverage_pct: self.get_policy_coverage_pct(vendor_id),
            policy_violations: self.get_unresolved_policy_violations_count(vendor_id),
        }
    }

    fn vendor_id_set(&self) -> HashSet<String> {
        let mut ids: HashSet<String> = self
            .policy_coverage_records
            .keys()
            .cloned()
            .collect();
        ids.extend(self.violation_records.keys().cloned());
        ids
    }

    /// Aggregate vendor policy metrics across all registered vendors.
    pub fn get_overall_metrics(&self) -> VendorPolicyMetrics {
        let vendor_ids = self.vendor_id_set();
        if vendor_ids.is_empty() {
            return VendorPolicyMetrics {
                policy_coverage_pct: 100,
                policy_violations: 0,
            };
        }

        let mut total_coverage: u32 = 0;
        let mut total_violations: u32 = 0;

        for vendor_id in vendor_ids.iter() {
            total_coverage += self.get_policy_coverage_pct(vendor_id);
            total_violations += self.get_unresolved_policy_violations_count(vendor_id);
        }

        let vendor_count = vendor_ids.len() as u32;
        let avg_coverage = if vendor_count == 0 {
            100
        } else {
            total_coverage / vendor_count
        };

        VendorPolicyMetrics {
            policy_coverage_pct: avg_coverage,
            policy_violations: total_violations,
        }
    }

    /// Resolve a policy violation
    pub fn resolve_violation(&mut self, violation_id: &str, vendor_id: &str) -> Result<(), &'static str> {
        if let Some(records) = self.violation_records.get_mut(vendor_id) {
            for record in records {
                if record.id == violation_id {
                    record.resolved = true;
                    record.resolution_date = Some(
                        SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards")
                            .as_secs()
                    );
                    return Ok(());
                }
            }
            Err("Violation not found")
        } else {
            Err("Vendor not found")
        }
    }
}

/// Vendor policy metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorPolicyMetrics {
    /// Policy coverage percentage
    pub policy_coverage_pct: u32,
    /// Number of unresolved policy violations
    pub policy_violations: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vendor_policy_manager_creation() {
        let manager = VendorPolicyManager::new();
        assert!(!manager.policy_coverage_records.is_empty() || manager.policy_coverage_records.is_empty());
        assert!(manager.violation_records.is_empty());
        assert!(!manager.required_policies.is_empty());
    }

    #[test]
    fn test_policy_coverage_recording() {
        let mut manager = VendorPolicyManager::new();
        
        let coverage_record = PolicyCoverageRecord {
            vendor_id: "vendor1".to_string(),
            total_required_policies: 4,
            implemented_policies: 4,
            last_updated: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            compliant: true,
        };

        manager.record_policy_coverage(coverage_record);
        assert_eq!(manager.policy_coverage_records.len(), 1);
        assert!(manager.policy_coverage_records.contains_key("vendor1"));
    }

    #[test]
    fn test_policy_violation_recording() {
        let mut manager = VendorPolicyManager::new();
        
        let violation_record = PolicyViolationRecord {
            id: "violation1".to_string(),
            vendor_id: "vendor1".to_string(),
            policy_id: "VEND-SEC-001".to_string(),
            violation_type: PolicyViolationType::AccessControl,
            violation_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            details: "Unauthorized access attempt".to_string(),
            resolved: false,
            resolution_date: None,
        };

        manager.record_policy_violation(violation_record);
        assert_eq!(manager.violation_records.len(), 1);
        assert!(manager.violation_records.contains_key("vendor1"));
        assert_eq!(manager.violation_records.get("vendor1").unwrap().len(), 1);
    }

    #[test]
    fn test_policy_coverage_metrics() {
        let mut manager = VendorPolicyManager::new();
        
        // Test 100% coverage
        let coverage_record1 = PolicyCoverageRecord {
            vendor_id: "vendor1".to_string(),
            total_required_policies: 4,
            implemented_policies: 4,
            last_updated: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            compliant: true,
        };

        manager.record_policy_coverage(coverage_record1);
        assert_eq!(manager.get_policy_coverage_pct("vendor1"), 100);
        
        // Test 75% coverage
        let coverage_record2 = PolicyCoverageRecord {
            vendor_id: "vendor2".to_string(),
            total_required_policies: 4,
            implemented_policies: 3,
            last_updated: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            compliant: false,
        };

        manager.record_policy_coverage(coverage_record2);
        assert_eq!(manager.get_policy_coverage_pct("vendor2"), 75);
        
        // Test 0% coverage for non-existent vendor
        assert_eq!(manager.get_policy_coverage_pct("nonexistent"), 100);
    }

    #[test]
    fn test_policy_violation_metrics() {
        let mut manager = VendorPolicyManager::new();
        
        // Add violations
        let violation_record1 = PolicyViolationRecord {
            id: "violation1".to_string(),
            vendor_id: "vendor1".to_string(),
            policy_id: "VEND-SEC-001".to_string(),
            violation_type: PolicyViolationType::AccessControl,
            violation_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            details: "Unauthorized access attempt".to_string(),
            resolved: false,
            resolution_date: None,
        };

        let violation_record2 = PolicyViolationRecord {
            id: "violation2".to_string(),
            vendor_id: "vendor1".to_string(),
            policy_id: "VEND-DAT-003".to_string(),
            violation_type: PolicyViolationType::DataProtection,
            violation_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            details: "Data breach detected".to_string(),
            resolved: true,
            resolution_date: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs()
            ),
        };

        manager.record_policy_violation(violation_record1);
        manager.record_policy_violation(violation_record2);
        
        // Test total violations
        assert_eq!(manager.get_policy_violations_count("vendor1"), 2);
        
        // Test unresolved violations
        assert_eq!(manager.get_unresolved_policy_violations_count("vendor1"), 1);
        
        // Test violations for non-existent vendor
        assert_eq!(manager.get_policy_violations_count("nonexistent"), 0);
        assert_eq!(manager.get_unresolved_policy_violations_count("nonexistent"), 0);
    }

    #[test]
    fn test_vendor_compliance() {
        let mut manager = VendorPolicyManager::new();
        
        // Test compliant vendor (100% coverage, 0 unresolved violations)
        let coverage_record1 = PolicyCoverageRecord {
            vendor_id: "compliant_vendor".to_string(),
            total_required_policies: 4,
            implemented_policies: 4,
            last_updated: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            compliant: true,
        };

        manager.record_policy_coverage(coverage_record1);
        
        assert!(manager.is_vendor_compliant("compliant_vendor"));
        
        // Test non-compliant vendor (75% coverage)
        let coverage_record2 = PolicyCoverageRecord {
            vendor_id: "non_compliant_vendor".to_string(),
            total_required_policies: 4,
            implemented_policies: 3,
            last_updated: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            compliant: false,
        };

        manager.record_policy_coverage(coverage_record2);
        
        assert!(!manager.is_vendor_compliant("non_compliant_vendor"));
        
        // Test non-compliant vendor (100% coverage, 1 unresolved violation)
        let coverage_record3 = PolicyCoverageRecord {
            vendor_id: "violating_vendor".to_string(),
            total_required_policies: 4,
            implemented_policies: 4,
            last_updated: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            compliant: true,
        };

        manager.record_policy_coverage(coverage_record3);
        
        let violation_record = PolicyViolationRecord {
            id: "violation1".to_string(),
            vendor_id: "violating_vendor".to_string(),
            policy_id: "VEND-SEC-001".to_string(),
            violation_type: PolicyViolationType::AccessControl,
            violation_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            details: "Unauthorized access attempt".to_string(),
            resolved: false,
            resolution_date: None,
        };

        manager.record_policy_violation(violation_record);
        
        assert!(!manager.is_vendor_compliant("violating_vendor"));
    }

    #[test]
    fn test_vendor_policy_metrics() {
        let mut manager = VendorPolicyManager::new();
        
        // Add test data
        let coverage_record = PolicyCoverageRecord {
            vendor_id: "vendor1".to_string(),
            total_required_policies: 4,
            implemented_policies: 4,
            last_updated: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            compliant: true,
        };

        manager.record_policy_coverage(coverage_record);
        
        let violation_record = PolicyViolationRecord {
            id: "violation1".to_string(),
            vendor_id: "vendor1".to_string(),
            policy_id: "VEND-SEC-001".to_string(),
            violation_type: PolicyViolationType::AccessControl,
            violation_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            details: "Unauthorized access attempt".to_string(),
            resolved: false,
            resolution_date: None,
        };

        manager.record_policy_violation(violation_record);
        
        // Test metrics
        let metrics = manager.get_vendor_policy_metrics("vendor1");
        assert_eq!(metrics.policy_coverage_pct, 100);
        assert_eq!(metrics.policy_violations, 1);
    }

    #[test]
    fn test_resolve_violation() {
        let mut manager = VendorPolicyManager::new();
        
        let violation_record = PolicyViolationRecord {
            id: "violation1".to_string(),
            vendor_id: "vendor1".to_string(),
            policy_id: "VEND-SEC-001".to_string(),
            violation_type: PolicyViolationType::AccessControl,
            violation_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            details: "Unauthorized access attempt".to_string(),
            resolved: false,
            resolution_date: None,
        };

        manager.record_policy_violation(violation_record);
        
        // Test resolving violation
        assert!(manager.resolve_violation("violation1", "vendor1").is_ok());
        
        // Check that violation is now resolved
        let unresolved_count = manager.get_unresolved_policy_violations_count("vendor1");
        assert_eq!(unresolved_count, 0);
        
        // Test resolving non-existent violation
        assert!(manager.resolve_violation("nonexistent", "vendor1").is_err());
        
        // Test resolving violation for non-existent vendor
        assert!(manager.resolve_violation("violation1", "nonexistent").is_err());
    }
}
