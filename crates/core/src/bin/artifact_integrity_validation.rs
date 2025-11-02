//! Artifact Integrity Validation Script
//!
//! This script validates the artifact integrity features including:
//! - Sigstore/cosign signed container images
//! - SBOM attached to artifact
//! - Unsigned image block count telemetry

use core::supply_chain::SupplyChainManager;

fn main() {
    println!("Running Artifact Integrity Validation");

    // Create supply chain manager
    let mut manager = SupplyChainManager::new();
    
    // Test 1: Create an unsigned artifact
    println!("\n1. Testing unsigned artifact handling");
    let unsigned_artifact = manager
        .create_artifact(
            "test-unsigned-artifact",
            "artifact://test-unsigned",
            "sha256:unsigned123",
            1024,
            None,  // No signature
            None,  // No SBOM
        )
        .unwrap();
    
    // Verify unsigned artifact (should be blocked)
    let is_valid = manager.verify_artifact_integrity(&unsigned_artifact).unwrap();
    assert!(!is_valid, "Unsigned artifact should be invalid");
    
    // Check statistics
    let (total, unsigned, verified, failures) = manager.get_integrity_stats();
    assert_eq!(total, 1, "Total artifacts should be 1");
    assert_eq!(unsigned, 1, "Unsigned artifacts should be 1");
    assert_eq!(verified, 0, "Verified artifacts should be 0");
    assert_eq!(failures, 0, "Failures should be 0");
    
    println!("✓ Unsigned artifact correctly blocked");
    println!("✓ Statistics correctly updated: total={}, unsigned={}, verified={}, failures={}", 
             total, unsigned, verified, failures);
    
    // Test 2: Create a signed artifact with SBOM
    println!("\n2. Testing signed artifact with SBOM");
    let signature = manager.create_signature("sha256:signed123").unwrap();
    let sbom = manager.generate_sbom("test-component", "1.0.0").unwrap();
    
    let mut signed_artifact = manager
        .create_artifact(
            "test-signed-artifact",
            "artifact://test-signed",
            "sha256:signed123",
            2048,
            Some(signature.clone()),
            Some(sbom.clone()),
        )
        .unwrap();
    
    // Store the signature
    manager.store_signature("artifact://test-signed", signature).unwrap();
    
    // Attach SBOM to artifact
    manager.attach_sbom_to_artifact(&mut signed_artifact, sbom).unwrap();
    
    // Verify signed artifact (should be valid)
    let is_valid = manager.verify_artifact_integrity(&signed_artifact).unwrap();
    assert!(is_valid, "Signed artifact should be valid");
    
    // Check statistics
    let (total, unsigned, verified, failures) = manager.get_integrity_stats();
    assert_eq!(total, 2, "Total artifacts should be 2");
    assert_eq!(unsigned, 1, "Unsigned artifacts should still be 1");
    assert_eq!(verified, 1, "Verified artifacts should be 1");
    assert_eq!(failures, 0, "Failures should be 0");
    
    println!("✓ Signed artifact with SBOM correctly verified");
    println!("✓ Statistics correctly updated: total={}, unsigned={}, verified={}, failures={}", 
             total, unsigned, verified, failures);
    
    // Test 3: Test Sigstore/cosign signing
    println!("\n3. Testing Sigstore/cosign signing");
    let cosign_signature = manager
        .sign_artifact_with_cosign("artifact://test-cosign")
        .unwrap();
    
    // Store the signature for verification
    manager.create_cosign_signature("artifact://test-cosign", &cosign_signature).unwrap();
    
    // Verify cosign signature
    let is_valid = manager.verify_cosign_signature("artifact://test-cosign").unwrap();
    assert!(is_valid, "Cosign signature should be valid");
    
    println!("✓ Sigstore/cosign signing and verification working correctly");
    
    // Test 4: Create artifact with cosign signature
    println!("\n4. Testing artifact with cosign signature");
    let _cosign_signature = manager
        .sign_artifact_with_cosign("artifact://test-cosign-artifact")
        .unwrap();
    
    let artifact_with_cosign = manager
        .create_artifact(
            "test-cosign-artifact",
            "artifact://test-cosign-artifact",
            "sha256:cosign123",
            4096,
            None,  // No regular signature
            None,  // No SBOM
        )
        .unwrap();
    
    // Verify artifact with cosign signature (should be valid)
    let is_valid = manager.verify_artifact_integrity(&artifact_with_cosign).unwrap();
    assert!(is_valid, "Artifact with cosign signature should be valid");
    
    // Check statistics
    let (total, unsigned, verified, failures) = manager.get_integrity_stats();
    assert_eq!(total, 3, "Total artifacts should be 3");
    assert_eq!(unsigned, 1, "Unsigned artifacts should still be 1");
    assert_eq!(verified, 2, "Verified artifacts should be 2");
    assert_eq!(failures, 0, "Failures should be 0");
    
    println!("✓ Artifact with cosign signature correctly verified");
    println!("✓ Statistics correctly updated: total={}, unsigned={}, verified={}, failures={}", 
             total, unsigned, verified, failures);
    
    println!("\nAll Artifact Integrity Validation Tests Passed!");
    println!("Artifact Integrity Features Working Correctly:");
    println!("- Sigstore/cosign signed container images ✓");
    println!("- SBOM attached to artifact ✓");
    println!("- Unsigned image block count telemetry ✓");
}