//! Tests for supply chain build integrity features

use core::supply_chain::{
    SupplyChainManager, Sbom, Component, SourceInfo, BuildConfig, Artifact, 
    Provenance, SupplyChainError
};
use std::collections::HashMap;

/// Test reproducible builds functionality
#[test]
fn test_reproducible_builds() {
    let mut manager = SupplyChainManager::new();
    
    // Create a build configuration
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
    
    // Generate reproducible build hash
    let hash = manager.generate_reproducible_build_hash(&build_config).unwrap();
    assert_eq!(hash, "sha256:reproducible-build-hash");
    
    // Verify reproducible build
    let is_reproducible = manager.verify_reproducible_build(&build_config, &hash).unwrap();
    assert!(is_reproducible);
    
    println!("✓ Reproducible builds functionality validated");
}

/// Test SBOM generation and attachment to artifacts
#[test]
fn test_sbom_generation_and_attachment() {
    let mut manager = SupplyChainManager::new();
    
    // Generate SBOM
    let sbom = manager.generate_sbom("test-app", "1.0.0").unwrap();
    assert_eq!(sbom.name, "test-app");
    assert_eq!(sbom.version, "1.0.0");
    assert_eq!(sbom.components.len(), 2);
    
    // Store SBOM
    manager.store_sbom(sbom.clone()).unwrap();
    
    // Create artifact
    let mut artifact = Artifact {
        name: "test-artifact".to_string(),
        uri: "artifact://test-artifact".to_string(),
        hash: "sha256:test-hash".to_string(),
        size: 1024,
        signature: None,
        sbom: None,
        is_signed: false,
        created: 0,
    };
    
    // Attach SBOM to artifact
    manager.attach_sbom_to_artifact(&mut artifact, sbom.clone()).unwrap();
    assert!(artifact.sbom.is_some());
    assert_eq!(artifact.sbom.as_ref().unwrap().id, sbom.id);
    
    println!("✓ SBOM generation and attachment validated");
}

/// Test cosign signing functionality
#[test]
fn test_cosign_signing() {
    let mut manager = SupplyChainManager::new();
    
    // Sign artifact with cosign
    let artifact_uri = "artifact://test-app-binary";
    let signature = manager.sign_artifact_with_cosign(artifact_uri).unwrap();
    assert_eq!(signature, format!("cosign-signature-for-{}", artifact_uri));
    
    // Verify cosign signature
    let is_valid = manager.verify_cosign_signature(artifact_uri).unwrap();
    assert!(is_valid);
    
    println!("✓ Cosign signing functionality validated");
}

/// Test complete build integrity workflow
#[test]
fn test_complete_build_integrity_workflow() {
    let mut manager = SupplyChainManager::new();
    
    // 1. Generate SBOM
    let sbom = manager.generate_sbom("decentralized-app", "2.0.0").unwrap();
    manager.store_sbom(sbom.clone()).unwrap();
    
    // 2. Create source information
    let source = SourceInfo {
        repo_url: "https://github.com/decentralized-app/core".to_string(),
        commit_hash: "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2".to_string(),
        branch: "release".to_string(),
        tag: Some("v2.0.0".to_string()),
    };
    
    // 3. Create build configuration
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
    
    // 4. Create build artifacts
    let artifacts = vec![
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
    
    // 5. Create provenance
    let provenance = manager.create_provenance("build-20230715-001", source, build_config, artifacts).unwrap();
    manager.store_provenance(provenance.clone()).unwrap();
    
    // 6. Generate reproducible build hash
    let build_hash = manager.generate_reproducible_build_hash(&provenance.build_config).unwrap();
    
    // 7. Verify reproducible build
    let is_reproducible = manager.verify_reproducible_build(&provenance.build_config, &build_hash).unwrap();
    assert!(is_reproducible);
    
    // 8. Sign artifacts with cosign
    for artifact in &provenance.artifacts {
        let signature = manager.sign_artifact_with_cosign(&artifact.uri).unwrap();
        assert!(!signature.is_empty());
    }
    
    // 9. Attach SBOM to artifacts
    let mut updated_artifacts = provenance.artifacts.clone();
    for artifact in &mut updated_artifacts {
        manager.attach_sbom_to_artifact(artifact, sbom.clone()).unwrap();
        assert!(artifact.sbom.is_some());
    }
    
    // 10. Validate dependency trust
    let trust_result = manager.validate_dependency_trust(&sbom);
    assert!(trust_result.is_ok());
    
    // 11. Validate dependency pinning
    let pinning_result = manager.validate_dependency_pinning(&sbom);
    assert!(pinning_result.is_ok());
    
    println!("✓ Complete build integrity workflow validated");
}

/// Test build integrity policy enforcement
#[test]
fn test_build_integrity_policy_enforcement() {
    let manager = SupplyChainManager::new();
    
    // Check that reproducible builds policy is enabled
    let policy = manager.policy_rules.get("reproducible-builds-required").unwrap();
    assert!(policy.enabled);
    assert_eq!(policy.severity, "high");
    assert_eq!(policy.category, "build-integrity");
    
    // Check that SBOM generation policy is enabled
    let policy = manager.policy_rules.get("sbom-generation-required").unwrap();
    assert!(policy.enabled);
    assert_eq!(policy.severity, "medium");
    assert_eq!(policy.category, "sbom");
    
    println!("✓ Build integrity policy enforcement validated");
}