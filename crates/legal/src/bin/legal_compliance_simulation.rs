//! Binary test runner for legal compliance simulation

use core::types::Address;
use legal::{
    AgeRestriction, GeoRestriction, KycRecord, KycStatus, LegalCompliance, LegalConfig,
    PrivacyPolicy, SanctionsConfig, TermsOfService,
};

fn main() {
    println!("Starting Legal Compliance Simulation Tests");
    println!("=====================================\n");

    test_complete_legal_compliance_workflow();
    test_legal_compliance_under_stress();
    test_legal_compliance_edge_cases();

    println!("All Legal Compliance Simulation Tests Passed!");
}

/// Test complete legal compliance workflow
fn test_complete_legal_compliance_workflow() {
    println!("1. Testing Complete Legal Compliance Workflow");
    println!("-------------------------------------------");

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
    println!("   ✓ Legal compliance manager created");

    // 3. Configure terms of service
    let terms = TermsOfService {
        version: "1.0.0".to_string(),
        content_hash: "abc123".to_string(),
        effective_date: 1234567890,
        accepted_users: Vec::new(),
    };
    compliance.terms = terms;
    println!("   ✓ Terms of service configured");

    // 4. Configure privacy policy
    let privacy = PrivacyPolicy {
        version: "1.0.0".to_string(),
        content_hash: "def456".to_string(),
        effective_date: 1234567890,
    };
    compliance.privacy = privacy;
    println!("   ✓ Privacy policy configured");

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
    println!("   ✓ Geographic restrictions added");

    // 6. Configure age restriction
    let age_restriction = AgeRestriction {
        min_age: 18,
        verification_required: true,
    };
    compliance.age_restriction = age_restriction;
    println!("   ✓ Age restriction configured");

    // 7. Configure sanctions screening
    let sanctions_config = SanctionsConfig {
        provider: "ofac".to_string(),
        api_key: "test_key".to_string(),
        check_frequency: 3600,
        blocked_addresses: vec!["blocked_address".to_string()],
    };
    compliance.sanctions_config = sanctions_config;
    println!("   ✓ Sanctions screening configured");

    // 8. Test user interactions
    let user1 = Address("user1_address".to_string());
    let user2 = Address("user2_address".to_string());

    // Check terms acceptance
    assert!(!compliance.check_terms_accepted(&user1));
    assert!(compliance.accept_terms(user1.clone()).is_ok());
    assert!(compliance.check_terms_accepted(&user1));
    println!("   ✓ Terms of service acceptance verified");

    // Check geographic restrictions
    assert!(compliance.check_geo_allowed("US"));
    assert!(!compliance.check_geo_allowed("CN"));
    assert!(compliance.check_geo_allowed("CA")); // Not restricted
    println!("   ✓ Geographic restrictions verified");

    // Check age restrictions
    assert!(compliance.check_age_allowed(25));
    assert!(!compliance.check_age_allowed(16));
    println!("   ✓ Age restrictions verified");

    // Check sanctions screening
    assert!(compliance.check_sanctions("blocked_address"));
    assert!(!compliance.check_sanctions("allowed_address"));
    println!("   ✓ Sanctions screening verified");

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
    println!("   ✓ KYC management verified");

    // 10. Test audit logging
    assert!(compliance
        .log_action("Test compliance action".to_string())
        .is_ok());
    assert!(!compliance.audit_log.is_empty());
    println!("   ✓ Audit logging verified");

    println!("   ✓ Complete legal compliance workflow test passed\n");
}

/// Test legal compliance under stress conditions
fn test_legal_compliance_under_stress() {
    println!("2. Testing Legal Compliance Under Stress");
    println!("--------------------------------------");

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
    println!("   ✓ Legal compliance manager created");

    // 3. Add many geographic restrictions
    for i in 0..100 {
        let country_code = format!("COUNTRY{}", i);
        let restriction = GeoRestriction {
            country_code: country_code.clone(),
            allowed: i % 2 == 0, // Alternate allowed/blocked
            reason: format!("Test restriction {}", i),
        };

        assert!(compliance.add_geo_restriction(restriction).is_ok());
    }
    println!("   ✓ 100 geographic restrictions added");

    // 4. Add many sanctioned addresses
    for i in 0..1000 {
        let address = format!("sanctioned_address_{}", i);
        assert!(compliance.add_sanctioned_address(address).is_ok());
    }
    println!("   ✓ 1000 sanctioned addresses added");

    // 5. Add many KYC records
    for i in 0..500 {
        let user = Address(format!("user_{}", i));
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let kyc_record = KycRecord {
            user_address: user,
            status: if i % 3 == 0 {
                KycStatus::Verified
            } else if i % 3 == 1 {
                KycStatus::Pending
            } else {
                KycStatus::Rejected
            },
            verification_date: current_time,
            expiry_date: current_time + 365 * 24 * 3600,
            provider: format!("provider_{}", i % 10),
        };

        assert!(compliance.update_kyc_record(kyc_record).is_ok());
    }
    println!("   ✓ 500 KYC records added");

    // 6. Test performance of geographic checks
    let start_time = std::time::Instant::now();
    for i in 0..1000 {
        let country_code = format!("COUNTRY{}", i % 100);
        let _allowed = compliance.check_geo_allowed(&country_code);
    }
    let geo_duration = start_time.elapsed();
    println!(
        "   ✓ 1000 geographic checks completed in {:?}",
        geo_duration
    );

    // 7. Test performance of sanctions checks
    let start_time = std::time::Instant::now();
    for i in 0..10000 {
        let address = format!("sanctioned_address_{}", i % 1000);
        let _sanctioned = compliance.check_sanctions(&address);
    }
    let sanctions_duration = start_time.elapsed();
    println!(
        "   ✓ 10000 sanctions checks completed in {:?}",
        sanctions_duration
    );

    // 8. Test performance of KYC checks
    let start_time = std::time::Instant::now();
    for i in 0..5000 {
        let user = Address(format!("user_{}", i % 500));
        let _kyc_status = compliance.check_kyc_status(&user);
    }
    let kyc_duration = start_time.elapsed();
    println!("   ✓ 5000 KYC checks completed in {:?}", kyc_duration);

    // 9. Verify final states
    assert_eq!(compliance.geo_restrictions.len(), 100);
    assert_eq!(compliance.sanctions_config.blocked_addresses.len(), 1000);
    assert_eq!(compliance.kyc_records.len(), 500);
    println!("   ✓ Final states verified");

    println!("   ✓ Legal compliance stress test passed\n");
}

/// Test legal compliance edge cases
fn test_legal_compliance_edge_cases() {
    println!("3. Testing Legal Compliance Edge Cases");
    println!("------------------------------------");

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
    println!("   ✓ Legal compliance manager created");

    // 3. Test edge case: minimum age of 0
    let age_restriction = AgeRestriction {
        min_age: 0,
        verification_required: false,
    };
    compliance.age_restriction = age_restriction;
    assert!(compliance.check_age_allowed(0));
    assert!(compliance.check_age_allowed(100));
    println!("   ✓ Minimum age of 0 handled correctly");

    // 4. Test edge case: maximum age
    let age_restriction = AgeRestriction {
        min_age: 255,
        verification_required: true,
    };
    compliance.age_restriction = age_restriction;
    assert!(!compliance.check_age_allowed(254));
    assert!(compliance.check_age_allowed(255));
    println!("   ✓ Maximum age handled correctly");

    // 5. Test edge case: empty country code
    let empty_restriction = GeoRestriction {
        country_code: "".to_string(),
        allowed: false,
        reason: "Empty country code test".to_string(),
    };
    assert!(compliance.add_geo_restriction(empty_restriction).is_ok());
    assert!(!compliance.check_geo_allowed(""));
    println!("   ✓ Empty country code handled correctly");

    // 6. Test edge case: very long country code
    let long_country_code = "A".repeat(1000);
    let long_restriction = GeoRestriction {
        country_code: long_country_code.clone(),
        allowed: true,
        reason: "Long country code test".to_string(),
    };
    assert!(compliance.add_geo_restriction(long_restriction).is_ok());
    assert!(compliance.check_geo_allowed(&long_country_code));
    println!("   ✓ Long country code handled correctly");

    // 7. Test edge case: KYC record with past expiry
    let user = Address("edge_case_user".to_string());
    let past_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
        - 1000000; // Far in the past

    let expired_kyc = KycRecord {
        user_address: user.clone(),
        status: KycStatus::Verified,
        verification_date: past_time,
        expiry_date: past_time + 1000, // Expired long ago
        provider: "test_provider".to_string(),
    };

    assert!(compliance.update_kyc_record(expired_kyc).is_ok());
    assert_eq!(compliance.check_kyc_status(&user), KycStatus::Expired);
    println!("   ✓ Expired KYC record handled correctly");

    // 8. Test edge case: KYC record with future expiry
    let future_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
        + 1000000; // Far in the future

    let future_kyc = KycRecord {
        user_address: user.clone(),
        status: KycStatus::Verified,
        verification_date: future_time - 1000,
        expiry_date: future_time,
        provider: "test_provider".to_string(),
    };

    assert!(compliance.update_kyc_record(future_kyc).is_ok());
    assert_eq!(compliance.check_kyc_status(&user), KycStatus::Verified);
    println!("   ✓ Future KYC record handled correctly");

    println!("   ✓ Legal compliance edge cases test passed\n");
}
