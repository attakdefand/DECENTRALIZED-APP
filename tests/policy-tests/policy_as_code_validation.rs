// Policy-as-Code Validation Tests
// This file validates the policy-as-code implementations for Group B features
// Focus: Policy Gatekeeping through OPA/Cedar policies
// Testing Types: Policy-as-code validation
// Tools: OPA/Cedar unit tests

#[cfg(test)]
mod tests {
    use std::fs;
    use std::process::Command;

    // Test that OPA/Cedar policies can be parsed without syntax errors
    #[test]
    fn test_cedar_policies_parse_correctly() {
        let policy_dir = "infra/policies/OPA-Cedar";
        let paths = fs::read_dir(policy_dir).expect("Failed to read policy directory");
        
        let mut policy_files_tested = 0;
        
        for path in paths {
            let entry = path.expect("Failed to read directory entry");
            let file_name = entry.file_name();
            
            if file_name.to_string_lossy().ends_with(".cedar") {
                let policy_path = entry.path();
                println!("Testing Cedar policy: {:?}", policy_path);
                
                // In a real implementation, we would use a Cedar parser or validator
                // For now, we'll do basic syntax checks
                let content = fs::read_to_string(&policy_path)
                    .expect(&format!("Failed to read policy file: {:?}", policy_path));
                
                // Basic validation that the policy contains required elements
                assert!(!content.is_empty(), "Policy file is empty: {:?}", policy_path);
                assert!(content.contains("permit") || content.contains("forbid"), 
                    "Policy missing permit/forbid statements: {:?}", policy_path);
                
                policy_files_tested += 1;
            }
        }
        
        assert!(policy_files_tested > 0, "No Cedar policy files were tested");
    }

    // Test that Cedar policies cover the required policy areas
    #[test]
    fn test_cedar_policies_coverage() {
        let policy_dir = "infra/policies/OPA-Cedar";
        let paths = fs::read_dir(policy_dir).expect("Failed to read policy directory");
        
        let mut data_access_found = false;
        let mut infrastructure_access_found = false;
        let mut repository_access_found = false;
        
        for path in paths {
            let entry = path.expect("Failed to read directory entry");
            let file_name = entry.file_name();
            
            if file_name.to_string_lossy().ends_with(".cedar") {
                let policy_path = entry.path();
                let content = fs::read_to_string(&policy_path)
                    .expect(&format!("Failed to read policy file: {:?}", policy_path));
                
                if policy_path.to_string_lossy().contains("data_access") {
                    data_access_found = true;
                    assert!(content.contains("DataClassification"), 
                        "Data access policy missing DataClassification");
                    assert!(content.contains("permit") || content.contains("forbid"),
                        "Data access policy missing access control statements");
                }
                
                if policy_path.to_string_lossy().contains("infrastructure_access") {
                    infrastructure_access_found = true;
                    assert!(content.contains("Infrastructure"), 
                        "Infrastructure access policy missing Infrastructure references");
                }
                
                if policy_path.to_string_lossy().contains("repository_access") {
                    repository_access_found = true;
                    assert!(content.contains("Repository"), 
                        "Repository access policy missing Repository references");
                }
            }
        }
        
        assert!(data_access_found, "Data access policy not found");
        assert!(infrastructure_access_found, "Infrastructure access policy not found");
        assert!(repository_access_found, "Repository access policy not found");
    }

    // Test that policy registry references actual policy files
    #[test]
    fn test_policy_registry_references_actual_files() {
        let registry_content = fs::read_to_string("infra/policies/policy-registry.md")
            .expect("Failed to read policy-registry.md");
        
        // Check that registry references the actual Cedar policy files
        assert!(registry_content.contains("data_access.cedar"), 
            "Policy registry does not reference data_access.cedar");
        assert!(registry_content.contains("infrastructure_access.cedar"), 
            "Policy registry does not reference infrastructure_access.cedar");
        assert!(registry_content.contains("repository_access.cedar"), 
            "Policy registry does not reference repository_access.cedar");
    }

    // Test that allow/deny lists have proper format
    #[test]
    fn test_allow_deny_list_format() {
        // Test IP allow list
        let ip_allow_content = fs::read_to_string("infra/policies/allow-deny-lists/ip-allow-list.txt")
            .expect("Failed to read ip-allow-list.txt");
        
        // Test IP deny list
        let ip_deny_content = fs::read_to_string("infra/policies/allow-deny-lists/ip-deny-list.txt")
            .expect("Failed to read ip-deny-list.txt");
        
        // Test domain allow list
        let domain_allow_content = fs::read_to_string("infra/policies/allow-deny-lists/domain-allow-list.txt")
            .expect("Failed to read domain-allow-list.txt");
        
        // Test domain deny list
        let domain_deny_content = fs::read_to_string("infra/policies/allow-deny-lists/domain-deny-list.txt")
            .expect("Failed to read domain-deny-list.txt");
        
        // Basic validation - files should not be empty
        assert!(!ip_allow_content.is_empty(), "IP allow list is empty");
        assert!(!ip_deny_content.is_empty(), "IP deny list is empty");
        assert!(!domain_allow_content.is_empty(), "Domain allow list is empty");
        assert!(!domain_deny_content.is_empty(), "Domain deny list is empty");
    }

    // Test that rate classes configuration is valid
    #[test]
    fn test_rate_classes_configuration() {
        let yaml_content = fs::read_to_string("infra/policies/rate-classes.yaml")
            .expect("Failed to read rate-classes.yaml");
        
        // Check for required sections
        assert!(yaml_content.contains("api:"), "Rate classes missing 'api' section");
        assert!(yaml_content.contains("web:"), "Rate classes missing 'web' section");
        assert!(yaml_content.contains("internal:"), "Rate classes missing 'internal' section");
        assert!(yaml_content.contains("special:"), "Rate classes missing 'special' section");
        
        // Check for required fields
        assert!(yaml_content.contains("requests_per_minute:"), 
            "Rate classes missing requests_per_minute field");
        assert!(yaml_content.contains("burst_limit:"), 
            "Rate classes missing burst_limit field");
    }

    // Test that policy provenance document exists and has proper structure
    #[test]
    fn test_policy_provenance_document() {
        let provenance_path = "infra/policies/policy-provenance.md";
        
        assert!(std::path::Path::new(provenance_path).exists(), 
            "Policy provenance document does not exist: {}", provenance_path);
        
        let content = fs::read_to_string(provenance_path)
            .expect(&format!("Failed to read policy provenance document: {}", provenance_path));
        
        assert!(!content.is_empty(), "Policy provenance document is empty");
        assert!(content.contains("#"), "Policy provenance document missing markdown headers");
    }

    // Test that IAM reviews documentation exists
    #[test]
    fn test_iam_reviews_documentation() {
        let iam_rbac_path = "docs/security/IAM-RBAC-MAP.md";
        
        assert!(std::path::Path::new(iam_rbac_path).exists(), 
            "IAM RBAC map document does not exist: {}", iam_rbac_path);
        
        let content = fs::read_to_string(iam_rbac_path)
            .expect(&format!("Failed to read IAM RBAC map: {}", iam_rbac_path));
        
        assert!(!content.is_empty(), "IAM RBAC map document is empty");
        assert!(content.contains("Role Definitions"), "IAM RBAC map missing Role Definitions");
        assert!(content.contains("Permission Mappings"), "IAM RBAC map missing Permission Mappings");
    }

    // Test that key management documentation exists
    #[test]
    fn test_key_management_documentation() {
        let key_rotation_path = "docs/runbooks/key-rotation.md";
        
        assert!(std::path::Path::new(key_rotation_path).exists(), 
            "Key rotation runbook does not exist: {}", key_rotation_path);
        
        let content = fs::read_to_string(key_rotation_path)
            .expect(&format!("Failed to read key rotation runbook: {}", key_rotation_path));
        
        assert!(!content.is_empty(), "Key rotation runbook is empty");
        assert!(content.contains("Key Rotation Principles"), 
            "Key rotation runbook missing Key Rotation Principles");
        assert!(content.contains("Key Types and Rotation Procedures"), 
            "Key rotation runbook missing Key Types and Rotation Procedures");
    }

    // Test that policy gatekeeping documentation exists
    #[test]
    fn test_policy_gatekeeping_documentation() {
        let policy_readme_path = "infra/policies/README.md";
        
        assert!(std::path::Path::new(policy_readme_path).exists(), 
            "Policy README does not exist: {}", policy_readme_path);
        
        let content = fs::read_to_string(policy_readme_path)
            .expect(&format!("Failed to read policy README: {}", policy_readme_path));
        
        assert!(!content.is_empty(), "Policy README is empty");
        assert!(content.contains("# Policy Gatekeeping"), "Policy README missing title");
        assert!(content.contains("Testing Approach"), "Policy README missing Testing Approach");
        assert!(content.contains("CI Gate Requirements"), "Policy README missing CI Gate Requirements");
    }

    // Test that all policy documents are referenced in validation scripts
    #[test]
    fn test_policy_documents_referenced_in_validation() {
        // Read the PowerShell validation script
        let ps_script = fs::read_to_string("tests/policy-tests/policy-validation.ps1")
            .expect("Failed to read policy-validation.ps1");
        
        // Check that all required documents are referenced
        let required_docs = vec![
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
        
        for doc in required_docs {
            assert!(ps_script.contains(doc), 
                "Required document {} not referenced in PowerShell validation script", doc);
        }
        
        // Read the Bash validation script
        let sh_script = fs::read_to_string("tests/policy-tests/policy-validation.sh")
            .expect("Failed to read policy-validation.sh");
        
        for doc in required_docs {
            assert!(sh_script.contains(doc), 
                "Required document {} not referenced in Bash validation script", doc);
        }
    }

    // Test that policy test directories exist
    #[test]
    fn test_policy_test_directories_exist() {
        // Check that the policy test directory exists
        assert!(std::path::Path::new("tests/policy-tests").exists(), 
            "Policy tests directory does not exist");
        
        // Check that validation test files exist
        assert!(std::path::Path::new("tests/policy-tests/policy-validation.ps1").exists(), 
            "PowerShell validation script does not exist");
        assert!(std::path::Path::new("tests/policy-tests/policy-validation.sh").exists(), 
            "Bash validation script does not exist");
    }

    // Test that CODEOWNERS references policy files
    #[test]
    fn test_codeowners_policy_references() {
        let codeowners_content = fs::read_to_string("docs/security/CODEOWNERS")
            .expect("Failed to read CODEOWNERS file");
        
        // Check that key policy files are referenced
        assert!(codeowners_content.contains("infra/policies/OPA-Cedar/infrastructure_access.cedar"), 
            "CODEOWNERS missing infrastructure access policy reference");
        assert!(codeowners_content.contains("infra/policies/OPA-Cedar/repository_access.cedar"), 
            "CODEOWNERS missing repository access policy reference");
        assert!(codeowners_content.contains("infra/policies/OPA-Cedar/data_access.cedar"), 
            "CODEOWNERS missing data access policy reference");
        assert!(codeowners_content.contains("infra/policies/policy-registry.md"), 
            "CODEOWNERS missing policy registry reference");
    }
}