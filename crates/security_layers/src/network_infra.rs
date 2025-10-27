//! Network & Infrastructure Security Module
//!
//! This module implements security layers 6: Network & Infrastructure Security
//! Specifically implementing:
//! - Perimeter Defense (Edge Firewall / CDN)
//! - Segmentation (Zero Trust / Microsegmentation)
//! - OSI Hardening (Protocol/Port Hygiene)
//! - Host Hardening (Baseline Images & CIS Benchmarks)
//! - Secrets on Host (Runtime Secret Mounting)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;

/// Configuration for Edge Firewall / CDN protection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeFirewallConfig {
    /// Enable DDoS protection
    pub ddos_protection: bool,
    /// Geo-blocking rules (country codes to block)
    pub geo_blocklist: Vec<String>,
    /// IP address blocklist
    pub ip_blocklist: Vec<IpAddr>,
    /// IP address allowlist
    pub ip_allowlist: Vec<IpAddr>,
    /// Layer 4 filtering rules
    pub l4_filtering: bool,
    /// Layer 7 filtering rules
    pub l7_filtering: bool,
    /// Rate limiting rules (requests per second per IP)
    pub rate_limit_rps: u32,
}

/// Telemetry data for Edge Firewall / CDN
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeFirewallTelemetry {
    /// Number of packets dropped at edge
    pub edge_drop_rate: u64,
    /// Volume of DDoS traffic absorbed
    pub ddos_absorbed_volume: u64,
    /// Number of blocked requests by geo
    pub geo_blocked_requests: u64,
    /// Number of blocked requests by IP
    pub ip_blocked_requests: u64,
    /// Number of rate limited requests
    pub rate_limited_requests: u64,
}

/// Configuration for network segmentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSegmentationConfig {
    /// Enable zero trust network architecture
    pub zero_trust_enabled: bool,
    /// Service-to-service allowlist rules
    pub service_allowlist: HashMap<String, Vec<String>>,
    /// Namespace isolation rules
    pub namespace_isolation: bool,
    /// VPC peering restrictions
    pub vpc_peering_restrictions: bool,
}

/// Telemetry data for network segmentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSegmentationTelemetry {
    /// Number of denied east-west traffic attempts
    pub denied_east_west_attempts: u64,
    /// Number of successful service-to-service communications
    pub successful_service_communications: u64,
    /// Number of segmentation policy violations
    pub policy_violations: u64,
}

/// Configuration for OSI hardening
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsiHardeningConfig {
    /// List of allowed ports
    pub allowed_ports: Vec<u16>,
    /// List of blocked ports
    pub blocked_ports: Vec<u16>,
    /// Enable TLS 1.2+ only
    pub tls_min_version: String,
    /// Disable legacy TLS ciphers
    pub disable_legacy_ciphers: bool,
    /// DNS security rules
    pub dns_security_rules: Vec<String>,
}

/// Telemetry data for OSI hardening
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsiHardeningTelemetry {
    /// Open ports compared to baseline
    pub open_port_diff: HashMap<u16, bool>, // true = open, false = closed
    /// Number of blocked connection attempts
    pub blocked_connection_attempts: u64,
    /// Number of DNS security violations
    pub dns_violations: u64,
}

/// Configuration for host hardening
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostHardeningConfig {
    /// Enable read-only root filesystem
    pub readonly_root_fs: bool,
    /// Minimal base image usage
    pub minimal_base_image: bool,
    /// Kernel hardening parameters
    pub kernel_hardening: HashMap<String, String>,
    /// SSH lockdown configuration
    pub ssh_lockdown: SshLockdownConfig,
}

/// SSH lockdown configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshLockdownConfig {
    /// Disable password authentication
    pub disable_password_auth: bool,
    /// Enable key-based authentication only
    pub key_based_auth_only: bool,
    /// Allowed SSH users
    pub allowed_users: Vec<String>,
    /// SSH port (default 22)
    pub ssh_port: u16,
}

/// Telemetry data for host hardening
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostHardeningTelemetry {
    /// Drift reports from baseline
    pub drift_reports: Vec<String>,
    /// CIS benchmark scores
    pub cis_scores: HashMap<String, f64>,
    /// Number of security violations
    pub security_violations: u64,
}

/// Configuration for runtime secret mounting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeSecretConfig {
    /// Enable tmpfs for secrets
    pub tmpfs_enabled: bool,
    /// Environment variable injection
    pub env_var_injection: bool,
    /// Secret rotation interval (seconds)
    pub rotation_interval: u64,
    /// Secret encryption at rest
    pub encryption_at_rest: bool,
}

/// Telemetry data for runtime secrets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeSecretTelemetry {
    /// Number of secrets injected at runtime
    pub secrets_injected: u64,
    /// Number of secrets found in images (should be 0)
    pub secrets_in_images: u64,
    /// Secret rotation events
    pub rotation_events: u64,
}

/// Main Network Infrastructure Security Manager
pub struct NetworkInfraManager {
    /// Edge firewall configuration
    edge_firewall_config: EdgeFirewallConfig,
    /// Edge firewall telemetry data
    edge_firewall_telemetry: EdgeFirewallTelemetry,
    
    /// Network segmentation configuration
    segmentation_config: NetworkSegmentationConfig,
    /// Network segmentation telemetry data
    segmentation_telemetry: NetworkSegmentationTelemetry,
    
    /// OSI hardening configuration
    osi_hardening_config: OsiHardeningConfig,
    /// OSI hardening telemetry data
    osi_hardening_telemetry: OsiHardeningTelemetry,
    
    /// Host hardening configuration
    host_hardening_config: HostHardeningConfig,
    /// Host hardening telemetry data
    host_hardening_telemetry: HostHardeningTelemetry,
    
    /// Runtime secret configuration
    runtime_secret_config: RuntimeSecretConfig,
    /// Runtime secret telemetry data
    runtime_secret_telemetry: RuntimeSecretTelemetry,
}

impl NetworkInfraManager {
    /// Create a new Network Infrastructure Security Manager
    pub fn new() -> Self {
        Self {
            edge_firewall_config: EdgeFirewallConfig {
                ddos_protection: true,
                geo_blocklist: vec![],
                ip_blocklist: vec![],
                ip_allowlist: vec![],
                l4_filtering: true,
                l7_filtering: true,
                rate_limit_rps: 100,
            },
            edge_firewall_telemetry: EdgeFirewallTelemetry {
                edge_drop_rate: 0,
                ddos_absorbed_volume: 0,
                geo_blocked_requests: 0,
                ip_blocked_requests: 0,
                rate_limited_requests: 0,
            },
            segmentation_config: NetworkSegmentationConfig {
                zero_trust_enabled: true,
                service_allowlist: HashMap::new(),
                namespace_isolation: true,
                vpc_peering_restrictions: true,
            },
            segmentation_telemetry: NetworkSegmentationTelemetry {
                denied_east_west_attempts: 0,
                successful_service_communications: 0,
                policy_violations: 0,
            },
            osi_hardening_config: OsiHardeningConfig {
                allowed_ports: vec![22, 80, 443, 3000, 3001], // SSH, HTTP, HTTPS, API ports
                blocked_ports: vec![],
                tls_min_version: "1.2".to_string(),
                disable_legacy_ciphers: true,
                dns_security_rules: vec!["block-private-dns-resolve".to_string()],
            },
            osi_hardening_telemetry: OsiHardeningTelemetry {
                open_port_diff: HashMap::new(),
                blocked_connection_attempts: 0,
                dns_violations: 0,
            },
            host_hardening_config: HostHardeningConfig {
                readonly_root_fs: true,
                minimal_base_image: true,
                kernel_hardening: HashMap::from([
                    ("kernel.modules_disabled".to_string(), "1".to_string()),
                    ("kernel.randomize_va_space".to_string(), "2".to_string()),
                ]),
                ssh_lockdown: SshLockdownConfig {
                    disable_password_auth: true,
                    key_based_auth_only: true,
                    allowed_users: vec!["admin".to_string(), "service".to_string()],
                    ssh_port: 22,
                },
            },
            host_hardening_telemetry: HostHardeningTelemetry {
                drift_reports: vec![],
                cis_scores: HashMap::new(),
                security_violations: 0,
            },
            runtime_secret_config: RuntimeSecretConfig {
                tmpfs_enabled: true,
                env_var_injection: true,
                rotation_interval: 3600, // 1 hour
                encryption_at_rest: true,
            },
            runtime_secret_telemetry: RuntimeSecretTelemetry {
                secrets_injected: 0,
                secrets_in_images: 0,
                rotation_events: 0,
            },
        }
    }

    /// Update edge firewall configuration
    pub fn update_edge_firewall_config(&mut self, config: EdgeFirewallConfig) -> Result<(), String> {
        // Validate configuration
        if config.rate_limit_rps > 10000 {
            return Err("Rate limit too high".to_string());
        }
        
        self.edge_firewall_config = config;
        Ok(())
    }

    /// Get current edge firewall configuration
    pub fn get_edge_firewall_config(&self) -> &EdgeFirewallConfig {
        &self.edge_firewall_config
    }

    /// Log edge firewall telemetry data
    pub fn log_edge_firewall_telemetry(&mut self, telemetry: EdgeFirewallTelemetry) {
        self.edge_firewall_telemetry = telemetry;
    }

    /// Get edge firewall telemetry data
    pub fn get_edge_firewall_telemetry(&self) -> &EdgeFirewallTelemetry {
        &self.edge_firewall_telemetry
    }

    /// Update network segmentation configuration
    pub fn update_segmentation_config(&mut self, config: NetworkSegmentationConfig) -> Result<(), String> {
        self.segmentation_config = config;
        Ok(())
    }

    /// Get current network segmentation configuration
    pub fn get_segmentation_config(&self) -> &NetworkSegmentationConfig {
        &self.segmentation_config
    }

    /// Log network segmentation telemetry data
    pub fn log_segmentation_telemetry(&mut self, telemetry: NetworkSegmentationTelemetry) {
        self.segmentation_telemetry = telemetry;
    }

    /// Get network segmentation telemetry data
    pub fn get_segmentation_telemetry(&self) -> &NetworkSegmentationTelemetry {
        &self.segmentation_telemetry
    }

    /// Update OSI hardening configuration
    pub fn update_osi_hardening_config(&mut self, config: OsiHardeningConfig) -> Result<(), String> {
        self.osi_hardening_config = config;
        Ok(())
    }

    /// Get current OSI hardening configuration
    pub fn get_osi_hardening_config(&self) -> &OsiHardeningConfig {
        &self.osi_hardening_config
    }

    /// Log OSI hardening telemetry data
    pub fn log_osi_hardening_telemetry(&mut self, telemetry: OsiHardeningTelemetry) {
        self.osi_hardening_telemetry = telemetry;
    }

    /// Get OSI hardening telemetry data
    pub fn get_osi_hardening_telemetry(&self) -> &OsiHardeningTelemetry {
        &self.osi_hardening_telemetry
    }

    /// Update host hardening configuration
    pub fn update_host_hardening_config(&mut self, config: HostHardeningConfig) -> Result<(), String> {
        self.host_hardening_config = config;
        Ok(())
    }

    /// Get current host hardening configuration
    pub fn get_host_hardening_config(&self) -> &HostHardeningConfig {
        &self.host_hardening_config
    }

    /// Log host hardening telemetry data
    pub fn log_host_hardening_telemetry(&mut self, telemetry: HostHardeningTelemetry) {
        self.host_hardening_telemetry = telemetry;
    }

    /// Get host hardening telemetry data
    pub fn get_host_hardening_telemetry(&self) -> &HostHardeningTelemetry {
        &self.host_hardening_telemetry
    }

    /// Update runtime secret configuration
    pub fn update_runtime_secret_config(&mut self, config: RuntimeSecretConfig) -> Result<(), String> {
        if config.rotation_interval < 60 {
            return Err("Rotation interval too short".to_string());
        }
        
        self.runtime_secret_config = config;
        Ok(())
    }

    /// Get current runtime secret configuration
    pub fn get_runtime_secret_config(&self) -> &RuntimeSecretConfig {
        &self.runtime_secret_config
    }

    /// Log runtime secret telemetry data
    pub fn log_runtime_secret_telemetry(&mut self, telemetry: RuntimeSecretTelemetry) {
        self.runtime_secret_telemetry = telemetry;
    }

    /// Get runtime secret telemetry data
    pub fn get_runtime_secret_telemetry(&self) -> &RuntimeSecretTelemetry {
        &self.runtime_secret_telemetry
    }

    /// Validate all network infrastructure configurations
    pub fn validate_all_configs(&self) -> Result<(), String> {
        // Validate edge firewall config
        if self.edge_firewall_config.rate_limit_rps > 10000 {
            return Err("Edge firewall rate limit too high".to_string());
        }
        
        // Validate host hardening config
        if self.host_hardening_config.ssh_lockdown.ssh_port == 0 {
            return Err("Invalid SSH port".to_string());
        }
        
        // Validate runtime secret config
        if self.runtime_secret_config.rotation_interval < 60 {
            return Err("Secret rotation interval too short".to_string());
        }
        
        Ok(())
    }
}

impl Default for NetworkInfraManager {
    fn default() -> Self {
        Self::new()
    }
}