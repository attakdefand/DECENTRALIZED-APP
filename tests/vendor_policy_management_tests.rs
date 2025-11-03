// Vendor Policy Management Tests
// This file tests the vendor policy management features

#[cfg(test)]
mod tests {
    use core::vendor::{VendorManagementSystem, vendor_management::{Vendor, VendorContact, RiskTier}, vendor_access::{VendorAccess, AccessType, AccessLevel, AccessStatus}};
    use core::vendor_policy_management::{VendorPolicyManager, PolicyCoverageRecord, PolicyViolationRecord, PolicyViolationType};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_vendor_policy_manager_creation() {
        let manager = VendorPolicyManager::new();
        assert!(manager.policy_coverage_records.is_empty());
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
        assert_eq!(manager.policy_coverage_records.get("vendor1").unwrap().total_required_policies, 4);
        assert_eq!(manager.policy_coverage_records.get("vendor1").unwrap().implemented_policies, 4);
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
        assert_eq!(manager.violation_records.get("vendor1").unwrap()[0].policy_id, "VEND-SEC-001");
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
        
        // Test 0% coverage for non-existent vendor (defaults to 100%)
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
        
        // Test non-compliant vendor (75% coverage < 95%)
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

    #[test]
    fn test_integration_with_vendor_management_system() {
        let mut system = VendorManagementSystem::new();
        
        // Register a vendor
        let contact = VendorContact {
            name: "John Doe".to_string(),
            email: "john@vendor.com".to_string(),
            phone: "+1234567890".to_string(),
            address: "123 Vendor St, City, Country".to_string(),
        };
        
        let vendor = Vendor {
            id: "vendor1".to_string(),
            name: "Test Vendor".to_string(),
            contact,
            services: vec!["Cloud Storage".to_string()],
            onboarding_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            contract_expiration: None,
            risk_tier: RiskTier::Medium,
            risk_score: 50,
            last_assessment: None,
            assessment_frequency: 90,
            next_assessment_due: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() + 90 * 24 * 60 * 60,
            caIQ_mappings: vec![],
            questionnaires: vec![],
        };
        
        system.risk_manager.register_vendor(vendor);
        
        // Grant access to vendor
        let access = VendorAccess {
            id: "access1".to_string(),
            vendor_id: "vendor1".to_string(),
            resource: "Database".to_string(),
            access_type: AccessType::Database,
            access_level: AccessLevel::Read,
            granted_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            expiration_date: None,
            review_frequency: 90,
            next_review_due: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() + 90 * 24 * 60 * 60,
            justification: "Data analysis".to_string(),
            granted_by: "Security Team".to_string(),
            status: AccessStatus::Active,
        };
        
        system.access_manager.grant_access(access);
        
        // Test that the policy manager is accessible
        let metrics = system.get_vendor_metrics();
        assert_eq!(metrics.vendor_policy_coverage_pct, 100); // Default value
        assert_eq!(metrics.vendor_policy_violations, 0); // Default value
        
        // Add policy management data
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

        system.policy_manager.record_policy_coverage(coverage_record);
        
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

        system.policy_manager.record_policy_violation(violation_record);
        
        // Test updated metrics
        let updated_metrics = system.get_vendor_metrics();
        assert_eq!(updated_metrics.vendor_policy_coverage_pct, 100);
        assert_eq!(updated_metrics.vendor_policy_violations, 1);
    }

    #[test]
    fn test_overall_policy_metrics_across_vendors() {
        let mut manager = VendorPolicyManager::new();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        manager.record_policy_coverage(PolicyCoverageRecord {
            vendor_id: "vendor_a".to_string(),
            total_required_policies: 4,
            implemented_policies: 4,
            last_updated: now,
            compliant: true,
        });

        manager.record_policy_coverage(PolicyCoverageRecord {
            vendor_id: "vendor_b".to_string(),
            total_required_policies: 4,
            implemented_policies: 2,
            last_updated: now,
            compliant: false,
        });

        manager.record_policy_violation(PolicyViolationRecord {
            id: "violation_b".to_string(),
            vendor_id: "vendor_b".to_string(),
            policy_id: "VEND-ACC-002".to_string(),
            violation_type: PolicyViolationType::AccessControl,
            violation_date: now,
            details: "Missing MFA entry".to_string(),
            resolved: false,
            resolution_date: None,
        });

        let metrics = manager.get_overall_metrics();
        assert_eq!(metrics.policy_coverage_pct, 75);
        assert_eq!(metrics.policy_violations, 1);
    }
}
