//! Supply Chain Security Simulation Tests Binary
//!
//! This binary runs the supply chain security simulation tests to verify the effectiveness
//! of our supply chain security measures.

use core::supply_chain::{
    Artifact, BuildConfig, Component, Sbom, SourceInfo, SupplyChainManager, Vulnerability,
};
use std::collections::HashMap;

fn main() {
    println!("Running Supply Chain Security Simulation Tests");

    // Run all simulation tests
    test_realistic_supply_chain_scenario();
    test_supply_chain_under_stress();
    test_supply_chain_error_scenarios();

    println!("All Supply Chain Security Simulation Tests Passed!");
}

/// Test supply chain security in a realistic scenario with multiple components
fn test_realistic_supply_chain_scenario() {
    println!("Starting realistic supply chain scenario test");

    // Create supply chain manager
    let mut manager = SupplyChainManager::new();

    // Generate SBOM for a realistic application
    let sbom = manager.generate_sbom("decentralized-app", "1.2.0").unwrap();
    assert_eq!(sbom.name, "decentralized-app");
    assert_eq!(sbom.version, "1.2.0");
    assert!(!sbom.components.is_empty());

    println!("✓ SBOM generated for decentralized-app v1.2.0");

    // Store SBOM
    assert!(manager.store_sbom(sbom.clone()).is_ok());
    assert_eq!(manager.sboms.len(), 1);

    println!("✓ SBOM stored successfully");

    // Create signatures for multiple artifacts
    let artifacts = vec![
        ("sha256:binary-hash", "artifact://decentralized-app-binary"),
        ("sha256:config-hash", "artifact://decentralized-app-config"),
        ("sha256:docs-hash", "artifact://decentralized-app-docs"),
    ];

    for (hash, uri) in &artifacts {
        let signature = manager.create_signature(hash).unwrap();
        assert!(manager.store_signature(uri, signature).is_ok());
    }

    println!(
        "✓ Signatures created and stored for {} artifacts",
        artifacts.len()
    );

    // Verify all signatures
    for (hash, uri) in &artifacts {
        let signature = manager.signatures.get(*uri).unwrap();
        assert!(manager.verify_signature(hash, signature).unwrap());
    }

    println!("✓ All signatures verified successfully");

    // Create provenance for a build
    let source = SourceInfo {
        repo_url: "https://github.com/decentralized-app/core".to_string(),
        commit_hash: "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2".to_string(),
        branch: "release".to_string(),
        tag: Some("v1.2.0".to_string()),
    };

    let build_config = BuildConfig {
        build_script: "scripts/build.sh".to_string(),
        environment: {
            let mut env = HashMap::new();
            env.insert("RUSTFLAGS".to_string(), "-C opt-level=z".to_string());
            env.insert("CARGO_PROFILE".to_string(), "release".to_string());
            env
        },
        tools: {
            let mut tools = HashMap::new();
            tools.insert("rustc".to_string(), "1.70.0".to_string());
            tools.insert("cargo".to_string(), "1.70.0".to_string());
            tools
        },
    };

    let build_artifacts = vec![
        Artifact {
            name: "decentralized-app-binary".to_string(),
            uri: "artifact://decentralized-app-binary".to_string(),
            hash: "sha256:binary-hash".to_string(),
            size: 1024000,
            signature: None,
            sbom: None,
            is_signed: false,
            created: 0,
        },
        Artifact {
            name: "decentralized-app-config".to_string(),
            uri: "artifact://decentralized-app-config".to_string(),
            hash: "sha256:config-hash".to_string(),
            size: 5120,
            signature: None,
            sbom: None,
            is_signed: false,
            created: 0,
        },
    ];

    let provenance = manager
        .create_provenance("build-20230715-001", source, build_config, build_artifacts)
        .unwrap();
    assert!(manager.store_provenance(provenance).is_ok());

    println!("✓ Provenance information created and stored");

    // Scan dependencies for vulnerabilities
    let vulnerabilities = manager.scan_dependencies(&sbom).unwrap();
    assert_eq!(vulnerabilities.len(), 0);

    println!("✓ Dependency scanning completed with no vulnerabilities found");

    // Validate dependency pinning
    assert!(manager.validate_dependency_pinning(&sbom).is_ok());

    println!("✓ Dependency pinning validation passed");

    // Test SBOM retrieval
    let retrieved_sbom = manager.get_sbom(&sbom.id).unwrap();
    assert_eq!(retrieved_sbom.name, "decentralized-app");
    assert_eq!(retrieved_sbom.version, "1.2.0");

    println!("✓ SBOM retrieval verified");

    // Test provenance retrieval
    let retrieved_provenance = manager
        .get_provenance(&format!("prov-{}", "build-20230715-001"))
        .unwrap();
    assert_eq!(retrieved_provenance.build_id, "build-20230715-001");
    assert_eq!(
        retrieved_provenance.source.commit_hash,
        "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2"
    );
    assert_eq!(retrieved_provenance.artifacts.len(), 2);

    println!("✓ Provenance retrieval verified");

    println!("Realistic supply chain scenario test passed!");
}

/// Test supply chain security under stress conditions with many components
fn test_supply_chain_under_stress() {
    println!("Starting supply chain stress test");

    let mut manager = SupplyChainManager::new();

    // Generate SBOM with many components
    let mut components = Vec::new();
    for i in 0..100 {
        components.push(Component {
            id: format!("component-{}", i),
            name: format!("dependency-{}", i),
            version: format!("1.{}.0", i % 10),
            purl: format!("pkg:cargo/dependency-{}@1.{}.0", i, i % 10),
            licenses: vec![if i % 3 == 0 {
                "MIT".to_string()
            } else if i % 3 == 1 {
                "Apache-2.0".to_string()
            } else {
                "BSD-3-Clause".to_string()
            }],
            hash: format!("sha256:hash{}", i),
            is_direct: i % 5 == 0,
            vulnerabilities: if i % 20 == 0 {
                vec![Vulnerability {
                    id: format!("CVE-2023-{}", 1000 + i),
                    severity: (i % 10) as f32,
                    description: format!("Test vulnerability for component {}", i),
                    fix_available: i % 2 == 0,
                }]
            } else {
                vec![]
            },
            is_pinned: true,
            checksum_verified: true,
            is_approved: true,
        });
    }

    let sbom = Sbom {
        id: "stress-test-sbom".to_string(),
        name: "stress-test-app".to_string(),
        version: "1.0.0".to_string(),
        components,
        created: 1234567890,
        hash: "sha256:stress-test-sbom-hash".to_string(),
    };

    assert!(manager.store_sbom(sbom).is_ok());
    assert_eq!(manager.sboms.len(), 1);

    println!("✓ SBOM with 100 components stored successfully");

    // Create many signatures
    for i in 0..50 {
        let artifact_hash = format!("sha256:artifact-hash-{}", i);
        let signature = manager.create_signature(&artifact_hash).unwrap();
        let artifact_uri = format!("artifact://stress-test-{}", i);
        assert!(manager.store_signature(&artifact_uri, signature).is_ok());
    }

    assert_eq!(manager.signatures.len(), 50);

    println!("✓ 50 signatures created and stored successfully");

    // Verify all signatures
    for i in 0..50 {
        let artifact_hash = format!("sha256:artifact-hash-{}", i);
        let artifact_uri = format!("artifact://stress-test-{}", i);
        let signature = manager.signatures.get(&artifact_uri).unwrap();
        assert!(manager.verify_signature(&artifact_hash, signature).unwrap());
    }

    println!("✓ All 50 signatures verified successfully");

    // Create many provenance records
    for i in 0..20 {
        let source = SourceInfo {
            repo_url: format!("https://github.com/example/repo-{}", i),
            commit_hash: format!("commit-hash-{}", i),
            branch: "main".to_string(),
            tag: if i % 5 == 0 {
                Some(format!("v1.{}.0", i))
            } else {
                None
            },
        };

        let build_config = BuildConfig {
            build_script: format!("build-{}.sh", i),
            environment: HashMap::new(),
            tools: HashMap::new(),
        };

        let artifacts = vec![Artifact {
            name: format!("artifact-{}", i),
            uri: format!("artifact://test-{}", i),
            hash: format!("sha256:hash-{}", i),
            size: 1024 + i as u64,
            signature: None,
            sbom: None,
            is_signed: false,
            created: 0,
        }];

        let provenance = manager
            .create_provenance(&format!("build-{}", i), source, build_config, artifacts)
            .unwrap();
        assert!(manager.store_provenance(provenance).is_ok());
    }

    assert_eq!(manager.provenance.len(), 20);

    println!("✓ 20 provenance records created and stored successfully");

    // Test SBOM with vulnerabilities
    let sbom_with_vulns = manager.get_sbom("stress-test-sbom").unwrap();
    let vuln_count = sbom_with_vulns
        .components
        .iter()
        .map(|c| c.vulnerabilities.len())
        .sum::<usize>();
    assert_eq!(vuln_count, 5); // 100 components, every 20th has a vulnerability

    println!(
        "✓ SBOM vulnerability count verified ({} vulnerabilities found)",
        vuln_count
    );

    println!("Supply chain stress test passed!");
}

/// Test supply chain security error scenarios and edge cases
fn test_supply_chain_error_scenarios() {
    println!("Starting supply chain error scenarios test");

    let manager = SupplyChainManager::new();

    // Test error handling for non-existent SBOM
    assert!(manager.get_sbom("non-existent-sbom").is_none());
    println!("✓ Non-existent SBOM handling verified");

    // Test error handling for non-existent provenance
    assert!(manager.get_provenance("non-existent-provenance").is_none());
    println!("✓ Non-existent provenance handling verified");

    // Test signature verification with wrong hash
    let signature = manager.create_signature("sha256:correct-hash").unwrap();
    assert!(!manager
        .verify_signature("sha256:wrong-hash", &signature)
        .unwrap());
    println!("✓ Signature verification with wrong hash handled correctly");

    // Test with empty SBOM
    let empty_sbom = Sbom {
        id: "empty-sbom".to_string(),
        name: "empty-app".to_string(),
        version: "1.0.0".to_string(),
        components: vec![],
        created: 1234567890,
        hash: "sha256:empty-hash".to_string(),
    };

    assert!(manager.validate_dependency_pinning(&empty_sbom).is_ok());
    println!("✓ Empty SBOM dependency pinning validation handled correctly");

    // Test with large SBOM
    let mut large_components = Vec::new();
    for i in 0..1000 {
        large_components.push(Component {
            id: format!("large-component-{}", i),
            name: format!("large-dependency-{}", i),
            version: format!("1.0.{}", i),
            purl: format!("pkg:cargo/large-dependency-{}@1.0.{}", i, i),
            licenses: vec!["MIT".to_string()],
            hash: format!("sha256:large-hash-{}", i),
            is_direct: false,
            vulnerabilities: vec![],
            is_pinned: true,
            checksum_verified: true,
            is_approved: true,
        });
    }

    let large_sbom = Sbom {
        id: "large-sbom".to_string(),
        name: "large-app".to_string(),
        version: "1.0.0".to_string(),
        components: large_components,
        created: 1234567890,
        hash: "sha256:large-sbom-hash".to_string(),
    };

    let vulnerabilities = manager.scan_dependencies(&large_sbom).unwrap();
    assert_eq!(vulnerabilities.len(), 0);
    println!("✓ Large SBOM dependency scanning handled correctly");

    println!("Supply chain error scenarios test passed!");
}
