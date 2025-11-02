//! Dependency Trust Validation Script
//!
//! This script validates the dependency trust features including:
//! - Pin versions via lockfiles
//! - Verify checksums
//! - Disallow typosquat packages
//! - Stop malicious libs
//! - Unapproved dependency install attempts

use core::supply_chain::{Component, Sbom, SupplyChainManager};

fn main() {
    println!("Running Dependency Trust Validation");

    // Create supply chain manager
    let mut manager = SupplyChainManager::new();
    
    // Test 1: Valid dependency trust validation
    println!("\n1. Testing valid dependency trust validation");
    let sbom = manager.generate_sbom("test-app", "1.0.0").unwrap();
    let validation_result = manager.validate_dependency_trust(&sbom);
    assert!(validation_result.is_ok(), "Valid SBOM should pass trust validation");
    println!("✓ Valid dependency trust validation passed");
    
    // Check statistics
    let (total, unapproved, typosquat, checksum) = manager.get_trust_stats();
    println!("✓ Trust statistics: total={}, unapproved={}, typosquat={}, checksum={}", 
             total, unapproved, typosquat, checksum);
    
    // Test 2: Unapproved dependency detection
    println!("\n2. Testing unapproved dependency detection");
    let unapproved_component = Component {
        id: "unapproved-component".to_string(),
        name: "malicious-dep".to_string(),
        version: "1.0.0".to_string(),
        purl: "pkg:cargo/malicious-dep@1.0.0".to_string(),
        licenses: vec!["MIT".to_string()],
        hash: "sha256:malicioushash".to_string(),
        is_direct: true,
        vulnerabilities: vec![],
        is_pinned: true,
        checksum_verified: true,
        is_approved: false, // Not approved
    };

    let unapproved_sbom = Sbom {
        id: "test-sbom-unapproved".to_string(),
        name: "test-app".to_string(),
        version: "1.0.0".to_string(),
        components: vec![unapproved_component],
        created: 1234567890,
        hash: "sha256:sbomhash".to_string(),
    };

    let result = manager.validate_dependency_trust(&unapproved_sbom);
    assert!(result.is_err(), "SBOM with unapproved dependency should fail trust validation");
    println!("✓ Unapproved dependency correctly detected and blocked");
    
    // Check statistics
    let (total, unapproved, typosquat, checksum) = manager.get_trust_stats();
    assert_eq!(unapproved, 1, "Should have 1 unapproved dependency blocked");
    println!("✓ Trust statistics updated: total={}, unapproved={}, typosquat={}, checksum={}", 
             total, unapproved, typosquat, checksum);
    
    // Test 3: Typosquat package detection
    println!("\n3. Testing typosquat package detection");
    manager.add_typosquat_package("serede", "serde");
    
    let typosquat_component = Component {
        id: "typosquat-component".to_string(),
        name: "serede".to_string(), // Typosquat for "serde"
        version: "1.0.0".to_string(),
        purl: "pkg:cargo/serede@1.0.0".to_string(),
        licenses: vec!["MIT".to_string()],
        hash: "sha256:typosquathash".to_string(),
        is_direct: true,
        vulnerabilities: vec![],
        is_pinned: true,
        checksum_verified: true,
        is_approved: true,
    };

    let typosquat_sbom = Sbom {
        id: "test-sbom-typosquat".to_string(),
        name: "test-app".to_string(),
        version: "1.0.0".to_string(),
        components: vec![typosquat_component],
        created: 1234567890,
        hash: "sha256:sbomhash".to_string(),
    };

    let result = manager.validate_dependency_trust(&typosquat_sbom);
    assert!(result.is_err(), "SBOM with typosquat package should fail trust validation");
    println!("✓ Typosquat package correctly detected");
    
    // Check statistics
    let (total, unapproved, typosquat, checksum) = manager.get_trust_stats();
    assert_eq!(typosquat, 1, "Should have 1 typosquat package detected");
    println!("✓ Trust statistics updated: total={}, unapproved={}, typosquat={}, checksum={}", 
             total, unapproved, typosquat, checksum);
    
    // Test 4: Unpinned dependency detection
    println!("\n4. Testing unpinned dependency detection");
    let unpinned_component = Component {
        id: "unpinned-component".to_string(),
        name: "unpinned-dep".to_string(),
        version: "1.0.0".to_string(),
        purl: "pkg:cargo/unpinned-dep@1.0.0".to_string(),
        licenses: vec!["MIT".to_string()],
        hash: "sha256:unpinnedhash".to_string(),
        is_direct: true,
        vulnerabilities: vec![],
        is_pinned: false, // Not pinned
        checksum_verified: true,
        is_approved: true,
    };

    let unpinned_sbom = Sbom {
        id: "test-sbom-unpinned".to_string(),
        name: "test-app".to_string(),
        version: "1.0.0".to_string(),
        components: vec![unpinned_component],
        created: 1234567890,
        hash: "sha256:sbomhash".to_string(),
    };

    let result = manager.validate_dependency_trust(&unpinned_sbom);
    assert!(result.is_err(), "SBOM with unpinned dependency should fail trust validation");
    println!("✓ Unpinned dependency correctly detected");
    
    // Test 5: Checksum verification failure detection
    println!("\n5. Testing checksum verification failure detection");
    let checksum_component = Component {
        id: "checksum-component".to_string(),
        name: "checksum-dep".to_string(),
        version: "1.0.0".to_string(),
        purl: "pkg:cargo/checksum-dep@1.0.0".to_string(),
        licenses: vec!["MIT".to_string()],
        hash: "sha256:checksumhash".to_string(),
        is_direct: true,
        vulnerabilities: vec![],
        is_pinned: true,
        checksum_verified: false, // Checksum verification failed
        is_approved: true,
    };

    let checksum_sbom = Sbom {
        id: "test-sbom-checksum".to_string(),
        name: "test-app".to_string(),
        version: "1.0.0".to_string(),
        components: vec![checksum_component],
        created: 1234567890,
        hash: "sha256:sbomhash".to_string(),
    };

    let result = manager.validate_dependency_trust(&checksum_sbom);
    assert!(result.is_err(), "SBOM with checksum verification failure should fail trust validation");
    println!("✓ Checksum verification failure correctly detected");
    
    // Check final statistics
    let (total, unapproved, typosquat, checksum) = manager.get_trust_stats();
    println!("✓ Final trust statistics: total={}, unapproved={}, typosquat={}, checksum={}", 
             total, unapproved, typosquat, checksum);
    
    // Test 6: Approved dependency management
    println!("\n6. Testing approved dependency management");
    manager.add_approved_dependency("new-dep", r"^1\.[0-9]+\.[0-9]+$");
    assert!(manager.approved_dependencies.contains_key("new-dep"), "Should add approved dependency");
    
    manager.remove_approved_dependency("new-dep");
    assert!(!manager.approved_dependencies.contains_key("new-dep"), "Should remove approved dependency");
    println!("✓ Approved dependency management working correctly");
    
    // Test 7: Typosquat detection management
    println!("\n7. Testing typosquat detection management");
    manager.add_typosquat_package("suspicious-name", "legitimate-name");
    assert!(manager.typosquat_packages.contains_key("suspicious-name"), "Should add typosquat package");
    println!("✓ Typosquat detection management working correctly");
    
    println!("\nAll Dependency Trust Validation Tests Passed!");
    println!("Dependency Trust Features Working Correctly:");
    println!("- Pin versions via lockfiles ✓");
    println!("- Verify checksums ✓");
    println!("- Disallow typosquat packages ✓");
    println!("- Stop malicious libs ✓");
    println!("- Unapproved dependency install attempts ✓");
}