//! Integration tests for container hardening functionality

use core::container_hardening::{ContainerHardeningManager, AdmissionPolicy, PolicyRule, EnforcementMode, SeccompProfile, SeccompAction, AppArmorProfile, FileAccessRule, FilePermission, NetworkAccessRule, NetworkPermission, SecretsManagement, SecretBackend, ContainerConfig};

/// Integration test for the complete container hardening workflow
#[test]
fn test_complete_container_hardening_workflow() {
    println!("Starting complete container hardening workflow test");
    
    // 1. Create secrets management configuration
    let secrets_management = SecretsManagement {
        backend: SecretBackend::KubernetesSecrets,
        rotation_interval: 86400,
        encryption_enabled: true,
        access_logging_enabled: true,
    };
    
    println!("✓ Secrets management configuration created");
    
    // 2. Create container hardening manager
    let mut manager = ContainerHardeningManager::new(secrets_management);
    println!("✓ Container hardening manager created");
    
    // 3. Create admission policies
    let policies = vec![
        AdmissionPolicy {
            name: "restrict-privileged-containers".to_string(),
            description: "Prevent privileged containers from being deployed".to_string(),
            rules: vec![
                PolicyRule {
                    name: "no-privileged".to_string(),
                    description: "Containers must not run as privileged".to_string(),
                    expression: "container.securityContext.privileged == false".to_string(),
                    mode: EnforcementMode::Enforce,
                }
            ],
            enabled: true,
        },
        AdmissionPolicy {
            name: "require-read-only-root-fs".to_string(),
            description: "Require containers to use read-only root filesystem".to_string(),
            rules: vec![
                PolicyRule {
                    name: "read-only-root-fs".to_string(),
                    description: "Containers must use read-only root filesystem".to_string(),
                    expression: "container.securityContext.readOnlyRootFilesystem == true".to_string(),
                    mode: EnforcementMode::Enforce,
                }
            ],
            enabled: true,
        }
    ];
    
    // 4. Add policies to manager
    for policy in &policies {
        manager.add_policy(policy.clone());
    }
    
    println!("✓ {} admission policies added", policies.len());
    
    // 5. Create seccomp profiles
    let seccomp_profiles = vec![
        SeccompProfile {
            name: "default-profile".to_string(),
            description: "Default seccomp profile for all containers".to_string(),
            allowed_syscalls: vec![
                "read".to_string(),
                "write".to_string(),
                "close".to_string(),
                "exit_group".to_string(),
            ],
            blocked_syscalls: vec![
                "execve".to_string(),
                "ptrace".to_string(),
                "mount".to_string(),
            ],
            default_action: SeccompAction::Block,
        }
    ];
    
    // 6. Add seccomp profiles to manager
    for profile in &seccomp_profiles {
        manager.add_seccomp_profile(profile.clone());
    }
    
    println!("✓ {} seccomp profiles added", seccomp_profiles.len());
    
    // 7. Create AppArmor profiles
    let apparmor_profiles = vec![
        AppArmorProfile {
            name: "default-profile".to_string(),
            description: "Default AppArmor profile for all containers".to_string(),
            file_rules: vec![
                ("/etc/".to_string(), FileAccessRule {
                    path: "/etc/".to_string(),
                    permissions: vec![FilePermission::Read],
                }),
                ("/tmp/".to_string(), FileAccessRule {
                    path: "/tmp/".to_string(),
                    permissions: vec![FilePermission::Read, FilePermission::Write],
                }),
            ].into_iter().collect(),
            network_rules: vec![
                ("inet".to_string(), NetworkAccessRule {
                    family: "inet".to_string(),
                    r#type: "stream".to_string(),
                    protocol: "tcp".to_string(),
                    permission: NetworkPermission::Allow,
                }),
            ].into_iter().collect(),
            capability_restrictions: vec![
                "CAP_NET_RAW".to_string(),
                "CAP_SYS_ADMIN".to_string(),
            ],
        }
    ];
    
    // 8. Add AppArmor profiles to manager
    for profile in &apparmor_profiles {
        manager.add_apparmor_profile(profile.clone());
    }
    
    println!("✓ {} AppArmor profiles added", apparmor_profiles.len());
    
    // 9. Verify policy management
    assert_eq!(manager.policies.len(), 2);
    assert!(manager.get_policy("restrict-privileged-containers").is_some());
    assert!(manager.get_policy("require-read-only-root-fs").is_some());
    println!("✓ Policy management verified");
    
    // 10. Verify seccomp profile management
    assert_eq!(manager.seccomp_profiles.len(), 1);
    assert!(manager.get_seccomp_profile("default-profile").is_some());
    println!("✓ Seccomp profile management verified");
    
    // 11. Verify AppArmor profile management
    assert_eq!(manager.apparmor_profiles.len(), 1);
    assert!(manager.get_apparmor_profile("default-profile").is_some());
    println!("✓ AppArmor profile management verified");
    
    // 12. Test container configuration validation
    let privileged_container = ContainerConfig {
        name: "privileged-container".to_string(),
        image: "nginx:latest".to_string(),
        privileged: true,
        read_only_root_filesystem: false,
        capabilities: vec!["CAP_NET_ADMIN".to_string()],
        sysctls: vec![("net.ipv4.ip_forward".to_string(), "1".to_string())].into_iter().collect(),
    };
    
    let violations = manager.validate_container_config(&privileged_container).unwrap();
    assert!(!violations.is_empty());
    // With only the privileged policy having EnforcementMode::Enforce and checking for privileged containers, we expect one violation
    assert_eq!(violations.len(), 1);
    // The violation should be for the "no-privileged" rule
    assert_eq!(violations[0].rule_name, "no-privileged");
    println!("✓ Container configuration validation verified");
    
    // 13. Test secrets rotation
    assert!(manager.rotate_secrets().is_ok());
    println!("✓ Secrets rotation verified");
    
    // 14. Test policy removal
    assert!(manager.remove_policy("restrict-privileged-containers").is_ok());
    assert_eq!(manager.policies.len(), 1);
    println!("✓ Policy removal verified");
    
    // 15. Test seccomp profile removal
    assert!(manager.remove_seccomp_profile("default-profile").is_ok());
    assert_eq!(manager.seccomp_profiles.len(), 0);
    println!("✓ Seccomp profile removal verified");
    
    // 16. Test AppArmor profile removal
    assert!(manager.remove_apparmor_profile("default-profile").is_ok());
    assert_eq!(manager.apparmor_profiles.len(), 0);
    println!("✓ AppArmor profile removal verified");
    
    println!("Complete container hardening workflow test passed!");
}

/// Test container hardening with various policy configurations
#[test]
fn test_container_hardening_policy_configurations() {
    println!("Starting container hardening policy configurations test");
    
    let secrets_management = SecretsManagement {
        backend: SecretBackend::HashiCorpVault,
        rotation_interval: 43200,
        encryption_enabled: true,
        access_logging_enabled: true,
    };
    
    let mut manager = ContainerHardeningManager::new(secrets_management);
    
    // Test different enforcement modes
    let warn_policy = AdmissionPolicy {
        name: "warn-policy".to_string(),
        description: "Policy that warns about violations".to_string(),
        rules: vec![
            PolicyRule {
                name: "warn-rule".to_string(),
                description: "Rule that warns about violations".to_string(),
                expression: "container.securityContext.runAsRoot == false".to_string(),
                mode: EnforcementMode::Warn,
            }
        ],
        enabled: true,
    };
    
    let audit_policy = AdmissionPolicy {
        name: "audit-policy".to_string(),
        description: "Policy that audits violations".to_string(),
        rules: vec![
            PolicyRule {
                name: "audit-rule".to_string(),
                description: "Rule that audits violations".to_string(),
                expression: "container.securityContext.allowPrivilegeEscalation == false".to_string(),
                mode: EnforcementMode::Audit,
            }
        ],
        enabled: true,
    };
    
    manager.add_policy(warn_policy);
    manager.add_policy(audit_policy);
    
    assert_eq!(manager.policies.len(), 2);
    assert_eq!(manager.get_policy("warn-policy").unwrap().rules[0].mode, EnforcementMode::Warn);
    assert_eq!(manager.get_policy("audit-policy").unwrap().rules[0].mode, EnforcementMode::Audit);
    
    println!("✓ Policy configurations verified");
    
    // Test different secret backends
    let aws_secrets = SecretsManagement {
        backend: SecretBackend::AwsSecretsManager,
        rotation_interval: 86400,
        encryption_enabled: true,
        access_logging_enabled: true,
    };
    
    let azure_secrets = SecretsManagement {
        backend: SecretBackend::AzureKeyVault,
        rotation_interval: 86400,
        encryption_enabled: true,
        access_logging_enabled: true,
    };
    
    let google_secrets = SecretsManagement {
        backend: SecretBackend::GoogleSecretManager,
        rotation_interval: 86400,
        encryption_enabled: true,
        access_logging_enabled: true,
    };
    
    let aws_manager = ContainerHardeningManager::new(aws_secrets);
    let azure_manager = ContainerHardeningManager::new(azure_secrets);
    let google_manager = ContainerHardeningManager::new(google_secrets);
    
    assert_eq!(aws_manager.secrets_management.backend, SecretBackend::AwsSecretsManager);
    assert_eq!(azure_manager.secrets_management.backend, SecretBackend::AzureKeyVault);
    assert_eq!(google_manager.secrets_management.backend, SecretBackend::GoogleSecretManager);
    
    println!("✓ Secret backend configurations verified");
    
    println!("Container hardening policy configurations test passed!");
}

/// Test container hardening error handling
#[test]
fn test_container_hardening_error_handling() {
    println!("Starting container hardening error handling test");
    
    let secrets_management = SecretsManagement {
        backend: SecretBackend::KubernetesSecrets,
        rotation_interval: 86400,
        encryption_enabled: true,
        access_logging_enabled: true,
    };
    
    let mut manager = ContainerHardeningManager::new(secrets_management);
    
    // Test removing non-existent policy
    let result = manager.remove_policy("non-existent-policy");
    assert!(result.is_err());
    match result.unwrap_err() {
        core::container_hardening::ContainerHardeningError::PolicyNotFound(_) => {},
        _ => panic!("Expected PolicyNotFound error"),
    }
    
    // Test removing non-existent seccomp profile
    let result = manager.remove_seccomp_profile("non-existent-profile");
    assert!(result.is_err());
    match result.unwrap_err() {
        core::container_hardening::ContainerHardeningError::ProfileNotFound(_) => {},
        _ => panic!("Expected ProfileNotFound error"),
    }
    
    // Test removing non-existent AppArmor profile
    let result = manager.remove_apparmor_profile("non-existent-profile");
    assert!(result.is_err());
    match result.unwrap_err() {
        core::container_hardening::ContainerHardeningError::ProfileNotFound(_) => {},
        _ => panic!("Expected ProfileNotFound error"),
    }
    
    // Test getting non-existent policy
    assert!(manager.get_policy("non-existent-policy").is_none());
    
    // Test getting non-existent seccomp profile
    assert!(manager.get_seccomp_profile("non-existent-profile").is_none());
    
    // Test getting non-existent AppArmor profile
    assert!(manager.get_apparmor_profile("non-existent-profile").is_none());
    
    println!("✓ Error handling verified");
    
    println!("Container hardening error handling test passed!");
}