//! Data Protection Simulation Tests
//!
//! This module contains tests that simulate various data protection scenarios
//! to verify the effectiveness of our security measures.

use core::data_protection::{FieldEncryption, PiiMap, DsrErasureManager, PiiField, PiiClassification, DsrRequest, DsrRequestType, DsrRequestStatus, EncryptionError};
use std::collections::HashSet;

/// Test field encryption and decryption
#[test]
fn test_field_encryption_decryption() {
    let key = [1u8; 32]; // Test key
    let encryption = FieldEncryption::new(key, "test-key-1".to_string());
    
    let plaintext = b"Hello, World! This is a test message for encryption.";
    let encrypted = encryption.encrypt(plaintext).unwrap();
    let decrypted = encryption.decrypt(&encrypted).unwrap();
    
    assert_eq!(plaintext, decrypted.as_slice());
    assert_ne!(plaintext, encrypted.as_slice()); // Ensure data is actually encrypted
    
    println!("Field encryption and decryption test passed");
}

/// Test encryption with different keys
#[test]
fn test_encryption_with_different_keys() {
    let key1 = [1u8; 32];
    let key2 = [2u8; 32];
    
    let encryption1 = FieldEncryption::new(key1, "key-1".to_string());
    let encryption2 = FieldEncryption::new(key2, "key-2".to_string());
    
    let plaintext = b"SAME PLAINTEXT";
    
    let encrypted1 = encryption1.encrypt(plaintext).unwrap();
    let encrypted2 = encryption2.encrypt(plaintext).unwrap();
    
    // Encrypted data should be different with different keys
    assert_ne!(encrypted1, encrypted2);
    
    // Both should decrypt correctly to the same plaintext
    let decrypted1 = encryption1.decrypt(&encrypted1).unwrap();
    let decrypted2 = encryption2.decrypt(&encrypted2).unwrap();
    
    assert_eq!(decrypted1, plaintext);
    assert_eq!(decrypted2, plaintext);
    
    println!("Encryption with different keys test passed");
}

/// Test PII map functionality
#[test]
fn test_pii_map_functionality() {
    let mut pii_map = PiiMap::new();
    
    // Add multiple PII fields with different classifications
    let email_field = PiiField {
        name: "user_email".to_string(),
        description: "User email address".to_string(),
        classification: PiiClassification::Confidential,
        storage_location: "users table".to_string(),
        retention_period: 365,
        legal_basis: "Contract performance".to_string(),
    };
    
    let ssn_field = PiiField {
        name: "user_ssn".to_string(),
        description: "User social security number".to_string(),
        classification: PiiClassification::HighlyConfidential,
        storage_location: "users table".to_string(),
        retention_period: 730,
        legal_basis: "Legal obligation".to_string(),
    };
    
    let username_field = PiiField {
        name: "username".to_string(),
        description: "User chosen username".to_string(),
        classification: PiiClassification::Internal,
        storage_location: "users table".to_string(),
        retention_period: 1825,
        legal_basis: "Legitimate interest".to_string(),
    };
    
    pii_map.add_field(email_field);
    pii_map.add_field(ssn_field);
    pii_map.add_field(username_field);
    
    // Test field retrieval
    assert!(pii_map.get_field("user_email").is_some());
    assert!(pii_map.get_field("user_ssn").is_some());
    assert!(pii_map.get_field("username").is_some());
    assert!(pii_map.get_field("nonexistent").is_none());
    
    // Test field existence checking
    assert!(pii_map.has_field("user_email"));
    assert!(!pii_map.has_field("nonexistent_field"));
    
    // Test classification filtering
    let confidential_fields = pii_map.get_fields_by_classification(PiiClassification::Confidential);
    assert_eq!(confidential_fields.len(), 1);
    assert_eq!(confidential_fields[0].name, "user_email");
    
    let highly_confidential_fields = pii_map.get_fields_by_classification(PiiClassification::HighlyConfidential);
    assert_eq!(highly_confidential_fields.len(), 1);
    assert_eq!(highly_confidential_fields[0].name, "user_ssn");
    
    let internal_fields = pii_map.get_fields_by_classification(PiiClassification::Internal);
    assert_eq!(internal_fields.len(), 1);
    assert_eq!(internal_fields[0].name, "username");
    
    // Test getting all fields
    let all_fields = pii_map.get_all_fields();
    assert_eq!(all_fields.len(), 3);
    
    println!("PII map functionality test passed");
}

/// Test data flows in PII map
#[test]
fn test_pii_data_flows() {
    let mut pii_map = PiiMap::new();
    
    // Add data flows
    let api_destinations = vec!["api_service".to_string(), "analytics_service".to_string()];
    pii_map.add_data_flow("user_email", api_destinations.clone());
    
    let db_destinations = vec!["backup_service".to_string(), "audit_service".to_string()];
    pii_map.add_data_flow("user_ssn", db_destinations.clone());
    
    // Test data flow retrieval
    let retrieved_api_destinations = pii_map.get_data_flows("user_email").unwrap();
    assert_eq!(retrieved_api_destinations, &api_destinations);
    
    let retrieved_db_destinations = pii_map.get_data_flows("user_ssn").unwrap();
    assert_eq!(retrieved_db_destinations, &db_destinations);
    
    // Test nonexistent data flow
    assert!(pii_map.get_data_flows("nonexistent_field").is_none());
    
    println!("PII data flows test passed");
}

/// Test DSR erasure manager
#[test]
fn test_dsr_erasure_manager() {
    // Create PII map with fields
    let mut pii_map = PiiMap::new();
    
    let email_field = PiiField {
        name: "user_email".to_string(),
        description: "User email address".to_string(),
        classification: PiiClassification::Confidential,
        storage_location: "users table".to_string(),
        retention_period: 365,
        legal_basis: "Contract performance".to_string(),
    };
    
    let ssn_field = PiiField {
        name: "user_ssn".to_string(),
        description: "User social security number".to_string(),
        classification: PiiClassification::HighlyConfidential,
        storage_location: "users table".to_string(),
        retention_period: 730,
        legal_basis: "Legal obligation".to_string(),
    };
    
    pii_map.add_field(email_field);
    pii_map.add_field(ssn_field);
    
    // Create DSR manager
    let mut dsr_manager = DsrErasureManager::new(pii_map);
    
    // Test initial state
    assert_eq!(dsr_manager.get_audit_log().len(), 0);
    assert_eq!(dsr_manager.get_all_requests().len(), 0);
    
    // Create DSR requests
    let mut affected_fields1 = HashSet::new();
    affected_fields1.insert("user_email".to_string());
    
    let request1 = DsrRequest {
        id: "req-001".to_string(),
        request_type: DsrRequestType::Erasure,
        requester_id: "user-123".to_string(),
        timestamp: 1234567890,
        status: DsrRequestStatus::Pending,
        affected_fields: affected_fields1,
        resolution_notes: None,
    };
    
    let mut affected_fields2 = HashSet::new();
    affected_fields2.insert("user_ssn".to_string());
    
    let request2 = DsrRequest {
        id: "req-002".to_string(),
        request_type: DsrRequestType::Access,
        requester_id: "user-456".to_string(),
        timestamp: 1234567891,
        status: DsrRequestStatus::Pending,
        affected_fields: affected_fields2,
        resolution_notes: None,
    };
    
    // Submit requests
    assert!(dsr_manager.submit_request(request1).is_ok());
    assert!(dsr_manager.submit_request(request2).is_ok());
    
    // Test request submission failure (duplicate ID)
    let duplicate_request = DsrRequest {
        id: "req-001".to_string(), // Same ID as request1
        request_type: DsrRequestType::Rectification,
        requester_id: "user-789".to_string(),
        timestamp: 1234567892,
        status: DsrRequestStatus::Pending,
        affected_fields: HashSet::new(),
        resolution_notes: None,
    };
    
    assert!(dsr_manager.submit_request(duplicate_request).is_err());
    
    // Test request retrieval
    assert!(dsr_manager.get_request_status("req-001").is_some());
    assert!(dsr_manager.get_request_status("req-002").is_some());
    assert!(dsr_manager.get_request_status("nonexistent").is_none());
    
    // Test audit log
    assert_eq!(dsr_manager.get_audit_log().len(), 2); // Two submissions
    assert!(dsr_manager.get_audit_log()[0].contains("req-001"));
    assert!(dsr_manager.get_audit_log()[1].contains("req-002"));
    
    // Test request filtering
    let all_requests = dsr_manager.get_all_requests();
    assert_eq!(all_requests.len(), 2);
    
    let pending_requests = dsr_manager.get_requests_by_status(DsrRequestStatus::Pending);
    assert_eq!(pending_requests.len(), 2);
    
    println!("DSR erasure manager test passed");
}

/// Test DSR erasure processing
#[test]
fn test_dsr_erasure_processing() {
    // Create PII map with fields
    let mut pii_map = PiiMap::new();
    
    let email_field = PiiField {
        name: "user_email".to_string(),
        description: "User email address".to_string(),
        classification: PiiClassification::Confidential,
        storage_location: "users table".to_string(),
        retention_period: 365,
        legal_basis: "Contract performance".to_string(),
    };
    
    pii_map.add_field(email_field);
    
    // Create DSR manager
    let mut dsr_manager = DsrErasureManager::new(pii_map);
    
    // Create erasure request
    let mut affected_fields = HashSet::new();
    affected_fields.insert("user_email".to_string());
    
    let request = DsrRequest {
        id: "erasure-001".to_string(),
        request_type: DsrRequestType::Erasure,
        requester_id: "user-999".to_string(),
        timestamp: 1234567893,
        status: DsrRequestStatus::Pending,
        affected_fields,
        resolution_notes: None,
    };
    
    // Submit request
    assert!(dsr_manager.submit_request(request).is_ok());
    
    // Process erasure
    assert!(dsr_manager.process_erasure("erasure-001").is_ok());
    
    // Check status
    let status = dsr_manager.get_request_status("erasure-001").unwrap();
    assert_eq!(status, &DsrRequestStatus::Completed);
    
    // Check audit log
    assert!(dsr_manager.get_audit_log().len() >= 2); // At least submission and completion
    assert!(dsr_manager.get_audit_log().iter().any(|log| log.contains("Erasure completed")));
    
    println!("DSR erasure processing test passed");
}

/// Test DSR erasure with nonexistent field
#[test]
fn test_dsr_erasure_with_nonexistent_field() {
    // Create empty PII map
    let pii_map = PiiMap::new();
    let mut dsr_manager = DsrErasureManager::new(pii_map);
    
    // Create erasure request with nonexistent field
    let mut affected_fields = HashSet::new();
    affected_fields.insert("nonexistent_field".to_string());
    
    let request = DsrRequest {
        id: "erasure-002".to_string(),
        request_type: DsrRequestType::Erasure,
        requester_id: "user-888".to_string(),
        timestamp: 1234567894,
        status: DsrRequestStatus::Pending,
        affected_fields,
        resolution_notes: None,
    };
    
    // Submit request
    assert!(dsr_manager.submit_request(request).is_ok());
    
    // Process erasure should fail
    let result = dsr_manager.process_erasure("erasure-002");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found in PII map"));
    
    // Check status should be rejected
    let status = dsr_manager.get_request_status("erasure-002").unwrap();
    assert_eq!(status, &DsrRequestStatus::Rejected);
    
    // Check audit log
    assert!(dsr_manager.get_audit_log().iter().any(|log| log.contains("Erasure failed")));
    
    println!("DSR erasure with nonexistent field test passed");
}

/// Test encryption error handling
#[test]
fn test_encryption_error_handling() {
    let key = [1u8; 32]; // Test key
    let encryption = FieldEncryption::new(key, "test-key-1".to_string());
    
    // Test decryption with invalid data
    let invalid_data = vec![1, 2, 3]; // Too short
    let result = encryption.decrypt(&invalid_data);
    assert!(result.is_err());
    match result.unwrap_err() {
        EncryptionError::InvalidData => {},
        _ => panic!("Expected InvalidData error"),
    }
    
    // Test with corrupted data
    let plaintext = b"Test data";
    let encrypted = encryption.encrypt(plaintext).unwrap();
    
    // Corrupt the encrypted data
    let mut corrupted_data = encrypted.clone();
    if !corrupted_data.is_empty() {
        corrupted_data[corrupted_data.len() - 1] ^= 0xFF; // Flip some bits
    }
    
    let result = encryption.decrypt(&corrupted_data);
    assert!(result.is_err());
    match result.unwrap_err() {
        EncryptionError::DecryptionFailed => {},
        _ => panic!("Expected DecryptionFailed error"),
    }
    
    println!("Encryption error handling test passed");
}

/// Integration test for complete data protection workflow
#[test]
fn test_data_protection_workflow() {
    // 1. Create field encryption
    let key = [1u8; 32];
    let encryption = FieldEncryption::new(key, "workflow-key".to_string());
    
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
    
    // 3. Encrypt sensitive data
    let sensitive_data = b"This is highly sensitive user information";
    let encrypted_data = encryption.encrypt(sensitive_data).unwrap();
    assert_ne!(sensitive_data, encrypted_data.as_slice());
    
    // 4. Create DSR manager
    let mut dsr_manager = DsrErasureManager::new(pii_map);
    
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
    
    assert!(dsr_manager.submit_request(dsr_request).is_ok());
    
    // 6. Process erasure
    assert!(dsr_manager.process_erasure("workflow-req-001").is_ok());
    
    // 7. Verify completion
    let status = dsr_manager.get_request_status("workflow-req-001").unwrap();
    assert_eq!(status, &DsrRequestStatus::Completed);
    
    // 8. Verify audit logging
    assert!(dsr_manager.get_audit_log().len() >= 2);
    assert!(dsr_manager.get_audit_log().iter().any(|log| log.contains("submitted")));
    assert!(dsr_manager.get_audit_log().iter().any(|log| log.contains("completed")));
    
    // 9. Decrypt data to verify it's still intact (in real scenario, this would be erased)
    let decrypted_data = encryption.decrypt(&encrypted_data).unwrap();
    assert_eq!(sensitive_data, decrypted_data.as_slice());
    
    println!("Complete data protection workflow test passed");
}

/// Test PII classification hierarchy
#[test]
fn test_pii_classification_hierarchy() {
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
    
    let highly_confidential_fields = pii_map.get_fields_by_classification(PiiClassification::HighlyConfidential);
    assert_eq!(highly_confidential_fields.len(), 1);
    assert_eq!(highly_confidential_fields[0].name, "highly_confidential_info");
    
    println!("PII classification hierarchy test passed");
}