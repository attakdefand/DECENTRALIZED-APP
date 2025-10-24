//! Data Protection Module
//!
//! This module implements data protection measures including field encryption,
//! PII mapping, and data subject rights handling.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce
};
use rand::RngCore;

/// PII data classification levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PiiClassification {
    /// Public data
    Public,
    /// Internal data
    Internal,
    /// Confidential data
    Confidential,
    /// Highly confidential data
    HighlyConfidential,
}

/// PII data field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiField {
    /// Field name
    pub name: String,
    /// Field description
    pub description: String,
    /// Classification level
    pub classification: PiiClassification,
    /// Storage location
    pub storage_location: String,
    /// Retention period in days
    pub retention_period: u32,
    /// Legal basis for processing
    pub legal_basis: String,
}

/// DSR request types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DsrRequestType {
    /// Right to access
    Access,
    /// Right to rectification
    Rectification,
    /// Right to erasure
    Erasure,
    /// Right to restrict processing
    RestrictProcessing,
    /// Right to data portability
    DataPortability,
    /// Right to object
    Object,
}

/// DSR request status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DsrRequestStatus {
    /// Pending review
    Pending,
    /// In progress
    InProgress,
    /// Completed
    Completed,
    /// Rejected
    Rejected,
}

/// DSR request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DsrRequest {
    /// Request ID
    pub id: String,
    /// Request type
    pub request_type: DsrRequestType,
    /// Requester identifier
    pub requester_id: String,
    /// Timestamp of request
    pub timestamp: u64,
    /// Request status
    pub status: DsrRequestStatus,
    /// PII fields affected
    pub affected_fields: HashSet<String>,
    /// Resolution notes
    pub resolution_notes: Option<String>,
}

/// Encryption error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionError {
    InvalidKey,
    EncryptionFailed,
    DecryptionFailed,
    InvalidData,
}

/// Field encryption manager
#[derive(Debug)]
pub struct FieldEncryption {
    /// Encryption key
    key: [u8; 32],
    /// Key identifier
    key_id: String,
}

/// PII data map
#[derive(Debug, Clone)]
pub struct PiiMap {
    /// Map of PII fields
    fields: HashMap<String, PiiField>,
    /// Data flow mappings
    data_flows: HashMap<String, Vec<String>>,
}

/// DSR and erasure manager
#[derive(Debug)]
pub struct DsrErasureManager {
    /// Active DSR requests
    requests: HashMap<String, DsrRequest>,
    /// PII map reference
    pii_map: PiiMap,
    /// Audit log
    audit_log: Vec<String>,
}

impl FieldEncryption {
    /// Create a new field encryption instance
    pub fn new(key: [u8; 32], key_id: String) -> Self {
        Self { key, key_id }
    }
    
    /// Encrypt a field
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|_| EncryptionError::InvalidKey)?;
        
        // Generate random nonce
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // Encrypt the data
        let ciphertext = cipher.encrypt(nonce, plaintext)
            .map_err(|_| EncryptionError::EncryptionFailed)?;
        
        // Combine nonce and ciphertext
        let mut result = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);
        
        Ok(result)
    }
    
    /// Decrypt a field
    pub fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        if encrypted_data.len() < 12 {
            return Err(EncryptionError::InvalidData);
        }
        
        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|_| EncryptionError::InvalidKey)?;
        
        // Extract nonce and ciphertext
        let nonce = Nonce::from_slice(&encrypted_data[0..12]);
        let ciphertext = &encrypted_data[12..];
        
        // Decrypt the data
        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|_| EncryptionError::DecryptionFailed)?;
        
        Ok(plaintext)
    }
    
    /// Get key identifier
    pub fn key_id(&self) -> &str {
        &self.key_id
    }
}

impl PiiMap {
    /// Create a new PII map
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            data_flows: HashMap::new(),
        }
    }
    
    /// Add a PII field
    pub fn add_field(&mut self, field: PiiField) {
        self.fields.insert(field.name.clone(), field);
    }
    
    /// Get a PII field
    pub fn get_field(&self, name: &str) -> Option<&PiiField> {
        self.fields.get(name)
    }
    
    /// Add a data flow
    pub fn add_data_flow(&mut self, source: &str, destinations: Vec<String>) {
        self.data_flows.insert(source.to_string(), destinations);
    }
    
    /// Get data flows for a source
    pub fn get_data_flows(&self, source: &str) -> Option<&Vec<String>> {
        self.data_flows.get(source)
    }
    
    /// Get all PII fields
    pub fn get_all_fields(&self) -> Vec<&PiiField> {
        self.fields.values().collect()
    }
    
    /// Get fields by classification
    pub fn get_fields_by_classification(&self, classification: PiiClassification) -> Vec<&PiiField> {
        self.fields
            .values()
            .filter(|field| field.classification == classification)
            .collect()
    }
    
    /// Check if a field exists
    pub fn has_field(&self, name: &str) -> bool {
        self.fields.contains_key(name)
    }
}

impl DsrErasureManager {
    /// Create a new DSR manager
    pub fn new(pii_map: PiiMap) -> Self {
        Self {
            requests: HashMap::new(),
            pii_map,
            audit_log: Vec::new(),
        }
    }
    
    /// Submit a DSR request
    pub fn submit_request(&mut self, request: DsrRequest) -> Result<(), String> {
        // Validate request
        if self.requests.contains_key(&request.id) {
            return Err("Request ID already exists".to_string());
        }
        
        // Log the request
        self.audit_log.push(format!(
            "DSR request submitted: {} at {}",
            request.id, request.timestamp
        ));
        
        // Store the request
        self.requests.insert(request.id.clone(), request);
        
        Ok(())
    }
    
    /// Process an erasure request
    pub fn process_erasure(&mut self, request_id: &str) -> Result<(), String> {
        // Clone the affected fields to avoid borrowing issues
        let affected_fields = {
            let request = self.requests.get(request_id)
                .ok_or("Request not found")?;
            
            // Verify request type
            if !matches!(request.request_type, DsrRequestType::Erasure) {
                return Err("Request is not an erasure request".to_string());
            }
            
            request.affected_fields.clone()
        };
        
        // Update status
        {
            let request = self.requests.get_mut(request_id)
                .ok_or("Request not found")?;
            request.status = DsrRequestStatus::InProgress;
        }
        
        // Perform erasure
        let result = self.perform_erasure(&affected_fields);
        
        // Update status based on result
        let request = self.requests.get_mut(request_id)
            .ok_or("Request not found")?;
            
        match result {
            Ok(_) => {
                request.status = DsrRequestStatus::Completed;
                request.resolution_notes = Some("Erasure completed successfully".to_string());
                self.audit_log.push(format!(
                    "Erasure completed for request: {}",
                    request_id
                ));
                Ok(())
            }
            Err(e) => {
                request.status = DsrRequestStatus::Rejected;
                request.resolution_notes = Some(format!("Erasure failed: {}", e));
                self.audit_log.push(format!(
                    "Erasure failed for request: {}: {}",
                    request_id, e
                ));
                Err(e)
            }
        }
    }
    
    /// Perform actual data erasure
    fn perform_erasure(&self, fields: &HashSet<String>) -> Result<(), String> {
        // In a real implementation, this would:
        // 1. Locate all instances of the specified PII fields
        // 2. Erase the data from primary storage
        // 3. Handle backups and archives
        // 4. Verify erasure completion
        // 5. Update audit logs
        
        // For this example, we'll just simulate the process
        for field in fields {
            // Verify field exists in PII map
            if !self.pii_map.has_field(field) {
                return Err(format!("Field {} not found in PII map", field));
            }
            
            // Simulate erasure
            println!("Erasing data for field: {}", field);
            // Actual erasure implementation would go here
        }
        
        Ok(())
    }
    
    /// Get request status
    pub fn get_request_status(&self, request_id: &str) -> Option<&DsrRequestStatus> {
        self.requests.get(request_id).map(|r| &r.status)
    }
    
    /// Get audit log
    pub fn get_audit_log(&self) -> &[String] {
        &self.audit_log
    }
    
    /// Get all requests
    pub fn get_all_requests(&self) -> Vec<&DsrRequest> {
        self.requests.values().collect()
    }
    
    /// Get requests by status
    pub fn get_requests_by_status(&self, status: DsrRequestStatus) -> Vec<&DsrRequest> {
        self.requests
            .values()
            .filter(|request| request.status == status)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_field_encryption() {
        let key = [1u8; 32]; // Test key
        let encryption = FieldEncryption::new(key, "test-key-1".to_string());
        
        let plaintext = b"Hello, World!";
        let encrypted = encryption.encrypt(plaintext).unwrap();
        let decrypted = encryption.decrypt(&encrypted).unwrap();
        
        assert_eq!(plaintext, decrypted.as_slice());
    }
    
    #[test]
    fn test_field_encryption_errors() {
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
    }
    
    #[test]
    fn test_pii_map() {
        let mut pii_map = PiiMap::new();
        
        let pii_field = PiiField {
            name: "user_email".to_string(),
            description: "User email address".to_string(),
            classification: PiiClassification::Confidential,
            storage_location: "users table".to_string(),
            retention_period: 365,
            legal_basis: "Contract performance".to_string(),
        };
        
        pii_map.add_field(pii_field);
        
        // Test field retrieval
        let field = pii_map.get_field("user_email").unwrap();
        assert_eq!(field.name, "user_email");
        assert_eq!(field.classification, PiiClassification::Confidential);
        
        // Test classification filtering
        let confidential_fields = pii_map.get_fields_by_classification(PiiClassification::Confidential);
        assert_eq!(confidential_fields.len(), 1);
        assert_eq!(confidential_fields[0].name, "user_email");
        
        // Test field existence
        assert!(pii_map.has_field("user_email"));
        assert!(!pii_map.has_field("nonexistent_field"));
    }
    
    #[test]
    fn test_data_flows() {
        let mut pii_map = PiiMap::new();
        
        // Add data flow
        let destinations = vec!["api_service".to_string(), "analytics_service".to_string()];
        pii_map.add_data_flow("user_email", destinations.clone());
        
        // Retrieve data flow
        let retrieved_destinations = pii_map.get_data_flows("user_email").unwrap();
        assert_eq!(retrieved_destinations, &destinations);
    }
    
    #[test]
    fn test_dsr_erasure_manager() {
        let pii_map = PiiMap::new();
        let mut dsr_manager = DsrErasureManager::new(pii_map);
        
        let mut affected_fields = HashSet::new();
        affected_fields.insert("user_email".to_string());
        
        let request = DsrRequest {
            id: "req-001".to_string(),
            request_type: DsrRequestType::Erasure,
            requester_id: "user-123".to_string(),
            timestamp: 1234567890,
            status: DsrRequestStatus::Pending,
            affected_fields,
            resolution_notes: None,
        };
        
        // Test request submission
        assert!(dsr_manager.submit_request(request).is_ok());
        assert!(dsr_manager.requests.contains_key("req-001"));
        
        // Test request status retrieval
        let status = dsr_manager.get_request_status("req-001").unwrap();
        assert_eq!(status, &DsrRequestStatus::Pending);
    }
    
    #[test]
    fn test_dsr_erasure_processing() {
        let mut pii_map = PiiMap::new();
        
        // Add a field to the PII map
        let pii_field = PiiField {
            name: "user_email".to_string(),
            description: "User email address".to_string(),
            classification: PiiClassification::Confidential,
            storage_location: "users table".to_string(),
            retention_period: 365,
            legal_basis: "Contract performance".to_string(),
        };
        pii_map.add_field(pii_field);
        
        let mut dsr_manager = DsrErasureManager::new(pii_map);
        
        let mut affected_fields = HashSet::new();
        affected_fields.insert("user_email".to_string());
        
        let request = DsrRequest {
            id: "req-002".to_string(),
            request_type: DsrRequestType::Erasure,
            requester_id: "user-456".to_string(),
            timestamp: 1234567891,
            status: DsrRequestStatus::Pending,
            affected_fields,
            resolution_notes: None,
        };
        
        // Submit request
        assert!(dsr_manager.submit_request(request).is_ok());
        
        // Process erasure
        assert!(dsr_manager.process_erasure("req-002").is_ok());
        
        // Check status
        let status = dsr_manager.get_request_status("req-002").unwrap();
        assert_eq!(status, &DsrRequestStatus::Completed);
    }
    
    #[test]
    fn test_dsr_erasure_with_nonexistent_field() {
        let pii_map = PiiMap::new();
        let mut dsr_manager = DsrErasureManager::new(pii_map);
        
        let mut affected_fields = HashSet::new();
        affected_fields.insert("nonexistent_field".to_string());
        
        let request = DsrRequest {
            id: "req-003".to_string(),
            request_type: DsrRequestType::Erasure,
            requester_id: "user-789".to_string(),
            timestamp: 1234567892,
            status: DsrRequestStatus::Pending,
            affected_fields,
            resolution_notes: None,
        };
        
        // Submit request
        assert!(dsr_manager.submit_request(request).is_ok());
        
        // Process erasure should fail due to nonexistent field
        let result = dsr_manager.process_erasure("req-003");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found in PII map"));
        
        // Check status should be rejected
        let status = dsr_manager.get_request_status("req-003").unwrap();
        assert_eq!(status, &DsrRequestStatus::Rejected);
    }
    
    #[test]
    fn test_audit_logging() {
        let pii_map = PiiMap::new();
        let mut dsr_manager = DsrErasureManager::new(pii_map);
        
        let initial_log_size = dsr_manager.get_audit_log().len();
        
        let request = DsrRequest {
            id: "req-004".to_string(),
            request_type: DsrRequestType::Access,
            requester_id: "user-999".to_string(),
            timestamp: 1234567893,
            status: DsrRequestStatus::Pending,
            affected_fields: HashSet::new(),
            resolution_notes: None,
        };
        
        dsr_manager.submit_request(request).unwrap();
        
        // Verify audit log was updated
        assert_eq!(dsr_manager.get_audit_log().len(), initial_log_size + 1);
        assert!(dsr_manager.get_audit_log()[initial_log_size].contains("DSR request submitted"));
    }
    
    #[test]
    fn test_request_filtering() {
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
        
        dsr_manager.submit_request(request1).unwrap();
        dsr_manager.submit_request(request2).unwrap();
        
        // Test filtering by status
        let pending_requests = dsr_manager.get_requests_by_status(DsrRequestStatus::Pending);
        assert_eq!(pending_requests.len(), 1);
        assert_eq!(pending_requests[0].id, "req-005");
        
        let completed_requests = dsr_manager.get_requests_by_status(DsrRequestStatus::Completed);
        assert_eq!(completed_requests.len(), 1);
        assert_eq!(completed_requests[0].id, "req-006");
    }
}