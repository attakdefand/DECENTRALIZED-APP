//! Tests for vendor policy enforcement

use std::fs;
use std::path::Path;

/// Test that vendor Cedar policies exist
#[test]
fn test_vendor_cedar_policies_exist() {
    let policy_dir = "infra/policies/OPA-Cedar";
    
    // Check that the policy directory exists
    assert!(Path::new(policy_dir).is_dir(), "Policy directory missing: {}", policy_dir);
    
    // Check that required vendor policies exist
    let required_policies = vec![
        "vendor_access.cedar",
        "vendor_policy_enforcement.cedar",
        "vendor_rbac.cedar",
    ];
    
    for policy_file in required_policies {
        let policy_path = format!("{}/{}", policy_dir, policy_file);
        assert!(
            Path::new(&policy_path).is_file(),
            "Required vendor policy file missing: {}",
            policy_path
        );
    }
}

/// Test that vendor policy files are not empty
#[test]
fn test_vendor_policy_files_not_empty() {
    let policy_dir = "infra/policies/OPA-Cedar";
    let vendor_policies = vec![
        "vendor_access.cedar",
        "vendor_policy_enforcement.cedar",
        "vendor_rbac.cedar",
    ];
    
    for policy_file in vendor_policies {
        let policy_path = format!("{}/{}", policy_dir, policy_file);
        let content = fs::read_to_string(&policy_path)
            .expect(&format!("Failed to read policy file: {}", policy_path));
        
        assert!(
            !content.is_empty(),
            "Vendor policy file is empty: {}",
            policy_path
        );
    }
}

/// Test that vendor policy files contain required elements
#[test]
fn test_vendor_policy_files_content() {
    // Test vendor access policy
    let vendor_access_policy = fs::read_to_string("infra/policies/OPA-Cedar/vendor_access.cedar")
        .expect("Failed to read vendor_access.cedar");
    
    assert!(
        vendor_access_policy.contains("VendorUser"),
        "Vendor access policy missing VendorUser entity"
    );
    
    assert!(
        vendor_access_policy.contains("VendorResource"),
        "Vendor access policy missing VendorResource entity"
    );
    
    // Test vendor policy enforcement policy
    let vendor_policy_enforcement = fs::read_to_string("infra/policies/OPA-Cedar/vendor_policy_enforcement.cedar")
        .expect("Failed to read vendor_policy_enforcement.cedar");
    
    assert!(
        vendor_policy_enforcement.contains("VendorPolicy"),
        "Vendor policy enforcement policy missing VendorPolicy entity"
    );
    
    assert!(
        vendor_policy_enforcement.contains("VendorComplianceRequirement"),
        "Vendor policy enforcement policy missing VendorComplianceRequirement entity"
    );
    
    // Test vendor RBAC policy
    let vendor_rbac_policy = fs::read_to_string("infra/policies/OPA-Cedar/vendor_rbac.cedar")
        .expect("Failed to read vendor_rbac.cedar");
    
    assert!(
        vendor_rbac_policy.contains("VendorRole"),
        "Vendor RBAC policy missing VendorRole entity"
    );
    
    assert!(
        vendor_rbac_policy.contains("VendorPermission"),
        "Vendor RBAC policy missing VendorPermission entity"
    );
}

/// Test that vendor documentation exists
#[test]
fn test_vendor_documentation_exists() {
    // Check that vendor access controls documentation exists
    assert!(
        Path::new("docs/security/vendor-access-controls.md").is_file(),
        "Vendor access controls documentation missing"
    );
    
    // Check that vendor RBAC map exists
    assert!(
        Path::new("docs/security/vendor-rbac-map.md").is_file(),
        "Vendor RBAC map missing"
    );
    
    // Verify documentation content
    let vendor_access_controls = fs::read_to_string("docs/security/vendor-access-controls.md")
        .expect("Failed to read vendor-access-controls.md");
    
    assert!(
        vendor_access_controls.contains("# Vendor Access Controls"),
        "Vendor access controls documentation has incorrect title"
    );
    
    let vendor_rbac_map = fs::read_to_string("docs/security/vendor-rbac-map.md")
        .expect("Failed to read vendor-rbac-map.md");
    
    assert!(
        vendor_rbac_map.contains("# Vendor RBAC Map"),
        "Vendor RBAC map has incorrect title"
    );
}