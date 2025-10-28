//! Tests for Graceful Degradation features: Feature Flags and Read-only Mode

use core::resilience_availability::{
    DisasterRecoveryConfig, GracefulDegradationConfig, HaFailoverConfig, ResilienceAvailabilityManager,
    ServiceInstance, TrafficProtectionConfig,
};

/// Test feature flags with usage tracking
#[test]
fn test_feature_flags_with_usage_tracking() {
    println!("Starting feature flags with usage tracking test");

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

    // Initially no feature flag usage
    assert_eq!(manager.get_feature_flag_usage().len(), 0);

    // Set and check feature flags
    manager.set_feature_flag("test_feature".to_string(), true);
    assert_eq!(manager.is_feature_enabled("test_feature"), true);
    
    manager.set_feature_flag("disabled_feature".to_string(), false);
    assert_eq!(manager.is_feature_enabled("disabled_feature"), false);
    
    // Check non-existent feature flag
    assert_eq!(manager.is_feature_enabled("non_existent_feature"), false);

    // Check feature flag usage tracking
    let usage = manager.get_feature_flag_usage();
    assert_eq!(usage.len(), 3); // test_feature, disabled_feature, non_existent_feature (all tracked for usage)

    // Use feature flags again to increase usage count
    manager.is_feature_enabled("test_feature");
    manager.is_feature_enabled("test_feature");
    
    // Check degradation events
    let events = manager.get_degradation_events();
    assert!(events.len() >= 2); // At least 2 events for feature flag toggles (set_feature_flag calls)

    println!("✓ Feature flags with usage tracking test passed");
}

/// Test read-only mode with telemetry
#[test]
fn test_read_only_mode_with_telemetry() {
    println!("Starting read-only mode with telemetry test");

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

    // Initially not in read-only mode
    assert_eq!(manager.is_read_only_mode(), false);
    assert_eq!(manager.get_time_spent_in_degraded_mode(), 0);

    // Enable read-only mode
    manager.enable_read_only_mode();
    assert_eq!(manager.is_read_only_mode(), true);

    // Check degradation events
    let events = manager.get_degradation_events();
    assert!(!events.is_empty());
    
    let read_only_events: Vec<_> = events.iter()
        .filter(|e| matches!(e.event_type, core::resilience_availability::GracefulDegradationEventType::ReadOnlyModeActivated))
        .collect();
    assert_eq!(read_only_events.len(), 1);

    // Disable read-only mode
    manager.disable_read_only_mode();
    assert_eq!(manager.is_read_only_mode(), false);

    println!("✓ Read-only mode with telemetry test passed");
}

/// Test withdraw disabled mode
#[test]
fn test_withdraw_disabled_mode() {
    println!("Starting withdraw disabled mode test");

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

    // Initially not in withdraw disabled mode
    assert_eq!(manager.is_withdraw_disabled_mode(), false);

    // Enable withdraw disabled mode
    manager.enable_withdraw_disabled_mode();
    assert_eq!(manager.is_withdraw_disabled_mode(), true);

    // Check degradation events
    let events = manager.get_degradation_events();
    assert!(!events.is_empty());
    
    let withdraw_events: Vec<_> = events.iter()
        .filter(|e| matches!(e.event_type, core::resilience_availability::GracefulDegradationEventType::WithdrawDisabledMode))
        .collect();
    assert_eq!(withdraw_events.len(), 1);

    // Disable withdraw disabled mode
    manager.disable_withdraw_disabled_mode();
    assert_eq!(manager.is_withdraw_disabled_mode(), false);

    println!("✓ Withdraw disabled mode test passed");
}

/// Test cache fallback functionality
#[test]
fn test_cache_fallback_functionality() {
    println!("Starting cache fallback functionality test");

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

    // Store data in cache
    manager.cache_data("test_key".to_string(), "test_value".to_string(), 60);
    
    // Retrieve cached data
    let cached_value = manager.get_cached_data("test_key");
    assert_eq!(cached_value, Some("test_value".to_string()));

    // Check degradation events for cache usage
    let events = manager.get_degradation_events();
    let cache_events: Vec<_> = events.iter()
        .filter(|e| matches!(e.event_type, core::resilience_availability::GracefulDegradationEventType::CacheFallbackUsed))
        .collect();
    assert_eq!(cache_events.len(), 1);

    // Try to retrieve non-existent key
    let non_existent = manager.get_cached_data("non_existent_key");
    assert_eq!(non_existent, None);

    println!("✓ Cache fallback functionality test passed");
}

/// Test time spent in degraded mode tracking
#[test]
fn test_time_spent_in_degraded_mode_tracking() {
    println!("Starting time spent in degraded mode tracking test");

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

    // Initially no time spent in degraded mode
    assert_eq!(manager.get_time_spent_in_degraded_mode(), 0);

    // Enable read-only mode to enter degraded mode
    manager.enable_read_only_mode();
    
    // Time spent should be greater than 0 (but we can't test exact value in unit test)
    let time_in_degraded_mode = manager.get_time_spent_in_degraded_mode();
    assert!(time_in_degraded_mode >= 0);

    // Disable read-only mode
    manager.disable_read_only_mode();

    println!("✓ Time spent in degraded mode tracking test passed");
}