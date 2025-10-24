//! Integration tests for the observability module
//!
//! These tests verify the complete observability workflow including
//! OpenTelemetry collector configuration, Prometheus rules, SIEM rules,
//! and admin audit logging.

use core::observability::{
    ObservabilityManager, OtelCollector, PrometheusRule, SiemRule, SiemSeverity
};
use std::collections::HashMap;

/// Integration test for the complete observability workflow
#[test]
fn test_complete_observability_workflow() {
    println!("Starting complete observability workflow test");
    
    // 1. Create observability manager
    let mut manager = ObservabilityManager::new();
    println!("✓ Observability manager created");
    
    // 2. Configure OpenTelemetry collector
    let collector = OtelCollector {
        id: "production-collector".to_string(),
        endpoint: "http://otel-collector:4317".to_string(),
        telemetry_types: vec![
            "traces".to_string(),
            "metrics".to_string(),
            "logs".to_string()
        ],
        sampling_rate: 0.1,
        export_interval: 30,
    };
    
    assert!(manager.add_otel_collector(collector).is_ok());
    println!("✓ OpenTelemetry collector configured");
    
    // 3. Add Prometheus alerting rules
    let mut labels = HashMap::new();
    labels.insert("severity".to_string(), "critical".to_string());
    labels.insert("team".to_string(), "backend".to_string());
    
    let mut annotations = HashMap::new();
    annotations.insert("summary".to_string(), "High API latency detected".to_string());
    annotations.insert("description".to_string(), "API latency has been above 1 second for more than 5 minutes".to_string());
    
    let api_latency_rule = PrometheusRule {
        id: "api-high-latency".to_string(),
        expr: "rate(http_request_duration_seconds_sum[5m]) / rate(http_request_duration_seconds_count[5m]) > 1".to_string(),
        for_duration: "5m".to_string(),
        labels: labels.clone(),
        annotations: annotations.clone(),
    };
    
    assert!(manager.add_prometheus_rule(api_latency_rule).is_ok());
    println!("✓ Prometheus API latency rule added");
    
    // 4. Add indexer lag rule
    labels.insert("service".to_string(), "indexer".to_string());
    annotations.insert("summary".to_string(), "Indexer synchronization lag detected".to_string());
    annotations.insert("description".to_string(), "Indexer is lagging behind the chain tip by more than 10 blocks".to_string());
    
    let indexer_lag_rule = PrometheusRule {
        id: "indexer-lag".to_string(),
        expr: "chain_tip - indexer_height > 10".to_string(),
        for_duration: "2m".to_string(),
        labels: labels.clone(),
        annotations: annotations.clone(),
    };
    
    assert!(manager.add_prometheus_rule(indexer_lag_rule).is_ok());
    println!("✓ Prometheus indexer lag rule added");
    
    // 5. Add SIEM rules
    let suspicious_tx_rule = SiemRule {
        id: "suspicious-transactions".to_string(),
        description: "Detects suspicious transaction patterns".to_string(),
        criteria: "transaction_value > 1000000 AND gas_price > average_gas_price * 5".to_string(),
        severity: SiemSeverity::High,
        enabled: true,
    };
    
    assert!(manager.add_siem_rule(suspicious_tx_rule).is_ok());
    println!("✓ SIEM suspicious transactions rule added");
    
    let unauthorized_access_rule = SiemRule {
        id: "unauthorized-access".to_string(),
        description: "Detects unauthorized access attempts".to_string(),
        criteria: "failed_auth_attempts > 5 AND ip_reputation_score < 0.3".to_string(),
        severity: SiemSeverity::Critical,
        enabled: true,
    };
    
    assert!(manager.add_siem_rule(unauthorized_access_rule).is_ok());
    println!("✓ SIEM unauthorized access rule added");
    
    // 6. Log administrative actions
    let mut metadata = HashMap::new();
    metadata.insert("environment".to_string(), "production".to_string());
    metadata.insert("service".to_string(), "api-server".to_string());
    
    let log_id = manager.log_admin_action(
        "admin-user".to_string(),
        "configuration-update".to_string(),
        "api-service-config".to_string(),
        metadata.clone(),
        Some("192.168.1.100".to_string()),
    ).expect("Failed to log admin action");
    
    println!("✓ Admin action logged with ID: {}", log_id);
    
    // Log another action
    metadata.insert("change_type".to_string(), "security-policy".to_string());
    let log_id2 = manager.log_admin_action(
        "security-admin".to_string(),
        "policy-update".to_string(),
        "access-control".to_string(),
        metadata,
        Some("192.168.1.101".to_string()),
    ).expect("Failed to log admin action");
    
    println!("✓ Security policy update logged with ID: {}", log_id2);
    
    // 7. Verify configuration
    assert!(manager.validate_configuration().is_ok());
    println!("✓ Observability configuration validated");
    
    // 8. Retrieve and verify components
    assert!(manager.get_otel_collector("production-collector").is_some());
    assert!(manager.get_prometheus_rule("api-high-latency").is_some());
    assert!(manager.get_siem_rule("suspicious-transactions").is_some());
    
    let audit_logs = manager.get_audit_logs(None);
    assert_eq!(audit_logs.len(), 2);
    println!("✓ All components retrieved and verified");
    
    // 9. Filter audit logs by user
    let admin_logs = manager.get_audit_logs(Some("admin-user"));
    assert_eq!(admin_logs.len(), 1);
    println!("✓ Audit logs filtered by user");
    
    println!("All observability workflow tests passed!");
}

/// Test Prometheus rule management
#[test]
fn test_prometheus_rule_management() {
    let mut manager = ObservabilityManager::new();
    
    // Add collector first (required for validation)
    let collector = OtelCollector {
        id: "test-collector".to_string(),
        endpoint: "http://localhost:4317".to_string(),
        telemetry_types: vec!["metrics".to_string()],
        sampling_rate: 1.0,
        export_interval: 15,
    };
    manager.add_otel_collector(collector).unwrap();
    
    // Add SIEM rule (required for validation)
    let siem_rule = SiemRule {
        id: "test-rule".to_string(),
        description: "Test rule".to_string(),
        criteria: "test".to_string(),
        severity: SiemSeverity::Low,
        enabled: true,
    };
    manager.add_siem_rule(siem_rule).unwrap();
    
    // Test adding multiple rules
    let rules_data = vec![
        ("high-cpu", "rate(process_cpu_seconds_total[5m]) > 0.8", "CPU usage above 80%"),
        ("high-memory", "process_resident_memory_bytes > 1073741824", "Memory usage above 1GB"),
        ("disk-full", "disk_used_percent > 90", "Disk usage above 90%"),
    ];
    
    for (id, expr, description) in rules_data {
        let mut labels = HashMap::new();
        labels.insert("severity".to_string(), "warning".to_string());
        
        let mut annotations = HashMap::new();
        annotations.insert("summary".to_string(), description.to_string());
        
        let rule = PrometheusRule {
            id: id.to_string(),
            expr: expr.to_string(),
            for_duration: "1m".to_string(),
            labels,
            annotations,
        };
        
        assert!(manager.add_prometheus_rule(rule).is_ok());
    }
    
    // Verify all rules were added
    assert!(manager.get_prometheus_rule("high-cpu").is_some());
    assert!(manager.get_prometheus_rule("high-memory").is_some());
    assert!(manager.get_prometheus_rule("disk-full").is_some());
    
    // Validate configuration
    assert!(manager.validate_configuration().is_ok());
}

/// Test SIEM rule management
#[test]
fn test_siem_rule_management() {
    let mut manager = ObservabilityManager::new();
    
    // Add collector first (required for validation)
    let collector = OtelCollector {
        id: "test-collector".to_string(),
        endpoint: "http://localhost:4317".to_string(),
        telemetry_types: vec!["logs".to_string()],
        sampling_rate: 1.0,
        export_interval: 15,
    };
    manager.add_otel_collector(collector).unwrap();
    
    // Add Prometheus rule (required for validation)
    let prom_rule = PrometheusRule {
        id: "test-rule".to_string(),
        expr: "test_expr".to_string(),
        for_duration: "1m".to_string(),
        labels: HashMap::new(),
        annotations: HashMap::new(),
    };
    manager.add_prometheus_rule(prom_rule).unwrap();
    
    // Test adding multiple SIEM rules with different severities
    let siem_rules_data = vec![
        ("brute-force", "failed_login_attempts > 10", SiemSeverity::Medium),
        ("data-exfiltration", "unusual_data_transfer > 100MB", SiemSeverity::High),
        ("privilege-escalation", "unauthorized_privilege_change", SiemSeverity::Critical),
    ];
    
    for (id, criteria, severity) in siem_rules_data {
        let rule = SiemRule {
            id: id.to_string(),
            description: format!("{} detection rule", id),
            criteria: criteria.to_string(),
            severity: severity.clone(),
            enabled: true,
        };
        
        assert!(manager.add_siem_rule(rule).is_ok());
    }
    
    // Verify all rules were added with correct severities
    let brute_force_rule = manager.get_siem_rule("brute-force").unwrap();
    assert!(matches!(brute_force_rule.severity, SiemSeverity::Medium));
    
    let data_exfil_rule = manager.get_siem_rule("data-exfiltration").unwrap();
    assert!(matches!(data_exfil_rule.severity, SiemSeverity::High));
    
    let priv_esc_rule = manager.get_siem_rule("privilege-escalation").unwrap();
    assert!(matches!(priv_esc_rule.severity, SiemSeverity::Critical));
    
    // Validate configuration
    assert!(manager.validate_configuration().is_ok());
}

/// Test audit log filtering and querying
#[test]
fn test_audit_log_filtering() {
    let mut manager = ObservabilityManager::new();
    
    // Add required components for validation
    let collector = OtelCollector {
        id: "test-collector".to_string(),
        endpoint: "http://localhost:4317".to_string(),
        telemetry_types: vec!["logs".to_string()],
        sampling_rate: 1.0,
        export_interval: 15,
    };
    manager.add_otel_collector(collector).unwrap();
    
    let prom_rule = PrometheusRule {
        id: "test-rule".to_string(),
        expr: "test_expr".to_string(),
        for_duration: "1m".to_string(),
        labels: HashMap::new(),
        annotations: HashMap::new(),
    };
    manager.add_prometheus_rule(prom_rule).unwrap();
    
    let siem_rule = SiemRule {
        id: "test-siem".to_string(),
        description: "Test SIEM rule".to_string(),
        criteria: "test".to_string(),
        severity: SiemSeverity::Low,
        enabled: true,
    };
    manager.add_siem_rule(siem_rule).unwrap();
    
    // Log actions from different users
    let users = vec!["alice", "bob", "charlie", "alice", "bob"];
    let actions = vec!["config-update", "deploy", "monitor", "patch", "rollback"];
    
    for (i, (user, action)) in users.iter().zip(actions.iter()).enumerate() {
        let mut metadata = HashMap::new();
        metadata.insert("session_id".to_string(), format!("sess-{}", i));
        
        manager.log_admin_action(
            user.to_string(),
            action.to_string(),
            "system".to_string(),
            metadata,
            Some(format!("192.168.1.{}", i + 10)),
        ).expect("Failed to log admin action");
    }
    
    // Test filtering by user
    let alice_logs = manager.get_audit_logs(Some("alice"));
    assert_eq!(alice_logs.len(), 2);
    
    let bob_logs = manager.get_audit_logs(Some("bob"));
    assert_eq!(bob_logs.len(), 2);
    
    let charlie_logs = manager.get_audit_logs(Some("charlie"));
    assert_eq!(charlie_logs.len(), 1);
    
    // Test getting all logs
    let all_logs = manager.get_audit_logs(None);
    assert_eq!(all_logs.len(), 5);
    
    // Validate configuration
    assert!(manager.validate_configuration().is_ok());
}