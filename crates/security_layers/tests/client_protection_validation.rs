//! Client Protection Features Validation Tests
//!
//! This module contains tests that validate the client protection features:
//! - Transaction confirmation flows
//! - Phishing resistance (domain verification, visual security cues)
//! - Fraud alert systems

use security_layers::client_protection::*;

/// Test that validates the client protection features
#[test]
fn test_client_protection_features() {
    println!("Testing Client Protection features...");
    
    // Test Transaction Confirmation
    test_transaction_confirmation();
    
    // Test Phishing Resistance
    test_phishing_resistance();
    
    // Test Fraud Alert System
    test_fraud_alert_system();
    
    println!("All Client Protection features validated successfully!");
}

/// Test Transaction Confirmation
fn test_transaction_confirmation() {
    println!("  Testing Transaction Confirmation...");
    
    let transaction = TransactionDetails {
        id: "tx-001".to_string(),
        amount: 1000.0,
        token: "ETH".to_string(),
        to_address: "0x742d35Cc6634C0532925a3b8D4C0532925a3b8D4".to_string(),
        from_address: "0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B".to_string(),
        gas_limit: 21000,
        gas_price: 20000000000,
        chain_id: 1,
    };
    
    let confirmation = TransactionConfirmation {
        transaction,
        status: ConfirmationStatus::Pending,
        security_checks: vec![
            SecurityCheck {
                check_type: SecurityCheckType::DomainVerification,
                result: SecurityCheckResult::Passed,
                details: "Domain verified".to_string(),
                timestamp: 1000000,
            },
            SecurityCheck {
                check_type: SecurityCheckType::AddressVerification,
                result: SecurityCheckResult::Passed,
                details: "Address verified".to_string(),
                timestamp: 1000001,
            },
        ],
        timestamp: 1000000,
    };
    
    assert_eq!(confirmation.status, ConfirmationStatus::Pending);
    assert_eq!(confirmation.security_checks.len(), 2);
    
    println!("    ✓ Transaction confirmation validated");
}

/// Test Phishing Resistance
fn test_phishing_resistance() {
    println!("  Testing Phishing Resistance...");
    
    let phishing_resistance = PhishingResistance::new();
    
    // Test legitimate domain
    let result = phishing_resistance.verify_domain("app.decentralized-app.com");
    match result {
        DomainVerificationResult::Legitimate { confidence, .. } => {
            assert_eq!(confidence, 100);
        }
        _ => panic!("Expected legitimate domain result"),
    }
    
    // Test phishing domain
    let result = phishing_resistance.verify_domain("decentralized-app.login.com");
    match result {
        DomainVerificationResult::Phishing { confidence, .. } => {
            assert_eq!(confidence, 100);
        }
        _ => panic!("Expected phishing domain result"),
    }
    
    // Test visual cues configuration
    let visual_cues = phishing_resistance.get_visual_cues();
    assert!(visual_cues.show_verification_badges);
    assert!(visual_cues.highlight_suspicious);
    assert!(visual_cues.warn_unknown_domains);
    assert!(visual_cues.show_status_indicators);
    
    println!("    ✓ Phishing resistance validated");
}

/// Test Fraud Alert System
fn test_fraud_alert_system() {
    println!("  Testing Fraud Alert System...");
    
    let mut fraud_alerts = FraudAlertSystem::new();
    
    // Test large transfer alert
    let alert = fraud_alerts.check_large_transfer(15000.0, "user123");
    assert!(alert.is_some());
    
    let alert = alert.unwrap();
    assert_eq!(alert.alert_type, FraudAlertType::LargeTransfer);
    assert_eq!(alert.severity, AlertSeverity::High);
    assert_eq!(alert.user_id, "user123");
    
    // Test rapid transactions alert
    let alert = fraud_alerts.check_rapid_transactions(15, 60, "user456");
    assert!(alert.is_some());
    
    let alert = alert.unwrap();
    assert_eq!(alert.alert_type, FraudAlertType::RapidTransactions);
    assert_eq!(alert.severity, AlertSeverity::Medium);
    assert_eq!(alert.user_id, "user456");
    
    // Test failed logins alert
    let alert = fraud_alerts.check_failed_logins(7, "user789");
    assert!(alert.is_some());
    
    let alert = alert.unwrap();
    assert_eq!(alert.alert_type, FraudAlertType::FailedLogins);
    assert_eq!(alert.severity, AlertSeverity::High);
    assert_eq!(alert.user_id, "user789");
    
    // Test alert management
    let alert = FraudAlert {
        id: "test-alert-1".to_string(),
        alert_type: FraudAlertType::SuspiciousTransaction,
        severity: AlertSeverity::Medium,
        description: "Test alert".to_string(),
        user_id: "user123".to_string(),
        timestamp: 1000000,
        acknowledged: false,
        metadata: std::collections::HashMap::new(),
    };
    
    // Generate the alert
    assert!(fraud_alerts.generate_alert(alert).is_ok());
    
    // Acknowledge the alert
    assert!(fraud_alerts.acknowledge_alert("test-alert-1").is_ok());
    
    // Check unacknowledged alerts
    let unacknowledged = fraud_alerts.get_unacknowledged_alerts("user123");
    assert!(unacknowledged.is_empty());
    
    println!("    ✓ Fraud alert system validated");
}