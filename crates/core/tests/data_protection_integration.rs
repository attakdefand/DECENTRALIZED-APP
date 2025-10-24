//! Integration tests for data protection functionality

use core::data_protection::{
    DsrErasureManager, DsrRequest, DsrRequestStatus, DsrRequestType, FieldEncryption,
    PiiClassification, PiiField, PiiMap,
};
use std::collections::HashSet;

/// Integration test for the complete data protection workflow
#[test]
fn test_complete_data_protection_workflow() {
    println!("Starting complete data protection workflow test");

    // 1. Create field encryption
    let key = [1u8; 32];
    let encryption = FieldEncryption::new(key, "workflow-key".to_string());
    println!("✓ Field encryption created");

    // 2. Create PII map
    let mut pii_map = PiiMap::new();

    let pii_field = PiiField {
        name: "sensitive_data".to_string(),
        description: "Highly sensitive user data".to_string(),
        classification: PiiClassification::HighlyConfidential,
        storage_location: "secure_table".to_string(),
        retention_period: 365,
        legal_basis: "Consent".to_string(),
    };

    pii_map.add_field(pii_field);
    println!("✓ PII map created with sensitive field");

    // 3. Encrypt sensitive data
    let sensitive_data = b"This is highly sensitive user information that needs protection";
    let encrypted_data = encryption
        .encrypt(sensitive_data)
        .expect("Encryption should succeed");
    assert_ne!(
        sensitive_data,
        encrypted_data.as_slice(),
        "Data should be encrypted"
    );
    println!("✓ Sensitive data encrypted");

    // 4. Create DSR manager
    let mut dsr_manager = DsrErasureManager::new(pii_map);
    println!("✓ DSR erasure manager created");

    // 5. Submit DSR request
    let mut affected_fields = HashSet::new();
    affected_fields.insert("sensitive_data".to_string());

    let dsr_request = DsrRequest {
        id: "workflow-req-001".to_string(),
        request_type: DsrRequestType::Erasure,
        requester_id: "user-workflow".to_string(),
        timestamp: 1234567895,
        status: DsrRequestStatus::Pending,
        affected_fields,
        resolution_notes: None,
    };

    assert!(
        dsr_manager.submit_request(dsr_request).is_ok(),
        "DSR request submission should succeed"
    );
    println!("✓ DSR erasure request submitted");

    // 6. Process erasure
    assert!(
        dsr_manager.process_erasure("workflow-req-001").is_ok(),
        "DSR erasure processing should succeed"
    );
    println!("✓ DSR erasure processed");

    // 7. Verify completion
    let status = dsr_manager
        .get_request_status("workflow-req-001")
        .expect("Request status should exist");
    assert_eq!(
        status,
        &DsrRequestStatus::Completed,
        "Request should be completed"
    );
    println!("✓ DSR request status verified as completed");

    // 8. Verify audit logging
    let audit_log = dsr_manager.get_audit_log();
    assert!(audit_log.len() >= 2, "Audit log should contain entries");
    assert!(
        audit_log.iter().any(|log| log.contains("submitted")),
        "Audit log should contain submission entry"
    );
    assert!(
        audit_log.iter().any(|log| log.contains("completed")),
        "Audit log should contain completion entry"
    );
    println!("✓ Audit logging verified");

    // 9. Decrypt data to verify it's still intact (in real scenario, this would be erased)
    let decrypted_data = encryption
        .decrypt(&encrypted_data)
        .expect("Decryption should succeed");
    assert_eq!(
        sensitive_data,
        decrypted_data.as_slice(),
        "Decrypted data should match original"
    );
    println!("✓ Data decryption verified");

    println!("Complete data protection workflow test passed!");
}

/// Test encryption with different keys produces different results
#[test]
fn test_encryption_with_different_keys() {
    println!("Starting encryption with different keys test");

    let key1 = [1u8; 32];
    let key2 = [2u8; 32];

    let encryption1 = FieldEncryption::new(key1, "key-1".to_string());
    let encryption2 = FieldEncryption::new(key2, "key-2".to_string());

    let plaintext = b"SAME PLAINTEXT";

    let encrypted1 = encryption1
        .encrypt(plaintext)
        .expect("Encryption 1 should succeed");
    let encrypted2 = encryption2
        .encrypt(plaintext)
        .expect("Encryption 2 should succeed");

    // Encrypted data should be different with different keys
    assert_ne!(
        encrypted1, encrypted2,
        "Encrypted data should differ with different keys"
    );

    // Both should decrypt correctly to the same plaintext
    let decrypted1 = encryption1
        .decrypt(&encrypted1)
        .expect("Decryption 1 should succeed");
    let decrypted2 = encryption2
        .decrypt(&encrypted2)
        .expect("Decryption 2 should succeed");

    assert_eq!(
        decrypted1, plaintext,
        "Decrypted data 1 should match original"
    );
    assert_eq!(
        decrypted2, plaintext,
        "Decrypted data 2 should match original"
    );

    println!("✓ Encryption with different keys test passed");
}

/// Test PII classification hierarchy
#[test]
fn test_pii_classification_hierarchy() {
    println!("Starting PII classification hierarchy test");

    let mut pii_map = PiiMap::new();

    // Add fields with all classification levels
    let public_field = PiiField {
        name: "public_info".to_string(),
        description: "Public information".to_string(),
        classification: PiiClassification::Public,
        storage_location: "public table".to_string(),
        retention_period: 3650,
        legal_basis: "Legitimate interest".to_string(),
    };

    let internal_field = PiiField {
        name: "internal_info".to_string(),
        description: "Internal information".to_string(),
        classification: PiiClassification::Internal,
        storage_location: "internal table".to_string(),
        retention_period: 1825,
        legal_basis: "Legitimate interest".to_string(),
    };

    let confidential_field = PiiField {
        name: "confidential_info".to_string(),
        description: "Confidential information".to_string(),
        classification: PiiClassification::Confidential,
        storage_location: "confidential table".to_string(),
        retention_period: 365,
        legal_basis: "Contract performance".to_string(),
    };

    let highly_confidential_field = PiiField {
        name: "highly_confidential_info".to_string(),
        description: "Highly confidential information".to_string(),
        classification: PiiClassification::HighlyConfidential,
        storage_location: "secure table".to_string(),
        retention_period: 730,
        legal_basis: "Legal obligation".to_string(),
    };

    pii_map.add_field(public_field);
    pii_map.add_field(internal_field);
    pii_map.add_field(confidential_field);
    pii_map.add_field(highly_confidential_field);

    // Test classification filtering
    let public_fields = pii_map.get_fields_by_classification(PiiClassification::Public);
    assert_eq!(public_fields.len(), 1);
    assert_eq!(public_fields[0].name, "public_info");

    let internal_fields = pii_map.get_fields_by_classification(PiiClassification::Internal);
    assert_eq!(internal_fields.len(), 1);
    assert_eq!(internal_fields[0].name, "internal_info");

    let confidential_fields = pii_map.get_fields_by_classification(PiiClassification::Confidential);
    assert_eq!(confidential_fields.len(), 1);
    assert_eq!(confidential_fields[0].name, "confidential_info");

    let highly_confidential_fields =
        pii_map.get_fields_by_classification(PiiClassification::HighlyConfidential);
    assert_eq!(highly_confidential_fields.len(), 1);
    assert_eq!(
        highly_confidential_fields[0].name,
        "highly_confidential_info"
    );

    println!("✓ PII classification hierarchy test passed");
}

/// Test DSR request filtering by status
#[test]
fn test_dsr_request_filtering() {
    println!("Starting DSR request filtering test");

    let pii_map = PiiMap::new();
    let mut dsr_manager = DsrErasureManager::new(pii_map);

    // Add requests with different statuses
    let request1 = DsrRequest {
        id: "req-005".to_string(),
        request_type: DsrRequestType::Access,
        requester_id: "user-111".to_string(),
        timestamp: 1234567894,
        status: DsrRequestStatus::Pending,
        affected_fields: HashSet::new(),
        resolution_notes: None,
    };

    let request2 = DsrRequest {
        id: "req-006".to_string(),
        request_type: DsrRequestType::Erasure,
        requester_id: "user-222".to_string(),
        timestamp: 1234567895,
        status: DsrRequestStatus::Completed,
        affected_fields: HashSet::new(),
        resolution_notes: None,
    };

    dsr_manager
        .submit_request(request1)
        .expect("Request 1 submission should succeed");
    dsr_manager
        .submit_request(request2)
        .expect("Request 2 submission should succeed");

    // Test filtering by status
    let pending_requests = dsr_manager.get_requests_by_status(DsrRequestStatus::Pending);
    assert_eq!(pending_requests.len(), 1);
    assert_eq!(pending_requests[0].id, "req-005");

    let completed_requests = dsr_manager.get_requests_by_status(DsrRequestStatus::Completed);
    assert_eq!(completed_requests.len(), 1);
    assert_eq!(completed_requests[0].id, "req-006");

    println!("✓ DSR request filtering test passed");
}
