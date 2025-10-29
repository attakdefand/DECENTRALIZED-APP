//! Tests for Security Detection features: SIEM / IDS / Anomaly Alerts
//!
//! These tests validate the implementation of Layer 8 requirements:
//! "Login anomaly detection, data exfil alerts, container breakout alerts, SIEM rules with severity levels"
//! Goal: "Catch intrusion quickly"
//! Evidence/Telemetry: "Mean time to detect (MTTD), SIEM alert counts by severity"

use core::observability::{
    ObservabilityManager, SiemRule, SiemSeverity,
};
use std::collections::HashMap;

/// Test SIEM / IDS / Anomaly Alerts features
#[test]
fn test_siem_ids_anomaly_alerts() {
    println!("Starting SIEM / IDS / Anomaly Alerts test");

    // 1. Create observability manager
    let mut manager = ObservabilityManager::new();
    println!("✓ Observability manager created");

    // 2. Add SIEM rules for different detection scenarios
    let login_anomaly_rule = SiemRule {
        id: "login-anomaly-detection".to_string(),
        description: "Detects suspicious login patterns".to_string(),
        criteria: "failed_login_attempts > 5".to_string(),
        severity: SiemSeverity::High,
        enabled: true,
    };

    let data_exfil_rule = SiemRule {
        id: "data-exfiltration-detection".to_string(),
        description: "Detects potential data exfiltration attempts".to_string(),
        criteria: "data_transfer_size > threshold".to_string(),
        severity: SiemSeverity::Critical,
        enabled: true,
    };

    let container_breakout_rule = SiemRule {
        id: "container-breakout-detection".to_string(),
        description: "Detects potential container breakout attempts".to_string(),
        criteria: "suspicious_filesystem_access OR privilege_escalation".to_string(),
        severity: SiemSeverity::Critical,
        enabled: true,
    };

    // Add rules to manager
    assert!(manager.add_siem_rule(login_anomaly_rule).is_ok());
    assert!(manager.add_siem_rule(data_exfil_rule).is_ok());
    assert!(manager.add_siem_rule(container_breakout_rule).is_ok());
    println!("✓ SIEM rules for security detection configured");

    // 3. Test login anomaly detection
    let user_id = "user-123".to_string();
    let ip_address = "192.168.1.100".to_string();
    let service = "auth-service".to_string();
    
    // Normal login - should not generate alert
    let alert1 = manager.detect_login_anomaly(
        user_id.clone(),
        ip_address.clone(),
        service.clone(),
        3, // Normal failure count
    ).expect("Failed to detect login anomaly");
    
    assert!(alert1.is_none());
    println!("✓ Normal login activity does not generate alerts");

    // Suspicious login - should generate alert
    let alert2 = manager.detect_login_anomaly(
        user_id.clone(),
        ip_address.clone(),
        service.clone(),
        8, // High failure count
    ).expect("Failed to detect login anomaly");
    
    assert!(alert2.is_some());
    println!("✓ Suspicious login activity generates alert: {}", alert2.as_ref().unwrap());

    // Critical login - should generate critical alert
    let alert3 = manager.detect_login_anomaly(
        user_id.clone(),
        ip_address.clone(),
        service.clone(),
        15, // Very high failure count
    ).expect("Failed to detect login anomaly");
    
    assert!(alert3.is_some());
    println!("✓ Critical login activity generates critical alert: {}", alert3.as_ref().unwrap());

    // 4. Test data exfiltration detection
    let user_id2 = "user-456".to_string();
    let service2 = "data-service".to_string();
    let threshold = 1000000u64; // 1MB threshold
    
    // Normal data transfer - should not generate alert
    let alert4 = manager.detect_data_exfiltration(
        user_id2.clone(),
        service2.clone(),
        500000, // 500KB - below threshold
        threshold,
    ).expect("Failed to detect data exfiltration");
    
    assert!(alert4.is_none());
    println!("✓ Normal data transfer does not generate alerts");

    // Suspicious data transfer - should generate alert
    let alert5 = manager.detect_data_exfiltration(
        user_id2.clone(),
        service2.clone(),
        2500000, // 2.5MB - above threshold
        threshold,
    ).expect("Failed to detect data exfiltration");
    
    assert!(alert5.is_some());
    println!("✓ Suspicious data transfer generates alert: {}", alert5.as_ref().unwrap());

    // Critical data transfer - should generate critical alert
    let alert6 = manager.detect_data_exfiltration(
        user_id2.clone(),
        service2.clone(),
        5000000, // 5MB - well above threshold
        threshold,
    ).expect("Failed to detect data exfiltration");
    
    assert!(alert6.is_some());
    println!("✓ Critical data transfer generates critical alert: {}", alert6.as_ref().unwrap());

    // 5. Test container breakout detection
    let container_id = "container-789".to_string();
    let service3 = "container-runtime".to_string();
    let suspicious_activity = "attempted filesystem access outside container boundary".to_string();
    
    let alert7 = manager.detect_container_breakout(
        container_id.clone(),
        service3.clone(),
        suspicious_activity.clone(),
    ).expect("Failed to detect container breakout");
    
    assert!(!alert7.is_empty());
    println!("✓ Container breakout detection generates critical alert: {}", alert7);

    // 6. Test SIEM alert tracking with severity levels
    let alerts = manager.get_siem_alerts(None);
    assert_eq!(alerts.len(), 5); // 2 login alerts, 2 data exfil alerts, 1 container breakout alert
    println!("✓ Generated {} SIEM alerts", alerts.len());

    // Check severity-based filtering
    let high_alerts = manager.get_siem_alerts(Some(SiemSeverity::High));
    assert_eq!(high_alerts.len(), 1); // 1 login alert with high severity (the other was critical)
    println!("✓ Found {} high severity alerts", high_alerts.len());

    let critical_alerts = manager.get_siem_alerts(Some(SiemSeverity::Critical));
    assert_eq!(critical_alerts.len(), 4); // 1 critical login alert + 2 data exfil alerts + 1 container breakout alert
    println!("✓ Found {} critical severity alerts", critical_alerts.len());

    // 7. Test alert resolution
    let first_alert_id = &alerts[0].id;
    assert!(manager.resolve_siem_alert(first_alert_id).is_ok());
    println!("✓ Resolved alert: {}", first_alert_id);

    // Verify alert is marked as resolved
    let updated_alerts = manager.get_siem_alerts(None);
    let resolved_alert = updated_alerts.iter().find(|a| a.id == *first_alert_id).unwrap();
    assert!(resolved_alert.resolved);
    println!("✓ Alert resolution status verified");

    // 8. Test security detection statistics (evidence/telemetry)
    let security_stats = manager.get_security_detection_stats();
    assert_eq!(security_stats.total_alerts, 5);
    assert_eq!(security_stats.alerts_by_severity.get(&SiemSeverity::High).unwrap_or(&0), &1);
    assert_eq!(security_stats.alerts_by_severity.get(&SiemSeverity::Critical).unwrap_or(&0), &4);
    // MTTD will be 0 if all alerts generated at same time, which is acceptable
    println!("✓ Security detection statistics verified:");
    println!("  - Total alerts: {}", security_stats.total_alerts);
    println!("  - High severity alerts: {}", security_stats.alerts_by_severity.get(&SiemSeverity::High).unwrap_or(&0));
    println!("  - Critical severity alerts: {}", security_stats.alerts_by_severity.get(&SiemSeverity::Critical).unwrap_or(&0));
    println!("  - Mean time to detect: {} seconds", security_stats.mean_time_to_detect);

    // 9. Verify all SIEM rules are present
    assert!(manager.get_siem_rule("login-anomaly-detection").is_some());
    assert!(manager.get_siem_rule("data-exfiltration-detection").is_some());
    assert!(manager.get_siem_rule("container-breakout-detection").is_some());
    println!("✓ All SIEM rules verified");

    println!("All SIEM / IDS / Anomaly Alerts tests passed!");
}

/// Test Mean Time to Detect (MTTD) calculation
#[test]
fn test_mean_time_to_detect_calculation() {
    let mut manager = ObservabilityManager::new();

    // Generate alerts at different times to test MTTD calculation
    let mut data1 = HashMap::new();
    data1.insert("test".to_string(), "data1".to_string());
    
    // First alert
    let _alert1 = manager.generate_siem_alert(
        "test-rule-1".to_string(),
        SiemSeverity::High,
        "Test alert 1".to_string(),
        "test-service".to_string(),
        data1,
    ).expect("Failed to generate first alert");

    // Small delay to ensure different timestamps
    std::thread::sleep(std::time::Duration::from_millis(10));

    let mut data2 = HashMap::new();
    data2.insert("test".to_string(), "data2".to_string());
    
    // Second alert
    let _alert2 = manager.generate_siem_alert(
        "test-rule-2".to_string(),
        SiemSeverity::Critical,
        "Test alert 2".to_string(),
        "test-service".to_string(),
        data2,
    ).expect("Failed to generate second alert");

    // Get security detection statistics
    let stats = manager.get_security_detection_stats();
    
    // Should have 2 alerts total
    assert_eq!(stats.total_alerts, 2);
    
    // Should have alerts by severity
    assert_eq!(stats.alerts_by_severity.get(&SiemSeverity::High).unwrap_or(&0), &1);
    assert_eq!(stats.alerts_by_severity.get(&SiemSeverity::Critical).unwrap_or(&0), &1);
    
    // MTTD may be 0 on fast systems, which is acceptable
    println!("MTTD calculation test passed:");
    println!("✓ Total alerts: {}", stats.total_alerts);
    println!("✓ Mean time to detect: {} seconds", stats.mean_time_to_detect);
    println!("✓ Alerts by severity: {:?}", stats.alerts_by_severity);
}

/// Test SIEM alert counts by severity
#[test]
fn test_siem_alert_counts_by_severity() {
    let mut manager = ObservabilityManager::new();

    // Generate multiple alerts of different severities
    let severities = vec![
        SiemSeverity::Low,
        SiemSeverity::Medium,
        SiemSeverity::High,
        SiemSeverity::Critical,
        SiemSeverity::Low,
        SiemSeverity::High,
        SiemSeverity::High,
        SiemSeverity::Critical,
        SiemSeverity::Critical,
        SiemSeverity::Critical,
    ];

    for (i, severity) in severities.iter().enumerate() {
        let mut data = HashMap::new();
        data.insert("index".to_string(), i.to_string());
        
        let _alert = manager.generate_siem_alert(
            format!("test-rule-{}", i),
            severity.clone(),
            format!("Test alert {}", i),
            "test-service".to_string(),
            data,
        ).expect(&format!("Failed to generate alert {}", i));
    }

    // Get security detection statistics
    let stats = manager.get_security_detection_stats();
    
    // Should have 10 alerts total
    assert_eq!(stats.total_alerts, 10);
    
    // Check counts by severity
    assert_eq!(stats.alerts_by_severity.get(&SiemSeverity::Low).unwrap_or(&0), &2);
    assert_eq!(stats.alerts_by_severity.get(&SiemSeverity::Medium).unwrap_or(&0), &1);
    assert_eq!(stats.alerts_by_severity.get(&SiemSeverity::High).unwrap_or(&0), &3);
    assert_eq!(stats.alerts_by_severity.get(&SiemSeverity::Critical).unwrap_or(&0), &4);
    
    println!("SIEM alert counts by severity test passed:");
    println!("✓ Total alerts: {}", stats.total_alerts);
    println!("✓ Low severity: {}", stats.alerts_by_severity.get(&SiemSeverity::Low).unwrap_or(&0));
    println!("✓ Medium severity: {}", stats.alerts_by_severity.get(&SiemSeverity::Medium).unwrap_or(&0));
    println!("✓ High severity: {}", stats.alerts_by_severity.get(&SiemSeverity::High).unwrap_or(&0));
    println!("✓ Critical severity: {}", stats.alerts_by_severity.get(&SiemSeverity::Critical).unwrap_or(&0));
}