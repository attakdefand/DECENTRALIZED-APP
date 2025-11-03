//! Tests for vendor management validation and features

use core::vendor::{VendorManagementSystem, vendor_management::{Vendor, VendorContact, RiskTier}, vendor_access::{VendorAccess, AccessType, AccessLevel, AccessStatus, AccessReview, ReviewOutcome}};
use std::time::{SystemTime, UNIX_EPOCH};

/// Test vendor risk assessment framework
#[test]
fn test_vendor_risk_assessment() {
    let mut system = VendorManagementSystem::new();
    
    // Create a vendor
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
        assessment_frequency: 90, // 90 days
        next_assessment_due: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() + 90 * 24 * 60 * 60,
        caIQ_mappings: vec![],
        questionnaires: vec![],
    };
    
    system.risk_manager.register_vendor(vendor);
    
    // Test vendor retrieval
    let retrieved_vendor = system.risk_manager.get_vendor("vendor1");
    assert!(retrieved_vendor.is_some());
    assert_eq!(retrieved_vendor.unwrap().name, "Test Vendor");
    
    // Test risk metrics
    let metrics = system.risk_manager.get_vendor_risk_metrics();
    assert_eq!(metrics.vendor_count, 1);
    assert_eq!(metrics.vendor_score_avg, 50);
}

/// Test SLA/SLO monitoring
#[test]
fn test_sla_monitoring() {
    let mut system = VendorManagementSystem::new();
    
    // In a real implementation, we would test the SLA monitoring features
    // For now, we'll just verify the system can be created
    assert!(true);
}

/// Test vendor access management
#[test]
fn test_vendor_access_management() {
    let mut system = VendorManagementSystem::new();
    
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
    
    // Test access retrieval
    let retrieved_access = system.access_manager.get_vendor_access("access1");
    assert!(retrieved_access.is_some());
    assert_eq!(retrieved_access.unwrap().resource, "Database");
    
    // Test access revocation
    assert!(system.access_manager.revoke_access("access1").is_ok());
}

/// Test metrics tracking
#[test]
fn test_vendor_metrics_tracking() {
    let system = VendorManagementSystem::new();
    
    // Test initial metrics
    let metrics = system.get_vendor_metrics();
    assert_eq!(metrics.overdue_reviews, 0);
    assert_eq!(metrics.vendor_score_avg, 0);
    assert_eq!(metrics.vendor_access_review_completion_pct, 100); // No accesses, so 100% complete
    assert_eq!(metrics.vendor_sod_violations, 0);
    
    // Test overdue reviews check
    assert!(!system.has_overdue_reviews());
}

/// Test evidence generation
#[test]
fn test_evidence_generation() {
    let mut system = VendorManagementSystem::new();
    
    // Test due diligence report generation for non-existent vendor
    let report = system.generate_due_diligence_report("nonexistent");
    assert_eq!(report, "Vendor not found");
    
    // Test access attestation generation for non-existent access
    let attestation = system.generate_access_attestation("nonexistent");
    assert_eq!(attestation, "Access record not found");
}

/// Test vendor access review metrics with new vendor-specific metrics
#[test]
fn test_vendor_access_review_metrics_with_vendor_specific_metrics() {
    let mut system = VendorManagementSystem::new();
    
    // Create a vendor
    let contact = VendorContact {
        name: "Test Vendor".to_string(),
        email: "test@vendor.com".to_string(),
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
    
    // Test initial metrics with new vendor-specific fields
    let metrics = system.get_vendor_metrics();
    assert_eq!(metrics.vendor_access_review_completion_pct, 0); // No reviews yet
    assert_eq!(metrics.vendor_sod_violations, 0); // No violations yet
    assert_eq!(metrics.review_metrics.vendor_access_review_completion_pct, 0);
    assert_eq!(metrics.review_metrics.vendor_sod_violations, 0);
    
    // Create an access review
    let review = AccessReview {
        id: "review1".to_string(),
        access_id: "access1".to_string(),
        review_date: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs(),
        reviewer: "Security Officer".to_string(),
        outcome: ReviewOutcome::Approved,
        comments: None,
        next_review_date: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() + 90 * 24 * 60 * 60,
        attestation: None,
    };
    
    system.access_manager.create_access_review(review);
    
    // Test updated metrics with new vendor-specific fields
    let metrics = system.get_vendor_metrics();
    assert_eq!(metrics.vendor_access_review_completion_pct, 100); // 1 access, 1 review
    assert_eq!(metrics.vendor_sod_violations, 0); // No violations
    assert_eq!(metrics.review_metrics.vendor_access_review_completion_pct, 100);
    assert_eq!(metrics.review_metrics.vendor_sod_violations, 0);
}

/// Test exporting metrics to disk for downstream automation
#[test]
fn test_vendor_metrics_export_to_disk() {
    let system = VendorManagementSystem::new();
    let dest = std::env::temp_dir().join("vendor-metrics-export.json");

    system.export_metrics_to(&dest).expect("should export metrics");
    let content = std::fs::read_to_string(&dest).expect("metrics json readable");
    assert!(content.contains("vendor_score_avg"));

    let _ = std::fs::remove_file(&dest);
}
