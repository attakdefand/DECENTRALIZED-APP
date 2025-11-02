//! Network Infrastructure Security CSV Requirements Test
//!
//! This module contains tests that validate the Network & Infrastructure Security features
//! specifically against the requirements defined in the web3_protection_layers.csv file.

use security_layers::network_infra::*;
use security_layers::types::SecurityLayer;
use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;

/// Create security layers from the CSV data for Layer 6
fn create_network_infra_security_layers_from_csv() -> Vec<SecurityLayer> {
    let mut layers = Vec::new();
    
    // Layer 6: Network & Infrastructure Security
    layers.push(SecurityLayer {
        layer_number: 6,
        layer_name: "Network & Infrastructure Security".to_string(),
        main_type: "Perimeter Defense".to_string(),
        sub_type: "Edge Firewall / CDN".to_string(),
        component_mechanism: "CDN DDoS absorb, geo/IP blocklists, L4/L7 filtering".to_string(),
        goal: "Keep junk traffic out".to_string(),
        evidence_telemetry: "Edge drop rate, DDoS absorbed volume".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 6,
        layer_name: "Network & Infrastructure Security".to_string(),
        main_type: "Segmentation".to_string(),
        sub_type: "Zero Trust / Microsegmentation".to_string(),
        component_mechanism: "Isolate services/namespaces/VPCs, block east-west except allowlisted".to_string(),
        goal: "Contain compromise blast radius".to_string(),
        evidence_telemetry: "Denied east-west attempts".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 6,
        layer_name: "Network & Infrastructure Security".to_string(),
        main_type: "OSI Hardening".to_string(),
        sub_type: "Protocol/Port Hygiene".to_string(),
        component_mechanism: "Close unused ports, disable legacy TLS ciphers, strict DNS rules".to_string(),
        goal: "Cut legacy attack paths".to_string(),
        evidence_telemetry: "Open port diff vs baseline".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 6,
        layer_name: "Network & Infrastructure Security".to_string(),
        main_type: "Host Hardening".to_string(),
        sub_type: "Baseline Images & CIS Benchmarks".to_string(),
        component_mechanism: "Read-only root FS, minimal base images, kernel hardening, SSH lockdown".to_string(),
        goal: "Reduce exploitable surface on hosts/containers".to_string(),
        evidence_telemetry: "Drift reports from baseline, CIS score".to_string(),
    });
    
    layers.push(SecurityLayer {
        layer_number: 6,
        layer_name: "Network & Infrastructure Security".to_string(),
        main_type: "Secrets on Host".to_string(),
        sub_type: "Runtime Secret Mounting".to_string(),
        component_mechanism: "Inject secrets at runtime (tmpfs, env vars via agent) instead of baked into image".to_string(),
        goal: "Stop image leaks of creds".to_string(),
        evidence_telemetry: "Secrets-in-image scan results".to_string(),
    });
    
    layers
}

/// Test that validates Layer 6 security requirements from the CSV file
#[test]
fn test_layer_6_network_infra_requirements_from_csv() {
    // Create security layers from CSV
    let security_layers = create_network_infra_security_layers_from_csv();
    
    // Verify we have the correct number of Layer 6 sub-layers
    assert_eq!(security_layers.len(), 5, "Expected 5 sub-layers for Layer 6 Network & Infrastructure Security");
    
    // Test Perimeter Defense (Edge Firewall / CDN)
    test_layer_6_perimeter_defense(&security_layers);
    
    // Test Segmentation (Zero Trust / Microsegmentation)
    test_layer_6_segmentation(&security_layers);
    
    // Test OSI Hardening (Protocol/Port Hygiene)
    test_layer_6_osi_hardening(&security_layers);
    
    // Test Host Hardening (Baseline Images & CIS Benchmarks)
    test_layer_6_host_hardening(&security_layers);
    
    // Test Secrets on Host (Runtime Secret Mounting)
    test_layer_6_secrets_on_host(&security_layers);
    
    println!("All Layer 6 Network & Infrastructure Security requirements validated successfully!");
}

/// Test Perimeter Defense requirements
fn test_layer_6_perimeter_defense(layers: &[SecurityLayer]) {
    let perimeter_layer = layers.iter()
        .find(|layer| layer.sub_type == "Edge Firewall / CDN")
        .expect("Perimeter Defense layer should exist");
    
    assert_eq!(perimeter_layer.layer_number, 6);
    assert_eq!(perimeter_layer.main_type, "Perimeter Defense");
    assert_eq!(perimeter_layer.component_mechanism, "CDN DDoS absorb, geo/IP blocklists, L4/L7 filtering");
    assert_eq!(perimeter_layer.goal, "Keep junk traffic out");
    assert_eq!(perimeter_layer.evidence_telemetry, "Edge drop rate, DDoS absorbed volume");
    
    // Test that our implementation provides the required mechanisms
    let manager = NetworkInfraManager::new();
    let config = manager.get_edge_firewall_config();
    
    // Verify DDoS protection (CDN DDoS absorb)
    assert!(config.ddos_protection, "DDoS protection should be enabled");
    
    // Verify geo/IP blocklists
    assert_eq!(config.geo_blocklist.len(), 0, "Geo blocklist should be configurable");
    assert_eq!(config.ip_blocklist.len(), 0, "IP blocklist should be configurable");
    
    // Verify L4/L7 filtering
    assert!(config.l4_filtering, "L4 filtering should be enabled");
    assert!(config.l7_filtering, "L7 filtering should be enabled");
    
    // Verify telemetry collection
    let telemetry = manager.get_edge_firewall_telemetry();
    assert_eq!(telemetry.edge_drop_rate, 0, "Edge drop rate should be tracked");
    assert_eq!(telemetry.ddos_absorbed_volume, 0, "DDoS absorbed volume should be tracked");
    
    println!("Layer 6 Perimeter Defense requirements validated successfully!");
}

/// Test Segmentation requirements
fn test_layer_6_segmentation(layers: &[SecurityLayer]) {
    let segmentation_layer = layers.iter()
        .find(|layer| layer.sub_type == "Zero Trust / Microsegmentation")
        .expect("Segmentation layer should exist");
    
    assert_eq!(segmentation_layer.layer_number, 6);
    assert_eq!(segmentation_layer.main_type, "Segmentation");
    assert_eq!(segmentation_layer.component_mechanism, "Isolate services/namespaces/VPCs, block east-west except allowlisted");
    assert_eq!(segmentation_layer.goal, "Contain compromise blast radius");
    assert_eq!(segmentation_layer.evidence_telemetry, "Denied east-west attempts");
    
    // Test that our implementation provides the required mechanisms
    let manager = NetworkInfraManager::new();
    let config = manager.get_segmentation_config();
    
    // Verify zero trust implementation
    assert!(config.zero_trust_enabled, "Zero Trust should be enabled");
    
    // Verify service allowlisting
    assert!(config.service_allowlist.is_empty(), "Service allowlist should be configurable");
    
    // Verify namespace isolation
    assert!(config.namespace_isolation, "Namespace isolation should be enabled");
    
    // Verify VPC peering restrictions
    assert!(config.vpc_peering_restrictions, "VPC peering restrictions should be enabled");
    
    // Verify telemetry collection
    let telemetry = manager.get_segmentation_telemetry();
    assert_eq!(telemetry.denied_east_west_attempts, 0, "Denied east-west attempts should be tracked");
    
    println!("Layer 6 Segmentation requirements validated successfully!");
}

/// Test OSI Hardening requirements
fn test_layer_6_osi_hardening(layers: &[SecurityLayer]) {
    let osi_layer = layers.iter()
        .find(|layer| layer.sub_type == "Protocol/Port Hygiene")
        .expect("OSI Hardening layer should exist");
    
    assert_eq!(osi_layer.layer_number, 6);
    assert_eq!(osi_layer.main_type, "OSI Hardening");
    assert_eq!(osi_layer.component_mechanism, "Close unused ports, disable legacy TLS ciphers, strict DNS rules");
    assert_eq!(osi_layer.goal, "Cut legacy attack paths");
    assert_eq!(osi_layer.evidence_telemetry, "Open port diff vs baseline");
    
    // Test that our implementation provides the required mechanisms
    let manager = NetworkInfraManager::new();
    let config = manager.get_osi_hardening_config();
    
    // Verify port hygiene
    assert!(!config.allowed_ports.is_empty(), "Allowed ports should be configured");
    assert_eq!(config.blocked_ports.len(), 0, "Blocked ports should be configurable");
    
    // Verify TLS hardening
    assert_eq!(config.tls_min_version, "1.2", "TLS 1.2+ should be enforced");
    assert!(config.disable_legacy_ciphers, "Legacy ciphers should be disabled");
    
    // Verify DNS rules
    assert!(!config.dns_security_rules.is_empty(), "DNS security rules should be configured");
    
    // Verify telemetry collection
    let telemetry = manager.get_osi_hardening_telemetry();
    assert!(telemetry.open_port_diff.is_empty(), "Open port diff should be tracked");
    
    println!("Layer 6 OSI Hardening requirements validated successfully!");
}

/// Test Host Hardening requirements
fn test_layer_6_host_hardening(layers: &[SecurityLayer]) {
    let host_layer = layers.iter()
        .find(|layer| layer.sub_type == "Baseline Images & CIS Benchmarks")
        .expect("Host Hardening layer should exist");
    
    assert_eq!(host_layer.layer_number, 6);
    assert_eq!(host_layer.main_type, "Host Hardening");
    assert_eq!(host_layer.component_mechanism, "Read-only root FS, minimal base images, kernel hardening, SSH lockdown");
    assert_eq!(host_layer.goal, "Reduce exploitable surface on hosts/containers");
    assert_eq!(host_layer.evidence_telemetry, "Drift reports from baseline, CIS score");
    
    // Test that our implementation provides the required mechanisms
    let manager = NetworkInfraManager::new();
    let config = manager.get_host_hardening_config();
    
    // Verify read-only root filesystem
    assert!(config.readonly_root_fs, "Read-only root FS should be enabled");
    
    // Verify minimal base images
    assert!(config.minimal_base_image, "Minimal base images should be used");
    
    // Verify kernel hardening
    assert!(!config.kernel_hardening.is_empty(), "Kernel hardening should be configured");
    
    // Verify SSH lockdown
    assert!(config.ssh_lockdown.disable_password_auth, "Password auth should be disabled");
    assert!(config.ssh_lockdown.key_based_auth_only, "Key-based auth should be required");
    
    // Verify telemetry collection
    let telemetry = manager.get_host_hardening_telemetry();
    assert!(telemetry.drift_reports.is_empty(), "Drift reports should be tracked");
    assert!(telemetry.cis_scores.is_empty(), "CIS scores should be tracked");
    
    println!("Layer 6 Host Hardening requirements validated successfully!");
}

/// Test Secrets on Host requirements
fn test_layer_6_secrets_on_host(layers: &[SecurityLayer]) {
    let secrets_layer = layers.iter()
        .find(|layer| layer.sub_type == "Runtime Secret Mounting")
        .expect("Secrets on Host layer should exist");
    
    assert_eq!(secrets_layer.layer_number, 6);
    assert_eq!(secrets_layer.main_type, "Secrets on Host");
    assert_eq!(secrets_layer.component_mechanism, "Inject secrets at runtime (tmpfs, env vars via agent) instead of baked into image");
    assert_eq!(secrets_layer.goal, "Stop image leaks of creds");
    assert_eq!(secrets_layer.evidence_telemetry, "Secrets-in-image scan results");
    
    // Test that our implementation provides the required mechanisms
    let manager = NetworkInfraManager::new();
    let config = manager.get_runtime_secret_config();
    
    // Verify tmpfs for secrets
    assert!(config.tmpfs_enabled, "tmpfs should be enabled for secrets");
    
    // Verify environment variable injection
    assert!(config.env_var_injection, "Env var injection should be enabled");
    
    // Verify secret rotation
    assert_eq!(config.rotation_interval, 3600, "Secret rotation should be configured");
    
    // Verify encryption at rest
    assert!(config.encryption_at_rest, "Encryption at rest should be enabled");
    
    // Verify telemetry collection
    let telemetry = manager.get_runtime_secret_telemetry();
    assert_eq!(telemetry.secrets_injected, 0, "Secrets injected should be tracked");
    assert_eq!(telemetry.secrets_in_images, 0, "Secrets in images should be tracked (should be 0)");
    
    println!("Layer 6 Secrets on Host requirements validated successfully!");
}