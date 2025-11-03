//! Tests for vendor access controls and RBAC features

use core::vendor::{VendorManagementSystem, vendor_management::{Vendor, VendorContact, RiskTier}, vendor_access::{VendorAccess, AccessType, AccessLevel, AccessStatus, AccessReview, ReviewOutcome}};
use std::time::{SystemTime, UNIX_EPOCH};

/// Test vendor access review metrics tracking
#[test]
fn test_vendor_access_review_metrics() {
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
    
    // Test initial metrics
    let metrics = system.get_vendor_metrics();
    assert_eq!(metrics.vendor_access_review_completion_pct, 0); // No reviews yet
    assert_eq!(metrics.vendor_sod_violations, 0); // No violations yet
    
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
    
    // Test updated metrics
    let metrics = system.get_vendor_metrics();
    assert_eq!(metrics.vendor_access_review_completion_pct, 100); // 1 access, 1 review
    assert_eq!(metrics.vendor_sod_violations, 0); // No violations
}

/// Test vendor SoD violation detection
#[test]
fn test_vendor_sod_violation_detection() {
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
    
    // Grant database access to vendor
    let db_access = VendorAccess {
        id: "db_access".to_string(),
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
    
    system.access_manager.grant_access(db_access);
    
    // Grant filesystem access to the same vendor (creates SoD violation)
    let fs_access = VendorAccess {
        id: "fs_access".to_string(),
        vendor_id: "vendor1".to_string(),
        resource: "FileSystem".to_string(),
        access_type: AccessType::FileSystem,
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
        justification: "File access".to_string(),
        granted_by: "Security Team".to_string(),
        status: AccessStatus::Active,
    };
    
    system.access_manager.grant_access(fs_access);
    
    // Test SoD violation detection
    let metrics = system.get_vendor_metrics();
    assert!(metrics.vendor_sod_violations > 0); // Should detect SoD violation
}

/// Test vendor access review completion percentage calculation
#[test]
fn test_vendor_access_review_completion_percentage() {
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
    
    // Grant multiple accesses to vendor
    let access1 = VendorAccess {
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
    
    system.access_manager.grant_access(access1);
    
    let access2 = VendorAccess {
        id: "access2".to_string(),
        vendor_id: "vendor1".to_string(),
        resource: "API".to_string(),
        access_type: AccessType::API,
        access_level: AccessLevel::Write,
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
        justification: "API access".to_string(),
        granted_by: "Security Team".to_string(),
        status: AccessStatus::Active,
    };
    
    system.access_manager.grant_access(access2);
    
    // Test initial completion percentage (0% - no reviews)
    let metrics = system.get_vendor_metrics();
    assert_eq!(metrics.vendor_access_review_completion_pct, 0);
    
    // Create review for first access
    let review1 = AccessReview {
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
    
    system.access_manager.create_access_review(review1);
    
    // Test completion percentage (50% - 1 out of 2 accesses reviewed)
    let metrics = system.get_vendor_metrics();
    assert_eq!(metrics.vendor_access_review_completion_pct, 50);
    
    // Create review for second access
    let review2 = AccessReview {
        id: "review2".to_string(),
        access_id: "access2".to_string(),
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
    
    system.access_manager.create_access_review(review2);
    
    // Test completion percentage (100% - all accesses reviewed)
    let metrics = system.get_vendor_metrics();
    assert_eq!(metrics.vendor_access_review_completion_pct, 100);
}

/// Test vendor access controls with no accesses
#[test]
fn test_vendor_access_controls_no_accesses() {
    let system = VendorManagementSystem::new();
    
    // Test metrics with no accesses
    let metrics = system.get_vendor_metrics();
    assert_eq!(metrics.vendor_access_review_completion_pct, 100); // Consider 100% complete when no accesses
    assert_eq!(metrics.vendor_sod_violations, 0); // No violations when no accesses
}

/// Test vendor access controls with overdue reviews
#[test]
fn test_vendor_access_controls_overdue_reviews() {
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
    
    // Grant access with past review date (overdue)
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
            .as_secs() - 86400, // 1 day ago (overdue)
        justification: "Data analysis".to_string(),
        granted_by: "Security Team".to_string(),
        status: AccessStatus::Active,
    };
    
    system.access_manager.grant_access(access);
    
    // Test overdue reviews count
    let overdue_count = system.access_manager.get_overdue_vendor_reviews_count();
    assert_eq!(overdue_count, 1);
    
    // Test overall metrics
    let metrics = system.get_vendor_metrics();
    assert_eq!(metrics.overdue_reviews, 1);
}