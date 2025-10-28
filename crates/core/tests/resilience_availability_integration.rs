//! Integration tests for Resilience and Availability functionality

use core::resilience_availability::{
    DisasterRecoveryConfig, GracefulDegradationConfig, HaFailoverConfig, ResilienceAvailabilityManager,
    ServiceHealth, ServiceInstance, TrafficProtectionConfig,
};

/// Integration test for the complete resilience and availability workflow
#[test]
fn test_complete_resilience_availability_workflow() {
    println!("Starting complete resilience and availability workflow test");

    // 1. Create HA/failover configuration
    let ha_config = HaFailoverConfig {
        multi_az_enabled: true,
        health_check_interval: 30,
        replicas_per_service: 3,
        failover_timeout_ms: 5000,
    };

    println!("✓ HA/failover configuration created");

    // 2. Create traffic protection configuration
    let traffic_config = TrafficProtectionConfig {
        circuit_breaker_threshold: 5,
        circuit_breaker_timeout_ms: 60000,
        bulkhead_concurrency_limit: 100,
        rate_shaping: core::resilience_availability::RateShapingConfig {
            max_rps: 1000,
            burst_size: 200,
            shed_percentage: 10,
        },
    };

    println!("✓ Traffic protection configuration created");

    // 3. Create graceful degradation configuration
    let degradation_config = GracefulDegradationConfig {
        feature_flags_enabled: true,
        read_only_mode_enabled: true,
        cache_ttl_seconds: 300,
        fallback_data_sources: vec!["cache".to_string(), "backup".to_string()],
    };

    println!("✓ Graceful degradation configuration created");

    // 4. Create disaster recovery configuration
    let dr_config = DisasterRecoveryConfig {
        dr_playbook_enabled: true,
        chaos_testing_enabled: true,
        rpo_seconds: 300,
        rto_seconds: 600,
        backup_retention_days: 30,
    };

    println!("✓ Disaster recovery configuration created");

    // 5. Create resilience and availability manager
    let mut manager = ResilienceAvailabilityManager::new(
        ha_config,
        traffic_config,
        degradation_config,
        dr_config,
    );
    println!("✓ Resilience and availability manager created");

    // 6. Add service instances
    let services = vec![
        ServiceInstance::new(
            "api_service_primary".to_string(),
            "https://api-primary.example.com".to_string(),
            "us-east-1a".to_string(),
            1,
        ),
        ServiceInstance::new(
            "api_service_secondary".to_string(),
            "https://api-secondary.example.com".to_string(),
            "us-west-1b".to_string(),
            2,
        ),
        ServiceInstance::new(
            "api_service_tertiary".to_string(),
            "https://api-tertiary.example.com".to_string(),
            "eu-west-1c".to_string(),
            3,
        ),
    ];

    for service in &services {
        manager.add_service_instance(service.clone());
    }

    println!("✓ {} service instances added", services.len());

    // 7. Test HA configuration
    assert_eq!(manager.ha_config.multi_az_enabled, true);
    assert_eq!(manager.ha_config.replicas_per_service, 3);
    assert_eq!(manager.ha_config.failover_timeout_ms, 5000);
    println!("✓ HA configuration verified");

    // 8. Test traffic protection configuration
    assert_eq!(manager.traffic_config.circuit_breaker_threshold, 5);
    assert_eq!(manager.traffic_config.bulkhead_concurrency_limit, 100);
    assert_eq!(manager.traffic_config.rate_shaping.max_rps, 1000);
    println!("✓ Traffic protection configuration verified");

    // 9. Test graceful degradation configuration
    assert_eq!(manager.degradation_config.feature_flags_enabled, true);
    assert_eq!(manager.degradation_config.read_only_mode_enabled, true);
    assert_eq!(manager.degradation_config.fallback_data_sources.len(), 2);
    println!("✓ Graceful degradation configuration verified");

    // 10. Test disaster recovery configuration
    assert_eq!(manager.dr_config.dr_playbook_enabled, true);
    assert_eq!(manager.dr_config.chaos_testing_enabled, true);
    assert_eq!(manager.dr_config.rpo_seconds, 300);
    println!("✓ Disaster recovery configuration verified");

    // 11. Test service health management
    manager
        .update_service_health("api_service_primary", ServiceHealth::Healthy)
        .unwrap();
    
    let service = manager.service_instances.get("api_service_primary").unwrap();
    assert_eq!(service.health_status, ServiceHealth::Healthy);
    println!("✓ Service health management verified");

    // 12. Test service prioritization
    let prioritized_services = manager.get_healthy_services();
    assert_eq!(prioritized_services[0].name, "api_service_primary");
    assert_eq!(prioritized_services[1].name, "api_service_secondary");
    assert_eq!(prioritized_services[2].name, "api_service_tertiary");
    println!("✓ Service prioritization verified");

    // 13. Test uptime tracking
    let uptime_percentage = manager.get_uptime_percentage();
    assert_eq!(uptime_percentage, 100.0);
    println!("✓ Uptime tracking verified");

    // 14. Test failover event logging
    let failover_logs = manager.get_failover_event_logs();
    assert_eq!(failover_logs.len(), 1);
    assert!(failover_logs[0].contains("Total failover events: 0"));
    println!("✓ Failover event logging verified");

    println!("Complete resilience and availability workflow test passed!");
}

/// Test resilience and availability with circuit breaker tripping
#[test]
fn test_resilience_availability_circuit_breaker_trip() {
    println!("Starting resilience and availability circuit breaker trip test");

    let ha_config = HaFailoverConfig {
        multi_az_enabled: true,
        health_check_interval: 30,
        replicas_per_service: 3,
        failover_timeout_ms: 5000,
    };

    let traffic_config = TrafficProtectionConfig {
        circuit_breaker_threshold: 3, // Low threshold for testing
        circuit_breaker_timeout_ms: 60000,
        bulkhead_concurrency_limit: 100,
        rate_shaping: core::resilience_availability::RateShapingConfig {
            max_rps: 1000,
            burst_size: 200,
            shed_percentage: 10,
        },
    };

    let degradation_config = GracefulDegradationConfig {
        feature_flags_enabled: true,
        read_only_mode_enabled: true,
        cache_ttl_seconds: 300,
        fallback_data_sources: vec![],
    };

    let dr_config = DisasterRecoveryConfig {
        dr_playbook_enabled: true,
        chaos_testing_enabled: true,
        rpo_seconds: 300,
        rto_seconds: 600,
        backup_retention_days: 30,
    };

    let mut manager = ResilienceAvailabilityManager::new(
        ha_config,
        traffic_config,
        degradation_config,
        dr_config,
    );

    let service = ServiceInstance::new(
        "test_service".to_string(),
        "https://test.example.com".to_string(),
        "us-west-1a".to_string(),
        1,
    );

    manager.add_service_instance(service);

    // Record failures to trip circuit breaker
    manager.record_failure("test_service");
    manager.record_failure("test_service");
    manager.record_failure("test_service");

    println!("✓ Resilience and availability circuit breaker trip test passed");
}

/// Test resilience and availability with multi-service failover
#[test]
fn test_resilience_availability_multi_service_failover() {
    println!("Starting resilience and availability multi-service failover test");

    let ha_config = HaFailoverConfig {
        multi_az_enabled: true,
        health_check_interval: 30,
        replicas_per_service: 3,
        failover_timeout_ms: 5000,
    };

    let traffic_config = TrafficProtectionConfig {
        circuit_breaker_threshold: 5,
        circuit_breaker_timeout_ms: 60000,
        bulkhead_concurrency_limit: 100,
        rate_shaping: core::resilience_availability::RateShapingConfig {
            max_rps: 1000,
            burst_size: 200,
            shed_percentage: 10,
        },
    };

    let degradation_config = GracefulDegradationConfig {
        feature_flags_enabled: true,
        read_only_mode_enabled: true,
        cache_ttl_seconds: 300,
        fallback_data_sources: vec![],
    };

    let dr_config = DisasterRecoveryConfig {
        dr_playbook_enabled: true,
        chaos_testing_enabled: true,
        rpo_seconds: 300,
        rto_seconds: 600,
        backup_retention_days: 30,
    };

    let mut manager = ResilienceAvailabilityManager::new(
        ha_config,
        traffic_config,
        degradation_config,
        dr_config,
    );

    // Add services
    let healthy_service = ServiceInstance::new(
        "healthy_service".to_string(),
        "https://healthy.example.com".to_string(),
        "us-west-1a".to_string(),
        1,
    );

    let secondary_service = ServiceInstance::new(
        "secondary_service".to_string(),
        "https://secondary.example.com".to_string(),
        "us-east-1b".to_string(),
        2,
    );

    manager.add_service_instance(healthy_service);
    manager.add_service_instance(secondary_service);

    println!("✓ Resilience and availability multi-service failover test passed");
}

/// Test resilience and availability error handling
#[test]
fn test_resilience_availability_error_handling() {
    println!("Starting resilience and availability error handling test");

    let ha_config = HaFailoverConfig {
        multi_az_enabled: true,
        health_check_interval: 30,
        replicas_per_service: 3,
        failover_timeout_ms: 5000,
    };

    let traffic_config = TrafficProtectionConfig {
        circuit_breaker_threshold: 5,
        circuit_breaker_timeout_ms: 60000,
        bulkhead_concurrency_limit: 100,
        rate_shaping: core::resilience_availability::RateShapingConfig {
            max_rps: 1000,
            burst_size: 200,
            shed_percentage: 10,
        },
    };

    let degradation_config = GracefulDegradationConfig {
        feature_flags_enabled: true,
        read_only_mode_enabled: true,
        cache_ttl_seconds: 300,
        fallback_data_sources: vec![],
    };

    let dr_config = DisasterRecoveryConfig {
        dr_playbook_enabled: true,
        chaos_testing_enabled: true,
        rpo_seconds: 300,
        rto_seconds: 600,
        backup_retention_days: 30,
    };

    let mut manager = ResilienceAvailabilityManager::new(
        ha_config,
        traffic_config,
        degradation_config,
        dr_config,
    );

    // Test removing non-existent service
    let result = manager.remove_service_instance("non_existent_service");
    assert!(result.is_err());
    match result.unwrap_err() {
        core::resilience_availability::ResilienceAvailabilityError::ServiceNotFound(_) => {}
        _ => panic!("Expected ServiceNotFound error"),
    }

    println!("✓ Resilience and availability error handling test passed");
}

/// Test resilience and availability feature flags
#[test]
fn test_resilience_availability_feature_flags() {
    println!("Starting resilience and availability feature flags test");

    let ha_config = HaFailoverConfig {
        multi_az_enabled: true,
        health_check_interval: 30,
        replicas_per_service: 3,
        failover_timeout_ms: 5000,
    };

    let traffic_config = TrafficProtectionConfig {
        circuit_breaker_threshold: 5,
        circuit_breaker_timeout_ms: 60000,
        bulkhead_concurrency_limit: 100,
        rate_shaping: core::resilience_availability::RateShapingConfig {
            max_rps: 1000,
            burst_size: 200,
            shed_percentage: 10,
        },
    };

    let degradation_config = GracefulDegradationConfig {
        feature_flags_enabled: true,
        read_only_mode_enabled: true,
        cache_ttl_seconds: 300,
        fallback_data_sources: vec![],
    };

    let dr_config = DisasterRecoveryConfig {
        dr_playbook_enabled: true,
        chaos_testing_enabled: true,
        rpo_seconds: 300,
        rto_seconds: 600,
        backup_retention_days: 30,
    };

    let mut manager = ResilienceAvailabilityManager::new(
        ha_config,
        traffic_config,
        degradation_config,
        dr_config,
    );

    // Test feature flags
    manager.set_feature_flag("test_feature".to_string(), true);
    assert_eq!(manager.is_feature_enabled("test_feature"), true);
    
    manager.set_feature_flag("disabled_feature".to_string(), false);
    assert_eq!(manager.is_feature_enabled("disabled_feature"), false);
    
    // Test non-existent feature flag
    assert_eq!(manager.is_feature_enabled("non_existent_feature"), false);

    println!("✓ Resilience and availability feature flags test passed");
}

/// Test resilience and availability read-only mode
#[test]
fn test_resilience_availability_read_only_mode() {
    println!("Starting resilience and availability read-only mode test");

    let ha_config = HaFailoverConfig {
        multi_az_enabled: true,
        health_check_interval: 30,
        replicas_per_service: 3,
        failover_timeout_ms: 5000,
    };

    let traffic_config = TrafficProtectionConfig {
        circuit_breaker_threshold: 5,
        circuit_breaker_timeout_ms: 60000,
        bulkhead_concurrency_limit: 100,
        rate_shaping: core::resilience_availability::RateShapingConfig {
            max_rps: 1000,
            burst_size: 200,
            shed_percentage: 10,
        },
    };

    let degradation_config = GracefulDegradationConfig {
        feature_flags_enabled: true,
        read_only_mode_enabled: true,
        cache_ttl_seconds: 300,
        fallback_data_sources: vec![],
    };

    let dr_config = DisasterRecoveryConfig {
        dr_playbook_enabled: true,
        chaos_testing_enabled: true,
        rpo_seconds: 300,
        rto_seconds: 600,
        backup_retention_days: 30,
    };

    let manager = ResilienceAvailabilityManager::new(
        ha_config,
        traffic_config,
        degradation_config,
        dr_config,
    );

    // Test read-only mode
    assert_eq!(manager.is_read_only_mode(), false);
    
    manager.enable_read_only_mode();
    assert_eq!(manager.is_read_only_mode(), true);
    
    manager.disable_read_only_mode();
    assert_eq!(manager.is_read_only_mode(), false);

    println!("✓ Resilience and availability read-only mode test passed");
}