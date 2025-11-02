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
    /// Number of geo-blocked requests
    pub geo_blocked_requests: u64,
    /// Number of IP-blocked requests
    pub ip_blocked_requests: u64,
    /// Number of rate-limited requests
    pub rate_limited_requests: u64,
}

/// Configuration for Network Segmentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSegmentationConfig {
    /// Enable zero trust network architecture
    pub zero_trust_enabled: bool,
    /// Service communication allowlist (service -> allowed services)
    pub service_allowlist: HashMap<String, Vec<String>>,
    /// Enable namespace isolation
    pub namespace_isolation: bool,
    /// Enable VPC peering restrictions
    pub vpc_peering_restrictions: bool,
}

/// Telemetry data for Network Segmentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSegmentationTelemetry {
    /// Number of denied east-west communication attempts
    pub denied_east_west_attempts: u64,
    /// Number of successful service communications
    pub successful_service_communications: u64,
    /// Number of policy violations
    pub policy_violations: u64,
}

/// Configuration for OSI Hardening
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsiHardeningConfig {
    /// Minimum TLS version required
    pub tls_min_version: String,
    /// Disable legacy TLS ciphers
    pub disable_legacy_ciphers: bool,
    /// Open port allowlist
    pub port_allowlist: Vec<u16>,
    /// DNS security rules
    pub dns_security_rules: Vec<String>,
}

/// Telemetry data for OSI Hardening
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsiHardeningTelemetry {
    /// Open port differences from baseline
    pub open_port_diff: Vec<u16>,
    /// Number of protocol violations
    pub protocol_violations: u64,
    /// DNS security violations
    pub dns_violations: u64,
}

/// Configuration for Host Hardening
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostHardeningConfig {
    /// Enable read-only root filesystem
    pub readonly_root_fs: bool,
    /// Use minimal base images
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
    /// SSH port
    pub ssh_port: u16,
}

/// Telemetry data for Host Hardening
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostHardeningTelemetry {
    /// Drift reports from baseline
    pub drift_reports: Vec<String>,
    /// CIS benchmark scores
    pub cis_scores: HashMap<String, f64>,
    /// Security violations
    pub security_violations: u64,
}

/// Configuration for Runtime Secrets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeSecretConfig {
    /// Enable tmpfs for secrets
    pub tmpfs_enabled: bool,
    /// Enable environment variable injection
    pub env_var_injection: bool,
    /// Secret rotation interval (seconds)
    pub rotation_interval: u64,
    /// Enable encryption at rest for secrets
    pub encryption_at_rest: bool,
}

/// Telemetry data for Runtime Secrets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeSecretTelemetry {
    /// Number of secrets injected at runtime
    pub secrets_injected: u64,
    /// Number of secrets found in images (should be 0)
    pub secrets_in_images: u64,
    /// Secret rotation events
    pub rotation_events: u64,
}

/// Configuration for Service Mesh
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMeshConfig {
    /// Enable service mesh
    pub enabled: bool,
    /// mTLS configuration
    pub mtls_config: MtlsConfig,
    /// Egress whitelist configuration
    pub egress_whitelist: Vec<String>,
    /// Network policy configuration
    pub network_policies: Vec<NetworkPolicy>,
}

/// mTLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MtlsConfig {
    /// Enable mTLS
    pub enabled: bool,
    /// Certificate authority for mTLS
    pub ca_cert: String,
    /// Certificate rotation interval (seconds)
    pub cert_rotation_interval: u64,
    /// Mutual authentication strictness
    pub strict_mode: bool,
}

/// Network policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPolicy {
    /// Policy name
    pub name: String,
    /// Source services
    pub sources: Vec<String>,
    /// Destination services
    pub destinations: Vec<String>,
    /// Allowed ports
    pub ports: Vec<u16>,
    /// Protocol (TCP, UDP, etc.)
    pub protocol: String,
}

/// Telemetry data for Service Mesh
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMeshTelemetry {
    /// Number of mTLS connections
    pub mtls_connections: u64,
    /// Number of policy violations
    pub policy_violations: u64,
    /// Number of blocked egress attempts
    pub blocked_egress_attempts: u64,
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
    
    /// Service mesh configuration
    service_mesh_config: ServiceMeshConfig,
    /// Service mesh telemetry data
    service_mesh_telemetry: ServiceMeshTelemetry,
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
                tls_min_version: "1.2".to_string(),
                disable_legacy_ciphers: true,
                port_allowlist: vec![22, 80, 443, 8080, 8545],
                dns_security_rules: vec!["strict-validation".to_string()],
            },
            osi_hardening_telemetry: OsiHardeningTelemetry {
                open_port_diff: vec![],
                protocol_violations: 0,
                dns_violations: 0,
            },
            host_hardening_config: HostHardeningConfig {
                readonly_root_fs: true,
                minimal_base_image: true,
                kernel_hardening: {
                    let mut map = HashMap::new();
                    map.insert("kernel.modules_disabled".to_string(), "1".to_string());
                    map.insert("kernel.randomize_va_space".to_string(), "2".to_string());
                    map.insert("net.ipv4.conf.all.rp_filter".to_string(), "1".to_string());
                    map
                },
                ssh_lockdown: SshLockdownConfig {
                    disable_password_auth: true,
                    key_based_auth_only: true,
                    allowed_users: vec!["admin".to_string(), "deploy".to_string()],
                    ssh_port: 2222,
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
                rotation_interval: 7200, // 2 hours
                encryption_at_rest: true,
            },
            runtime_secret_telemetry: RuntimeSecretTelemetry {
                secrets_injected: 0,
                secrets_in_images: 0,
                rotation_events: 0,
            },
            service_mesh_config: ServiceMeshConfig {
                enabled: true,
                mtls_config: MtlsConfig {
                    enabled: true,
                    ca_cert: "default-ca-cert".to_string(),
                    cert_rotation_interval: 86400, // 24 hours
                    strict_mode: true,
                },
                egress_whitelist: vec![
                    "kubernetes.default.svc.cluster.local".to_string(),
                    "kube-dns.kube-system.svc.cluster.local".to_string(),
                ],
                network_policies: vec![],
            },
            service_mesh_telemetry: ServiceMeshTelemetry {
                mtls_connections: 0,
                policy_violations: 0,
                blocked_egress_attempts: 0,
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
    
    /// Get service mesh configuration
    pub fn get_service_mesh_config(&self) -> &ServiceMeshConfig {
        &self.service_mesh_config
    }
    
    /// Update service mesh configuration
    pub fn update_service_mesh_config(&mut self, config: ServiceMeshConfig) -> Result<(), String> {
        // Validate configuration
        if config.mtls_config.enabled && config.mtls_config.ca_cert.is_empty() {
            return Err("CA certificate is required when mTLS is enabled".to_string());
        }
        
        if config.mtls_config.cert_rotation_interval < 3600 {
            return Err("Certificate rotation interval must be at least 1 hour".to_string());
        }
        
        self.service_mesh_config = config;
        Ok(())
    }
    
    /// Log service mesh telemetry
    pub fn log_service_mesh_telemetry(&mut self, telemetry: ServiceMeshTelemetry) {
        self.service_mesh_telemetry = telemetry;
    }
    
    /// Get service mesh telemetry
    pub fn get_service_mesh_telemetry(&self) -> &ServiceMeshTelemetry {
        &self.service_mesh_telemetry
    }
    
    /// Validate all configurations
    pub fn validate_all_configs(&self) -> Result<(), String> {
        // Validate edge firewall config
        if self.edge_firewall_config.rate_limit_rps > 10000 {
            return Err("Rate limit RPS must be <= 10000".to_string());
        }
        
        // Validate runtime secret config
        if self.runtime_secret_config.rotation_interval < 300 {
            return Err("Secret rotation interval must be at least 5 minutes".to_string());
        }
        
        // Validate service mesh config
        if self.service_mesh_config.mtls_config.enabled && 
           self.service_mesh_config.mtls_config.cert_rotation_interval < 3600 {
            return Err("Certificate rotation interval must be at least 1 hour".to_string());
        }
        
        Ok(())
    }
}

impl Default for NetworkInfraManager {
    fn default() -> Self {
        Self::new()
    }
}