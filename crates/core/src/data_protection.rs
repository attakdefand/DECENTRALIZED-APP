//! Data Protection Module
//!
//! This module implements data protection measures including field encryption,
//! PII mapping, and data subject rights handling.

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

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

/// Consent record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRecord {
    /// Consent ID
    pub id: String,
    /// User identifier
    pub user_id: String,
    /// Purpose of consent
    pub purpose: String,
    /// Timestamp of consent
    pub timestamp: u64,
    /// Whether consent is given
    pub granted: bool,
    /// Expiration timestamp (optional)
    pub expires_at: Option<u64>,
    /// Version of terms
    pub terms_version: String,
}

/// Consent management
#[derive(Debug)]
pub struct ConsentManager {
    /// Stored consent records
    consents: HashMap<String, ConsentRecord>,
    /// Audit log
    audit_log: Vec<String>,
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
        let cipher =
            Aes256Gcm::new_from_slice(&self.key).map_err(|_| EncryptionError::InvalidKey)?;

        // Generate random nonce
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt the data
        let ciphertext = cipher
            .encrypt(nonce, plaintext)
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

        let cipher =
            Aes256Gcm::new_from_slice(&self.key).map_err(|_| EncryptionError::InvalidKey)?;

        // Extract nonce and ciphertext
        let nonce = Nonce::from_slice(&encrypted_data[0..12]);
        let ciphertext = &encrypted_data[12..];

        // Decrypt the data
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
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

    /// Get a PII field by name
    pub fn get_field(&self, field_name: &str) -> Option<&PiiField> {
        self.fields.get(field_name)
    }

    /// Get all PII fields by classification level
    pub fn get_fields_by_classification(
        &self,
        classification: PiiClassification,
    ) -> Vec<&PiiField> {
        self.fields
            .values()
            .filter(|field| field.classification == classification)
            .collect()
    }

    /// Check if a PII field exists
    pub fn has_field(&self, field_name: &str) -> bool {
        self.fields.contains_key(field_name)
    }

    /// Add a data flow mapping
    pub fn add_data_flow(&mut self, field_name: &str, destinations: Vec<String>) {
        self.data_flows.insert(field_name.to_string(), destinations);
    }

    /// Get data flow mappings for a PII field
    pub fn get_data_flows(&self, field_name: &str) -> Option<&Vec<String>> {
        self.data_flows.get(field_name)
    }
}

// Add Default implementation
impl Default for PiiMap {
    fn default() -> Self {
        Self::new()
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
            let request = self.requests.get(request_id).ok_or("Request not found")?;

            // Verify request type
            if !matches!(request.request_type, DsrRequestType::Erasure) {
                return Err("Request is not an erasure request".to_string());
            }

            request.affected_fields.clone()
        };

        // Update status
        {
            let request = self
                .requests
                .get_mut(request_id)
                .ok_or("Request not found")?;
            request.status = DsrRequestStatus::InProgress;
        }

        // Perform erasure
        let result = self.perform_erasure(&affected_fields);

        // Update status based on result
        let request = self
            .requests
            .get_mut(request_id)
            .ok_or("Request not found")?;

        match result {
            Ok(_) => {
                request.status = DsrRequestStatus::Completed;
                request.resolution_notes = Some("Erasure completed successfully".to_string());
                self.audit_log
                    .push(format!("Erasure completed for request: {}", request_id));
                Ok(())
            }
            Err(e) => {
                request.status = DsrRequestStatus::Rejected;
                request.resolution_notes = Some(format!("Erasure failed: {}", e));
                self.audit_log
                    .push(format!("Erasure failed for request: {}: {}", request_id, e));
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

    /// Process a data portability request
    pub fn process_data_portability(&mut self, request_id: &str) -> Result<String, String> {
        // Clone the affected fields to avoid borrowing issues
        let (affected_fields, is_data_portability) = {
            let request = self.requests.get(request_id).ok_or("Request not found")?;
            
            // Verify request type
            let is_data_portability = matches!(request.request_type, DsrRequestType::DataPortability);
            (request.affected_fields.clone(), is_data_portability)
        };
        
        if !is_data_portability {
            return Err("Request is not a data portability request".to_string());
        }

        // Update status
        {
            let request = self
                .requests
                .get_mut(request_id)
                .ok_or("Request not found")?;
            request.status = DsrRequestStatus::InProgress;
        }

        // Generate portable data (simulated)
        let portable_data = self.generate_portable_data(&affected_fields)?;
        
        // Update status to completed
        {
            let request = self
                .requests
                .get_mut(request_id)
                .ok_or("Request not found")?;
            request.status = DsrRequestStatus::Completed;
            request.resolution_notes = Some("Data portability completed successfully".to_string());
        }
        
        self.audit_log
            .push(format!("Data portability completed for request: {}", request_id));

        Ok(portable_data)
    }

    /// Generate portable data in JSON format
    fn generate_portable_data(&self, fields: &HashSet<String>) -> Result<String, String> {
        // In a real implementation, this would:
        // 1. Collect all data for the specified fields
        // 2. Format it in a standard portable format (e.g., JSON)
        // 3. Return the formatted data

        // For this example, we'll just simulate the process
        let mut data = serde_json::Map::new();
        
        for field in fields {
            // Verify field exists in PII map
            if !self.pii_map.has_field(field) {
                return Err(format!("Field {} not found in PII map", field));
            }
            
            // Add simulated data
            data.insert(field.clone(), serde_json::Value::String(format!("Data for {}", field)));
        }

        Ok(serde_json::to_string(&data).map_err(|e| format!("Failed to serialize data: {}", e))?)
    }

    /// Get request status
    pub fn get_request_status(&self, request_id: &str) -> Option<&DsrRequestStatus> {
        self.requests.get(request_id).map(|r| &r.status)
    }

    /// Get a specific request by ID
    pub fn get_request(&self, request_id: &str) -> Option<&DsrRequest> {
        self.requests.get(request_id)
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

    /// Get requests by type
    pub fn get_requests_by_type(&self, request_type: DsrRequestType) -> Vec<&DsrRequest> {
        self.requests
            .values()
            .filter(|request| request.request_type == request_type)
            .collect()
    }

    /// Encrypt a field if it is protected
    pub fn encrypt_field(&self, data: &[u8], field_name: &str) -> Result<Vec<u8>, String> {
        // Check if field should be protected
        if self.pii_map.has_field(field_name) {
            // For this example, we'll just return the data as-is
            // In a real implementation, we would apply field-level encryption
            Ok(data.to_vec())
        } else {
            // Return data as-is
            Ok(data.to_vec())
        }
    }
}

impl ConsentManager {
    /// Create a new consent manager
    pub fn new() -> Self {
        Self {
            consents: HashMap::new(),
            audit_log: Vec::new(),
        }
    }

    /// Record user consent
    pub fn record_consent(&mut self, consent: ConsentRecord) -> Result<(), String> {
        // Validate consent
        if self.consents.contains_key(&consent.id) {
            return Err("Consent ID already exists".to_string());
        }

        // Log the consent
        self.audit_log.push(format!(
            "Consent recorded: {} for user {} at {}",
            consent.id, consent.user_id, consent.timestamp
        ));

        // Store the consent
        self.consents.insert(consent.id.clone(), consent);

        Ok(())
    }

    /// Check if user has given consent for a purpose
    pub fn has_consent(&self, user_id: &str, purpose: &str) -> bool {
        self.consents.values().any(|consent| {
            consent.user_id == user_id
                && consent.purpose == purpose
                && consent.granted
                && (consent.expires_at.is_none()
                    || consent.expires_at.unwrap()
                        > std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs())
        })
    }

    /// Revoke user consent
    pub fn revoke_consent(&mut self, consent_id: &str) -> Result<(), String> {
        if let Some(consent) = self.consents.get_mut(consent_id) {
            consent.granted = false;
            self.audit_log.push(format!(
                "Consent revoked: {} at {}",
                consent_id,
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            ));
            Ok(())
        } else {
            Err("Consent not found".to_string())
        }
    }

    /// Get all consents for a user
    pub fn get_user_consents(&self, user_id: &str) -> Vec<&ConsentRecord> {
        self.consents
            .values()
            .filter(|consent| consent.user_id == user_id)
            .collect()
    }

    /// Get audit log
    pub fn get_audit_log(&self) -> &[String] {
        &self.audit_log
    }
}

impl Default for ConsentManager {
    fn default() -> Self {
        Self::new()
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
            EncryptionError::InvalidData => {}
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
        let confidential_fields =
            pii_map.get_fields_by_classification(PiiClassification::Confidential);
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
    fn test_data_portability() {
        let mut pii_map = PiiMap::new();

        // Add fields to the PII map
        let pii_field1 = PiiField {
            name: "user_email".to_string(),
            description: "User email address".to_string(),
            classification: PiiClassification::Confidential,
            storage_location: "users table".to_string(),
            retention_period: 365,
            legal_basis: "Contract performance".to_string(),
        };
        pii_map.add_field(pii_field1);

        let pii_field2 = PiiField {
            name: "user_name".to_string(),
            description: "User name".to_string(),
            classification: PiiClassification::Confidential,
            storage_location: "users table".to_string(),
            retention_period: 365,
            legal_basis: "Contract performance".to_string(),
        };
        pii_map.add_field(pii_field2);

        let mut dsr_manager = DsrErasureManager::new(pii_map);

        let mut affected_fields = HashSet::new();
        affected_fields.insert("user_email".to_string());
        affected_fields.insert("user_name".to_string());

        let request = DsrRequest {
            id: "req-004".to_string(),
            request_type: DsrRequestType::DataPortability,
            requester_id: "user-999".to_string(),
            timestamp: 1234567893,
            status: DsrRequestStatus::Pending,
            affected_fields,
            resolution_notes: None,
        };

        // Submit request
        assert!(dsr_manager.submit_request(request).is_ok());

        // Process data portability
        let portable_data = dsr_manager.process_data_portability("req-004").unwrap();
        assert!(!portable_data.is_empty());
        assert!(portable_data.contains("user_email"));
        assert!(portable_data.contains("user_name"));

        // Check status
        let status = dsr_manager.get_request_status("req-004").unwrap();
        assert_eq!(status, &DsrRequestStatus::Completed);
    }

    #[test]
    fn test_audit_logging() {
        let pii_map = PiiMap::new();
        let mut dsr_manager = DsrErasureManager::new(pii_map);

        let initial_log_size = dsr_manager.get_audit_log().len();

        let request = DsrRequest {
            id: "req-005".to_string(),
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
            id: "req-006".to_string(),
            request_type: DsrRequestType::Access,
            requester_id: "user-111".to_string(),
            timestamp: 1234567894,
            status: DsrRequestStatus::Pending,
            affected_fields: HashSet::new(),
            resolution_notes: None,
        };

        let request2 = DsrRequest {
            id: "req-007".to_string(),
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
        assert_eq!(pending_requests[0].id, "req-006");

        let completed_requests = dsr_manager.get_requests_by_status(DsrRequestStatus::Completed);
        assert_eq!(completed_requests.len(), 1);
        assert_eq!(completed_requests[0].id, "req-007");
    }

    #[test]
    fn test_consent_manager() {
        let mut consent_manager = ConsentManager::new();

        let consent = ConsentRecord {
            id: "consent-001".to_string(),
            user_id: "user-123".to_string(),
            purpose: "marketing".to_string(),
            timestamp: 1234567890,
            granted: true,
            expires_at: None,
            terms_version: "1.0".to_string(),
        };

        // Test recording consent
        assert!(consent_manager.record_consent(consent).is_ok());
        assert!(consent_manager.consents.contains_key("consent-001"));

        // Test checking consent
        assert!(consent_manager.has_consent("user-123", "marketing"));
        assert!(!consent_manager.has_consent("user-123", "analytics"));
        assert!(!consent_manager.has_consent("user-456", "marketing"));

        // Test revoking consent
        assert!(consent_manager.revoke_consent("consent-001").is_ok());
        assert!(!consent_manager.has_consent("user-123", "marketing"));
    }

    #[test]
    fn test_consent_expiration() {
        let mut consent_manager = ConsentManager::new();

        let consent = ConsentRecord {
            id: "consent-002".to_string(),
            user_id: "user-456".to_string(),
            purpose: "analytics".to_string(),
            timestamp: 1234567890,
            granted: true,
            expires_at: Some(1234567891), // Expired timestamp
            terms_version: "1.0".to_string(),
        };

        consent_manager.record_consent(consent).unwrap();

        // Test that expired consent is not valid
        assert!(!consent_manager.has_consent("user-456", "analytics"));
    }

    #[test]
    fn test_user_consents() {
        let mut consent_manager = ConsentManager::new();

        // Add multiple consents for the same user
        let consent1 = ConsentRecord {
            id: "consent-003".to_string(),
            user_id: "user-789".to_string(),
            purpose: "marketing".to_string(),
            timestamp: 1234567890,
            granted: true,
            expires_at: None,
            terms_version: "1.0".to_string(),
        };

        let consent2 = ConsentRecord {
            id: "consent-004".to_string(),
            user_id: "user-789".to_string(),
            purpose: "analytics".to_string(),
            timestamp: 1234567891,
            granted: true,
            expires_at: None,
            terms_version: "1.0".to_string(),
        };

        consent_manager.record_consent(consent1).unwrap();
        consent_manager.record_consent(consent2).unwrap();

        // Test retrieving all consents for a user
        let user_consents = consent_manager.get_user_consents("user-789");
        assert_eq!(user_consents.len(), 2);
    }
}