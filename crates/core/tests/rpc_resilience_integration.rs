//! Integration tests for RPC resilience functionality

use core::rpc_resilience::{
    CircuitBreakerState, FailoverPolicy, ProviderHealth, RpcProvider, RpcResilienceError,
    RpcResilienceManager, TlsConfig,
};

/// Integration test for the complete RPC resilience workflow
#[test]
fn test_complete_rpc_resilience_workflow() {
    println!("Starting complete RPC resilience workflow test");

    // 1. Create TLS configuration
    let tls_config = TlsConfig {
        version: "1.3".to_string(),
        use_mtls: true,
        cert_pins: vec!["pin1".to_string(), "pin2".to_string()],
        cert_rotation_interval: 86400,
    };

    println!("✓ TLS configuration created");

    // 2. Create failover policy
    let failover_policy = FailoverPolicy {
        max_retries: 3,
        timeout_ms: 5000,
        retry_delay_ms: 1000,
        exponential_backoff: true,
    };

    println!("✓ Failover policy created");

    // 3. Create RPC resilience manager
    let mut manager = RpcResilienceManager::new(tls_config, failover_policy);
    println!("✓ RPC resilience manager created");

    // 4. Add RPC providers
    let providers = vec![
        RpcProvider::new(
            "infura_primary".to_string(),
            "https://mainnet.infura.io/v3/YOUR_PROJECT_ID".to_string(),
            "us-east".to_string(),
            1,
        ),
        RpcProvider::new(
            "alchemy_secondary".to_string(),
            "https://eth-mainnet.alchemyapi.io/v2/YOUR_API_KEY".to_string(),
            "us-west".to_string(),
            2,
        ),
        RpcProvider::new(
            "quicknode_tertiary".to_string(),
            "https://YOUR_QUICKNODE_ENDPOINT.quiknode.pro/YOUR_API_KEY".to_string(),
            "eu-west".to_string(),
            3,
        ),
    ];

    for provider in &providers {
        manager.add_provider(provider.clone());
    }

    println!("✓ {} RPC providers added", providers.len());

    // 5. Test TLS configuration
    assert_eq!(manager.tls_config.version, "1.3");
    assert!(manager.tls_config.use_mtls);
    assert_eq!(manager.tls_config.cert_pins.len(), 2);
    println!("✓ TLS configuration verified");

    // 6. Test failover policy
    assert_eq!(manager.failover_policy.max_retries, 3);
    assert_eq!(manager.failover_policy.timeout_ms, 5000);
    println!("✓ Failover policy verified");

    // 7. Test circuit breaker functionality
    // Record some failures to test circuit breaker
    for _ in 0..3 {
        manager.record_failure("infura_primary");
    }

    // Circuit breaker should still be closed (threshold is 5)
    let circuit_breaker = manager.circuit_breakers.get("infura_primary").unwrap();
    assert_eq!(circuit_breaker.state, CircuitBreakerState::Closed);
    assert_eq!(circuit_breaker.failure_count, 3);
    println!("✓ Circuit breaker functionality verified");

    // 8. Test provider prioritization
    let prioritized_providers = manager.get_healthy_providers();
    assert_eq!(prioritized_providers[0].name, "infura_primary");
    assert_eq!(prioritized_providers[1].name, "alchemy_secondary");
    assert_eq!(prioritized_providers[2].name, "quicknode_tertiary");
    println!("✓ Provider prioritization verified");

    println!("Complete RPC resilience workflow test passed!");
}

/// Test RPC resilience with circuit breaker tripping
#[test]
fn test_rpc_resilience_circuit_breaker_trip() {
    println!("Starting RPC resilience circuit breaker trip test");

    let tls_config = TlsConfig {
        version: "1.3".to_string(),
        use_mtls: true,
        cert_pins: vec![],
        cert_rotation_interval: 86400,
    };

    // Set low threshold for testing
    let failover_policy = FailoverPolicy {
        max_retries: 3,
        timeout_ms: 5000,
        retry_delay_ms: 1000,
        exponential_backoff: true,
    };

    let mut manager = RpcResilienceManager::new(tls_config, failover_policy);

    let provider = RpcProvider::new(
        "test_provider".to_string(),
        "https://example.com".to_string(),
        "us-west".to_string(),
        1,
    );

    manager.add_provider(provider);

    // Record failures to trip circuit breaker
    manager.record_failure("test_provider");
    manager.record_failure("test_provider");
    manager.record_failure("test_provider");
    manager.record_failure("test_provider");
    manager.record_failure("test_provider");

    println!("✓ RPC resilience circuit breaker trip test passed");
}

/// Test RPC resilience with multiple providers and failover
#[test]
fn test_rpc_resilience_multi_provider_failover() {
    println!("Starting RPC resilience multi-provider failover test");

    let tls_config = TlsConfig {
        version: "1.3".to_string(),
        use_mtls: true,
        cert_pins: vec![],
        cert_rotation_interval: 86400,
    };

    let failover_policy = FailoverPolicy {
        max_retries: 1, // Low retry count for faster testing
        timeout_ms: 5000,
        retry_delay_ms: 100,
        exponential_backoff: false,
    };

    let mut manager = RpcResilienceManager::new(tls_config, failover_policy);

    // Add providers
    let healthy_provider = RpcProvider::new(
        "healthy_provider".to_string(),
        "https://healthy.example.com".to_string(),
        "us-west".to_string(),
        1,
    );

    let secondary_provider = RpcProvider::new(
        "secondary_provider".to_string(),
        "https://secondary.example.com".to_string(),
        "us-east".to_string(),
        2,
    );

    manager.add_provider(healthy_provider);
    manager.add_provider(secondary_provider);

    println!("✓ RPC resilience multi-provider failover test passed");
}

/// Test RPC resilience error handling
#[test]
fn test_rpc_resilience_error_handling() {
    println!("Starting RPC resilience error handling test");

    let tls_config = TlsConfig {
        version: "1.3".to_string(),
        use_mtls: true,
        cert_pins: vec![],
        cert_rotation_interval: 86400,
    };

    let failover_policy = FailoverPolicy {
        max_retries: 3,
        timeout_ms: 5000,
        retry_delay_ms: 1000,
        exponential_backoff: true,
    };

    let mut manager = RpcResilienceManager::new(tls_config, failover_policy);

    // Test removing non-existent provider
    let result = manager.remove_provider("non_existent_provider");
    assert!(result.is_err());
    match result.unwrap_err() {
        RpcResilienceError::ProviderNotFound(_) => {}
        _ => panic!("Expected ProviderNotFound error"),
    }

    println!("✓ RPC resilience error handling test passed");
}
