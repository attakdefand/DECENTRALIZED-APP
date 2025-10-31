// Infrastructure Security Testing Validation
// This file validates the implementation of Group B - Infrastructure Security Testing features
// Focus: IAM, Key Management, Policy Gatekeeping
// Testing Types: Reviews, drills, policy-as-code validation
// Tools: IdP test flows, OPA/Cedar unit tests, KMS API tests

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    // Test that all required documentation files exist
    #[test]
    fn test_required_documentation_files_exist() {
        let required_files = vec![
            "docs/security/POLICY-CATALOG.md",
            "docs/security/EXCEPTIONS.md",
            "docs/security/sign-off-template.md",
            "docs/security/CODEOWNERS",
            "docs/security/IAM-RBAC-MAP.md",
            "docs/runbooks/key-rotation.md",
            "docs/security/mpc-hsm-policy.md",
            "docs/security/multisig-addresses.md",
            "infra/policies/policy-registry.md",
            "infra/policies/rate-classes.yaml",
            "infra/policies/policy-provenance.md",
        ];

        for file in required_files {
            assert!(
                Path::new(file).exists(),
                "Required documentation file missing: {}",
                file
            );
        }
    }

    // Test that all required directories exist
    #[test]
    fn test_required_directories_exist() {
        let required_dirs = vec![
            "infra/policies/OPA-Cedar",
            "infra/policies/allow-deny-lists",
        ];

        for dir in required_dirs {
            assert!(
                Path::new(dir).is_dir(),
                "Required directory missing: {}",
                dir
            );
        }
    }

    // Test that Cedar policy files exist
    #[test]
    fn test_cedar_policy_files_exist() {
        let policy_dir = "infra/policies/OPA-Cedar";
        let paths = fs::read_dir(policy_dir).expect("Failed to read policy directory");
        
        let mut policy_files_found = 0;
        for path in paths {
            let entry = path.expect("Failed to read directory entry");
            if entry.file_name().to_string_lossy().ends_with(".cedar") {
                policy_files_found += 1;
            }
        }
        
        assert!(
            policy_files_found > 0,
            "No Cedar policy files found in {}",
            policy_dir
        );
    }

    // Test that allow/deny list files exist
    #[test]
    fn test_allow_deny_list_files_exist() {
        let required_files = vec![
            "infra/policies/allow-deny-lists/ip-allow-list.txt",
            "infra/policies/allow-deny-lists/ip-deny-list.txt",
            "infra/policies/allow-deny-lists/domain-allow-list.txt",
            "infra/policies/allow-deny-lists/domain-deny-list.txt",
        ];

        for file in required_files {
            assert!(
                Path::new(file).exists(),
                "Required allow/deny list file missing: {}",
                file
            );
        }
    }

    // Test that rate-classes.yaml is valid YAML
    #[test]
    fn test_rate_classes_yaml_valid() {
        let yaml_content = fs::read_to_string("infra/policies/rate-classes.yaml")
            .expect("Failed to read rate-classes.yaml");
        
        // Basic check that file is not empty and contains expected content
        assert!(!yaml_content.is_empty(), "rate-classes.yaml is empty");
        assert!(
            yaml_content.contains("api:") && yaml_content.contains("requests_per_minute:"),
            "rate-classes.yaml does not contain expected structure"
        );
    }

    // Test that markdown files have basic structure
    #[test]
    fn test_markdown_files_structure() {
        let markdown_files = vec![
            "docs/security/POLICY-CATALOG.md",
            "docs/security/EXCEPTIONS.md",
            "docs/security/sign-off-template.md",
            "docs/security/IAM-RBAC-MAP.md",
            "docs/runbooks/key-rotation.md",
            "docs/security/mpc-hsm-policy.md",
            "docs/security/multisig-addresses.md",
            "infra/policies/policy-registry.md",
            "infra/policies/policy-provenance.md",
        ];

        for file in markdown_files {
            let content = fs::read_to_string(file)
                .expect(&format!("Failed to read markdown file: {}", file));
            
            // Check that file is not empty
            assert!(!content.is_empty(), "Markdown file is empty: {}", file);
            
            // Check for basic markdown headers
            assert!(
                content.contains("#"),
                "Markdown file may be missing headers: {}",
                file
            );
        }
    }

    // Test IAM RBAC Map structure
    #[test]
    fn test_iam_rbac_map_content() {
        let content = fs::read_to_string("docs/security/IAM-RBAC-MAP.md")
            .expect("Failed to read IAM-RBAC-MAP.md");
        
        // Check for key sections
        assert!(
            content.contains("# IAM RBAC Map"),
            "IAM-RBAC-MAP.md missing title"
        );
        assert!(
            content.contains("Role Definitions"),
            "IAM-RBAC-MAP.md missing Role Definitions section"
        );
        assert!(
            content.contains("Permission Mappings"),
            "IAM-RBAC-MAP.md missing Permission Mappings section"
        );
        assert!(
            content.contains("Access Control Policies"),
            "IAM-RBAC-MAP.md missing Access Control Policies section"
        );
    }

    // Test Key Rotation Runbook structure
    #[test]
    fn test_key_rotation_runbook_content() {
        let content = fs::read_to_string("docs/runbooks/key-rotation.md")
            .expect("Failed to read key-rotation.md");
        
        // Check for key sections
        assert!(
            content.contains("# Key Rotation Runbook"),
            "key-rotation.md missing title"
        );
        assert!(
            content.contains("Key Rotation Principles"),
            "key-rotation.md missing Key Rotation Principles section"
        );
        assert!(
            content.contains("Key Types and Rotation Procedures"),
            "key-rotation.md missing Key Types and Rotation Procedures section"
        );
        assert!(
            content.contains("Emergency Procedures"),
            "key-rotation.md missing Emergency Procedures section"
        );
    }

    // Test Policy Catalog structure
    #[test]
    fn test_policy_catalog_content() {
        let content = fs::read_to_string("docs/security/POLICY-CATALOG.md")
            .expect("Failed to read POLICY-CATALOG.md");
        
        // Check for key sections
        assert!(
            content.contains("# Policy Catalog"),
            "POLICY-CATALOG.md missing title"
        );
        assert!(
            content.contains("Policy Framework"),
            "POLICY-CATALOG.md missing Policy Framework section"
        );
        assert!(
            content.contains("Policy Categories"),
            "POLICY-CATALOG.md missing Policy Categories section"
        );
        assert!(
            content.contains("Policy Enforcement"),
            "POLICY-CATALOG.md missing Policy Enforcement section"
        );
    }

    // Test that CODEOWNERS file has proper structure
    #[test]
    fn test_codeowners_structure() {
        let content = fs::read_to_string("docs/security/CODEOWNERS")
            .expect("Failed to read CODEOWNERS file");
        
        // Check that file is not empty
        assert!(!content.is_empty(), "CODEOWNERS file is empty");
        
        // Check for basic structure (should contain patterns and teams)
        assert!(
            content.contains("@attakdefand/"),
            "CODEOWNERS file missing team references"
        );
        assert!(
            content.contains("docs/security/"),
            "CODEOWNERS file missing security document patterns"
        );
    }

    // Test that policy registry has proper structure
    #[test]
    fn test_policy_registry_structure() {
        let content = fs::read_to_string("infra/policies/policy-registry.md")
            .expect("Failed to read policy-registry.md");
        
        // Check for key sections
        assert!(
            content.contains("# Policy Registry"),
            "policy-registry.md missing title"
        );
        assert!(
            content.contains("Policy Categories"),
            "policy-registry.md missing Policy Categories section"
        );
        assert!(
            content.contains("Policy Enforcement Mechanisms"),
            "policy-registry.md missing Policy Enforcement Mechanisms section"
        );
        assert!(
            content.contains("Access Control Policies") ||
            content.contains("Data Protection Policies") ||
            content.contains("Infrastructure Security Policies"),
            "policy-registry.md missing specific policy categories"
        );
    }

    // Test that MPC/HSM policy has proper structure
    #[test]
    fn test_mpc_hsm_policy_structure() {
        let content = fs::read_to_string("docs/security/mpc-hsm-policy.md")
            .expect("Failed to read mpc-hsm-policy.md");
        
        // Check for key sections
        assert!(
            content.contains("# MPC/HSM Policy"),
            "mpc-hsm-policy.md missing title"
        );
        assert!(
            content.contains("Policy Statements"),
            "mpc-hsm-policy.md missing Policy Statements section"
        );
        assert!(
            content.contains("Implementation Requirements"),
            "mpc-hsm-policy.md missing Implementation Requirements section"
        );
        assert!(
            content.contains("Compliance Requirements"),
            "mpc-hsm-policy.md missing Compliance Requirements section"
        );
    }

    // Test that multisig addresses document has proper structure
    #[test]
    fn test_multisig_addresses_structure() {
        let content = fs::read_to_string("docs/security/multisig-addresses.md")
            .expect("Failed to read multisig-addresses.md");
        
        // Check for key sections
        assert!(
            content.contains("# Multisig Addresses"),
            "multisig-addresses.md missing title"
        );
        assert!(
            content.contains("Multisig Address Registry"),
            "multisig-addresses.md missing Multisig Address Registry section"
        );
        assert!(
            content.contains("Multisig Policies"),
            "multisig-addresses.md missing Multisig Policies section"
        );
        assert!(
            content.contains("Treasury Addresses") ||
            content.contains("Operational Addresses") ||
            content.contains("Emergency Addresses"),
            "multisig-addresses.md missing address categories"
        );
    }

    // Test that exception management document has proper structure
    #[test]
    fn test_exceptions_document_structure() {
        let content = fs::read_to_string("docs/security/EXCEPTIONS.md")
            .expect("Failed to read EXCEPTIONS.md");
        
        // Check for key sections
        assert!(
            content.contains("# Exception Management"),
            "EXCEPTIONS.md missing title"
        );
        assert!(
            content.contains("Exception Process"),
            "EXCEPTIONS.md missing Exception Process section"
        );
        assert!(
            content.contains("Exception Categories"),
            "EXCEPTIONS.md missing Exception Categories section"
        );
        assert!(
            content.contains("Risk-Based Classification"),
            "EXCEPTIONS.md missing Risk-Based Classification section"
        );
    }

    // Test that sign-off template has proper structure
    #[test]
    fn test_signoff_template_structure() {
        let content = fs::read_to_string("docs/security/sign-off-template.md")
            .expect("Failed to read sign-off-template.md");
        
        // Check for key sections
        assert!(
            content.contains("# Policy Sign-Off Template"),
            "sign-off-template.md missing title"
        );
        assert!(
            content.contains("Review Participants"),
            "sign-off-template.md missing Review Participants section"
        );
        assert!(
            content.contains("Approval"),
            "sign-off-template.md missing Approval section"
        );
        assert!(
            content.contains("Distribution List"),
            "sign-off-template.md missing Distribution List section"
        );
    }

    // Test that allow/deny lists are not empty
    #[test]
    fn test_allow_deny_lists_not_empty() {
        let list_files = vec![
            "infra/policies/allow-deny-lists/ip-allow-list.txt",
            "infra/policies/allow-deny-lists/ip-deny-list.txt",
            "infra/policies/allow-deny-lists/domain-allow-list.txt",
            "infra/policies/allow-deny-lists/domain-deny-list.txt",
        ];

        for file in list_files {
            let content = fs::read_to_string(file)
                .expect(&format!("Failed to read list file: {}", file));
            
            // For now, we're just checking that the files exist and aren't empty
            // In a real implementation, we might want to validate the format
            assert!(
                !content.is_empty(),
                "Allow/deny list file is empty: {}",
                file
            );
        }
    }

    // Test that Cedar policies have proper syntax (basic check)
    #[test]
    fn test_cedar_policies_basic_syntax() {
        let policy_dir = "infra/policies/OPA-Cedar";
        let paths = fs::read_dir(policy_dir).expect("Failed to read policy directory");
        
        for path in paths {
            let entry = path.expect("Failed to read directory entry");
            if entry.file_name().to_string_lossy().ends_with(".cedar") {
                let content = fs::read_to_string(entry.path())
                    .expect(&format!("Failed to read Cedar policy file: {:?}", entry.path()));
                
                // Basic syntax checks
                assert!(
                    !content.is_empty(),
                    "Cedar policy file is empty: {:?}",
                    entry.path()
                );
                
                // Check for basic Cedar syntax elements
                assert!(
                    content.contains("permit") || content.contains("forbid"),
                    "Cedar policy file missing permit/forbid statements: {:?}",
                    entry.path()
                );
                
                assert!(
                    content.contains("principal") && content.contains("action") && content.contains("resource"),
                    "Cedar policy file missing basic policy elements: {:?}",
                    entry.path()
                );
            }
        }
    }
}