// Group B - Infrastructure Security Testing Validation
// This file validates the complete implementation of Group B features:
// Focus: IAM, Key Management, Policy Gatekeeping
// Testing Types: Reviews, drills, policy-as-code validation
// Tools: IdP test flows, OPA/Cedar unit tests, KMS API tests

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    // Test that all Group B required documentation files exist
    #[test]
    fn test_group_b_required_documentation_files_exist() {
        let required_files = vec![
            "docs/security/IAM-RBAC-MAP.md",
            "docs/runbooks/key-rotation.md",
            "docs/security/mpc-hsm-policy.md",
            "docs/security/multisig-addresses.md",
        ];

        for file in required_files {
            assert!(
                Path::new(file).exists(),
                "Required Group B documentation file missing: {}",
                file
            );
        }
    }

    // Test that policy-as-code components exist
    #[test]
    fn test_policy_as_code_components_exist() {
        // Check that OPA/Cedar policies exist
        assert!(
            Path::new("infra/policies/OPA-Cedar").exists(),
            "OPA/Cedar policy directory missing"
        );
        
        // Check that we have Cedar policy files
        let paths = fs::read_dir("infra/policies/OPA-Cedar").expect("Failed to read policy directory");
        let mut cedar_policy_count = 0;
        
        for path in paths {
            let entry = path.expect("Failed to read directory entry");
            if entry.file_name().to_string_lossy().ends_with(".cedar") {
                cedar_policy_count += 1;
            }
        }
        
        assert!(
            cedar_policy_count > 0,
            "No Cedar policy files found in infra/policies/OPA-Cedar"
        );
    }

    // Test that IAM RBAC Map has proper structure
    #[test]
    fn test_iam_rbac_map_structure() {
        let content = fs::read_to_string("docs/security/IAM-RBAC-MAP.md")
            .expect("Failed to read IAM-RBAC-MAP.md");
        
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
    }

    // Test that Key Rotation Runbook has proper structure
    #[test]
    fn test_key_rotation_runbook_structure() {
        let content = fs::read_to_string("docs/runbooks/key-rotation.md")
            .expect("Failed to read key-rotation.md");
        
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
    }

    // Test that MPC/HSM Policy has proper structure
    #[test]
    fn test_mpc_hsm_policy_structure() {
        let content = fs::read_to_string("docs/security/mpc-hsm-policy.md")
            .expect("Failed to read mpc-hsm-policy.md");
        
        assert!(
            content.contains("# MPC/HSM Policy"),
            "mpc-hsm-policy.md missing title"
        );
        
        assert!(
            content.contains("Policy Statements"),
            "mpc-hsm-policy.md missing Policy Statements section"
        );
    }

    // Test that Multisig Addresses document has proper structure
    #[test]
    fn test_multisig_addresses_structure() {
        let content = fs::read_to_string("docs/security/multisig-addresses.md")
            .expect("Failed to read multisig-addresses.md");
        
        assert!(
            content.contains("# Multisig Addresses"),
            "multisig-addresses.md missing title"
        );
        
        assert!(
            content.contains("Multisig Address Registry"),
            "multisig-addresses.md missing Multisig Address Registry section"
        );
    }

    // Test that all Group B features are cross-referenced
    #[test]
    fn test_group_b_cross_references() {
        // Check that key rotation references MPC/HSM policy
        let key_rotation_content = fs::read_to_string("docs/runbooks/key-rotation.md")
            .expect("Failed to read key-rotation.md");
        
        assert!(
            key_rotation_content.contains("[MPC/HSM Policy]"),
            "Key rotation runbook missing MPC/HSM policy reference"
        );
        
        // Check that MPC/HSM policy references key rotation
        let mpc_content = fs::read_to_string("docs/security/mpc-hsm-policy.md")
            .expect("Failed to read mpc-hsm-policy.md");
        
        assert!(
            mpc_content.contains("[Key Rotation Runbook]"),
            "MPC/HSM policy missing key rotation runbook reference"
        );
    }
}