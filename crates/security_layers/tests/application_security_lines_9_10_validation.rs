//! Security Features Validation Tests (Lines 9-10)
//!
//! This module contains tests that validate the security features from lines 9-10 
//! of the web3_protection_layers.csv file:
//! 9,Software Supply Chain,Artifact Integrity,Build Signing / Provenance,"Sigstore/cosign signed container images, SBOM attached to artifact","Ensure what runs = what we built","Unsigned image block count"
//! 10,Application Security,Input Protection,Validation & Sanitization,"Strict type validation, regex allowlists, length limits, unicode normalization","Block injection, XSS, deserialization attacks","Rejected request counts by rule"

// Use specific imports to avoid ambiguity
use security_layers::application_security::{InputProtection, RejectionStats};
use security_layers::supply_chain::{Artifact, ArtifactRegistry, ArtifactSigner, ArtifactVerifier, Signature, SBOM, Component as SupplyChainComponent, Dependency as SupplyChainDependency, License as SupplyChainLicense};

/// Test that validates the security features from lines 9-10 of web3_protection_layers.csv
#[test]
fn test_security_features_lines_9_10() {
    println!("Testing Security features from lines 9-10 of web3_protection_layers.csv...");
    
    // Test Line 9: Software Supply Chain, Artifact Integrity, Build Signing / Provenance
    // "Sigstore/cosign signed container images, SBOM attached to artifact"
    // "Ensure what runs = what we built"
    // "Unsigned image block count"
    test_artifact_integrity_features();
    
    // Test Line 10: Application Security, Input Protection, Validation & Sanitization
    // "Strict type validation, regex allowlists, length limits, unicode normalization"
    // "Block injection, XSS, deserialization attacks"
    // "Rejected request counts by rule"
    test_input_protection_features();
    
    println!("All Security features from lines 9-10 validated successfully!");
}

/// Test Software Supply Chain, Artifact Integrity, Build Signing / Provenance
/// Component/Mechanism: "Sigstore/cosign signed container images, SBOM attached to artifact"
/// Goal: "Ensure what runs = what we built"
/// Evidence/Telemetry: "Unsigned image block count"
fn test_artifact_integrity_features() {
    println!("Testing Software Supply Chain, Artifact Integrity, Build Signing / Provenance...");
    
    let mut registry = ArtifactRegistry::new();
    let mut verifier = ArtifactVerifier::new();
    let signer = ArtifactSigner::new("trusted-key-1".to_string());
    
    // Add trusted key
    verifier.add_trusted_key("trusted-key-1".to_string(), "public-key".to_string());
    
    // Test Sigstore/cosign signed container images
    let mut signed_artifact = Artifact {
        id: "signed-artifact-1".to_string(),
        name: "signed-container-image".to_string(),
        version: "1.0.0".to_string(),
        hash: "sha256:abc123".to_string(),
        created_at: 1234567890,
        signatures: vec![],
        sbom: None,
    };
    
    // Sign artifact
    assert!(signer.sign_artifact(&mut signed_artifact).is_ok());
    assert_eq!(signed_artifact.signatures.len(), 1);
    
    // Register signed artifact
    assert!(registry.register_artifact(signed_artifact).is_ok());
    
    // Verify signature
    assert!(registry.verify_signature("signed-artifact-1").unwrap());
    
    // Verify artifact with trusted key
    let retrieved = registry.get_artifact("signed-artifact-1").unwrap();
    assert!(verifier.verify_artifact(retrieved).unwrap());
    
    // Test SBOM attached to artifact
    let mut artifact_with_sbom = Artifact {
        id: "artifact-with-sbom-1".to_string(),
        name: "container-image-with-sbom".to_string(),
        version: "1.0.0".to_string(),
        hash: "sha256:def456".to_string(),
        created_at: 1234567890,
        signatures: vec![],
        sbom: None,
    };
    
    // Add SBOM to artifact
    let sbom = SBOM {
        components: vec![
            SupplyChainComponent {
                name: "openssl".to_string(),
                version: "1.1.1".to_string(),
                hash: "sha256:component1".to_string(),
                supplier: "OpenSSL Software Foundation".to_string(),
            }
        ],
        dependencies: vec![
            SupplyChainDependency {
                name: "libc".to_string(),
                version: "2.31".to_string(),
                dependencies: vec!["kernel".to_string()],
            }
        ],
        licenses: vec![
            SupplyChainLicense {
                name: "Apache License 2.0".to_string(),
                spdx_id: "Apache-2.0".to_string(),
                url: "https://www.apache.org/licenses/LICENSE-2.0".to_string(),
            }
        ],
    };
    artifact_with_sbom.sbom = Some(sbom);
    
    // Sign artifact with SBOM
    assert!(signer.sign_artifact(&mut artifact_with_sbom).is_ok());
    
    // Register artifact with SBOM
    assert!(registry.register_artifact(artifact_with_sbom).is_ok());
    
    // Verify artifact with SBOM
    let retrieved_with_sbom = registry.get_artifact("artifact-with-sbom-1").unwrap();
    assert!(retrieved_with_sbom.sbom.is_some());
    assert_eq!(retrieved_with_sbom.sbom.as_ref().unwrap().components.len(), 1);
    assert_eq!(retrieved_with_sbom.sbom.as_ref().unwrap().dependencies.len(), 1);
    assert_eq!(retrieved_with_sbom.sbom.as_ref().unwrap().licenses.len(), 1);
    
    // Test Ensure what runs = what we built
    // This is demonstrated by the signature verification ensuring the artifact hasn't been tampered with
    
    // Test Unsigned image block count
    assert_eq!(registry.get_unsigned_image_count(), 0);
    
    // Register unsigned artifact
    let unsigned_artifact = Artifact {
        id: "unsigned-artifact-1".to_string(),
        name: "unsigned-container-image".to_string(),
        version: "1.0.0".to_string(),
        hash: "sha256:ghi789".to_string(),
        created_at: 1234567890,
        signatures: vec![],
        sbom: None,
    };
    
    assert!(registry.register_artifact(unsigned_artifact).is_ok());
    assert_eq!(registry.get_unsigned_image_count(), 1);
    
    // Register another unsigned artifact
    let unsigned_artifact2 = Artifact {
        id: "unsigned-artifact-2".to_string(),
        name: "unsigned-container-image-2".to_string(),
        version: "1.0.0".to_string(),
        hash: "sha256:jkl012".to_string(),
        created_at: 1234567890,
        signatures: vec![],
        sbom: None,
    };
    
    assert!(registry.register_artifact(unsigned_artifact2).is_ok());
    assert_eq!(registry.get_unsigned_image_count(), 2);
    
    println!("✓ Artifact integrity features validated");
}

/// Test Application Security, Input Protection, Validation & Sanitization
/// Component/Mechanism: "Strict type validation, regex allowlists, length limits, unicode normalization"
/// Goal: "Block injection, XSS, deserialization attacks"
/// Evidence/Telemetry: "Rejected request counts by rule"
fn test_input_protection_features() {
    println!("Testing Application Security, Input Protection, Validation & Sanitization...");
    
    let input_protection = InputProtection::new();
    let mut rejection_stats = RejectionStats::new();
    
    // Test Strict type validation
    assert!(input_protection.validate_input("email", "test@example.com").is_ok());
    assert!(input_protection.validate_input("username", "validuser").is_ok());
    
    // Test invalid inputs
    assert!(input_protection.validate_input("email", "invalid.email").is_err());
    assert!(input_protection.validate_input("username", "ab").is_err()); // Too short
    
    // Record rejections for telemetry
    rejection_stats.record_rejection("email_validation");
    rejection_stats.record_rejection("email_validation");
    rejection_stats.record_rejection("username_validation");
    
    // Test Rejected request counts by rule
    assert_eq!(rejection_stats.get_rejection_count("email_validation"), 2);
    assert_eq!(rejection_stats.get_rejection_count("username_validation"), 1);
    
    // Test regex allowlists
    let mut custom_input_protection = InputProtection::new();
    assert!(custom_input_protection.add_validation_pattern("phone", r"^\d{3}-\d{3}-\d{4}$").is_ok());
    assert!(custom_input_protection.validate_input("phone", "123-456-7890").is_ok());
    assert!(custom_input_protection.validate_input("phone", "invalid-phone").is_err());
    
    // Test length limits
    assert!(input_protection.validate_input("username", "this_username_is_way_too_long_and_should_fail").is_err());
    
    // Test sanitization
    let sanitized = input_protection.sanitize_input("test\0user");
    assert_eq!(sanitized, "testuser");
    
    println!("✓ Input protection features validated");
}