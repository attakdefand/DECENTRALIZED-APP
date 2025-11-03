// Vendor Key Management Tests
// This file tests the vendor key management features

#[cfg(test)]
mod tests {
    use core::vendor::{VendorManagementSystem, vendor_management::{Vendor, VendorContact, RiskTier}, vendor_access::{VendorAccess, AccessType, AccessLevel, AccessStatus}};
    use core::vendor_key_management::{VendorKeyManager, KeyRotationRecord, KeyType, RotationStatus, HealthCheckRecord, HealthCheckType, HealthCheckResult};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_vendor_key_manager_creation() {
        let manager = VendorKeyManager::new();
        assert!(manager.key_rotation_records.is_empty());
        assert!(manager.health_check_records.is_empty());
        assert!(manager.policy_compliance.is_empty());
    }

    #[test]
    fn test_key_rotation_recording() {
        let mut manager = VendorKeyManager::new();
        
        let rotation_record = KeyRotationRecord {
            id: "rotation1".to_string(),
            vendor_id: "vendor1".to_string(),
            key_type: KeyType::APIKey,
            rotation_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            next_rotation_due: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() + 90 * 24 * 60 * 60,
            status: RotationStatus::Completed,
            compliant: true,
        };

        manager.record_key_rotation(rotation_record);
        assert_eq!(manager.key_rotation_records.len(), 1);
        assert!(manager.key_rotation_records.contains_key("vendor1"));
        assert_eq!(manager.key_rotation_records.get("vendor1").unwrap().len(), 1);
    }

    #[test]
    fn test_health_check_recording() {
        let mut manager = VendorKeyManager::new();
        
        let health_check_record = HealthCheckRecord {
            id: "health1".to_string(),
            vendor_id: "vendor1".to_string(),
            check_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            check_type: HealthCheckType::SystemStatus,
            result: HealthCheckResult::Pass,
            details: Some("System is healthy".to_string()),
        };

        manager.record_health_check(health_check_record);
        assert_eq!(manager.health_check_records.len(), 1);
        assert!(manager.health_check_records.contains_key("vendor1"));
        assert_eq!(manager.health_check_records.get("vendor1").unwrap().len(), 1);
    }

    #[test]
    fn test_key_rotation_compliance_metrics() {
        let mut manager = VendorKeyManager::new();
        
        // Add compliant rotation record
        let rotation_record1 = KeyRotationRecord {
            id: "rotation1".to_string(),
            vendor_id: "vendor1".to_string(),
            key_type: KeyType::APIKey,
            rotation_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            next_rotation_due: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() + 90 * 24 * 60 * 60,
            status: RotationStatus::Completed,
            compliant: true,
        };

        manager.record_key_rotation(rotation_record1);
        
        // Add non-compliant rotation record
        let rotation_record2 = KeyRotationRecord {
            id: "rotation2".to_string(),
            vendor_id: "vendor1".to_string(),
            key_type: KeyType::DatabaseCredential,
            rotation_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            next_rotation_due: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() + 180 * 24 * 60 * 60,
            status: RotationStatus::Overdue,
            compliant: false,
        };

        manager.record_key_rotation(rotation_record2);
        
        // Test compliance percentage
        let compliance_pct = manager.get_key_rotation_compliance_pct("vendor1");
        assert_eq!(compliance_pct, 50); // 1 compliant out of 2 total
        
        // Test with non-existent vendor
        let compliance_pct_nonexistent = manager.get_key_rotation_compliance_pct("nonexistent");
        assert_eq!(compliance_pct_nonexistent, 100); // Default to 100% for non-existent vendor
    }

    #[test]
    fn test_health_check_metrics() {
        let mut manager = VendorKeyManager::new();
        
        // Add passing health check
        let health_check1 = HealthCheckRecord {
            id: "health1".to_string(),
            vendor_id: "vendor1".to_string(),
            check_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            check_type: HealthCheckType::SystemStatus,
            result: HealthCheckResult::Pass,
            details: Some("System is healthy".to_string()),
        };

        manager.record_health_check(health_check1);
        
        // Add failing health check
        let health_check2 = HealthCheckRecord {
            id: "health2".to_string(),
            vendor_id: "vendor1".to_string(),
            check_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            check_type: HealthCheckType::KeyAccessibility,
            result: HealthCheckResult::Fail,
            details: Some("Key accessibility issue detected".to_string()),
        };

        manager.record_health_check(health_check2);
        
        // Add warning health check
        let health_check3 = HealthCheckRecord {
            id: "health3".to_string(),
            vendor_id: "vendor1".to_string(),
            check_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            check_type: HealthCheckType::Performance,
            result: HealthCheckResult::Warning,
            details: Some("Performance degradation detected".to_string()),
        };

        manager.record_health_check(health_check3);
        
        // Test health check metrics
        let pass_count = manager.get_health_checks_pass_count("vendor1");
        assert_eq!(pass_count, 1); // Only 1 pass
        
        let total_count = manager.get_health_checks_total_count("vendor1");
        assert_eq!(total_count, 3); // 3 total checks
        
        let health_pass = manager.get_health_checks_pass("vendor1");
        assert_eq!(health_pass, 0); // Not all checks passed
        
        // Test with non-existent vendor
        let pass_count_nonexistent = manager.get_health_checks_pass_count("nonexistent");
        assert_eq!(pass_count_nonexistent, 0);
        
        let total_count_nonexistent = manager.get_health_checks_total_count("nonexistent");
        assert_eq!(total_count_nonexistent, 0);
        
        let health_pass_nonexistent = manager.get_health_checks_pass("nonexistent");
        assert_eq!(health_pass_nonexistent, 1); // Default to pass for non-existent vendor
    }

    #[test]
    fn test_vendor_compliance_check() {
        let mut manager = VendorKeyManager::new();
        
        // Test compliant vendor
        let rotation_record = KeyRotationRecord {
            id: "rotation1".to_string(),
            vendor_id: "compliant_vendor".to_string(),
            key_type: KeyType::APIKey,
            rotation_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            next_rotation_due: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() + 90 * 24 * 60 * 60,
            status: RotationStatus::Completed,
            compliant: true,
        };

        manager.record_key_rotation(rotation_record);
        
        let health_check = HealthCheckRecord {
            id: "health1".to_string(),
            vendor_id: "compliant_vendor".to_string(),
            check_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            check_type: HealthCheckType::SystemStatus,
            result: HealthCheckResult::Pass,
            details: Some("System is healthy".to_string()),
        };

        manager.record_health_check(health_check);
        
        assert!(manager.is_vendor_compliant("compliant_vendor"));
        
        // Test non-compliant vendor
        let rotation_record2 = KeyRotationRecord {
            id: "rotation2".to_string(),
            vendor_id: "non_compliant_vendor".to_string(),
            key_type: KeyType::APIKey,
            rotation_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            next_rotation_due: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() + 90 * 24 * 60 * 60,
            status: RotationStatus::Overdue,
            compliant: false,
        };

        manager.record_key_rotation(rotation_record2);
        
        let health_check2 = HealthCheckRecord {
            id: "health2".to_string(),
            vendor_id: "non_compliant_vendor".to_string(),
            check_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            check_type: HealthCheckType::SystemStatus,
            result: HealthCheckResult::Fail,
            details: Some("System failure detected".to_string()),
        };

        manager.record_health_check(health_check2);
        
        assert!(!manager.is_vendor_compliant("non_compliant_vendor"));
    }

    #[test]
    fn test_vendor_key_metrics() {
        let mut manager = VendorKeyManager::new();
        
        // Add test data
        let rotation_record = KeyRotationRecord {
            id: "rotation1".to_string(),
            vendor_id: "vendor1".to_string(),
            key_type: KeyType::APIKey,
            rotation_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            next_rotation_due: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() + 90 * 24 * 60 * 60,
            status: RotationStatus::Completed,
            compliant: true,
        };

        manager.record_key_rotation(rotation_record);
        
        let health_check = HealthCheckRecord {
            id: "health1".to_string(),
            vendor_id: "vendor1".to_string(),
            check_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            check_type: HealthCheckType::SystemStatus,
            result: HealthCheckResult::Pass,
            details: Some("System is healthy".to_string()),
        };

        manager.record_health_check(health_check);
        
        // Test metrics
        let metrics = manager.get_vendor_key_metrics("vendor1");
        assert_eq!(metrics.key_rotation_compliance_pct, 100);
        assert_eq!(metrics.health_checks_pass, 1);
        assert_eq!(metrics.health_checks_pass_count, 1);
        assert_eq!(metrics.health_checks_total_count, 1);
    }

    #[test]
    fn test_policy_compliance() {
        let mut manager = VendorKeyManager::new();
        
        // Set policy compliance
        manager.set_policy_compliance("vendor1".to_string(), true);
        manager.set_policy_compliance("vendor2".to_string(), false);
        
        // Check policy compliance
        assert_eq!(manager.policy_compliance.get("vendor1"), Some(&true));
        assert_eq!(manager.policy_compliance.get("vendor2"), Some(&false));
        assert_eq!(manager.policy_compliance.get("vendor3"), None);
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
        
        // Test that the key manager is accessible
        let metrics = system.get_vendor_metrics();
        assert_eq!(metrics.vendor_key_rotation_compliance_pct, 100); // Default value
        assert_eq!(metrics.vendor_key_health_checks_pass, 1); // Default value
        
        // Add key management data
        let rotation_record = KeyRotationRecord {
            id: "rotation1".to_string(),
            vendor_id: "vendor1".to_string(),
            key_type: KeyType::APIKey,
            rotation_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            next_rotation_due: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() + 90 * 24 * 60 * 60,
            status: RotationStatus::Completed,
            compliant: true,
        };

        system.key_manager.record_key_rotation(rotation_record);
        
        // TODO: Update the VendorManagementSystem to properly integrate with VendorKeyManager
        // This would require passing the actual vendor ID to get_vendor_key_metrics
    }

    #[test]
    fn test_overall_metrics_across_vendors() {
        let mut manager = VendorKeyManager::new();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        manager.record_key_rotation(KeyRotationRecord {
            id: "rotation_a".to_string(),
            vendor_id: "vendor_a".to_string(),
            key_type: KeyType::APIKey,
            rotation_date: now,
            next_rotation_due: now + 90 * 24 * 60 * 60,
            status: RotationStatus::Completed,
            compliant: true,
        });

        manager.record_key_rotation(KeyRotationRecord {
            id: "rotation_b".to_string(),
            vendor_id: "vendor_b".to_string(),
            key_type: KeyType::DatabaseCredential,
            rotation_date: now,
            next_rotation_due: now + 30 * 24 * 60 * 60,
            status: RotationStatus::Overdue,
            compliant: false,
        });

        manager.record_health_check(HealthCheckRecord {
            id: "health_a".to_string(),
            vendor_id: "vendor_a".to_string(),
            check_date: now,
            check_type: HealthCheckType::SystemStatus,
            result: HealthCheckResult::Pass,
            details: None,
        });

        manager.record_health_check(HealthCheckRecord {
            id: "health_b".to_string(),
            vendor_id: "vendor_b".to_string(),
            check_date: now,
            check_type: HealthCheckType::SystemStatus,
            result: HealthCheckResult::Fail,
            details: Some("Key outage".to_string()),
        });

        let metrics = manager.get_overall_metrics();
        assert_eq!(metrics.key_rotation_compliance_pct, 50);
        assert_eq!(metrics.health_checks_pass, 0);
        assert_eq!(metrics.health_checks_total_count, 2);
    }
}
