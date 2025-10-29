//! Integration tests for supply chain security functionality

use core::supply_chain::{
    Artifact, BuildConfig, Component, Sbom, SourceInfo, SupplyChainManager, Vulnerability,
};

/// Integration test for the complete supply chain workflow
#[test]
fn test_complete_supply_chain_workflow() {
    println!("Starting complete supply chain workflow test");

    // 1. Create supply chain manager
    let mut manager = SupplyChainManager::new();
    println!("✓ Supply chain manager created");

    // 2. Generate SBOM
    let sbom = manager.generate_sbom("test-app", "1.0.0").unwrap();
    let sbom_id = sbom.id.clone();
    println!("✓ SBOM generated for {} v{}", sbom.name, sbom.version);

    // 3. Store SBOM
    assert!(manager.store_sbom(sbom).is_ok());
    println!("✓ SBOM stored");

    // 4. Verify SBOM storage
    let retrieved_sbom = manager.get_sbom(&sbom_id).unwrap();
    assert_eq!(retrieved_sbom.name, "test-app");
    assert_eq!(retrieved_sbom.version, "1.0.0");
    assert_eq!(retrieved_sbom.components.len(), 2);
    println!("✓ SBOM retrieval verified");

    // 5. Create signature for an artifact
    let artifact_hash = "sha256:abc123def456";
    let signature = manager.create_signature(artifact_hash).unwrap();
    println!("✓ Signature created for artifact");

    // 6. Store signature
    let artifact_uri = "artifact://test-app-v1.0.0";
    assert!(manager
        .store_signature(artifact_uri, signature.clone())
        .is_ok());
    println!("✓ Signature stored");

    // 7. Verify signature
    assert!(manager.verify_signature(artifact_hash, &signature).unwrap());
    println!("✓ Signature verification passed");

    // 8. Create provenance information
    let source = SourceInfo {
        repo_url: "https://github.com/example/test-app".to_string(),
        commit_hash: "a1b2c3d4e5f".to_string(),
        branch: "main".to_string(),
        tag: Some("v1.0.0".to_string()),
    };

    let build_config = BuildConfig {
        build_script: "build.sh".to_string(),
        environment: std::collections::HashMap::new(),
        tools: std::collections::HashMap::new(),
    };

    let artifacts = vec![Artifact {
        name: "test-app-binary".to_string(),
        uri: artifact_uri.to_string(),
        hash: artifact_hash.to_string(),
        size: 2048,
        signature: None,
        sbom: None,
        is_signed: false,
        created: 0,
    }];

    let provenance = manager
        .create_provenance("build-123", source, build_config, artifacts)
        .unwrap();
    let provenance_id = provenance.id.clone();
    println!("✓ Provenance information created");

    // 9. Store provenance
    assert!(manager.store_provenance(provenance).is_ok());
    println!("✓ Provenance stored");

    // 10. Verify provenance storage
    let retrieved_provenance = manager.get_provenance(&provenance_id).unwrap();
    assert_eq!(retrieved_provenance.build_id, "build-123");
    assert_eq!(
        retrieved_provenance.source.repo_url,
        "https://github.com/example/test-app"
    );
    assert_eq!(retrieved_provenance.artifacts.len(), 1);
    println!("✓ Provenance retrieval verified");

    // 11. Scan dependencies for vulnerabilities
    let sbom_for_scanning = manager.get_sbom(&sbom_id).unwrap();
    let vulnerabilities = manager.scan_dependencies(sbom_for_scanning).unwrap();
    assert_eq!(vulnerabilities.len(), 0);
    println!("✓ Dependency scanning completed");

    // 12. Validate dependency pinning
    let is_pinned = manager
        .validate_dependency_pinning(sbom_for_scanning)
        .unwrap();
    assert!(is_pinned);
    println!("✓ Dependency pinning validation passed");

    // 13. Validate dependency trust
    let is_trusted = manager
        .validate_dependency_trust(sbom_for_scanning)
        .unwrap();
    assert!(is_trusted);
    println!("✓ Dependency trust validation passed");

    println!("Complete supply chain workflow test passed!");
}

/// Test supply chain with various component configurations
#[test]
fn test_supply_chain_component_configurations() {
    println!("Starting supply chain component configurations test");

    let mut manager = SupplyChainManager::new();

    // Test SBOM with many components
    let mut components = Vec::new();
    for i in 0..10 {
        components.push(Component {
            id: format!("component-{}", i),
            name: format!("dependency-{}", i),
            version: format!("1.{}.0", i),
            purl: format!("pkg:cargo/dependency-{}@1.{}.0", i, i),
            licenses: vec!["MIT".to_string()],
            hash: format!("sha256:hash{}", i),
            is_direct: i % 2 == 0,
            vulnerabilities: vec![],
            is_pinned: true,
            checksum_verified: true,
            is_approved: true,
        });
    }

    let sbom = Sbom {
        id: "test-sbom-many-components".to_string(),
        name: "test-app".to_string(),
        version: "2.0.0".to_string(),
        components,
        created: 1234567890,
        hash: "sha256:sbomhash".to_string(),
    };

    assert!(manager.store_sbom(sbom).is_ok());
    assert_eq!(manager.sboms.len(), 1);
    println!("✓ SBOM with many components handled correctly");

    // Test component with vulnerabilities
    let vulnerable_component = Component {
        id: "vulnerable-component".to_string(),
        name: "vulnerable-dep".to_string(),
        version: "1.0.0".to_string(),
        purl: "pkg:cargo/vulnerable-dep@1.0.0".to_string(),
        licenses: vec!["Apache-2.0".to_string()],
        hash: "sha256:vulnerablehash".to_string(),
        is_direct: true,
        vulnerabilities: vec![Vulnerability {
            id: "CVE-2023-12345".to_string(),
            severity: 7.5,
            description: "Buffer overflow vulnerability".to_string(),
            fix_available: true,
        }],
        is_pinned: true,
        checksum_verified: true,
        is_approved: true,
    };

    let sbom_with_vulns = Sbom {
        id: "test-sbom-with-vulns".to_string(),
        name: "vulnerable-app".to_string(),
        version: "1.0.0".to_string(),
        components: vec![vulnerable_component],
        created: 1234567891,
        hash: "sha256:sbomhash2".to_string(),
    };

    assert!(manager.store_sbom(sbom_with_vulns).is_ok());
    assert_eq!(manager.sboms.len(), 2);
    println!("✓ SBOM with vulnerabilities handled correctly");

    // Test multiple signatures
    for i in 0..5 {
        let artifact_hash = format!("sha256:artifact{}", i);
        let signature = manager.create_signature(&artifact_hash).unwrap();
        let artifact_uri = format!("artifact://test-{}", i);
        assert!(manager.store_signature(&artifact_uri, signature).is_ok());
    }

    assert_eq!(manager.signatures.len(), 5);
    println!("✓ Multiple signatures handled correctly");

    println!("Supply chain component configurations test passed!");
}

/// Test supply chain error handling
#[test]
fn test_supply_chain_error_handling() {
    println!("Starting supply chain error handling test");

    let manager = SupplyChainManager::new();

    // Test getting non-existent SBOM
    assert!(manager.get_sbom("non-existent").is_none());
    println!("✓ Non-existent SBOM handling verified");

    // Test getting non-existent provenance
    assert!(manager.get_provenance("non-existent").is_none());
    println!("✓ Non-existent provenance handling verified");

    // Test signature verification with wrong hash
    let signature = manager.create_signature("sha256:test").unwrap();
    assert!(!manager
        .verify_signature("sha256:wrong", &signature)
        .unwrap());
    println!("✓ Signature verification with wrong hash handled correctly");

    println!("Supply chain error handling test passed!");
}