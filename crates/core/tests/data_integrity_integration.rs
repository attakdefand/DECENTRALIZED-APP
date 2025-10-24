//! Integration tests for data integrity functionality

use core::data_integrity::{
    ContentItem, ContentSafetyPolicy, DataIntegrityManager, HashAnchor, PinningService,
};

/// Integration test for the complete data integrity workflow
#[test]
fn test_complete_data_integrity_workflow() {
    println!("Starting complete data integrity workflow test");

    // 1. Create content safety policy
    let safety_policy = ContentSafetyPolicy {
        allowed_types: vec!["text/plain".to_string(), "application/json".to_string()],
        max_size: 1024 * 1024, // 1MB
        moderation_required: false,
        encryption_required: false,
    };

    let mut manager = DataIntegrityManager::new(safety_policy);
    println!("✓ Data integrity manager created with safety policy");

    // 2. Add pinning services
    let pinata_service = PinningService {
        name: "pinata".to_string(),
        endpoint: "https://api.pinata.cloud".to_string(),
        status: "active".to_string(),
        last_check: 1234567890,
        coverage: 99.5,
    };

    let infura_service = PinningService {
        name: "infura".to_string(),
        endpoint: "https://ipfs.infura.io".to_string(),
        status: "active".to_string(),
        last_check: 1234567890,
        coverage: 98.7,
    };

    manager.add_pinning_service(pinata_service);
    manager.add_pinning_service(infura_service);
    println!("✓ Pinning services added");

    // 3. Add content item
    let content_item = ContentItem {
        cid: "QmTestContent1234567890".to_string(),
        size: 512,
        content_type: "text/plain".to_string(),
        added_timestamp: 1234567890,
        pinning_services: vec!["pinata".to_string(), "infura".to_string()],
        replicas: 2,
        is_critical: true,
    };

    assert!(
        manager.add_content_item(content_item).is_ok(),
        "Content item should be added successfully"
    );
    println!("✓ Content item added");

    // 4. Anchor hash on-chain
    let hash_anchor = HashAnchor {
        cid: "QmTestContent1234567890".to_string(),
        chain: "ethereum".to_string(),
        tx_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
        block_number: 1234567,
        timestamp: 1234567890,
    };

    assert!(
        manager.anchor_hash(hash_anchor).is_ok(),
        "Hash should be anchored successfully"
    );
    println!("✓ Hash anchored on-chain");

    // 5. Check pin coverage
    let coverage = manager
        .check_pin_coverage("QmTestContent1234567890")
        .expect("Pin coverage check should succeed");
    assert_eq!(
        coverage, 100.0,
        "Coverage should be 100% since content is pinned on both services"
    );
    println!("✓ Pin coverage verified: {}%", coverage);

    // 6. Verify content integrity
    let is_valid = manager
        .verify_content_integrity("QmTestContent1234567890", "QmTestContent1234567890")
        .expect("Content integrity verification should succeed");
    assert!(is_valid, "Content integrity should be valid");
    println!("✓ Content integrity verified");

    // 7. Check content safety
    let item = manager
        .get_content_item("QmTestContent1234567890")
        .expect("Content item should exist");
    assert!(
        manager.is_content_safe(item),
        "Content should be safe according to policy"
    );
    println!("✓ Content safety verified");

    // 8. Retrieve stored data
    assert!(
        manager.get_pinning_service("pinata").is_some(),
        "Pinning service should be retrievable"
    );
    assert!(
        manager.get_hash_anchor("QmTestContent1234567890").is_some(),
        "Hash anchor should be retrievable"
    );
    println!("✓ Stored data retrieval verified");

    println!("Complete data integrity workflow test passed!");
}

/// Test content safety policy enforcement
#[test]
fn test_content_safety_policy_enforcement() {
    println!("Starting content safety policy enforcement test");

    let safety_policy = ContentSafetyPolicy {
        allowed_types: vec!["text/plain".to_string()],
        max_size: 1024,
        moderation_required: false,
        encryption_required: false,
    };

    let mut manager = DataIntegrityManager::new(safety_policy);

    // Test allowed content type
    let allowed_item = ContentItem {
        cid: "QmAllowed123".to_string(),
        size: 512,
        content_type: "text/plain".to_string(),
        added_timestamp: 1234567890,
        pinning_services: vec!["pinata".to_string()],
        replicas: 1,
        is_critical: false,
    };

    assert!(
        manager.add_content_item(allowed_item).is_ok(),
        "Allowed content should be added"
    );
    println!("✓ Allowed content type accepted");

    // Test disallowed content type
    let disallowed_item = ContentItem {
        cid: "QmDisallowed123".to_string(),
        size: 512,
        content_type: "application/exe".to_string(),
        added_timestamp: 1234567890,
        pinning_services: vec!["pinata".to_string()],
        replicas: 1,
        is_critical: false,
    };

    assert!(
        manager.add_content_item(disallowed_item).is_err(),
        "Disallowed content should be rejected"
    );
    println!("✓ Disallowed content type rejected");

    // Test size limit enforcement
    let oversized_item = ContentItem {
        cid: "QmOversized123".to_string(),
        size: 2048, // Exceeds 1024 limit
        content_type: "text/plain".to_string(),
        added_timestamp: 1234567890,
        pinning_services: vec!["pinata".to_string()],
        replicas: 1,
        is_critical: false,
    };

    assert!(
        manager.add_content_item(oversized_item).is_err(),
        "Oversized content should be rejected"
    );
    println!("✓ Oversized content rejected");

    println!("✓ Content safety policy enforcement test passed");
}

/// Test pin coverage calculation with multiple services
#[test]
fn test_pin_coverage_calculation() {
    println!("Starting pin coverage calculation test");

    let safety_policy = ContentSafetyPolicy {
        allowed_types: vec!["text/plain".to_string()],
        max_size: 1024 * 1024,
        moderation_required: false,
        encryption_required: false,
    };

    let mut manager = DataIntegrityManager::new(safety_policy);

    // Add multiple pinning services
    let services = vec![
        ("pinata", 99.5),
        ("infura", 98.7),
        ("textile", 97.3),
        ("own-node", 100.0),
    ];

    for (name, coverage) in &services {
        let service = PinningService {
            name: name.to_string(),
            endpoint: format!("https://{}.example.com", name),
            status: "active".to_string(),
            last_check: 1234567890,
            coverage: *coverage,
        };
        manager.add_pinning_service(service);
    }

    println!("✓ {} pinning services added", services.len());

    // Test content pinned on all services
    let fully_pinned = ContentItem {
        cid: "QmFullyPinned123".to_string(),
        size: 512,
        content_type: "text/plain".to_string(),
        added_timestamp: 1234567890,
        pinning_services: services.iter().map(|(name, _)| name.to_string()).collect(),
        replicas: services.len() as u32,
        is_critical: true,
    };

    manager.add_content_item(fully_pinned).unwrap();
    let coverage = manager.check_pin_coverage("QmFullyPinned123").unwrap();
    assert_eq!(
        coverage, 100.0,
        "Fully pinned content should have 100% coverage"
    );
    println!("✓ Fully pinned content coverage: {}%", coverage);

    // Test content pinned on partial services
    let partially_pinned = ContentItem {
        cid: "QmPartiallyPinned123".to_string(),
        size: 512,
        content_type: "text/plain".to_string(),
        added_timestamp: 1234567890,
        pinning_services: vec!["pinata".to_string(), "infura".to_string()], // Only 2 out of 4
        replicas: 2,
        is_critical: false,
    };

    manager.add_content_item(partially_pinned).unwrap();
    let coverage = manager.check_pin_coverage("QmPartiallyPinned123").unwrap();
    assert_eq!(
        coverage, 50.0,
        "Partially pinned content should have 50% coverage"
    );
    println!("✓ Partially pinned content coverage: {}%", coverage);

    println!("✓ Pin coverage calculation test passed");
}

/// Test hash anchoring and retrieval
#[test]
fn test_hash_anchoring_and_retrieval() {
    println!("Starting hash anchoring and retrieval test");

    let safety_policy = ContentSafetyPolicy {
        allowed_types: vec!["text/plain".to_string()],
        max_size: 1024,
        moderation_required: false,
        encryption_required: false,
    };

    let mut manager = DataIntegrityManager::new(safety_policy);

    // Test anchoring multiple hashes
    let anchors = vec![
        ("QmHash1", "ethereum", "0x1234567890abcdef", 1234567),
        ("QmHash2", "polygon", "0xfedcba0987654321", 7654321),
        ("QmHash3", "arbitrum", "0x13579ace2468", 1357924),
    ];

    for (cid, chain, tx_hash, block_number) in &anchors {
        let anchor = HashAnchor {
            cid: cid.to_string(),
            chain: chain.to_string(),
            tx_hash: tx_hash.to_string(),
            block_number: *block_number,
            timestamp: 1234567890,
        };

        assert!(
            manager.anchor_hash(anchor).is_ok(),
            "Hash anchoring should succeed"
        );
    }

    println!("✓ {} hashes anchored", anchors.len());

    // Test retrieval of anchored hashes
    for (cid, chain, tx_hash, block_number) in &anchors {
        let anchor = manager
            .get_hash_anchor(cid)
            .expect("Anchored hash should be retrievable");
        assert_eq!(anchor.cid, *cid);
        assert_eq!(anchor.chain, *chain);
        assert_eq!(anchor.tx_hash, *tx_hash);
        assert_eq!(anchor.block_number, *block_number);
    }

    println!("✓ All anchored hashes retrieved and verified");

    // Test retrieval of non-existent anchor
    assert!(
        manager.get_hash_anchor("QmNonExistent").is_none(),
        "Non-existent anchor should return None"
    );
    println!("✓ Non-existent anchor correctly returns None");

    println!("✓ Hash anchoring and retrieval test passed");
}
