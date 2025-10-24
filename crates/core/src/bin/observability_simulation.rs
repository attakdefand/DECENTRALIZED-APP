//! Binary test runner for observability simulation tests
//!
//! This binary allows running the observability simulation tests as a standalone program.

use core::observability::{
    ObservabilityManager, OtelCollector, PrometheusRule, SiemRule, SiemSeverity,
};
use std::collections::HashMap;

fn main() {
    println!("Running Observability Simulation Tests");
    println!("=====================================\n");

    // Run all simulation tests
    test_production_monitoring();
    test_security_incident_response();
    test_compliance_audit_preparation();

    println!("All observability simulation tests completed successfully!");
}

fn test_production_monitoring() {
    println!("1. Testing Production System Monitoring...");

    let mut manager = ObservabilityManager::new();

    // Configure collectors
    let collector = OtelCollector {
        id: "prod-collector".to_string(),
        endpoint: "http://otel-prod:4317".to_string(),
        telemetry_types: vec![
            "traces".to_string(),
            "metrics".to_string(),
            "logs".to_string(),
        ],
        sampling_rate: 0.1,
        export_interval: 30,
    };
    assert!(manager.add_otel_collector(collector).is_ok());

    // Add Prometheus rules
    let mut labels = HashMap::new();
    labels.insert("severity".to_string(), "critical".to_string());

    let mut annotations = HashMap::new();
    annotations.insert("summary".to_string(), "High API latency".to_string());

    let rule = PrometheusRule {
        id: "api-latency".to_string(),
        expr: "rate(http_request_duration_seconds_sum[5m]) / rate(http_request_duration_seconds_count[5m]) > 1".to_string(),
        for_duration: "5m".to_string(),
        labels,
        annotations,
    };
    assert!(manager.add_prometheus_rule(rule).is_ok());

    // Add SIEM rule
    let siem_rule = SiemRule {
        id: "suspicious-transactions".to_string(),
        description: "Detects suspicious transactions".to_string(),
        criteria: "transaction_value > 1000000".to_string(),
        severity: SiemSeverity::High,
        enabled: true,
    };
    assert!(manager.add_siem_rule(siem_rule).is_ok());

    // Log admin action
    let mut metadata = HashMap::new();
    metadata.insert("environment".to_string(), "production".to_string());

    let log_id = manager
        .log_admin_action(
            "admin-user".to_string(),
            "deploy".to_string(),
            "api-service".to_string(),
            metadata,
            Some("192.168.1.100".to_string()),
        )
        .expect("Failed to log admin action");

    println!("   ✓ Logged admin action with ID: {}", log_id);

    // Validate
    assert!(manager.validate_configuration().is_ok());
    println!("   ✓ Production monitoring test passed\n");
}

fn test_security_incident_response() {
    println!("2. Testing Security Incident Response...");

    let mut manager = ObservabilityManager::new();

    // Set up security configuration
    let collector = OtelCollector {
        id: "security-collector".to_string(),
        endpoint: "http://otel-security:4317".to_string(),
        telemetry_types: vec!["logs".to_string()],
        sampling_rate: 1.0,
        export_interval: 5,
    };
    manager.add_otel_collector(collector).unwrap();

    // Add security rules
    let siem_rule = SiemRule {
        id: "unauthorized-access".to_string(),
        description: "Detects unauthorized access".to_string(),
        criteria: "failed_logins > 5".to_string(),
        severity: SiemSeverity::Critical,
        enabled: true,
    };
    manager.add_siem_rule(siem_rule).unwrap();

    // Add Prometheus rule for validation
    let mut labels = HashMap::new();
    labels.insert("severity".to_string(), "critical".to_string());

    let mut annotations = HashMap::new();
    annotations.insert(
        "summary".to_string(),
        "Unauthorized access detected".to_string(),
    );

    let prom_rule = PrometheusRule {
        id: "unauthorized-access-alert".to_string(),
        expr: "rate(unauthorized_access_attempts[1m]) > 10".to_string(),
        for_duration: "30s".to_string(),
        labels,
        annotations,
    };
    manager.add_prometheus_rule(prom_rule).unwrap();

    // Simulate incident detection
    let mut metadata = HashMap::new();
    metadata.insert(
        "incident_type".to_string(),
        "unauthorized_access".to_string(),
    );

    let log_id = manager
        .log_admin_action(
            "siem-system".to_string(),
            "threat-detected".to_string(),
            "user-accounts".to_string(),
            metadata,
            Some("10.0.5.1".to_string()),
        )
        .expect("Failed to log threat detection");

    println!("   ✓ Detected security threat with log ID: {}", log_id);

    // Validate
    assert!(manager.validate_configuration().is_ok());
    println!("   ✓ Security incident response test passed\n");
}

fn test_compliance_audit_preparation() {
    println!("3. Testing Compliance Audit Preparation...");

    let mut manager = ObservabilityManager::new();

    // Configure compliance monitoring
    let collector = OtelCollector {
        id: "compliance-collector".to_string(),
        endpoint: "http://otel-compliance:4317".to_string(),
        telemetry_types: vec![
            "traces".to_string(),
            "metrics".to_string(),
            "logs".to_string(),
        ],
        sampling_rate: 1.0,
        export_interval: 10,
    };
    manager.add_otel_collector(collector).unwrap();

    // Add compliance rules
    let siem_rule = SiemRule {
        id: "compliance-data-access".to_string(),
        description: "Monitors sensitive data access for compliance".to_string(),
        criteria: "sensitive_data_access = true".to_string(),
        severity: SiemSeverity::High,
        enabled: true,
    };
    manager.add_siem_rule(siem_rule).unwrap();

    // Add Prometheus rule for validation
    let mut labels = HashMap::new();
    labels.insert("severity".to_string(), "info".to_string());

    let mut annotations = HashMap::new();
    annotations.insert("summary".to_string(), "Compliance data access".to_string());

    let prom_rule = PrometheusRule {
        id: "compliance-access-alert".to_string(),
        expr: "rate(compliance_data_access[1h]) > 0".to_string(),
        for_duration: "1m".to_string(),
        labels,
        annotations,
    };
    manager.add_prometheus_rule(prom_rule).unwrap();

    // Log compliance action
    let mut metadata = HashMap::new();
    metadata.insert("compliance_standard".to_string(), "SOX".to_string());

    let log_id = manager
        .log_admin_action(
            "compliance-officer".to_string(),
            "policy-review".to_string(),
            "data-protection-policy".to_string(),
            metadata,
            Some("10.0.10.5".to_string()),
        )
        .expect("Failed to log compliance action");

    println!("   ✓ Logged compliance action with ID: {}", log_id);

    // Validate
    assert!(manager.validate_configuration().is_ok());
    println!("   ✓ Compliance audit preparation test passed\n");
}
