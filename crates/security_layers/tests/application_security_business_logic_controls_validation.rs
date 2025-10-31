//! Application Security Business Logic Controls Features Validation Tests (Line 12)
//!
//! This module contains tests that specifically validate the Business Logic Controls features from line 12 
//! of the web3_protection_layers.csv file:
//! 3,Application Security,Business Logic Controls,Rate/Velocity Rules,"OTP retry limits, withdrawal limits, anti-bruteforce counters, anti-spam throttles","Stop abuse of legit flows","Per-user throttle hits, lockouts"

use security_layers::application_security::*;

/// Test that validates the specific Business Logic Controls features from line 12 of web3_protection_layers.csv
#[test]
fn test_application_security_business_logic_controls_line_12() {
    println!("Testing Application Security Business Logic Controls features from line 12 of web3_protection_layers.csv...");
    
    // Test Line 12: Application Security, Business Logic Controls, Rate/Velocity Rules
    // "OTP retry limits, withdrawal limits, anti-bruteforce counters, anti-spam throttles"
    // "Stop abuse of legit flows"
    // "Per-user throttle hits, lockouts"
    test_business_logic_controls_features();
    
    println!("All Application Security Business Logic Controls features from line 12 validated successfully!");
}

/// Test Application Security, Business Logic Controls, Rate/Velocity Rules
/// Component/Mechanism: "OTP retry limits, withdrawal limits, anti-bruteforce counters, anti-spam throttles"
/// Goal: "Stop abuse of legit flows"
/// Evidence/Telemetry: "Per-user throttle hits, lockouts"
fn test_business_logic_controls_features() {
    println!("Testing Application Security, Business Logic Controls, Rate/Velocity Rules...");
    
    // Test OTP Retry Limits
    test_otp_retry_limits();
    
    // Test Withdrawal Limits
    test_withdrawal_limits();
    
    // Test Anti-Bruteforce Counters
    test_anti_bruteforce_counters();
    
    // Test Anti-Spam Throttles
    test_anti_spam_throttles();
    
    // Test Evidence/Telemetry Collection
    test_evidence_collection();
    
    println!("✓ Business logic controls features validated");
}

/// Test OTP Retry Limits
/// Component/Mechanism: "OTP retry limits"
/// Goal: "Stop abuse of legit flows"
fn test_otp_retry_limits() {
    println!("  Testing OTP Retry Limits...");
    
    let mut controls = BusinessLogicControls::new();
    controls.configure_otp_retry_limits(3, 60); // 3 attempts per minute
    
    let user_id = "otp_user";
    
    // First 3 attempts should be allowed
    assert!(controls.check_otp_retry_allowed(user_id).is_ok());
    assert!(controls.check_otp_retry_allowed(user_id).is_ok());
    assert!(controls.check_otp_retry_allowed(user_id).is_ok());
    
    // 4th attempt should be blocked
    let result = controls.check_otp_retry_allowed(user_id);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "OTP retry limit exceeded");
    
    println!("    ✓ OTP retry limits validated");
}

/// Test Withdrawal Limits
/// Component/Mechanism: "withdrawal limits"
/// Goal: "Stop abuse of legit flows"
fn test_withdrawal_limits() {
    println!("  Testing Withdrawal Limits...");
    
    let mut controls = BusinessLogicControls::new();
    controls.configure_withdrawal_limits(1000.0, 3600); // $1000 per hour
    
    let user_id = "withdrawal_user";
    
    // First withdrawal of $500 should be allowed
    assert!(controls.check_withdrawal_allowed(user_id, 500.0).is_ok());
    
    // Second withdrawal of $400 should be allowed (total $900)
    assert!(controls.check_withdrawal_allowed(user_id, 400.0).is_ok());
    
    // Third withdrawal of $200 should be blocked (would exceed $1000)
    let result = controls.check_withdrawal_allowed(user_id, 200.0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Withdrawal limit exceeded");
    
    println!("    ✓ Withdrawal limits validated");
}

/// Test Anti-Bruteforce Counters
/// Component/Mechanism: "anti-bruteforce counters"
/// Goal: "Stop abuse of legit flows"
fn test_anti_bruteforce_counters() {
    println!("  Testing Anti-Bruteforce Counters...");
    
    let mut controls = BusinessLogicControls::new();
    controls.configure_bruteforce_protection(3, 60, 300); // 3 attempts per minute, 5 min lockout
    
    let user_id = "bruteforce_user";
    
    // First 3 login attempts should be allowed
    assert!(controls.check_login_allowed(user_id).is_ok());
    assert!(controls.check_login_allowed(user_id).is_ok());
    assert!(controls.check_login_allowed(user_id).is_ok());
    
    // 4th attempt should be blocked and user locked out
    let result = controls.check_login_allowed(user_id);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Too many failed login attempts");
    
    // User should now be locked out
    assert!(controls.is_user_locked_out(user_id));
    
    // Even valid login attempts should be blocked while locked out
    let result = controls.check_login_allowed(user_id);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Account temporarily locked due to suspicious activity");
    
    println!("    ✓ Anti-bruteforce counters validated");
}

/// Test Anti-Spam Throttles
/// Component/Mechanism: "anti-spam throttles"
/// Goal: "Stop abuse of legit flows"
fn test_anti_spam_throttles() {
    println!("  Testing Anti-Spam Throttles...");
    
    let mut controls = BusinessLogicControls::new();
    controls.configure_spam_throttles(5, 60); // 5 requests per minute
    
    let user_id = "spam_user";
    
    // First 5 requests should be allowed
    for i in 0..5 {
        let result = controls.check_request_allowed(user_id);
        assert!(result.is_ok(), "Request {} should be allowed", i);
    }
    
    // 6th request should be blocked
    let result = controls.check_request_allowed(user_id);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Request rate limit exceeded");
    
    println!("    ✓ Anti-spam throttles validated");
}

/// Test Evidence/Telemetry Collection
/// Evidence/Telemetry: "Per-user throttle hits, lockouts"
fn test_evidence_collection() {
    println!("  Testing Evidence/Telemetry Collection...");
    
    let mut controls = BusinessLogicControls::new();
    controls.configure_otp_retry_limits(3, 60);
    controls.configure_withdrawal_limits(1000.0, 3600);
    controls.configure_bruteforce_protection(3, 60, 300);
    controls.configure_spam_throttles(5, 60);
    
    let user_id = "telemetry_user";
    
    // Trigger some limits to generate telemetry data
    // Trigger OTP retry limit
    assert!(controls.check_otp_retry_allowed(user_id).is_ok());
    assert!(controls.check_otp_retry_allowed(user_id).is_ok());
    assert!(controls.check_otp_retry_allowed(user_id).is_ok());
    assert!(controls.check_otp_retry_allowed(user_id).is_err()); // Blocked
    
    // Trigger spam throttle
    for _ in 0..5 {
        assert!(controls.check_request_allowed(user_id).is_ok());
    }
    assert!(controls.check_request_allowed(user_id).is_err()); // Blocked
    
    // Trigger bruteforce protection
    assert!(controls.check_login_allowed(user_id).is_ok());
    assert!(controls.check_login_allowed(user_id).is_ok());
    assert!(controls.check_login_allowed(user_id).is_ok());
    assert!(controls.check_login_allowed(user_id).is_err()); // Blocked and locked out
    
    // Check throttle hit counts
    let otp_hits = controls.get_throttle_hit_count(user_id, "otp_retry");
    let spam_hits = controls.get_throttle_hit_count(user_id, "spam");
    let bruteforce_hits = controls.get_throttle_hit_count(user_id, "bruteforce");
    
    println!("    OTP hits: {}, Spam hits: {}, Bruteforce hits: {}", otp_hits, spam_hits, bruteforce_hits);
    
    // Check all throttle hits for user
    let throttle_hits = controls.get_user_throttle_hits(user_id);
    println!("    All throttle hits: {:?}", throttle_hits);
    
    // Check lockout status
    assert!(controls.is_user_locked_out(user_id));
    
    // Test counter reset
    controls.reset_user_counters(user_id);
    
    // After reset, counters should be zero
    assert_eq!(controls.get_throttle_hit_count(user_id, "otp_retry"), 0);
    assert_eq!(controls.get_throttle_hit_count(user_id, "spam"), 0);
    assert_eq!(controls.get_throttle_hit_count(user_id, "bruteforce"), 0);
    
    // User should be unlocked
    assert!(!controls.is_user_locked_out(user_id));
    
    println!("    ✓ Evidence/telemetry collection validated");
}