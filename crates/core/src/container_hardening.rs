//! Container Hardening Module
//!
//! This module implements container hardening measures including Kubernetes admission policies,
//! seccomp/AppArmor profiles, read-only filesystem configuration, and secrets management.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a Kubernetes admission policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdmissionPolicy {
    /// Policy name
    pub name: String,
    /// Policy description
    pub description: String,
    /// Policy rules
    pub rules: Vec<PolicyRule>,
    /// Whether the policy is enabled
    pub enabled: bool,
}

/// Represents a policy rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Rule expression
    pub expression: String,
    /// Rule enforcement mode
    pub mode: EnforcementMode,
}

/// Represents enforcement mode for policies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EnforcementMode {
    /// Enforce policy violations
    Enforce,
    /// Warn about policy violations
    Warn,
    /// Audit policy violations
    Audit,
}

/// Represents a seccomp profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeccompProfile {
    /// Profile name
    pub name: String,
    /// Profile description
    pub description: String,
    /// Allowed syscalls
    pub allowed_syscalls: Vec<String>,
    /// Blocked syscalls
    pub blocked_syscalls: Vec<String>,
    /// Default action for unspecified syscalls
    pub default_action: SeccompAction,
}

/// Represents a seccomp action
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SeccompAction {
    /// Allow the syscall
    Allow,
    /// Log the syscall
    Log,
    /// Block the syscall
    Block,
    /// Kill the process
    Kill,
}

/// Represents an AppArmor profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppArmorProfile {
    /// Profile name
    pub name: String,
    /// Profile description
    pub description: String,
    /// File access rules
    pub file_rules: HashMap<String, FileAccessRule>,
    /// Network access rules
    pub network_rules: HashMap<String, NetworkAccessRule>,
    /// Capability restrictions
    pub capability_restrictions: Vec<String>,
}

/// Represents a file access rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAccessRule {
    /// Path pattern
    pub path: String,
    /// Access permissions
    pub permissions: Vec<FilePermission>,
}

/// Represents file permissions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FilePermission {
    /// Read permission
    Read,
    /// Write permission
    Write,
    /// Execute permission
    Execute,
    /// Link permission
    Link,
}

/// Represents a network access rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAccessRule {
    /// Network family (e.g., "inet", "inet6")
    pub family: String,
    /// Network type (e.g., "stream", "dgram")
    pub r#type: String,
    /// Network protocol (e.g., "tcp", "udp")
    pub protocol: String,
    /// Access permission
    pub permission: NetworkPermission,
}

/// Represents network permissions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NetworkPermission {
    /// Allow network access
    Allow,
    /// Deny network access
    Deny,
}

/// Represents secrets management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretsManagement {
    /// Secret storage backend
    pub backend: SecretBackend,
    /// Secret rotation interval in seconds
    pub rotation_interval: u64,
    /// Secret encryption enabled
    pub encryption_enabled: bool,
    /// Secret access logging enabled
    pub access_logging_enabled: bool,
}

/// Represents secret storage backends
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecretBackend {
    /// HashiCorp Vault
    HashiCorpVault,
    /// Kubernetes Secrets
    KubernetesSecrets,
    /// AWS Secrets Manager
    AwsSecretsManager,
    /// Azure Key Vault
    AzureKeyVault,
    /// Google Secret Manager
    GoogleSecretManager,
}

/// Custom error type for container hardening operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContainerHardeningError {
    /// Policy not found
    PolicyNotFound(String),
    /// Profile not found
    ProfileNotFound(String),
    /// Secret not found
    SecretNotFound(String),
    /// Configuration error
    ConfigurationError(String),
    /// Generic error
    GenericError(String),
}

impl std::fmt::Display for ContainerHardeningError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContainerHardeningError::PolicyNotFound(name) => {
                write!(f, "Policy not found: {}", name)
            }
            ContainerHardeningError::ProfileNotFound(name) => {
                write!(f, "Profile not found: {}", name)
            }
            ContainerHardeningError::SecretNotFound(name) => {
                write!(f, "Secret not found: {}", name)
            }
            ContainerHardeningError::ConfigurationError(msg) => {
                write!(f, "Configuration error: {}", msg)
            }
            ContainerHardeningError::GenericError(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for ContainerHardeningError {}

/// Manages container hardening including policies, profiles, and secrets
pub struct ContainerHardeningManager {
    /// Kubernetes admission policies
    pub policies: HashMap<String, AdmissionPolicy>,
    /// Seccomp profiles
    pub seccomp_profiles: HashMap<String, SeccompProfile>,
    /// AppArmor profiles
    pub apparmor_profiles: HashMap<String, AppArmorProfile>,
    /// Secrets management configuration
    pub secrets_management: SecretsManagement,
}

impl ContainerHardeningManager {
    /// Create a new container hardening manager
    pub fn new(secrets_management: SecretsManagement) -> Self {
        Self {
            policies: HashMap::new(),
            seccomp_profiles: HashMap::new(),
            apparmor_profiles: HashMap::new(),
            secrets_management,
        }
    }

    /// Add an admission policy
    pub fn add_policy(&mut self, policy: AdmissionPolicy) {
        self.policies.insert(policy.name.clone(), policy);
    }

    /// Remove an admission policy
    pub fn remove_policy(&mut self, name: &str) -> Result<(), ContainerHardeningError> {
        if self.policies.remove(name).is_some() {
            Ok(())
        } else {
            Err(ContainerHardeningError::PolicyNotFound(name.to_string()))
        }
    }

    /// Get an admission policy
    pub fn get_policy(&self, name: &str) -> Option<&AdmissionPolicy> {
        self.policies.get(name)
    }

    /// Add a seccomp profile
    pub fn add_seccomp_profile(&mut self, profile: SeccompProfile) {
        self.seccomp_profiles.insert(profile.name.clone(), profile);
    }

    /// Remove a seccomp profile
    pub fn remove_seccomp_profile(&mut self, name: &str) -> Result<(), ContainerHardeningError> {
        if self.seccomp_profiles.remove(name).is_some() {
            Ok(())
        } else {
            Err(ContainerHardeningError::ProfileNotFound(name.to_string()))
        }
    }

    /// Get a seccomp profile
    pub fn get_seccomp_profile(&self, name: &str) -> Option<&SeccompProfile> {
        self.seccomp_profiles.get(name)
    }

    /// Add an AppArmor profile
    pub fn add_apparmor_profile(&mut self, profile: AppArmorProfile) {
        self.apparmor_profiles.insert(profile.name.clone(), profile);
    }

    /// Remove an AppArmor profile
    pub fn remove_apparmor_profile(&mut self, name: &str) -> Result<(), ContainerHardeningError> {
        if self.apparmor_profiles.remove(name).is_some() {
            Ok(())
        } else {
            Err(ContainerHardeningError::ProfileNotFound(name.to_string()))
        }
    }

    /// Get an AppArmor profile
    pub fn get_apparmor_profile(&self, name: &str) -> Option<&AppArmorProfile> {
        self.apparmor_profiles.get(name)
    }

    /// Validate container configuration against policies
    pub fn validate_container_config(
        &self,
        container_config: &ContainerConfig,
    ) -> Result<Vec<PolicyViolation>, ContainerHardeningError> {
        let mut violations = Vec::new();

        // Check each policy
        for policy in self.policies.values() {
            if policy.enabled {
                for rule in &policy.rules {
                    if let Some(violation) = self.check_rule_violation(rule, container_config) {
                        violations.push(violation);
                    }
                }
            }
        }

        Ok(violations)
    }

    /// Check if a rule is violated by a container configuration
    fn check_rule_violation(
        &self,
        rule: &PolicyRule,
        container_config: &ContainerConfig,
    ) -> Option<PolicyViolation> {
        // This is a simplified implementation
        // In a real implementation, this would evaluate the rule expression
        // against the container configuration
        match rule.mode {
            EnforcementMode::Enforce => {
                // For demonstration, we'll create a violation if the container
                // is configured to run as privileged, but only for rules that
                // specifically check for privileged containers
                if container_config.privileged
                    && (rule.name.contains("privileged") || rule.expression.contains("privileged"))
                {
                    Some(PolicyViolation {
                        policy_name: "default".to_string(),
                        rule_name: rule.name.clone(),
                        description: "Container is configured to run as privileged".to_string(),
                        severity: ViolationSeverity::High,
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Rotate secrets according to configuration
    pub fn rotate_secrets(&self) -> Result<(), ContainerHardeningError> {
        // In a real implementation, this would connect to the secret backend
        // and rotate secrets according to the rotation interval
        println!(
            "Rotating secrets using {:?} backend",
            self.secrets_management.backend
        );
        Ok(())
    }
}

/// Represents a container configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerConfig {
    /// Container name
    pub name: String,
    /// Image name
    pub image: String,
    /// Whether the container runs as privileged
    pub privileged: bool,
    /// Whether the container has a read-only root filesystem
    pub read_only_root_filesystem: bool,
    /// Container capabilities
    pub capabilities: Vec<String>,
    /// Container sysctls
    pub sysctls: HashMap<String, String>,
}

/// Represents a policy violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyViolation {
    /// Policy name
    pub policy_name: String,
    /// Rule name
    pub rule_name: String,
    /// Violation description
    pub description: String,
    /// Violation severity
    pub severity: ViolationSeverity,
}

/// Represents violation severity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ViolationSeverity {
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity
    High,
    /// Critical severity
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_hardening_manager_creation() {
        let secrets_management = SecretsManagement {
            backend: SecretBackend::KubernetesSecrets,
            rotation_interval: 86400,
            encryption_enabled: true,
            access_logging_enabled: true,
        };

        let manager = ContainerHardeningManager::new(secrets_management);
        assert_eq!(manager.policies.len(), 0);
        assert_eq!(manager.seccomp_profiles.len(), 0);
        assert_eq!(manager.apparmor_profiles.len(), 0);
    }

    #[test]
    fn test_policy_management() {
        let secrets_management = SecretsManagement {
            backend: SecretBackend::KubernetesSecrets,
            rotation_interval: 86400,
            encryption_enabled: true,
            access_logging_enabled: true,
        };

        let mut manager = ContainerHardeningManager::new(secrets_management);

        let policy = AdmissionPolicy {
            name: "test-policy".to_string(),
            description: "Test policy".to_string(),
            rules: vec![],
            enabled: true,
        };

        manager.add_policy(policy);
        assert_eq!(manager.policies.len(), 1);
        assert!(manager.get_policy("test-policy").is_some());

        assert!(manager.remove_policy("test-policy").is_ok());
        assert_eq!(manager.policies.len(), 0);
        assert!(manager.get_policy("test-policy").is_none());
    }

    #[test]
    fn test_seccomp_profile_management() {
        let secrets_management = SecretsManagement {
            backend: SecretBackend::KubernetesSecrets,
            rotation_interval: 86400,
            encryption_enabled: true,
            access_logging_enabled: true,
        };

        let mut manager = ContainerHardeningManager::new(secrets_management);

        let profile = SeccompProfile {
            name: "test-profile".to_string(),
            description: "Test profile".to_string(),
            allowed_syscalls: vec!["read".to_string(), "write".to_string()],
            blocked_syscalls: vec!["execve".to_string()],
            default_action: SeccompAction::Block,
        };

        manager.add_seccomp_profile(profile);
        assert_eq!(manager.seccomp_profiles.len(), 1);
        assert!(manager.get_seccomp_profile("test-profile").is_some());

        assert!(manager.remove_seccomp_profile("test-profile").is_ok());
        assert_eq!(manager.seccomp_profiles.len(), 0);
        assert!(manager.get_seccomp_profile("test-profile").is_none());
    }

    #[test]
    fn test_apparmor_profile_management() {
        let secrets_management = SecretsManagement {
            backend: SecretBackend::KubernetesSecrets,
            rotation_interval: 86400,
            encryption_enabled: true,
            access_logging_enabled: true,
        };

        let mut manager = ContainerHardeningManager::new(secrets_management);

        let profile = AppArmorProfile {
            name: "test-profile".to_string(),
            description: "Test profile".to_string(),
            file_rules: HashMap::new(),
            network_rules: HashMap::new(),
            capability_restrictions: vec!["CAP_NET_RAW".to_string()],
        };

        manager.add_apparmor_profile(profile);
        assert_eq!(manager.apparmor_profiles.len(), 1);
        assert!(manager.get_apparmor_profile("test-profile").is_some());

        assert!(manager.remove_apparmor_profile("test-profile").is_ok());
        assert_eq!(manager.apparmor_profiles.len(), 0);
        assert!(manager.get_apparmor_profile("test-profile").is_none());
    }

    #[test]
    fn test_secrets_rotation() {
        let secrets_management = SecretsManagement {
            backend: SecretBackend::KubernetesSecrets,
            rotation_interval: 86400,
            encryption_enabled: true,
            access_logging_enabled: true,
        };

        let manager = ContainerHardeningManager::new(secrets_management);
        assert!(manager.rotate_secrets().is_ok());
    }
}
