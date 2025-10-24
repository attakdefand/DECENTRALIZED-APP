//! Integration tests for legal compliance functionality

use core::types::Address;
use legal::{
    AgeRestriction, GeoRestriction, KycRecord, KycStatus, LegalCompliance, LegalConfig,
    PrivacyPolicy, SanctionsConfig, TermsOfService,
};

/// Integration test for the complete legal compliance workflow
#[test]
fn test_complete_legal_compliance_workflow() {
    println!("Starting complete legal compliance workflow test");

    // 1. Create legal compliance configuration
    let config = LegalConfig {
        enforce_terms: true,
        enforce_privacy: true,
        enforce_geo: true,
        enforce_age: true,
        enforce_sanctions: true,
        enforce_kyc: true,
    };

    // 2. Create legal compliance manager
    let mut compliance = LegalCompliance::new(config);
    println!("✓ Legal compliance manager created");

    // 3. Configure terms of service
    let terms = TermsOfService {
        version: "1.0.0".to_string(),
        content_hash: "abc123".to_string(),
        effective_date: 1234567890,
        accepted_users: Vec::new(),
    };
    compliance.terms = terms;
    println!("✓ Terms of service configured");

    // 4. Configure privacy policy
    let privacy = PrivacyPolicy {
        version: "1.0.0".to_string(),
        content_hash: "def456".to_string(),
        effective_date: 1234567890,
    };
    compliance.privacy = privacy;
    println!("✓ Privacy policy configured");

    // 5. Add geographic restrictions
    let us_restriction = GeoRestriction {
        country_code: "US".to_string(),
        allowed: true,
        reason: "Allowed jurisdiction".to_string(),
    };

    let cn_restriction = GeoRestriction {
        country_code: "CN".to_string(),
        allowed: false,
        reason: "Sanctions".to_string(),
    };

    assert!(compliance.add_geo_restriction(us_restriction).is_ok());
    assert!(compliance.add_geo_restriction(cn_restriction).is_ok());
    println!("✓ Geographic restrictions added");

    // 6. Configure age restriction
    let age_restriction = AgeRestriction {
        min_age: 18,
        verification_required: true,
    };
    compliance.age_restriction = age_restriction;
    println!("✓ Age restriction configured");

    // 7. Configure sanctions screening
    let sanctions_config = SanctionsConfig {
        provider: "ofac".to_string(),
        api_key: "test_key".to_string(),
        check_frequency: 3600,
        blocked_addresses: vec!["blocked_address".to_string()],
    };
    compliance.sanctions_config = sanctions_config;
    println!("✓ Sanctions screening configured");

    // 8. Test user interactions
    let user1 = Address("user1_address".to_string());
    let user2 = Address("user2_address".to_string());

    // Check terms acceptance
    assert!(!compliance.check_terms_accepted(&user1));
    assert!(compliance.accept_terms(user1.clone()).is_ok());
    assert!(compliance.check_terms_accepted(&user1));
    println!("✓ Terms of service acceptance verified");

    // Check geographic restrictions
    assert!(compliance.check_geo_allowed("US"));
    assert!(!compliance.check_geo_allowed("CN"));
    assert!(compliance.check_geo_allowed("CA")); // Not restricted
    println!("✓ Geographic restrictions verified");

    // Check age restrictions
    assert!(compliance.check_age_allowed(25));
    assert!(!compliance.check_age_allowed(16));
    println!("✓ Age restrictions verified");

    // Check sanctions screening
    assert!(compliance.check_sanctions("blocked_address"));
    assert!(!compliance.check_sanctions("allowed_address"));
    println!("✓ Sanctions screening verified");

    // 9. Test KYC management
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let kyc_record = KycRecord {
        user_address: user2.clone(),
        status: KycStatus::Verified,
        verification_date: current_time,
        expiry_date: current_time + 365 * 24 * 3600, // 1 year from now
        provider: "test_provider".to_string(),
    };

    assert_eq!(compliance.check_kyc_status(&user2), KycStatus::Pending);
    assert!(compliance.update_kyc_record(kyc_record).is_ok());
    assert_eq!(compliance.check_kyc_status(&user2), KycStatus::Verified);
    println!("✓ KYC management verified");

    // 10. Test audit logging
    assert!(compliance
        .log_action("Test compliance action".to_string())
        .is_ok());
    assert!(!compliance.audit_log.is_empty());
    println!("✓ Audit logging verified");

    println!("Complete legal compliance workflow test passed!");
}

/// Integration test for legal compliance with disabled features
#[test]
fn test_legal_compliance_with_disabled_features() {
    println!("Starting legal compliance with disabled features test");

    // 1. Create legal compliance configuration with all features disabled
    let config = LegalConfig {
        enforce_terms: false,
        enforce_privacy: false,
        enforce_geo: false,
        enforce_age: false,
        enforce_sanctions: false,
        enforce_kyc: false,
    };

    // 2. Create legal compliance manager
    let compliance = LegalCompliance::new(config);
    println!("✓ Legal compliance manager created with disabled features");

    // 3. Test that all checks pass when features are disabled
    let user = Address("test_user".to_string());

    // Terms acceptance should pass when not enforced
    assert!(compliance.check_terms_accepted(&user));
    println!("✓ Terms acceptance check passed when disabled");

    // Geographic restrictions should pass when not enforced
    assert!(compliance.check_geo_allowed("RESTRICTED"));
    println!("✓ Geographic restriction check passed when disabled");

    // Age restrictions should pass when not enforced
    assert!(compliance.check_age_allowed(10));
    println!("✓ Age restriction check passed when disabled");

    // Sanctions screening should pass when not enforced
    assert!(!compliance.check_sanctions("SANCTIONED"));
    println!("✓ Sanctions screening check passed when disabled");

    // KYC should return NotRequired when not enforced
    assert_eq!(compliance.check_kyc_status(&user), KycStatus::NotRequired);
    println!("✓ KYC check returned NotRequired when disabled");

    println!("Legal compliance with disabled features test passed!");
}

/// Integration test for legal compliance error handling
#[test]
fn test_legal_compliance_error_handling() {
    println!("Starting legal compliance error handling test");

    // 1. Create legal compliance configuration
    let config = LegalConfig {
        enforce_terms: true,
        enforce_privacy: true,
        enforce_geo: true,
        enforce_age: true,
        enforce_sanctions: true,
        enforce_kyc: true,
    };

    // 2. Create legal compliance manager
    let mut compliance = LegalCompliance::new(config);
    println!("✓ Legal compliance manager created");

    // 3. Test adding duplicate geographic restrictions
    let restriction1 = GeoRestriction {
        country_code: "TEST".to_string(),
        allowed: true,
        reason: "Test".to_string(),
    };

    let restriction2 = GeoRestriction {
        country_code: "TEST".to_string(),
        allowed: false,
        reason: "Updated test".to_string(),
    };

    // Both should succeed (second one overwrites the first)
    assert!(compliance.add_geo_restriction(restriction1).is_ok());
    assert!(compliance.add_geo_restriction(restriction2).is_ok());
    assert!(!compliance.check_geo_allowed("TEST")); // Should be false now
    println!("✓ Duplicate geographic restriction handling verified");

    // 4. Test KYC expiration
    let user = Address("expiring_user".to_string());
    let past_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
        - 1000; // 1000 seconds in the past

    let expired_kyc = KycRecord {
        user_address: user.clone(),
        status: KycStatus::Verified,
        verification_date: past_time,
        expiry_date: past_time + 500, // Expired 500 seconds ago
        provider: "test_provider".to_string(),
    };

    assert!(compliance.update_kyc_record(expired_kyc).is_ok());
    assert_eq!(compliance.check_kyc_status(&user), KycStatus::Expired);
    println!("✓ KYC expiration handling verified");

    println!("Legal compliance error handling test passed!");
}
