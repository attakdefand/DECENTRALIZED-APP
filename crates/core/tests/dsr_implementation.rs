//! Data Subject Request (DSR) Implementation Tests
//!
//! This module contains tests that validate the Data Subject Request implementation
//! including access, rectification, erasure, and portability requests.

use core::data_protection::{
    DsrErasureManager, DsrRequest, DsrRequestType, DsrRequestStatus, 
    PiiMap, PiiField, PiiClassification
};
use std::collections::HashSet;

/// Test DSR access request functionality
#[test]
fn test_dsr_access_request_functionality() {
    println!("Starting DSR access request functionality test");
    
    // Create PII map with user data
    let mut pii_map = PiiMap::new();
    
    let user_email_field = PiiField {
        name: "user_email".to_string(),
        description: "User email address".to_string(),
        classification: PiiClassification::Confidential,
        storage_location: "users table".to_string(),
        retention_period: 365,
        legal_basis: "Contract performance".to_string(),
    };
    
    let user_ssn_field = PiiField {
        name: "user_ssn".to_string(),
        description: "User social security number".to_string(),
        classification: PiiClassification::HighlyConfidential,
        storage_location: "secure users table".to_string(),
        retention_period: 730,
        legal_basis: "Legal obligation".to_string(),
    };
    
    pii_map.add_field(user_email_field);
    pii_map.add_field(user_ssn_field);
    
    // Create DSR manager
    let mut dsr_manager = DsrErasureManager::new(pii_map);
    
    // Create access request
    let mut affected_fields = HashSet::new();
    affected_fields.insert("user_email".to_string());
    affected_fields.insert("user_ssn".to_string());
    
    let access_request = DsrRequest {
        id: "dsr-access-001".to_string(),
        request_type: DsrRequestType::Access,
        requester_id: "user-123".to_string(),
        timestamp: 1234567890,
        status: DsrRequestStatus::Pending,
        affected_fields,
        resolution_notes: None,
    };
    
    // Submit access request
    assert!(dsr_manager.submit_request(access_request).is_ok());
    println!("✓ DSR access request submitted successfully");
    
    // Verify request was stored
    let stored_request = dsr_manager.get_request("dsr-access-001").expect("Request should exist");
    assert_eq!(stored_request.request_type, DsrRequestType::Access);
    assert_eq!(stored_request.requester_id, "user-123");
    assert_eq!(stored_request.status, DsrRequestStatus::Pending);
    assert_eq!(stored_request.affected_fields.len(), 2);
    
    println!("✓ DSR access request stored correctly");
    
    // Test request retrieval by ID
    let request = dsr_manager.get_request("dsr-access-001").unwrap();
    assert_eq!(request.id, "dsr-access-001");
    assert_eq!(request.request_type, DsrRequestType::Access);
    
    println!("✓ DSR access request retrieval working correctly");
    
    println!("DSR access request functionality test passed!");
}

/// Test DSR erasure request functionality
#[test]
fn test_dsr_erasure_request_functionality() {
    println!("Starting DSR erasure request functionality test");
    
    // Create PII map with user data
    let mut pii_map = PiiMap::new();
    
    let user_email_field = PiiField {
        name: "user_email".to_string(),
        description: "User email address".to_string(),
        classification: PiiClassification::Confidential,
        storage_location: "users table".to_string(),
        retention_period: 365,
        legal_basis: "Contract performance".to_string(),
    };
    
    let user_phone_field = PiiField {
        name: "user_phone".to_string(),
        description: "User phone number".to_string(),
        classification: PiiClassification::Confidential,
        storage_location: "users table".to_string(),
        retention_period: 365,
        legal_basis: "Legitimate interest".to_string(),
    };
    
    pii_map.add_field(user_email_field);
    pii_map.add_field(user_phone_field);
    
    // Create DSR manager
    let mut dsr_manager = DsrErasureManager::new(pii_map);
    
    // Create erasure request
    let mut affected_fields = HashSet::new();
    affected_fields.insert("user_email".to_string());
    affected_fields.insert("user_phone".to_string());
    
    let erasure_request = DsrRequest {
        id: "dsr-erasure-001".to_string(),
        request_type: DsrRequestType::Erasure,
        requester_id: "user-456".to_string(),
        timestamp: 1234567891,
        status: DsrRequestStatus::Pending,
        affected_fields,
        resolution_notes: None,
    };
    
    // Submit erasure request
    assert!(dsr_manager.submit_request(erasure_request).is_ok());
    println!("✓ DSR erasure request submitted successfully");
    
    // Process erasure request
    assert!(dsr_manager.process_erasure("dsr-erasure-001").is_ok());
    println!("✓ DSR erasure request processed successfully");
    
    // Verify request status was updated
    let processed_request = dsr_manager.get_request("dsr-erasure-001").expect("Request should exist");
    assert_eq!(processed_request.status, DsrRequestStatus::Completed);
    assert!(processed_request.resolution_notes.is_some());
    assert!(processed_request.resolution_notes.as_ref().unwrap().contains("Erasure completed"));
    
    // Verify audit log was updated
    let audit_log = dsr_manager.get_audit_log();
    assert!(!audit_log.is_empty());
    assert!(audit_log.iter().any(|log| log.contains("dsr-erasure-001")));
    assert!(audit_log.iter().any(|log| log.contains("Erasure completed")));
    
    println!("✓ DSR erasure request audit logging working correctly");
    
    println!("DSR erasure request functionality test passed!");
}

/// Test DSR rectification and portability requests
#[test]
fn test_dsr_rectification_and_portability_requests() {
    println!("Starting DSR rectification and portability requests test");
    
    // Create PII map with user data
    let mut pii_map = PiiMap::new();
    
    let user_profile_field = PiiField {
        name: "user_profile".to_string(),
        description: "User profile information".to_string(),
        classification: PiiClassification::Confidential,
        storage_location: "profiles table".to_string(),
        retention_period: 365,
        legal_basis: "Contract performance".to_string(),
    };
    
    pii_map.add_field(user_profile_field);
    
    // Create DSR manager
    let mut dsr_manager = DsrErasureManager::new(pii_map);
    
    // Test rectification request
    let mut rectification_fields = HashSet::new();
    rectification_fields.insert("user_profile".to_string());
    
    let rectification_request = DsrRequest {
        id: "dsr-rectify-001".to_string(),
        request_type: DsrRequestType::Rectification,
        requester_id: "user-789".to_string(),
        timestamp: 1234567892,
        status: DsrRequestStatus::Pending,
        affected_fields: rectification_fields,
        resolution_notes: None,
    };
    
    assert!(dsr_manager.submit_request(rectification_request).is_ok());
    println!("✓ DSR rectification request submitted successfully");
    
    // Test data portability request
    let mut portability_fields = HashSet::new();
    portability_fields.insert("user_profile".to_string());
    
    let portability_request = DsrRequest {
        id: "dsr-portability-001".to_string(),
        request_type: DsrRequestType::DataPortability,
        requester_id: "user-789".to_string(),
        timestamp: 1234567893,
        status: DsrRequestStatus::Pending,
        affected_fields: portability_fields,
        resolution_notes: None,
    };
    
    assert!(dsr_manager.submit_request(portability_request).is_ok());
    println!("✓ DSR data portability request submitted successfully");
    
    // Verify all requests are stored
    assert!(dsr_manager.get_request("dsr-rectify-001").is_some());
    assert!(dsr_manager.get_request("dsr-portability-001").is_some());
    
    // Test request filtering by type
    let all_requests = dsr_manager.get_all_requests();
    assert_eq!(all_requests.len(), 2);
    
    let rectification_requests = dsr_manager.get_requests_by_type(DsrRequestType::Rectification);
    assert_eq!(rectification_requests.len(), 1);
    assert_eq!(rectification_requests[0].id, "dsr-rectify-001");
    
    let portability_requests = dsr_manager.get_requests_by_type(DsrRequestType::DataPortability);
    assert_eq!(portability_requests.len(), 1);
    assert_eq!(portability_requests[0].id, "dsr-portability-001");
    
    println!("✓ DSR request filtering by type working correctly");
    
    // Test audit logging for all requests
    let audit_log = dsr_manager.get_audit_log();
    assert!(audit_log.len() >= 2); // At least two log entries
    assert!(audit_log.iter().any(|log| log.contains("dsr-rectify-001")));
    assert!(audit_log.iter().any(|log| log.contains("dsr-portability-001")));
    
    println!("✓ DSR rectification and portability requests audit logging working correctly");
    
    println!("DSR rectification and portability requests test passed!");
}

/// Test DSR error handling and edge cases
#[test]
fn test_dsr_error_handling() {
    println!("Starting DSR error handling test");
    
    // Create empty PII map
    let pii_map = PiiMap::new();
    let mut dsr_manager = DsrErasureManager::new(pii_map);
    
    // Test duplicate request ID
    let request1 = DsrRequest {
        id: "duplicate-id".to_string(),
        request_type: DsrRequestType::Access,
        requester_id: "user-111".to_string(),
        timestamp: 1234567890,
        status: DsrRequestStatus::Pending,
        affected_fields: HashSet::new(),
        resolution_notes: None,
    };
    
    // First submission should succeed
    assert!(dsr_manager.submit_request(request1.clone()).is_ok());
    
    // Second submission with same ID should fail
    assert!(dsr_manager.submit_request(request1).is_err());
    println!("✓ Duplicate request ID rejection working correctly");
    
    // Test processing non-existent request
    assert!(dsr_manager.process_erasure("non-existent-id").is_err());
    println!("✓ Non-existent request processing rejection working correctly");
    
    // Test getting non-existent request
    assert!(dsr_manager.get_request("non-existent-id").is_none());
    println!("✓ Non-existent request retrieval handling working correctly");
    
    // Test processing wrong request type (access request as erasure)
    let access_request = DsrRequest {
        id: "access-request".to_string(),
        request_type: DsrRequestType::Access,
        requester_id: "user-222".to_string(),
        timestamp: 1234567891,
        status: DsrRequestStatus::Pending,
        affected_fields: HashSet::new(),
        resolution_notes: None,
    };
    
    assert!(dsr_manager.submit_request(access_request).is_ok());
    
    // Processing access request as erasure should fail
    let result = dsr_manager.process_erasure("access-request");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not an erasure request"));
    println!("✓ Wrong request type processing rejection working correctly");
    
    println!("DSR error handling test passed!");
}