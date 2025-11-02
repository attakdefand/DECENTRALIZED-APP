//! Storage Integrity Implementation Tests
//!
//! This module contains tests that validate the storage integrity implementation
//! including IPFS/Arweave pinning, hash anchoring, and CID parity.

use core::data_integrity::{DataIntegrityManager, ContentItem, PinningService, HashAnchor, ContentSafetyPolicy};

/// Test storage integrity with IPFS pinning and hash anchoring
#[test]
fn test_storage_integrity_ipfs_pinning_and_hash_anchoring() {
    println!("Starting storage integrity IPFS pinning and hash anchoring test");
    
    // Create a safety policy for storage integrity
    let safety_policy = ContentSafetyPolicy {
        allowed_types: vec![
            "text/plain".to_string(),
            "application/json".to_string(),
            "image/png".to_string(),
            "image/jpeg".to_string(),
        ],
        max_size: 10 * 1024 * 1024, // 10MB limit
        moderation_required: false,
        encryption_required: false,
    };
    
    let mut manager = DataIntegrityManager::new(safety_policy);
    
    // Add pinning services as specified in the requirements
    let pinning_services = vec![
        ("pinata", "https://api.pinata.cloud", 99.8),
        ("infura", "https://ipfs.infura.io", 99.2),
        ("textile", "https://hub.textile.io", 98.5),
        ("nft-storage", "https://nft.storage", 99.9),
        ("own-node-1", "https://ipfs.ourcompany.com", 100.0),
        ("own-node-2", "https://ipfs.backup.ourcompany.com", 99.7),
    ];
    
    for (name, endpoint, coverage) in &pinning_services {
        let service = PinningService {
            name: name.to_string(),
            endpoint: endpoint.to_string(),
            status: "active".to_string(),
            last_check: 1234567890,
            coverage: *coverage,
        };
        manager.add_pinning_service(service);
    }
    
    println!("✓ Added {} pinning services for storage integrity", pinning_services.len());
    
    // Add content items that should be pinned with high coverage
    let content_items = vec![
        // Critical metadata that should be pinned everywhere
        ContentItem {
            cid: "QmCriticalMetadata123".to_string(),
            size: 2048,
            content_type: "application/json".to_string(),
            added_timestamp: 1234567890,
            pinning_services: pinning_services.iter().map(|(name, _, _)| name.to_string()).collect(),
            replicas: pinning_services.len() as u32,
            is_critical: true,
        },
        // Regular content pinned on most services
        ContentItem {
            cid: "QmRegularContent123".to_string(),
            size: 512000, // 500KB
            content_type: "image/png".to_string(),
            added_timestamp: 1234567890,
            pinning_services: vec![
                "pinata".to_string(),
                "infura".to_string(),
                "textile".to_string(),
                "nft-storage".to_string(),
                "own-node-1".to_string(),
            ],
            replicas: 5,
            is_critical: false,
        },
    ];
    
    // Add all content items
    for item in &content_items {
        assert!(manager.add_content_item(item.clone()).is_ok(), "Content item should be added");
    }
    
    println!("✓ Added {} content items for storage integrity", content_items.len());
    
    // Anchor critical content hashes on-chain for CID parity
    let hash_anchors = vec![
        HashAnchor {
            cid: "QmCriticalMetadata123".to_string(),
            chain: "ethereum".to_string(),
            tx_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
            block_number: 12345678,
            timestamp: 1234567890,
        },
    ];
    
    for anchor in &hash_anchors {
        assert!(manager.anchor_hash(anchor.clone()).is_ok(), "Hash should be anchored");
    }
    
    println!("✓ Anchored {} critical content hashes for CID parity", hash_anchors.len());
    
    // Verify all content items meet storage integrity requirements
    for item in &content_items {
        // Check pin coverage meets requirements
        let coverage = manager.check_pin_coverage(&item.cid).expect("Pin coverage check should succeed");
        if item.is_critical {
            // Critical items should have high coverage (close to 100%)
            assert!(coverage >= 95.0, "Critical content {} should have high coverage ({}%)", item.cid, coverage);
        } else {
            // Regular items should have reasonable coverage
            assert!(coverage >= 80.0, "Regular content {} should have reasonable coverage ({}%)", item.cid, coverage);
        }
        
        // Verify content integrity
        let is_valid = manager.verify_content_integrity(&item.cid, &item.cid)
            .expect("Content integrity verification should succeed");
        assert!(is_valid, "Content integrity should be valid for {}", item.cid);
        
        println!("✓ Verified storage integrity for content: {} (coverage: {}%, size: {} bytes, critical: {})", 
                 item.cid, coverage, item.size, item.is_critical);
    }
    
    // Test retrieval of all data
    assert_eq!(manager.get_all_content_items().len(), content_items.len());
    assert_eq!(manager.get_all_pinning_services().len(), pinning_services.len());
    assert_eq!(manager.get_all_hash_anchors().len(), hash_anchors.len());
    
    println!("✓ All storage integrity data retrieval verified");
    
    println!("Storage integrity IPFS pinning and hash anchoring test passed!");
}

/// Test storage integrity with Arweave integration
#[test]
fn test_storage_integrity_arweave_integration() {
    println!("Starting storage integrity Arweave integration test");
    
    let safety_policy = ContentSafetyPolicy {
        allowed_types: vec![
            "text/plain".to_string(),
            "application/json".to_string(),
        ],
        max_size: 5 * 1024 * 1024, // 5MB limit
        moderation_required: false,
        encryption_required: false,
    };
    
    let mut manager = DataIntegrityManager::new(safety_policy);
    
    // Add Arweave as a pinning service
    let arweave_service = PinningService {
        name: "arweave".to_string(),
        endpoint: "https://arweave.net".to_string(),
        status: "active".to_string(),
        last_check: 1234567890,
        coverage: 99.9, // Arweave has high permanence
    };
    manager.add_pinning_service(arweave_service);
    
    // Add traditional IPFS services
    let ipfs_services = vec![
        ("pinata", "https://api.pinata.cloud", 99.8),
        ("infura", "https://ipfs.infura.io", 99.2),
    ];
    
    for (name, endpoint, coverage) in &ipfs_services {
        let service = PinningService {
            name: name.to_string(),
            endpoint: endpoint.to_string(),
            status: "active".to_string(),
            last_check: 1234567890,
            coverage: *coverage,
        };
        manager.add_pinning_service(service);
    }
    
    println!("✓ Added Arweave and IPFS pinning services");
    
    // Add permanent content items to Arweave
    let permanent_items = vec![
        ContentItem {
            cid: "QmPermanentData123".to_string(),
            size: 1024000, // 1MB
            content_type: "text/plain".to_string(),
            added_timestamp: 1234567890,
            pinning_services: vec!["arweave".to_string(), "pinata".to_string(), "infura".to_string()], // Pin on all services for high coverage
            replicas: 3,
            is_critical: true,
        },
        ContentItem {
            cid: "QmHybridData123".to_string(),
            size: 512000, // 500KB
            content_type: "application/json".to_string(),
            added_timestamp: 1234567890,
            pinning_services: vec![
                "arweave".to_string(),
                "pinata".to_string(),
                "infura".to_string(),  // Add infura to get 100% coverage
            ],
            replicas: 3,
            is_critical: true,
        },
    ];
    
    // Add all permanent items
    for item in &permanent_items {
        assert!(manager.add_content_item(item.clone()).is_ok(), "Permanent content item should be added");
    }
    
    println!("✓ Added {} permanent content items", permanent_items.len());
    
    // Anchor hashes on-chain for permanent data
    let anchors = vec![
        HashAnchor {
            cid: "QmPermanentData123".to_string(),
            chain: "ethereum".to_string(),
            tx_hash: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890".to_string(),
            block_number: 12345679,
            timestamp: 1234567891,
        },
        HashAnchor {
            cid: "QmHybridData123".to_string(),
            chain: "polygon".to_string(),
            tx_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
            block_number: 23456789,
            timestamp: 1234567892,
        },
    ];
    
    for anchor in &anchors {
        assert!(manager.anchor_hash(anchor.clone()).is_ok(), "Hash should be anchored");
    }
    
    println!("✓ Anchored hashes for permanent content");
    
    // Verify permanent storage integrity
    for item in &permanent_items {
        // Check coverage (Arweave items should have good coverage)
        let coverage = manager.check_pin_coverage(&item.cid).expect("Pin coverage check should succeed");
        assert!(coverage >= 95.0, "Permanent content {} should have high coverage ({}%)", item.cid, coverage);
        
        // Verify content integrity
        let is_valid = manager.verify_content_integrity(&item.cid, &item.cid)
            .expect("Content integrity verification should succeed");
        assert!(is_valid, "Content integrity should be valid for permanent content {}", item.cid);
        
        println!("✓ Verified permanent storage integrity for: {} (coverage: {}%, size: {} bytes)", 
                 item.cid, coverage, item.size);
    }
    
    println!("Storage integrity Arweave integration test passed!");
}

/// Test storage integrity error handling and edge cases
#[test]
fn test_storage_integrity_error_handling() {
    println!("Starting storage integrity error handling test");
    
    let safety_policy = ContentSafetyPolicy {
        allowed_types: vec!["text/plain".to_string()],
        max_size: 1024,
        moderation_required: false,
        encryption_required: false,
    };
    
    let mut manager = DataIntegrityManager::new(safety_policy);
    
    // Test content not found error
    assert!(manager.get_content_item("QmNonExistent").is_none());
    assert!(manager.check_pin_coverage("QmNonExistent").is_err());
    assert!(manager.verify_content_integrity("QmNonExistent", "QmNonExistent").is_err());
    
    println!("✓ Content not found errors handled correctly");
    
    // Test safety policy violations
    let oversized_item = ContentItem {
        cid: "QmOversized".to_string(),
        size: 2048, // Exceeds 1024 limit
        content_type: "text/plain".to_string(),
        added_timestamp: 1234567890,
        pinning_services: vec!["pinata".to_string()],
        replicas: 1,
        is_critical: false,
    };
    
    // This should fail due to size violation
    assert!(manager.add_content_item(oversized_item).is_err());
    println!("✓ Oversized content rejected by safety policy");
    
    let wrong_type_item = ContentItem {
        cid: "QmWrongType".to_string(),
        size: 512,
        content_type: "application/exe".to_string(), // Not allowed
        added_timestamp: 1234567890,
        pinning_services: vec!["pinata".to_string()],
        replicas: 1,
        is_critical: false,
    };
    
    // This should fail due to type violation
    assert!(manager.add_content_item(wrong_type_item).is_err());
    println!("✓ Wrong content type rejected by safety policy");
    
    println!("Storage integrity error handling test passed!");
}