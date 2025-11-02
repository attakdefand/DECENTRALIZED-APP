//! Tests for Telemetry features: Basic Monitoring/Logging/Tracing
//!
//! These tests validate the implementation of Layer 8 requirements:
//! "Centralized logs, metrics, traces, span IDs across hops"
//! Goal: "See attacks and failures fast"
//! Evidence/Telemetry: "p95 latency, error rate, auth failures over time"

use core::observability::{
    ObservabilityManager, OtelCollector, PrometheusRule, SiemRule, SiemSeverity,
};
use std::collections::HashMap;

/// Test Basic Monitoring/Logging/Tracing features
#[test]
fn test_basic_monitoring_logging_tracing() {
    println!("Starting Basic Monitoring/Logging/Tracing test");

    // 1. Create observability manager
    let mut manager = ObservabilityManager::new();
    println!("✓ Observability manager created");

    // 2. Configure OpenTelemetry collector for centralized telemetry
    let collector = OtelCollector {
        id: "central-telemetry-collector".to_string(),
        endpoint: "http://otel-collector:4317".to_string(),
        telemetry_types: vec![
            "traces".to_string(),
            "metrics".to_string(),
            "logs".to_string(),
        ],
        sampling_rate: 1.0, // 100% sampling for testing
        export_interval: 10, // 10 seconds
    };

    assert!(manager.add_otel_collector(collector).is_ok());
    println!("✓ Centralized telemetry collector configured");

    // 3. Add Prometheus rules for monitoring key metrics
    let mut labels = HashMap::new();
    labels.insert("severity".to_string(), "critical".to_string());
    labels.insert("team".to_string(), "backend".to_string());

    let mut annotations = HashMap::new();
    annotations.insert(
        "summary".to_string(),
        "High API latency detected".to_string(),
    );

    // p95 latency alert rule
    let latency_rule = PrometheusRule {
        id: "api-p95-latency".to_string(),
        expr: "histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m])) > 1".to_string(),
        for_duration: "2m".to_string(),
        labels: labels.clone(),
        annotations: annotations.clone(),
    };

    assert!(manager.add_prometheus_rule(latency_rule).is_ok());
    println!("✓ p95 latency monitoring rule added");

    // Error rate alert rule
    annotations.insert(
        "summary".to_string(),
        "High error rate detected".to_string(),
    );
    let error_rate_rule = PrometheusRule {
        id: "high-error-rate".to_string(),
        expr: "rate(http_requests_total{status=~\"5..\"}[5m]) / rate(http_requests_total[5m]) > 0.05".to_string(),
        for_duration: "1m".to_string(),
        labels: labels.clone(),
        annotations: annotations.clone(),
    };

    assert!(manager.add_prometheus_rule(error_rate_rule).is_ok());
    println!("✓ Error rate monitoring rule added");

    // Authentication failure alert rule
    annotations.insert(
        "summary".to_string(),
        "High authentication failure rate detected".to_string(),
    );
    let auth_failure_rule = PrometheusRule {
        id: "high-auth-failures".to_string(),
        expr: "rate(auth_failures_total[5m]) > 10".to_string(),
        for_duration: "1m".to_string(),
        labels: labels.clone(),
        annotations: annotations.clone(),
    };

    assert!(manager.add_prometheus_rule(auth_failure_rule).is_ok());
    println!("✓ Authentication failure monitoring rule added");

    // 4. Add SIEM rules for security detection
    let attack_detection_rule = SiemRule {
        id: "attack-detection".to_string(),
        description: "Detects potential attacks and failures".to_string(),
        criteria: "error_rate > 0.05 OR auth_failures > 10".to_string(),
        severity: SiemSeverity::High,
        enabled: true,
    };

    assert!(manager.add_siem_rule(attack_detection_rule).is_ok());
    println!("✓ Attack detection SIEM rule added");

    // 5. Test centralized logging with structured data
    let mut log_fields = HashMap::new();
    log_fields.insert("user_id".to_string(), "user-123".to_string());
    log_fields.insert("request_id".to_string(), "req-abcde".to_string());
    log_fields.insert("endpoint".to_string(), "/api/users".to_string());

    let log_id = manager
        .record_log(
            "INFO".to_string(),
            "User accessed API endpoint".to_string(),
            "api-service".to_string(),
            Some("span-123".to_string()),
            Some("trace-456".to_string()),
            log_fields.clone(),
        )
        .expect("Failed to record log entry");

    println!("✓ Centralized log entry recorded with ID: {}", log_id);

    // Test error log
    let mut error_fields = HashMap::new();
    error_fields.insert("error_code".to_string(), "500".to_string());
    error_fields.insert("error_message".to_string(), "Database connection failed".to_string());

    let error_log_id = manager
        .record_log(
            "ERROR".to_string(),
            "Database connection failed".to_string(),
            "api-service".to_string(),
            Some("span-124".to_string()),
            Some("trace-457".to_string()),
            error_fields.clone(),
        )
        .expect("Failed to record error log entry");

    println!("✓ Error log entry recorded with ID: {}", error_log_id);

    // Test authentication failure log
    let mut auth_fields = HashMap::new();
    auth_fields.insert("user_id".to_string(), "user-456".to_string());
    auth_fields.insert("ip_address".to_string(), "192.168.1.100".to_string());
    auth_fields.insert("failure_reason".to_string(), "invalid_credentials".to_string());

    let auth_log_id = manager
        .record_log(
            "WARN".to_string(),
            "Authentication failed".to_string(),
            "auth-service".to_string(),
            Some("span-125".to_string()),
            Some("trace-458".to_string()),
            auth_fields.clone(),
        )
        .expect("Failed to record auth failure log entry");

    println!("✓ Authentication failure log entry recorded with ID: {}", auth_log_id);

    // 6. Test distributed tracing with span IDs across hops
    // First hop - API service
    let mut api_attributes = HashMap::new();
    api_attributes.insert("service".to_string(), "api-service".to_string());
    api_attributes.insert("operation".to_string(), "handle_request".to_string());

    let api_span_id = manager
        .start_span(
            "api-request".to_string(),
            "api-service".to_string(),
            None, // No parent for first hop
            api_attributes.clone(),
        )
        .expect("Failed to start API span");

    println!("✓ First hop API span started with ID: {}", api_span_id);

    // Second hop - Database service (child of API span)
    let mut db_attributes = HashMap::new();
    db_attributes.insert("service".to_string(), "database-service".to_string());
    db_attributes.insert("operation".to_string(), "query_users".to_string());
    db_attributes.insert("query".to_string(), "SELECT * FROM users".to_string());

    let db_span_id = manager
        .start_span(
            "database-query".to_string(),
            "database-service".to_string(),
            Some(api_span_id.clone()), // Parent is API span
            db_attributes.clone(),
        )
        .expect("Failed to start database span");

    println!("✓ Second hop database span started with ID: {}", db_span_id);

    // Third hop - Cache service (child of API span)
    let mut cache_attributes = HashMap::new();
    cache_attributes.insert("service".to_string(), "cache-service".to_string());
    cache_attributes.insert("operation".to_string(), "get_user_cache".to_string());
    cache_attributes.insert("key".to_string(), "user-123".to_string());

    let cache_span_id = manager
        .start_span(
            "cache-get".to_string(),
            "cache-service".to_string(),
            Some(api_span_id.clone()), // Parent is API span
            cache_attributes.clone(),
        )
        .expect("Failed to start cache span");

    println!("✓ Third hop cache span started with ID: {}", cache_span_id);

    // Complete spans
    assert!(manager.end_span(&db_span_id).is_ok());
    assert!(manager.end_span(&cache_span_id).is_ok());
    assert!(manager.end_span(&api_span_id).is_ok());
    println!("✓ All spans completed");

    // 7. Test telemetry statistics for evidence collection
    // Simulate normal requests
    manager.record_request(150, false, false); // 150ms latency, no error, no auth failure
    manager.record_request(200, false, false); // 200ms latency, no error, no auth failure
    manager.record_request(180, false, false); // 180ms latency, no error, no auth failure

    // Simulate error requests
    manager.record_request(500, true, false);  // 500ms latency, error, no auth failure
    manager.record_request(520, true, false);  // 520ms latency, error, no auth failure

    // Simulate authentication failures
    manager.record_request(401, true, true);   // 401ms latency, error, auth failure
    manager.record_request(403, true, true);   // 403ms latency, error, auth failure

    println!("✓ Telemetry statistics recorded");

    // 8. Verify configuration
    assert!(manager.validate_configuration().is_ok());
    println!("✓ Observability configuration validated");

    // 9. Retrieve and verify telemetry data
    // Check metrics
    let metrics = manager.get_metrics();
    assert!(metrics.is_empty()); // We're not recording metrics in this test, just showing the capability

    // Check logs
    let all_logs = manager.get_logs(None);
    assert_eq!(all_logs.len(), 3);
    println!("✓ Retrieved {} log entries", all_logs.len());

    // Check logs filtered by service
    let api_logs = manager.get_logs(Some("api-service"));
    assert_eq!(api_logs.len(), 2);
    println!("✓ Filtered logs by service, found {} entries", api_logs.len());

    // Check spans
    let spans = manager.get_spans();
    assert_eq!(spans.len(), 3);
    println!("✓ Retrieved {} trace spans", spans.len());

    // Verify span hierarchy
    let api_span = spans.get(&api_span_id).expect("API span not found");
    let db_span = spans.get(&db_span_id).expect("Database span not found");
    let cache_span = spans.get(&cache_span_id).expect("Cache span not found");

    assert_eq!(api_span.trace_id, db_span.trace_id);
    assert_eq!(api_span.trace_id, cache_span.trace_id);
    assert_eq!(db_span.parent_id.as_ref().unwrap(), &api_span_id);
    assert_eq!(cache_span.parent_id.as_ref().unwrap(), &api_span_id);
    println!("✓ Verified distributed tracing with span IDs across hops");

    // 10. Check telemetry statistics (evidence)
    let stats = manager.get_telemetry_stats();
    assert_eq!(stats.total_requests, 7);
    assert!(stats.error_rate > 0.0);
    assert_eq!(stats.auth_failures, 2);
    assert!(stats.latency_p95 > 0);
    println!("✓ Telemetry statistics verified:");
    println!("  - Total requests: {}", stats.total_requests);
    println!("  - Error rate: {:.2}%", stats.error_rate);
    println!("  - Auth failures: {}", stats.auth_failures);
    println!("  - p95 latency: {}ms", stats.latency_p95);

    // 11. Verify all components
    assert!(manager.get_otel_collector("central-telemetry-collector").is_some());
    assert!(manager.get_prometheus_rule("api-p95-latency").is_some());
    assert!(manager.get_prometheus_rule("high-error-rate").is_some());
    assert!(manager.get_prometheus_rule("high-auth-failures").is_some());
    assert!(manager.get_siem_rule("attack-detection").is_some());
    println!("✓ All observability components verified");

    println!("All Basic Monitoring/Logging/Tracing tests passed!");
}

/// Test evidence/telemetry collection for attacks and failures
#[test]
fn test_evidence_telemetry_for_attacks_and_failures() {
    let mut manager = ObservabilityManager::new();

    // Add required components
    let collector = OtelCollector {
        id: "test-collector".to_string(),
        endpoint: "http://localhost:4317".to_string(),
        telemetry_types: vec!["traces".to_string(), "metrics".to_string(), "logs".to_string()],
        sampling_rate: 1.0,
        export_interval: 30,
    };
    manager.add_otel_collector(collector).unwrap();

    let siem_rule = SiemRule {
        id: "test-rule".to_string(),
        description: "Test rule".to_string(),
        criteria: "test".to_string(),
        severity: SiemSeverity::Low,
        enabled: true,
    };
    manager.add_siem_rule(siem_rule).unwrap();

    // Add Prometheus rules for monitoring
    let latency_rule = PrometheusRule {
        id: "p95-latency".to_string(),
        expr: "histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m])) > 1".to_string(),
        for_duration: "2m".to_string(),
        labels: HashMap::new(),
        annotations: HashMap::new(),
    };
    manager.add_prometheus_rule(latency_rule).unwrap();

    // Simulate various scenarios to collect evidence
    // Scenario 1: Normal operation
    for i in 0..100 {
        manager.record_request(50 + (i % 200), false, false);
    }

    // Scenario 2: High latency period
    for i in 0..20 {
        manager.record_request(1500 + (i * 50), false, false);
    }

    // Scenario 3: Error spike
    for _ in 0..15 {
        manager.record_request(200, true, false);
    }

    // Scenario 4: Authentication failures
    for _ in 0..25 {
        manager.record_request(100, true, true);
    }

    // Get telemetry statistics (the evidence)
    let stats = manager.get_telemetry_stats();
    
    // Verify we can see attacks and failures fast through telemetry
    assert_eq!(stats.total_requests, 160);
    assert!(stats.error_rate > 0.0);
    assert_eq!(stats.auth_failures, 25);
    assert!(stats.latency_p95 > 0); // Should capture some latency
    
    println!("Evidence/Telemetry verification:");
    println!("✓ Total requests tracked: {}", stats.total_requests);
    println!("✓ Error rate detected: {:.2}%", stats.error_rate);
    println!("✓ Authentication failures detected: {}", stats.auth_failures);
    println!("✓ p95 latency captured: {}ms", stats.latency_p95);
    
    // Verify we can detect issues quickly
    assert!(stats.error_rate > 5.0); // More than 5% error rate should be detectable
    assert!(stats.auth_failures > 10); // More than 10 auth failures should be detectable
    
    println!("✓ Attacks and failures detected quickly through telemetry");
}

/// Test centralized logs with span and trace IDs
#[test]
fn test_centralized_logs_with_span_trace_ids() {
    let manager = ObservabilityManager::new();

    // Record multiple log entries with different span and trace IDs
    let mut fields1 = HashMap::new();
    fields1.insert("operation".to_string(), "user_login".to_string());
    
    let log1_id = manager.record_log(
        "INFO".to_string(),
        "User login successful".to_string(),
        "auth-service".to_string(),
        Some("span-001".to_string()),
        Some("trace-001".to_string()),
        fields1,
    ).unwrap();

    let mut fields2 = HashMap::new();
    fields2.insert("operation".to_string(), "data_query".to_string());
    fields2.insert("query_type".to_string(), "SELECT".to_string());
    
    let log2_id = manager.record_log(
        "DEBUG".to_string(),
        "Database query executed".to_string(),
        "db-service".to_string(),
        Some("span-002".to_string()),
        Some("trace-001".to_string()), // Same trace as log1
        fields2,
    ).unwrap();

    let mut fields3 = HashMap::new();
    fields3.insert("operation".to_string(), "cache_update".to_string());
    fields3.insert("cache_key".to_string(), "user-123".to_string());
    
    let log3_id = manager.record_log(
        "WARN".to_string(),
        "Cache miss occurred".to_string(),
        "cache-service".to_string(),
        Some("span-003".to_string()),
        Some("trace-002".to_string()), // Different trace
        fields3,
    ).unwrap();

    // Retrieve all logs
    let logs = manager.get_logs(None);
    assert_eq!(logs.len(), 3);

    // Verify log IDs
    assert!(!log1_id.is_empty());
    assert!(!log2_id.is_empty());
    assert!(!log3_id.is_empty());

    // Verify log content and trace/span IDs
    let log1 = logs.iter().find(|l| l.id == log1_id).unwrap();
    assert_eq!(log1.span_id.as_ref().unwrap(), "span-001");
    assert_eq!(log1.trace_id.as_ref().unwrap(), "trace-001");
    assert_eq!(log1.service, "auth-service");

    let log2 = logs.iter().find(|l| l.id == log2_id).unwrap();
    assert_eq!(log2.span_id.as_ref().unwrap(), "span-002");
    assert_eq!(log2.trace_id.as_ref().unwrap(), "trace-001"); // Same trace as log1
    assert_eq!(log2.service, "db-service");

    let log3 = logs.iter().find(|l| l.id == log3_id).unwrap();
    assert_eq!(log3.span_id.as_ref().unwrap(), "span-003");
    assert_eq!(log3.trace_id.as_ref().unwrap(), "trace-002"); // Different trace
    assert_eq!(log3.service, "cache-service");

    // Test filtering by service
    let auth_logs = manager.get_logs(Some("auth-service"));
    assert_eq!(auth_logs.len(), 1);
    assert_eq!(auth_logs[0].id, log1_id);

    let db_logs = manager.get_logs(Some("db-service"));
    assert_eq!(db_logs.len(), 1);
    assert_eq!(db_logs[0].id, log2_id);

    // Test filtering by trace ID (simulated by checking trace_id field)
    let trace_001_logs: Vec<_> = logs.iter().filter(|l| l.trace_id.as_ref().map_or(false, |id| id == "trace-001")).collect();
    assert_eq!(trace_001_logs.len(), 2); // log1 and log2

    let trace_002_logs: Vec<_> = logs.iter().filter(|l| l.trace_id.as_ref().map_or(false, |id| id == "trace-002")).collect();
    assert_eq!(trace_002_logs.len(), 1); // log3

    println!("✓ Centralized logs with span and trace IDs working correctly");
    println!("✓ Log correlation across services verified");
    println!("✓ Service-based log filtering working");
    println!("✓ Trace-based log correlation working");
}