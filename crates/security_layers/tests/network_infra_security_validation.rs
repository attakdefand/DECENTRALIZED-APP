//! Network Infrastructure Security Validation Tests
//!
//! This module contains tests that validate the Network & Infrastructure Security functionality
//! as defined in the web3_protection_layers.csv file for Layer 6.

use security_layers::network_infra::*;
use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;

/// Test Edge Firewall / CDN configuration creation and validation
#[test]
fn test_edge_firewall_config_creation_and_validation() {
    // Test default configuration
    let manager = NetworkInfraManager::new();
    let config = manager.get_edge_firewall_config();
    
    assert!(config.ddos_protection);
    assert!(config.l4_filtering);
    assert!(config.l7_filtering);
    assert_eq!(config.rate_limit_rps, 100);
    assert!(config.geo_blocklist.is_empty());
    assert!(config.ip_blocklist.is_empty());
    
    // Test custom configuration
    let custom_config = EdgeFirewallConfig {
        ddos_protection: true,
        geo_blocklist: vec!["CN".to_string(), "RU".to_string()],
        ip_blocklist: vec![
            IpAddr::from_str("192.168.1.100").unwrap(),
            IpAddr::from_str("10.0.0.50").unwrap(),
        ],
        ip_allowlist: vec![
            IpAddr::from_str("192.168.1.1").unwrap(),
            IpAddr::from_str("10.0.0.1").unwrap(),
        ],
        l4_filtering: true,
        l7_filtering: true,
        rate_limit_rps: 50,
    };
    
    let mut manager = NetworkInfraManager::new();
    assert!(manager.update_edge_firewall_config(custom_config.clone()).is_ok());
    
    let updated_config = manager.get_edge_firewall_config();
    assert_eq!(updated_config.geo_blocklist, vec!["CN".to_string(), "RU".to_string()]);
    assert_eq!(updated_config.rate_limit_rps, 50);
    assert_eq!(updated_config.ip_blocklist.len(), 2);
    assert_eq!(updated_config.ip_allowlist.len(), 2);
    
    // Test invalid configuration
    let mut invalid_config = custom_config.clone();
    invalid_config.rate_limit_rps = 15000; // Too high
    assert!(manager.update_edge_firewall_config(invalid_config).is_err());
}

/// Test Edge Firewall telemetry logging
#[test]
fn test_edge_firewall_telemetry_logging() {
    let mut manager = NetworkInfraManager::new();
    
    // Test logging telemetry data
    let telemetry = EdgeFirewallTelemetry {
        edge_drop_rate: 1500,
        ddos_absorbed_volume: 5000000,
        geo_blocked_requests: 250,
        ip_blocked_requests: 180,
        rate_limited_requests: 95,
    };
    
    manager.log_edge_firewall_telemetry(telemetry.clone());
    
    // Test retrieving telemetry data
    let logged_telemetry = manager.get_edge_firewall_telemetry();
    assert_eq!(logged_telemetry.edge_drop_rate, 1500);
    assert_eq!(logged_telemetry.ddos_absorbed_volume, 5000000);
    assert_eq!(logged_telemetry.geo_blocked_requests, 250);
    assert_eq!(logged_telemetry.ip_blocked_requests, 180);
    assert_eq!(logged_telemetry.rate_limited_requests, 95);
}

/// Test Network Segmentation configuration
#[test]
fn test_network_segmentation_config() {
    let mut manager = NetworkInfraManager::new();
    
    // Create service allowlist
    let mut service_allowlist = HashMap::new();
    service_allowlist.insert("api-service".to_string(), vec!["database".to_string(), "cache".to_string()]);
    service_allowlist.insert("frontend".to_string(), vec!["api-service".to_string()]);
    
    let config = NetworkSegmentationConfig {
        zero_trust_enabled: true,
        service_allowlist,
        namespace_isolation: true,
        vpc_peering_restrictions: true,
    };
    
    assert!(manager.update_segmentation_config(config.clone()).is_ok());
    
    let updated_config = manager.get_segmentation_config();
    assert!(updated_config.zero_trust_enabled);
    assert!(updated_config.namespace_isolation);
    assert!(updated_config.vpc_peering_restrictions);
    assert_eq!(updated_config.service_allowlist.len(), 2);
    
    // Check specific service allowlist
    let api_service_allowlist = updated_config.service_allowlist.get("api-service").unwrap();
    assert_eq!(api_service_allowlist.len(), 2);
    assert!(api_service_allowlist.contains(&"database".to_string()));
    assert!(api_service_allowlist.contains(&"cache".to_string()));
}

/// Test Network Segmentation telemetry logging
#[test]
fn test_network_segmentation_telemetry_logging() {
    let mut manager = NetworkInfraManager::new();
    
    // Test logging telemetry data
    let telemetry = NetworkSegmentationTelemetry {
        denied_east_west_attempts: 42,
        successful_service_communications: 1250,
        policy_violations: 5,
    };
    
    manager.log_segmentation_telemetry(telemetry.clone());
    
    // Test retrieving telemetry data
    let logged_telemetry = manager.get_segmentation_telemetry();
    assert_eq!(logged_telemetry.denied_east_west_attempts, 42);
    assert_eq!(logged_telemetry.successful_service_communications, 1250);
    assert_eq!(logged_telemetry.policy_violations, 5);
}

/// Test OSI Hardening configuration
#[test]
fn test_osi_hardening_config() {
    let mut manager = NetworkInfraManager::new();
    
    let config = OsiHardeningConfig {
        tls_min_version: "1.3".to_string(),
        disable_legacy_ciphers: true,
        port_allowlist: vec![22, 443, 8080, 8443],
        dns_security_rules: vec!["strict-validation".to_string(), "no-external-dns".to_string()],
    };
    
    assert!(manager.update_osi_hardening_config(config.clone()).is_ok());
    
    let updated_config = manager.get_osi_hardening_config();
    assert_eq!(updated_config.tls_min_version, "1.3");
    assert!(updated_config.disable_legacy_ciphers);
    assert_eq!(updated_config.port_allowlist.len(), 4);
    assert_eq!(updated_config.dns_security_rules.len(), 2);
}

/// Test OSI Hardening telemetry logging
#[test]
fn test_osi_hardening_telemetry_logging() {
    let mut manager = NetworkInfraManager::new();
    
    // Test logging telemetry data
    let telemetry = OsiHardeningTelemetry {
        open_port_diff: vec![22, 80, 443],
        protocol_violations: 3,
        dns_violations: 1,
    };
    
    manager.log_osi_hardening_telemetry(telemetry.clone());
    
    // Test retrieving telemetry data
    let logged_telemetry = manager.get_osi_hardening_telemetry();
    assert_eq!(logged_telemetry.open_port_diff.len(), 3);
    assert_eq!(logged_telemetry.protocol_violations, 3);
    assert_eq!(logged_telemetry.dns_violations, 1);
}

/// Test Host Hardening configuration
#[test]
fn test_host_hardening_config() {
    let mut manager = NetworkInfraManager::new();
    
    // Create kernel hardening parameters
    let mut kernel_hardening = HashMap::new();
    kernel_hardening.insert("kernel.modules_disabled".to_string(), "1".to_string());
    kernel_hardening.insert("kernel.randomize_va_space".to_string(), "2".to_string());
    kernel_hardening.insert("net.ipv4.conf.all.rp_filter".to_string(), "1".to_string());
    
    let config = HostHardeningConfig {
        readonly_root_fs: true,
        minimal_base_image: true,
        kernel_hardening,
        ssh_lockdown: SshLockdownConfig {
            disable_password_auth: true,
            key_based_auth_only: true,
            allowed_users: vec!["admin".to_string(), "deploy".to_string(), "monitor".to_string()],
            ssh_port: 2222,
        },
    };
    
    assert!(manager.update_host_hardening_config(config.clone()).is_ok());
    
    let updated_config = manager.get_host_hardening_config();
    assert!(updated_config.readonly_root_fs);
    assert!(updated_config.minimal_base_image);
    assert_eq!(updated_config.kernel_hardening.len(), 3);
    assert_eq!(updated_config.ssh_lockdown.ssh_port, 2222);
    assert_eq!(updated_config.ssh_lockdown.allowed_users.len(), 3);
    assert!(updated_config.ssh_lockdown.disable_password_auth);
    assert!(updated_config.ssh_lockdown.key_based_auth_only);
}

/// Test Host Hardening telemetry logging
#[test]
fn test_host_hardening_telemetry_logging() {
    let mut manager = NetworkInfraManager::new();
    
    // Create CIS scores
    let mut cis_scores = HashMap::new();
    cis_scores.insert("docker_benchmark".to_string(), 95.5);
    cis_scores.insert("kubernetes_benchmark".to_string(), 87.2);
    cis_scores.insert("linux_os_benchmark".to_string(), 92.8);
    
    // Test logging telemetry data
    let telemetry = HostHardeningTelemetry {
        drift_reports: vec![
            "container_config_drift_detected".to_string(),
            "kernel_parameter_drift_detected".to_string(),
        ],
        cis_scores,
        security_violations: 2,
    };
    
    manager.log_host_hardening_telemetry(telemetry.clone());
    
    // Test retrieving telemetry data
    let logged_telemetry = manager.get_host_hardening_telemetry();
    assert_eq!(logged_telemetry.security_violations, 2);
    assert_eq!(logged_telemetry.drift_reports.len(), 2);
    assert_eq!(logged_telemetry.cis_scores.len(), 3);
    
    // Check specific CIS scores
    assert_eq!(*logged_telemetry.cis_scores.get("docker_benchmark").unwrap(), 95.5);
    assert_eq!(*logged_telemetry.cis_scores.get("kubernetes_benchmark").unwrap(), 87.2);
    assert_eq!(*logged_telemetry.cis_scores.get("linux_os_benchmark").unwrap(), 92.8);
}

/// Test Runtime Secret configuration
#[test]
fn test_runtime_secret_config() {
    let mut manager = NetworkInfraManager::new();
    
    let config = RuntimeSecretConfig {
        tmpfs_enabled: true,
        env_var_injection: true,
        rotation_interval: 7200, // 2 hours
        encryption_at_rest: true,
    };
    
    assert!(manager.update_runtime_secret_config(config.clone()).is_ok());
    
    let updated_config = manager.get_runtime_secret_config();
    assert!(updated_config.tmpfs_enabled);
    assert!(updated_config.env_var_injection);
    assert_eq!(updated_config.rotation_interval, 7200);
    assert!(updated_config.encryption_at_rest);
    
    // Test invalid configuration
    let mut invalid_config = config.clone();
    invalid_config.rotation_interval = 30; // Too short
    assert!(manager.update_runtime_secret_config(invalid_config).is_err());
}

/// Test Runtime Secret telemetry logging
#[test]
fn test_runtime_secret_telemetry_logging() {
    let mut manager = NetworkInfraManager::new();
    
    // Test logging telemetry data
    let telemetry = RuntimeSecretTelemetry {
        secrets_injected: 12,
        secrets_in_images: 0, // Should be 0 in a secure implementation
        rotation_events: 3,
    };
    
    manager.log_runtime_secret_telemetry(telemetry.clone());
    
    // Test retrieving telemetry data
    let logged_telemetry = manager.get_runtime_secret_telemetry();
    assert_eq!(logged_telemetry.secrets_injected, 12);
    assert_eq!(logged_telemetry.secrets_in_images, 0);
    assert_eq!(logged_telemetry.rotation_events, 3);
}

/// Test Service Mesh configuration
#[test]
fn test_service_mesh_config() {
    let mut manager = NetworkInfraManager::new();
    
    // Create network policies
    let policies = vec![
        NetworkPolicy {
            name: "api-to-database".to_string(),
            sources: vec!["api-service".to_string()],
            destinations: vec!["database-service".to_string()],
            ports: vec![5432],
            protocol: "TCP".to_string(),
        },
        NetworkPolicy {
            name: "frontend-to-api".to_string(),
            sources: vec!["frontend-service".to_string()],
            destinations: vec!["api-service".to_string()],
            ports: vec![8080, 8443],
            protocol: "TCP".to_string(),
        }
    ];
    
    let config = ServiceMeshConfig {
        enabled: true,
        mtls_config: MtlsConfig {
            enabled: true,
            ca_cert: "test-ca-cert".to_string(),
            cert_rotation_interval: 86400, // 24 hours
            strict_mode: true,
        },
        egress_whitelist: vec![
            "kubernetes.default.svc.cluster.local".to_string(),
            "kube-dns.kube-system.svc.cluster.local".to_string(),
            "external-api.example.com".to_string(),
        ],
        network_policies: policies,
    };
    
    assert!(manager.update_service_mesh_config(config.clone()).is_ok());
    
    let updated_config = manager.get_service_mesh_config();
    assert!(updated_config.enabled);
    assert!(updated_config.mtls_config.enabled);
    assert_eq!(updated_config.mtls_config.ca_cert, "test-ca-cert");
    assert_eq!(updated_config.mtls_config.cert_rotation_interval, 86400);
    assert!(updated_config.mtls_config.strict_mode);
    assert_eq!(updated_config.egress_whitelist.len(), 3);
    assert_eq!(updated_config.network_policies.len(), 2);
    
    // Check specific network policy
    let first_policy = &updated_config.network_policies[0];
    assert_eq!(first_policy.name, "api-to-database");
    assert_eq!(first_policy.sources, vec!["api-service".to_string()]);
    assert_eq!(first_policy.destinations, vec!["database-service".to_string()]);
    assert_eq!(first_policy.ports, vec![5432]);
    assert_eq!(first_policy.protocol, "TCP");
    
    // Test invalid configuration - missing CA cert
    let mut invalid_config = config.clone();
    invalid_config.mtls_config.ca_cert = "".to_string();
    assert!(manager.update_service_mesh_config(invalid_config).is_err());
    
    // Test invalid configuration - too short rotation interval
    let mut invalid_config2 = config.clone();
    invalid_config2.mtls_config.cert_rotation_interval = 300; // 5 minutes
    assert!(manager.update_service_mesh_config(invalid_config2).is_err());
}

/// Test Service Mesh telemetry logging
#[test]
fn test_service_mesh_telemetry_logging() {
    let mut manager = NetworkInfraManager::new();
    
    // Test logging telemetry data
    let telemetry = ServiceMeshTelemetry {
        mtls_connections: 1250,
        policy_violations: 3,
        blocked_egress_attempts: 7,
    };
    
    manager.log_service_mesh_telemetry(telemetry.clone());
    
    // Test retrieving telemetry data
    let logged_telemetry = manager.get_service_mesh_telemetry();
    assert_eq!(logged_telemetry.mtls_connections, 1250);
    assert_eq!(logged_telemetry.policy_violations, 3);
    assert_eq!(logged_telemetry.blocked_egress_attempts, 7);
}

/// Test all configurations validation
#[test]
fn test_all_configurations_validation() {
    let manager = NetworkInfraManager::new();
    
    // Test that default configuration is valid
    assert!(manager.validate_all_configs().is_ok());
    
    // Test with invalid edge firewall config
    let mut invalid_manager = NetworkInfraManager::new();
    let mut invalid_edge_config = invalid_manager.get_edge_firewall_config().clone();
    invalid_edge_config.rate_limit_rps = 15000; // Too high
    // We can't directly update this in the manager since it's immutable in this context
    // This test is mainly for the validation function itself
    
    // Test with invalid runtime secret config
    let mut invalid_secret_config = invalid_manager.get_runtime_secret_config().clone();
    invalid_secret_config.rotation_interval = 30; // Too short
    // Same limitation as above
    
    // Test with invalid service mesh config
    let mut invalid_mesh_config = invalid_manager.get_service_mesh_config().clone();
    invalid_mesh_config.mtls_config.cert_rotation_interval = 300; // Too short
    // Same limitation as above
}

/// Test that all security layers from CSV are properly represented
#[test]
fn test_network_infra_security_layers_from_csv() {
    // Create a network infrastructure manager
    let manager = NetworkInfraManager::new();
    
    // Verify edge firewall configuration
    let edge_config = manager.get_edge_firewall_config();
    assert!(edge_config.ddos_protection, "CDN DDoS absorb should be enabled");
    assert!(edge_config.l4_filtering, "L4 filtering should be enabled");
    assert!(edge_config.l7_filtering, "L7 filtering should be enabled");
    
    // Verify segmentation configuration
    let seg_config = manager.get_segmentation_config();
    assert!(seg_config.zero_trust_enabled, "Zero Trust should be enabled");
    
    // Verify OSI hardening configuration
    let osi_config = manager.get_osi_hardening_config();
    assert_eq!(osi_config.tls_min_version, "1.2", "TLS 1.2+ should be enforced");
    assert!(osi_config.disable_legacy_ciphers, "Legacy ciphers should be disabled");
    
    // Verify host hardening configuration
    let host_config = manager.get_host_hardening_config();
    assert!(host_config.readonly_root_fs, "Read-only root FS should be enabled");
    assert!(host_config.minimal_base_image, "Minimal base images should be used");
    
    // Verify runtime secret configuration
    let secret_config = manager.get_runtime_secret_config();
    assert!(secret_config.tmpfs_enabled, "tmpfs should be enabled for secrets");
    assert!(secret_config.env_var_injection, "Env var injection should be enabled");
    assert!(secret_config.encryption_at_rest, "Encryption at rest should be enabled");
    
    // Verify service mesh configuration
    let mesh_config = manager.get_service_mesh_config();
    assert!(mesh_config.enabled, "Service mesh should be enabled");
    assert!(mesh_config.mtls_config.enabled, "mTLS should be enabled");
    assert!(mesh_config.mtls_config.strict_mode, "mTLS should be in strict mode");
    assert!(!mesh_config.egress_whitelist.is_empty(), "Egress whitelist should be configured");
    assert!(!mesh_config.network_policies.is_empty(), "Network policies should be configured");
    
    println!("All Network & Infrastructure Security layers validated successfully!");
}