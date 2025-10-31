//! Data Integrity Simulation Tests
//!
//! This module contains tests that simulate various data integrity scenarios
//! to verify the effectiveness of our storage integrity measures.

use core::data_integrity::{DataIntegrityManager, ContentItem, PinningService, HashAnchor, ContentSafetyPolicy};

/// Test data integrity in a realistic scenario with multiple content items
#[test]
fn test_realistic_data_integrity_scenario() {
    println!("Starting realistic data integrity scenario test");
    
    // Create a realistic safety policy
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
    
    let mut manager = DataIntegrityManager::new(safety_policy, 99.0);
    
    // Add realistic pinning services
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
    
    println!("✓ Added {} pinning services", pinning_services.len());
    
    // Add realistic content items
    let content_items = vec![
        // NFT metadata (critical, should be pinned everywhere)
        ContentItem {
            cid: "QmNFTMetadata123".to_string(),
            size: 2048,
            content_type: "application/json".to_string(),
            added_timestamp: 1234567890,
            pinning_services: pinning_services.iter().map(|(name, _, _)| name.to_string()).collect(),
            replicas: pinning_services.len() as u32,
            is_critical: true,
        },
        // User profile image (important, pinned on most services)
        ContentItem {
            cid: "QmUserProfile123".to_string(),
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
        // Large document (pinned on fewer services due to size)
        ContentItem {
            cid: "QmLargeDocument123".to_string(),
            size: 5 * 1024 * 1024, // 5MB
            content_type: "text/plain".to_string(),
            added_timestamp: 1234567890,
            pinning_services: vec![
                "pinata".to_string(),
                "nft-storage".to_string(),
                "own-node-1".to_string(),
                "own-node-2".to_string(),
            ],
            replicas: 4,
            is_critical: false,
        },
    ];
    
    // Add all content items
    for item in &content_items {
        assert!(manager.add_content_item(item.clone()).is_ok(), "Content item should be added");
    }
    
    println!("✓ Added {} content items", content_items.len());
    
    // Anchor critical content hashes on-chain
    let hash_anchors = vec![
        HashAnchor {
            cid: "QmNFTMetadata123".to_string(),
            chain: "ethereum".to_string(),
            tx_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
            block_number: 12345678,
            timestamp: 1234567890,
        },
        HashAnchor {
            cid: "QmUserProfile123".to_string(),
            chain: "polygon".to_string(),
            tx_hash: "0xfedcba0987654321fedcba0987654321fedcba0987654321fedcba0987654321".to_string(),
            block_number: 23456789,
            timestamp: 1234567891,
        },
    ];
    
    for anchor in &hash_anchors {
        assert!(manager.anchor_hash(anchor.clone()).is_ok(), "Hash should be anchored");
    }
    
    println!("✓ Anchored {} critical content hashes", hash_anchors.len());
    
    // Verify all content items
    for item in &content_items {
        // Check pin coverage
        let coverage = manager.check_pin_coverage(&item.cid).expect("Pin coverage check should succeed");
        let expected_coverage = (item.replicas as f64 / pinning_services.len() as f64) * 100.0;
        assert_eq!(coverage, expected_coverage, "Coverage should match expected value for {}", item.cid);
        
        // Verify content integrity
        let is_valid = manager.verify_content_integrity(&item.cid, &item.cid)
            .expect("Content integrity verification should succeed");
        assert!(is_valid, "Content integrity should be valid for {}", item.cid);
        
        // Check content safety
        assert!(manager.is_content_safe(item), "Content should be safe for {}", item.cid);
        
        println!("✓ Verified content item: {} (coverage: {}%, size: {} bytes)", 
                 item.cid, coverage, item.size);
    }
    
    // Test retrieval of all data
    assert_eq!(manager.get_all_content_items().len(), content_items.len());
    assert_eq!(manager.get_all_pinning_services().len(), pinning_services.len());
    assert_eq!(manager.get_all_hash_anchors().len(), hash_anchors.len());
    
    println!("✓ All data retrieval verified");
    
    println!("Realistic data integrity scenario test passed!");
}

/// Test data integrity under stress conditions
#[test]
fn test_data_integrity_under_stress() {
    println!("Starting data integrity stress test");
    
    let safety_policy = ContentSafetyPolicy {
        allowed_types: vec!["text/plain".to_string()],
        max_size: 1024 * 1024, // 1MB
        moderation_required: false,
        encryption_required: false,
    };
    
    let mut manager = DataIntegrityManager::new(safety_policy, 95.0); // Lower threshold for stress test
    
    // Add many pinning services
    for i in 0..20 {
        let service = PinningService {
            name: format!("service-{}", i),
            endpoint: format!("https://service{}.example.com", i),
            status: "active".to_string(),
            last_check: 1234567890,
            coverage: 99.0 + (i as f64 * 0.1), // Varying coverage
        };
        manager.add_pinning_service(service);
    }
    
    println!("✓ Added 20 pinning services");
    
    // Add many content items with varying pinning patterns
    for i in 0..100 {
        // Create a pattern where items are pinned on a random subset of services
        let num_services = 5 + (i % 10); // 5-14 services
        let mut pinning_services = Vec::new();
        for j in 0..num_services {
            pinning_services.push(format!("service-{}", j));
        }
        
        let item = ContentItem {
            cid: format!("QmStressTest{}", i),
            size: 1024 + (i * 10), // Varying sizes
            content_type: "text/plain".to_string(),
            added_timestamp: 1234567890,
            pinning_services,
            replicas: num_services,
            is_critical: i < 10, // First 10 are critical
        };
        
        assert!(manager.add_content_item(item).is_ok(), "Content item {} should be added", i);
    }
    
    println!("✓ Added 100 content items");
    
    // Anchor hashes for critical items
    for i in 0..10 {
        let anchor = HashAnchor {
            cid: format!("QmStressTest{}", i),
            chain: "ethereum".to_string(),
            tx_hash: format!("0x{:0>64}", i), // Fake transaction hash
            block_number: 12345678 + i as u64,
            timestamp: 1234567890 + i as u64,
        };
        
        assert!(manager.anchor_hash(anchor).is_ok(), "Hash for item {} should be anchored", i);
    }
    
    println!("✓ Anchored hashes for 10 critical items");
    
    // Verify all items under stress
    for i in 0..100 {
        let cid = format!("QmStressTest{}", i);
        
        // Check pin coverage
        let coverage = manager.check_pin_coverage(&cid).expect("Pin coverage check should succeed");
        let expected_coverage = if i < 10 {
            // Critical items should have higher coverage
            (5 + (i % 10)) as f64 / 20.0 * 100.0
        } else {
            (5 + (i % 10)) as f64 / 20.0 * 100.0
        };
        assert_eq!(coverage, expected_coverage, "Coverage should match for item {}", i);
        
        // Verify content integrity
        let is_valid = manager.verify_content_integrity(&cid, &cid)
            .expect("Content integrity verification should succeed");
        assert!(is_valid, "Content integrity should be valid for item {}", i);
        
        // Get item and check safety
        let item = manager.get_content_item(&cid).expect("Item should exist");
        assert!(manager.is_content_safe(item), "Content should be safe for item {}", i);
    }
    
    // Verify retrieval of all data
    assert_eq!(manager.get_all_content_items().len(), 100);
    assert_eq!(manager.get_all_pinning_services().len(), 20);
    assert_eq!(manager.get_all_hash_anchors().len(), 10);
    
    println!("✓ Verified all 100 content items under stress");
    
    println!("Data integrity stress test passed!");
}

/// Test data integrity error handling
#[test]
fn test_data_integrity_error_handling() {
    println!("Starting data integrity error handling test");
    
    let safety_policy = ContentSafetyPolicy {
        allowed_types: vec!["text/plain".to_string()],
        max_size: 1024,
        moderation_required: false,
        encryption_required: false,
    };
    
    let mut manager = DataIntegrityManager::new(safety_policy, 99.0);
    
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
    
    assert!(manager.add_content_item(wrong_type_item).is_err());
    println!("✓ Wrong content type rejected by safety policy");
    
    // Test with services but no content
    let service = PinningService {
        name: "pinata".to_string(),
        endpoint: "https://api.pinata.cloud".to_string(),
        status: "active".to_string(),
        last_check: 1234567890,
        coverage: 99.5,
    };
    manager.add_pinning_service(service);
    
    // Try to check coverage for non-existent content
    assert!(manager.check_pin_coverage("QmNonExistent").is_err());
    println!("✓ Coverage check for non-existent content handled correctly");
    
    println!("Data integrity error handling test passed!");
}