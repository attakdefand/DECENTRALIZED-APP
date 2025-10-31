//! Software Supply Chain Module
//!
//! This module implements security layers 9: Software Supply Chain
//!
//! Line 9 from web3_protection_layers.csv:
//! 9,Software Supply Chain,Artifact Integrity,Build Signing / Provenance,"Sigstore/cosign signed container images, SBOM attached to artifact","Ensure what runs = what we built","Unsigned image block count"

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a software artifact with its metadata
#[derive(Debug, Clone)]
pub struct Artifact {
    /// Unique identifier for the artifact
    pub id: String,
    /// Name of the artifact
    pub name: String,
    /// Version of the artifact
    pub version: String,
    /// Hash of the artifact
    pub hash: String,
    /// Timestamp when the artifact was created
    pub created_at: u64,
    /// Signatures for the artifact
    pub signatures: Vec<Signature>,
    /// Software Bill of Materials (SBOM) attached to the artifact
    pub sbom: Option<SBOM>,
}

/// Represents a digital signature
#[derive(Debug, Clone)]
pub struct Signature {
    /// The signature data
    pub signature: String,
    /// Public key identifier of the signer
    pub key_id: String,
    /// Timestamp when the signature was created
    pub timestamp: u64,
    /// Algorithm used for signing
    pub algorithm: String,
}

/// Represents a Software Bill of Materials
#[derive(Debug, Clone)]
pub struct SBOM {
    /// Components included in the artifact
    pub components: Vec<Component>,
    /// Dependencies of the artifact
    pub dependencies: Vec<Dependency>,
    /// Licenses used by components
    pub licenses: Vec<License>,
}

/// Represents a component in the SBOM
#[derive(Debug, Clone)]
pub struct Component {
    /// Name of the component
    pub name: String,
    /// Version of the component
    pub version: String,
    /// Hash of the component
    pub hash: String,
    /// Supplier of the component
    pub supplier: String,
}

/// Represents a dependency in the SBOM
#[derive(Debug, Clone)]
pub struct Dependency {
    /// Name of the dependency
    pub name: String,
    /// Version of the dependency
    pub version: String,
    /// Dependencies of this dependency
    pub dependencies: Vec<String>,
}

/// Represents a license in the SBOM
#[derive(Debug, Clone)]
pub struct License {
    /// Name of the license
    pub name: String,
    /// SPDX identifier of the license
    pub spdx_id: String,
    /// URL to the license text
    pub url: String,
}

/// Artifact registry for managing software artifacts
pub struct ArtifactRegistry {
    /// Storage for artifacts
    artifacts: HashMap<String, Artifact>,
    /// Count of unsigned images
    unsigned_image_count: u64,
}

impl ArtifactRegistry {
    /// Create a new artifact registry
    pub fn new() -> Self {
        Self {
            artifacts: HashMap::new(),
            unsigned_image_count: 0,
        }
    }
    
    /// Register a new artifact
    pub fn register_artifact(&mut self, artifact: Artifact) -> Result<(), String> {
        // Check if artifact already exists
        if self.artifacts.contains_key(&artifact.id) {
            return Err("Artifact already exists".to_string());
        }
        
        // Check if artifact has signatures
        if artifact.signatures.is_empty() {
            self.unsigned_image_count += 1;
        }
        
        self.artifacts.insert(artifact.id.clone(), artifact);
        Ok(())
    }
    
    /// Get an artifact by ID
    pub fn get_artifact(&self, id: &str) -> Option<&Artifact> {
        self.artifacts.get(id)
    }
    
    /// Verify that an artifact is signed
    pub fn verify_signature(&self, artifact_id: &str) -> Result<bool, String> {
        let artifact = self.get_artifact(artifact_id)
            .ok_or("Artifact not found")?;
        
        Ok(!artifact.signatures.is_empty())
    }
    
    /// Get count of unsigned images
    pub fn get_unsigned_image_count(&self) -> u64 {
        self.unsigned_image_count
    }
    
    /// List all artifacts
    pub fn list_artifacts(&self) -> Vec<&Artifact> {
        self.artifacts.values().collect()
    }
    
    /// Remove an artifact
    pub fn remove_artifact(&mut self, id: &str) -> Result<(), String> {
        if self.artifacts.remove(id).is_some() {
            Ok(())
        } else {
            Err("Artifact not found".to_string())
        }
    }
}

impl Default for ArtifactRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Artifact signing service
pub struct ArtifactSigner {
    /// Public key identifier
    key_id: String,
}

impl ArtifactSigner {
    /// Create a new artifact signer
    pub fn new(key_id: String) -> Self {
        Self {
            key_id,
        }
    }
    
    /// Sign an artifact
    pub fn sign_artifact(&self, artifact: &mut Artifact) -> Result<(), String> {
        // In a real implementation, we would use a proper signing algorithm
        // For this example, we'll create a mock signature
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_secs();
        
        let signature = Signature {
            signature: format!("signature_for_{}_at_{}", artifact.id, timestamp),
            key_id: self.key_id.clone(),
            timestamp,
            algorithm: "mock-signature-algorithm".to_string(),
        };
        
        artifact.signatures.push(signature);
        Ok(())
    }
}

/// Artifact verification service
pub struct ArtifactVerifier {
    /// Trusted public keys
    trusted_keys: HashMap<String, String>,
}

impl ArtifactVerifier {
    /// Create a new artifact verifier
    pub fn new() -> Self {
        Self {
            trusted_keys: HashMap::new(),
        }
    }
    
    /// Add a trusted key
    pub fn add_trusted_key(&mut self, key_id: String, public_key: String) {
        self.trusted_keys.insert(key_id, public_key);
    }
    
    /// Verify an artifact's signatures
    pub fn verify_artifact(&self, artifact: &Artifact) -> Result<bool, String> {
        // Check if artifact has any signatures
        if artifact.signatures.is_empty() {
            return Ok(false);
        }
        
        // In a real implementation, we would verify the signatures using the public keys
        // For this example, we'll just check if the key is trusted
        for signature in &artifact.signatures {
            if !self.trusted_keys.contains_key(&signature.key_id) {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
}

impl Default for ArtifactVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artifact_registry() {
        let mut registry = ArtifactRegistry::new();
        
        let artifact = Artifact {
            id: "test-artifact-1".to_string(),
            name: "test-image".to_string(),
            version: "1.0.0".to_string(),
            hash: "abc123".to_string(),
            created_at: 1234567890,
            signatures: vec![],
            sbom: None,
        };
        
        // Register artifact
        assert!(registry.register_artifact(artifact).is_ok());
        
        // Try to register the same artifact again
        let duplicate = Artifact {
            id: "test-artifact-1".to_string(),
            name: "test-image".to_string(),
            version: "1.0.0".to_string(),
            hash: "abc123".to_string(),
            created_at: 1234567890,
            signatures: vec![],
            sbom: None,
        };
        assert!(registry.register_artifact(duplicate).is_err());
        
        // Get artifact
        let retrieved = registry.get_artifact("test-artifact-1");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "test-image");
        
        // List artifacts
        let artifacts = registry.list_artifacts();
        assert_eq!(artifacts.len(), 1);
        
        // Remove artifact
        assert!(registry.remove_artifact("test-artifact-1").is_ok());
        assert!(registry.get_artifact("test-artifact-1").is_none());
    }
    
    #[test]
    fn test_artifact_signing() {
        let mut registry = ArtifactRegistry::new();
        let signer = ArtifactSigner::new("key-1".to_string());
        
        let mut artifact = Artifact {
            id: "test-artifact-1".to_string(),
            name: "test-image".to_string(),
            version: "1.0.0".to_string(),
            hash: "abc123".to_string(),
            created_at: 1234567890,
            signatures: vec![],
            sbom: None,
        };
        
        // Sign artifact
        assert!(signer.sign_artifact(&mut artifact).is_ok());
        assert_eq!(artifact.signatures.len(), 1);
        
        // Register signed artifact
        assert!(registry.register_artifact(artifact).is_ok());
        
        // Verify signature
        assert!(registry.verify_signature("test-artifact-1").unwrap());
    }
    
    #[test]
    fn test_unsigned_image_count() {
        let mut registry = ArtifactRegistry::new();
        
        // Register unsigned artifact
        let unsigned_artifact = Artifact {
            id: "unsigned-artifact-1".to_string(),
            name: "unsigned-image".to_string(),
            version: "1.0.0".to_string(),
            hash: "def456".to_string(),
            created_at: 1234567890,
            signatures: vec![],
            sbom: None,
        };
        
        assert!(registry.register_artifact(unsigned_artifact).is_ok());
        assert_eq!(registry.get_unsigned_image_count(), 1);
        
        // Register another unsigned artifact
        let unsigned_artifact2 = Artifact {
            id: "unsigned-artifact-2".to_string(),
            name: "unsigned-image-2".to_string(),
            version: "1.0.0".to_string(),
            hash: "ghi789".to_string(),
            created_at: 1234567890,
            signatures: vec![],
            sbom: None,
        };
        
        assert!(registry.register_artifact(unsigned_artifact2).is_ok());
        assert_eq!(registry.get_unsigned_image_count(), 2);
    }
    
    #[test]
    fn test_artifact_verification() {
        let mut registry = ArtifactRegistry::new();
        let mut verifier = ArtifactVerifier::new();
        let signer = ArtifactSigner::new("trusted-key-1".to_string());
        
        // Add trusted key
        verifier.add_trusted_key("trusted-key-1".to_string(), "public-key".to_string());
        
        let mut artifact = Artifact {
            id: "signed-artifact-1".to_string(),
            name: "signed-image".to_string(),
            version: "1.0.0".to_string(),
            hash: "jkl012".to_string(),
            created_at: 1234567890,
            signatures: vec![],
            sbom: None,
        };
        
        // Sign artifact
        assert!(signer.sign_artifact(&mut artifact).is_ok());
        
        // Register signed artifact
        assert!(registry.register_artifact(artifact).is_ok());
        
        // Verify artifact
        let retrieved = registry.get_artifact("signed-artifact-1").unwrap();
        assert!(verifier.verify_artifact(retrieved).unwrap());
    }
}