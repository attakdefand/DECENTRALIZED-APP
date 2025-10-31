//! Data Integrity Module
//!
//! This module implements data integrity measures including IPFS pin coverage monitoring,
//! on-chain hash anchoring, and content safety policies.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;


/// Represents a content item stored in decentralized storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentItem {
    /// Content Identifier (CID)
    pub cid: String,
    /// Content size in bytes
    pub size: u64,
    /// Content type
    pub content_type: String,
    /// Timestamp when content was added
    pub added_timestamp: u64,
    /// List of pinning services where content is pinned
    pub pinning_services: Vec<String>,
    /// Number of replicas
    pub replicas: u32,
    /// Whether content is critical
    pub is_critical: bool,
}

/// Represents a pinning service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinningService {
    /// Service name
    pub name: String,
    /// Service endpoint
    pub endpoint: String,
    /// Service status
    pub status: String,
    /// Last check timestamp
    pub last_check: u64,
    /// Pin coverage percentage
    pub coverage: f64,
}

/// Hash anchoring record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashAnchor {
    /// Content Identifier (CID)
    pub cid: String,
    /// Blockchain where hash is anchored
    pub chain: String,
    /// Transaction hash
    pub tx_hash: String,
    /// Block number
    pub block_number: u64,
    /// Timestamp
    pub timestamp: u64,
}

/// Content safety policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentSafetyPolicy {
    /// Allowed file types
    pub allowed_types: Vec<String>,
    /// Maximum file size in bytes
    pub max_size: u64,
    /// Content moderation required
    pub moderation_required: bool,
    /// Encryption required
    pub encryption_required: bool,
}

/// Data integrity error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataIntegrityError {
    /// Content not found
    ContentNotFound,
    /// Hash mismatch
    HashMismatch,
    /// Pinning service unavailable
    PinningServiceUnavailable,
    /// Content safety violation
    ContentSafetyViolation,
    /// Anchoring failed
    AnchoringFailed,
}

/// Data integrity manager
#[derive(Debug)]
pub struct DataIntegrityManager {
    /// Content items
    content_items: HashMap<String, ContentItem>,
    /// Pinning services
    pinning_services: HashMap<String, PinningService>,
    /// Hash anchors
    hash_anchors: HashMap<String, HashAnchor>,
    /// Content safety policy
    safety_policy: ContentSafetyPolicy,

}

impl DataIntegrityManager {
    /// Create a new data integrity manager
    pub fn new(safety_policy: ContentSafetyPolicy) -> Self {
        Self {
            content_items: HashMap::new(),
            pinning_services: HashMap::new(),
            hash_anchors: HashMap::new(),
            safety_policy,
        }
    }

    /// Add a content item
    pub fn add_content_item(&mut self, item: ContentItem) -> Result<(), DataIntegrityError> {
        // Validate content against safety policy
        self.validate_content(&item)?;

        // Add content item
        self.content_items.insert(item.cid.clone(), item);
        Ok(())
    }

    /// Validate content against safety policy
    fn validate_content(&self, item: &ContentItem) -> Result<(), DataIntegrityError> {
        // Check file size
        if item.size > self.safety_policy.max_size {
            return Err(DataIntegrityError::ContentSafetyViolation);
        }

        // Check file type
        if !self
            .safety_policy
            .allowed_types
            .contains(&item.content_type)
        {
            return Err(DataIntegrityError::ContentSafetyViolation);
        }

        Ok(())
    }

    /// Add a pinning service
    pub fn add_pinning_service(&mut self, service: PinningService) {
        self.pinning_services.insert(service.name.clone(), service);
    }

    /// Anchor a hash on-chain
    pub fn anchor_hash(&mut self, anchor: HashAnchor) -> Result<(), DataIntegrityError> {
        self.hash_anchors.insert(anchor.cid.clone(), anchor);
        Ok(())
    }

    /// Get content item
    pub fn get_content_item(&self, cid: &str) -> Option<&ContentItem> {
        self.content_items.get(cid)
    }

    /// Get pinning service
    pub fn get_pinning_service(&self, name: &str) -> Option<&PinningService> {
        self.pinning_services.get(name)
    }

    /// Get hash anchor
    pub fn get_hash_anchor(&self, cid: &str) -> Option<&HashAnchor> {
        self.hash_anchors.get(cid)
    }

    /// Check pin coverage for a content item
    pub fn check_pin_coverage(&self, cid: &str) -> Result<f64, DataIntegrityError> {
        let item = self
            .content_items
            .get(cid)
            .ok_or(DataIntegrityError::ContentNotFound)?;

        // Calculate coverage based on available pinning services
        let total_services = self.pinning_services.len() as f64;
        if total_services == 0.0 {
            return Ok(0.0);
        }

        let pinned_services = item.pinning_services.len() as f64;
        let coverage = (pinned_services / total_services) * 100.0;
        Ok(coverage)
    }

    /// Verify content integrity
    pub fn verify_content_integrity(
        &self,
        cid: &str,
        expected_cid: &str,
    ) -> Result<bool, DataIntegrityError> {
        let item = self
            .content_items
            .get(cid)
            .ok_or(DataIntegrityError::ContentNotFound)?;

        // Verify CID matches
        Ok(item.cid == expected_cid)
    }

    /// Get content safety policy
    pub fn get_safety_policy(&self) -> &ContentSafetyPolicy {
        &self.safety_policy
    }

    /// Update content safety policy
    pub fn update_safety_policy(&mut self, policy: ContentSafetyPolicy) {
        self.safety_policy = policy;
    }

    /// Get all content items
    pub fn get_all_content_items(&self) -> Vec<&ContentItem> {
        self.content_items.values().collect()
    }

    /// Get all pinning services
    pub fn get_all_pinning_services(&self) -> Vec<&PinningService> {
        self.pinning_services.values().collect()
    }

    /// Get all hash anchors
    pub fn get_all_hash_anchors(&self) -> Vec<&HashAnchor> {
        self.hash_anchors.values().collect()
    }

    /// Check if content meets safety requirements
    pub fn is_content_safe(&self, item: &ContentItem) -> bool {
        // Check if content type is allowed
        if !self
            .safety_policy
            .allowed_types
            .contains(&item.content_type)
        {
            return false;
        }

        // Check if size is within limits
        if item.size > self.safety_policy.max_size {
            return false;
        }

        true
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_integrity_manager_creation() {
        let policy = ContentSafetyPolicy {
            allowed_types: vec!["text/plain".to_string(), "application/json".to_string()],
            max_size: 1024 * 1024, // 1MB
            moderation_required: false,
            encryption_required: false,
        };

        let manager = DataIntegrityManager::new(policy);
        assert_eq!(manager.get_all_content_items().len(), 0);
        assert_eq!(manager.get_all_pinning_services().len(), 0);
    }

    #[test]
    fn test_add_content_item() {
        let policy = ContentSafetyPolicy {
            allowed_types: vec!["text/plain".to_string()],
            max_size: 1024,
            moderation_required: false,
            encryption_required: false,
        };

        let mut manager = DataIntegrityManager::new(policy);

        let item = ContentItem {
            cid: "QmTest123".to_string(),
            size: 512,
            content_type: "text/plain".to_string(),
            added_timestamp: 1234567890,
            pinning_services: vec!["pinata".to_string()],
            replicas: 1,
            is_critical: false,
        };

        assert!(manager.add_content_item(item).is_ok());
        assert!(manager.get_content_item("QmTest123").is_some());
    }

    #[test]
    fn test_content_safety_violation() {
        let policy = ContentSafetyPolicy {
            allowed_types: vec!["text/plain".to_string()],
            max_size: 1024,
            moderation_required: false,
            encryption_required: false,
        };

        let mut manager = DataIntegrityManager::new(policy);

        // Try to add content that exceeds size limit
        let large_item = ContentItem {
            cid: "QmLarge123".to_string(),
            size: 2048, // Exceeds 1024 limit
            content_type: "text/plain".to_string(),
            added_timestamp: 1234567890,
            pinning_services: vec!["pinata".to_string()],
            replicas: 1,
            is_critical: false,
        };

        assert!(manager.add_content_item(large_item).is_err());

        // Try to add content with disallowed type
        let wrong_type_item = ContentItem {
            cid: "QmWrong123".to_string(),
            size: 512,
            content_type: "application/exe".to_string(), // Not allowed
            added_timestamp: 1234567890,
            pinning_services: vec!["pinata".to_string()],
            replicas: 1,
            is_critical: false,
        };

        assert!(manager.add_content_item(wrong_type_item).is_err());
    }

    #[test]
    fn test_pinning_service_management() {
        let policy = ContentSafetyPolicy {
            allowed_types: vec!["text/plain".to_string()],
            max_size: 1024,
            moderation_required: false,
            encryption_required: false,
        };

        let mut manager = DataIntegrityManager::new(policy);

        let service = PinningService {
            name: "pinata".to_string(),
            endpoint: "https://api.pinata.cloud".to_string(),
            status: "active".to_string(),
            last_check: 1234567890,
            coverage: 99.5,
        };

        manager.add_pinning_service(service);
        assert!(manager.get_pinning_service("pinata").is_some());
    }

    #[test]
    fn test_hash_anchoring() {
        let policy = ContentSafetyPolicy {
            allowed_types: vec!["text/plain".to_string()],
            max_size: 1024,
            moderation_required: false,
            encryption_required: false,
        };

        let mut manager = DataIntegrityManager::new(policy);

        let anchor = HashAnchor {
            cid: "QmTest123".to_string(),
            chain: "ethereum".to_string(),
            tx_hash: "0x1234567890abcdef".to_string(),
            block_number: 1234567,
            timestamp: 1234567890,
        };

        assert!(manager.anchor_hash(anchor).is_ok());
        assert!(manager.get_hash_anchor("QmTest123").is_some());
    }

    #[test]
    fn test_pin_coverage_check() {
        let policy = ContentSafetyPolicy {
            allowed_types: vec!["text/plain".to_string()],
            max_size: 1024,
            moderation_required: false,
            encryption_required: false,
        };

        let mut manager = DataIntegrityManager::new(policy);

        // Add pinning services
        let service1 = PinningService {
            name: "pinata".to_string(),
            endpoint: "https://api.pinata.cloud".to_string(),
            status: "active".to_string(),
            last_check: 1234567890,
            coverage: 99.5,
        };

        let service2 = PinningService {
            name: "infura".to_string(),
            endpoint: "https://ipfs.infura.io".to_string(),
            status: "active".to_string(),
            last_check: 1234567890,
            coverage: 98.7,
        };

        manager.add_pinning_service(service1);
        manager.add_pinning_service(service2);

        // Add content item pinned on both services
        let item = ContentItem {
            cid: "QmTest123".to_string(),
            size: 512,
            content_type: "text/plain".to_string(),
            added_timestamp: 1234567890,
            pinning_services: vec!["pinata".to_string(), "infura".to_string()],
            replicas: 2,
            is_critical: true,
        };

        manager.add_content_item(item).unwrap();

        // Check pin coverage (should be 100% since content is pinned on both services)
        let coverage = manager.check_pin_coverage("QmTest123").unwrap();
        assert_eq!(coverage, 100.0);
    }

    #[test]
    fn test_content_integrity_verification() {
        let policy = ContentSafetyPolicy {
            allowed_types: vec!["text/plain".to_string()],
            max_size: 1024,
            moderation_required: false,
            encryption_required: false,
        };

        let mut manager = DataIntegrityManager::new(policy);

        let item = ContentItem {
            cid: "QmTest123".to_string(),
            size: 512,
            content_type: "text/plain".to_string(),
            added_timestamp: 1234567890,
            pinning_services: vec!["pinata".to_string()],
            replicas: 1,
            is_critical: false,
        };

        manager.add_content_item(item).unwrap();

        // Verify content integrity
        assert!(manager
            .verify_content_integrity("QmTest123", "QmTest123")
            .unwrap());
        assert!(!manager
            .verify_content_integrity("QmTest123", "QmWrong123")
            .unwrap());
    }

    #[test]
    fn test_content_safety_check() {
        let policy = ContentSafetyPolicy {
            allowed_types: vec!["text/plain".to_string(), "application/json".to_string()],
            max_size: 1024,
            moderation_required: false,
            encryption_required: false,
        };

        let manager = DataIntegrityManager::new(policy);

        let safe_item = ContentItem {
            cid: "QmSafe123".to_string(),
            size: 512,
            content_type: "text/plain".to_string(),
            added_timestamp: 1234567890,
            pinning_services: vec!["pinata".to_string()],
            replicas: 1,
            is_critical: false,
        };

        let unsafe_item = ContentItem {
            cid: "QmUnsafe123".to_string(),
            size: 2048, // Exceeds size limit
            content_type: "text/plain".to_string(),
            added_timestamp: 1234567890,
            pinning_services: vec!["pinata".to_string()],
            replicas: 1,
            is_critical: false,
        };

        assert!(manager.is_content_safe(&safe_item));
        assert!(!manager.is_content_safe(&unsafe_item));
    }
}
