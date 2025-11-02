//! Key Management Module
//!
//! This module implements key management systems including KMS, HSM, and MPC
//! for protecting private keys and signing material as specified in the 
//! Identity, Access & Crypto Foundations requirements.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Key Management System (KMS) interface
pub trait KmsInterface {
    /// Generate a new key
    fn generate_key(&mut self, key_spec: &KeySpec) -> Result<String, String>;
    
    /// Encrypt data with a key
    fn encrypt(&self, key_id: &str, plaintext: &[u8]) -> Result<Vec<u8>, String>;
    
    /// Decrypt data with a key
    fn decrypt(&self, key_id: &str, ciphertext: &[u8]) -> Result<Vec<u8>, String>;
    
    /// Sign data with a key
    fn sign(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, String>;
    
    /// Verify signature
    fn verify(&self, key_id: &str, data: &[u8], signature: &[u8]) -> Result<bool, String>;
    
    /// Rotate a key
    fn rotate_key(&mut self, key_id: &str) -> Result<String, String>;
    
    /// Get key metadata
    fn get_key_metadata(&self, key_id: &str) -> Result<KeyMetadata, String>;
}

/// Hardware Security Module (HSM) interface
pub trait HsmInterface {
    /// Generate a key in the HSM
    fn generate_key_in_hsm(&mut self, key_spec: &KeySpec) -> Result<String, String>;
    
    /// Perform cryptographic operation in HSM
    fn perform_operation(&self, operation: HsmOperation) -> Result<Vec<u8>, String>;
    
    /// Get HSM status
    fn get_status(&self) -> HsmStatus;
    
    /// Backup HSM keys
    fn backup_keys(&self) -> Result<Vec<u8>, String>;
}

/// Multi-Party Computation (MPC) interface
pub trait MpcInterface {
    /// Generate a distributed key
    fn generate_distributed_key(&mut self, participants: &[String], threshold: usize) -> Result<String, String>;
    
    /// Perform distributed signing
    fn distributed_sign(&self, key_id: &str, data: &[u8], participants: &[String]) -> Result<Vec<u8>, String>;
    
    /// Verify distributed signature
    fn verify_distributed_signature(&self, key_id: &str, data: &[u8], signature: &[u8]) -> Result<bool, String>;
    
    /// Add participant to MPC network
    fn add_participant(&mut self, participant: &MpcParticipant) -> Result<(), String>;
    
    /// Remove participant from MPC network
    fn remove_participant(&mut self, participant_id: &str) -> Result<(), String>;
}

/// Key specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeySpec {
    /// Key algorithm (e.g., "RSA-2048", "ECDSA-P256", "AES-256")
    pub algorithm: String,
    /// Key usage (e.g., "ENCRYPT_DECRYPT", "SIGN_VERIFY", "KEY_WRAP")
    pub usage: String,
    /// Key origin (e.g., "KMS", "HSM", "SOFTWARE")
    pub origin: String,
    /// Key description
    pub description: String,
}

/// Key metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadata {
    /// Key identifier
    pub key_id: String,
    /// Key creation timestamp
    pub creation_date: u64,
    /// Key last rotation timestamp
    pub last_rotation_date: u64,
    /// Key algorithm
    pub algorithm: String,
    /// Key state
    pub state: KeyState,
    /// Key origin
    pub origin: String,
    /// Key owner
    pub owner: String,
}

/// Key state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KeyState {
    /// Key is enabled for use
    Enabled,
    /// Key is disabled
    Disabled,
    /// Key is pending deletion
    PendingDeletion,
    /// Key is deleted
    Deleted,
}

/// KMS implementation
#[derive(Debug)]
pub struct KmsManager {
    /// Key storage
    keys: HashMap<String, StoredKey>,
    /// Key metadata
    metadata: HashMap<String, KeyMetadata>,
    /// Audit logs
    audit_logs: Vec<KmsAuditLog>,
}

/// Stored key representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredKey {
    /// Key identifier
    pub key_id: String,
    /// Encrypted key material
    pub encrypted_key: Vec<u8>,
    /// Key wrapping key identifier
    pub wrapping_key_id: String,
    /// Creation timestamp
    pub created_at: u64,
}

/// KMS audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KmsAuditLog {
    /// Timestamp of the operation
    pub timestamp: u64,
    /// User or service that performed the operation
    pub principal: String,
    /// Operation performed
    pub operation: String,
    /// Key identifier
    pub key_id: String,
    /// Whether the operation was successful
    pub success: bool,
    /// Error message if operation failed
    pub error_message: Option<String>,
}

/// HSM implementation
#[derive(Debug)]
pub struct HsmManager {
    /// HSM configuration
    config: HsmConfig,
    /// HSM status
    status: HsmStatus,
    /// HSM keys
    keys: HashMap<String, HsmKey>,
    /// Audit logs
    audit_logs: Vec<HsmAuditLog>,
}

/// HSM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmConfig {
    /// HSM vendor
    pub vendor: String,
    /// HSM model
    pub model: String,
    /// HSM serial number
    pub serial_number: String,
    /// HSM firmware version
    pub firmware_version: String,
    /// HSM security level (e.g., "FIPS 140-2 Level 3")
    pub security_level: String,
}

/// HSM status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmStatus {
    /// Whether HSM is online
    pub online: bool,
    /// HSM health status
    pub health: String,
    /// Last heartbeat timestamp
    pub last_heartbeat: u64,
    /// Number of active sessions
    pub active_sessions: usize,
}

/// HSM key representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmKey {
    /// Key identifier
    pub key_id: String,
    /// Key handle in HSM
    pub key_handle: u32,
    /// Key metadata
    pub metadata: KeyMetadata,
    /// Key permissions
    pub permissions: Vec<String>,
}

/// HSM audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmAuditLog {
    /// Timestamp of the operation
    pub timestamp: u64,
    /// User or service that performed the operation
    pub principal: String,
    /// Operation performed
    pub operation: String,
    /// Key identifier
    pub key_id: String,
    /// Whether the operation was successful
    pub success: bool,
    /// Error message if operation failed
    pub error_message: Option<String>,
}

/// HSM operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmOperation {
    /// Operation type
    pub op_type: String,
    /// Key identifier
    pub key_id: String,
    /// Input data
    pub input: Vec<u8>,
    /// Additional parameters
    pub parameters: HashMap<String, String>,
}

/// MPC implementation
#[derive(Debug)]
pub struct MpcManager {
    /// MPC network participants
    participants: HashMap<String, MpcParticipant>,
    /// Distributed keys
    distributed_keys: HashMap<String, DistributedKey>,
    /// Audit logs
    audit_logs: Vec<MpcAuditLog>,
}

/// MPC participant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MpcParticipant {
    /// Participant identifier
    pub id: String,
    /// Participant public key
    pub public_key: String,
    /// Participant endpoint
    pub endpoint: String,
    /// Participant status
    pub status: ParticipantStatus,
    /// Last seen timestamp
    pub last_seen: u64,
}

/// Participant status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParticipantStatus {
    /// Participant is active
    Active,
    /// Participant is offline
    Offline,
    /// Participant is compromised
    Compromised,
}

/// Distributed key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedKey {
    /// Key identifier
    pub key_id: String,
    /// Threshold for signing
    pub threshold: usize,
    /// Number of participants
    pub participant_count: usize,
    /// Key creation timestamp
    pub created_at: u64,
    /// Key metadata
    pub metadata: KeyMetadata,
}

/// MPC audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MpcAuditLog {
    /// Timestamp of the operation
    pub timestamp: u64,
    /// User or service that performed the operation
    pub principal: String,
    /// Operation performed
    pub operation: String,
    /// Key identifier
    pub key_id: String,
    /// Participants involved
    pub participants: Vec<String>,
    /// Whether the operation was successful
    pub success: bool,
    /// Error message if operation failed
    pub error_message: Option<String>,
}

impl KmsManager {
    /// Create a new KMS manager
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            metadata: HashMap::new(),
            audit_logs: Vec::new(),
        }
    }
    
    /// Log an audit entry
    fn log_audit(&mut self, log: KmsAuditLog) {
        self.audit_logs.push(log);
        
        // Keep only the last 1000 logs to prevent memory issues
        if self.audit_logs.len() > 1000 {
            self.audit_logs.drain(0..self.audit_logs.len() - 1000);
        }
    }
    
    /// Get audit logs
    pub fn get_audit_logs(&self) -> &[KmsAuditLog] {
        &self.audit_logs
    }
    
    /// Generate telemetry report
    pub fn generate_telemetry_report(&self) -> String {
        let mut report = String::from("KMS Audit Logs:\n");
        report.push_str(&format!("Total Audit Logs: {}\n", self.audit_logs.len()));
        
        let successful_operations = self.audit_logs.iter().filter(|log| log.success).count();
        report.push_str(&format!("Successful Operations: {}\n", successful_operations));
        report.push_str(&format!("Failed Operations: {}\n", self.audit_logs.len() - successful_operations));
        
        // Add recent audit logs
        report.push_str("\nRecent KMS Audit Logs:\n");
        let recent_logs = self.audit_logs.iter().rev().take(10);
        for log in recent_logs {
            report.push_str(&format!(
                "  {} - {} - {} - {} - {}\n",
                log.timestamp,
                log.principal,
                log.operation,
                log.key_id,
                if log.success { "SUCCESS" } else { "FAILED" }
            ));
        }
        
        report
    }
}

impl KmsInterface for KmsManager {
    fn generate_key(&mut self, key_spec: &KeySpec) -> Result<String, String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Generate a unique key ID
        let key_id = format!("kms-key-{}", rand::random::<u64>());
        
        // In a real implementation, this would generate an actual key
        // For this implementation, we'll simulate the process
        let encrypted_key = vec![0u8; 32]; // Simulated encrypted key
        let wrapping_key_id = "master-key".to_string();
        
        let stored_key = StoredKey {
            key_id: key_id.clone(),
            encrypted_key,
            wrapping_key_id,
            created_at: timestamp,
        };
        
        let metadata = KeyMetadata {
            key_id: key_id.clone(),
            creation_date: timestamp,
            last_rotation_date: timestamp,
            algorithm: key_spec.algorithm.clone(),
            state: KeyState::Enabled,
            origin: key_spec.origin.clone(),
            owner: "system".to_string(),
        };
        
        self.keys.insert(key_id.clone(), stored_key);
        self.metadata.insert(key_id.clone(), metadata);
        
        // Log audit entry
        self.log_audit(KmsAuditLog {
            timestamp,
            principal: "system".to_string(),
            operation: "GenerateKey".to_string(),
            key_id: key_id.clone(),
            success: true,
            error_message: None,
        });
        
        Ok(key_id)
    }
    
    fn encrypt(&self, key_id: &str, plaintext: &[u8]) -> Result<Vec<u8>, String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Check if key exists
        if !self.keys.contains_key(key_id) {
            // Log audit entry
            // Note: We can't log to self.audit_logs here because this is an immutable reference
            return Err(format!("Key {} not found", key_id));
        }
        
        // In a real implementation, this would perform actual encryption
        // For this implementation, we'll simulate the process
        let mut ciphertext = plaintext.to_vec();
        ciphertext.push(0x01); // Simulated encryption
        
        Ok(ciphertext)
    }
    
    fn decrypt(&self, key_id: &str, ciphertext: &[u8]) -> Result<Vec<u8>, String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Check if key exists
        if !self.keys.contains_key(key_id) {
            return Err(format!("Key {} not found", key_id));
        }
        
        // In a real implementation, this would perform actual decryption
        // For this implementation, we'll simulate the process
        if ciphertext.is_empty() || ciphertext[ciphertext.len() - 1] != 0x01 {
            return Err("Invalid ciphertext".to_string());
        }
        
        let mut plaintext = ciphertext.to_vec();
        plaintext.pop(); // Remove simulated encryption marker
        
        Ok(plaintext)
    }
    
    fn sign(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Check if key exists
        if !self.keys.contains_key(key_id) {
            return Err(format!("Key {} not found", key_id));
        }
        
        // In a real implementation, this would perform actual signing
        // For this implementation, we'll simulate the process
        let signature = vec![0x02; 64]; // Simulated signature
        
        Ok(signature)
    }
    
    fn verify(&self, key_id: &str, data: &[u8], signature: &[u8]) -> Result<bool, String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Check if key exists
        if !self.keys.contains_key(key_id) {
            return Err(format!("Key {} not found", key_id));
        }
        
        // In a real implementation, this would perform actual verification
        // For this implementation, we'll simulate the process
        Ok(signature.len() == 64 && signature[0] == 0x02)
    }
    
    fn rotate_key(&mut self, key_id: &str) -> Result<String, String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Check if key exists
        if !self.keys.contains_key(key_id) {
            return Err(format!("Key {} not found", key_id));
        }
        
        // In a real implementation, this would generate a new key and update references
        // For this implementation, we'll simulate the process
        let new_key_id = format!("kms-key-{}", rand::random::<u64>());
        
        // Log audit entry
        self.log_audit(KmsAuditLog {
            timestamp,
            principal: "system".to_string(),
            operation: "RotateKey".to_string(),
            key_id: key_id.to_string(),
            success: true,
            error_message: None,
        });
        
        Ok(new_key_id)
    }
    
    fn get_key_metadata(&self, key_id: &str) -> Result<KeyMetadata, String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Check if key exists
        if let Some(metadata) = self.metadata.get(key_id) {
            Ok(metadata.clone())
        } else {
            Err(format!("Key {} not found", key_id))
        }
    }
}

impl Default for KmsManager {
    fn default() -> Self {
        Self::new()
    }
}

impl HsmManager {
    /// Create a new HSM manager
    pub fn new(config: HsmConfig) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            config,
            status: HsmStatus {
                online: true,
                health: "OK".to_string(),
                last_heartbeat: timestamp,
                active_sessions: 0,
            },
            keys: HashMap::new(),
            audit_logs: Vec::new(),
        }
    }
    
    /// Log an audit entry
    fn log_audit(&mut self, log: HsmAuditLog) {
        self.audit_logs.push(log);
        
        // Keep only the last 1000 logs to prevent memory issues
        if self.audit_logs.len() > 1000 {
            self.audit_logs.drain(0..self.audit_logs.len() - 1000);
        }
    }
    
    /// Get audit logs
    pub fn get_audit_logs(&self) -> &[HsmAuditLog] {
        &self.audit_logs
    }
    
    /// Generate telemetry report
    pub fn generate_telemetry_report(&self) -> String {
        let mut report = String::from("HSM Status:\n");
        report.push_str(&format!("Vendor: {}\n", self.config.vendor));
        report.push_str(&format!("Model: {}\n", self.config.model));
        report.push_str(&format!("Serial Number: {}\n", self.config.serial_number));
        report.push_str(&format!("Firmware Version: {}\n", self.config.firmware_version));
        report.push_str(&format!("Security Level: {}\n", self.config.security_level));
        report.push_str(&format!("Online: {}\n", self.status.online));
        report.push_str(&format!("Health: {}\n", self.status.health));
        report.push_str(&format!("Active Sessions: {}\n", self.status.active_sessions));
        
        report.push_str(&format!("\nHSM Audit Logs:\n"));
        report.push_str(&format!("Total Audit Logs: {}\n", self.audit_logs.len()));
        
        let successful_operations = self.audit_logs.iter().filter(|log| log.success).count();
        report.push_str(&format!("Successful Operations: {}\n", successful_operations));
        report.push_str(&format!("Failed Operations: {}\n", self.audit_logs.len() - successful_operations));
        
        // Add recent audit logs
        report.push_str("\nRecent HSM Audit Logs:\n");
        let recent_logs = self.audit_logs.iter().rev().take(10);
        for log in recent_logs {
            report.push_str(&format!(
                "  {} - {} - {} - {} - {}\n",
                log.timestamp,
                log.principal,
                log.operation,
                log.key_id,
                if log.success { "SUCCESS" } else { "FAILED" }
            ));
        }
        
        report
    }
}

impl HsmInterface for HsmManager {
    fn generate_key_in_hsm(&mut self, key_spec: &KeySpec) -> Result<String, String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Generate a unique key ID
        let key_id = format!("hsm-key-{}", rand::random::<u64>());
        
        // In a real implementation, this would generate an actual key in the HSM
        // For this implementation, we'll simulate the process
        let key_handle = rand::random::<u32>();
        
        let metadata = KeyMetadata {
            key_id: key_id.clone(),
            creation_date: timestamp,
            last_rotation_date: timestamp,
            algorithm: key_spec.algorithm.clone(),
            state: KeyState::Enabled,
            origin: "HSM".to_string(),
            owner: "system".to_string(),
        };
        
        let hsm_key = HsmKey {
            key_id: key_id.clone(),
            key_handle,
            metadata: metadata.clone(),
            permissions: vec!["encrypt".to_string(), "decrypt".to_string()],
        };
        
        self.keys.insert(key_id.clone(), hsm_key);
        
        // Log audit entry
        self.log_audit(HsmAuditLog {
            timestamp,
            principal: "system".to_string(),
            operation: "GenerateKeyInHsm".to_string(),
            key_id: key_id.clone(),
            success: true,
            error_message: None,
        });
        
        Ok(key_id)
    }
    
    fn perform_operation(&self, operation: HsmOperation) -> Result<Vec<u8>, String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Check if key exists
        if !self.keys.contains_key(&operation.key_id) {
            return Err(format!("Key {} not found", operation.key_id));
        }
        
        // In a real implementation, this would perform the actual operation in the HSM
        // For this implementation, we'll simulate the process
        let result = match operation.op_type.as_str() {
            "encrypt" => {
                let mut result = operation.input.clone();
                result.push(0x03); // Simulated encryption marker
                result
            },
            "decrypt" => {
                if operation.input.is_empty() || operation.input[operation.input.len() - 1] != 0x03 {
                    return Err("Invalid ciphertext".to_string());
                }
                let mut result = operation.input.clone();
                result.pop(); // Remove simulated encryption marker
                result
            },
            "sign" => {
                vec![0x04; 64] // Simulated signature
            },
            _ => return Err(format!("Unsupported operation: {}", operation.op_type)),
        };
        
        Ok(result)
    }
    
    fn get_status(&self) -> HsmStatus {
        self.status.clone()
    }
    
    fn backup_keys(&self) -> Result<Vec<u8>, String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // In a real implementation, this would create an actual backup of HSM keys
        // For this implementation, we'll simulate the process
        let backup_data = vec![0x05; 1024]; // Simulated backup data
        
        // Log audit entry
        // Note: We can't log to self.audit_logs here because this is an immutable reference
        
        Ok(backup_data)
    }
}

impl MpcManager {
    /// Create a new MPC manager
    pub fn new() -> Self {
        Self {
            participants: HashMap::new(),
            distributed_keys: HashMap::new(),
            audit_logs: Vec::new(),
        }
    }
    
    /// Log an audit entry
    fn log_audit(&mut self, log: MpcAuditLog) {
        self.audit_logs.push(log);
        
        // Keep only the last 1000 logs to prevent memory issues
        if self.audit_logs.len() > 1000 {
            self.audit_logs.drain(0..self.audit_logs.len() - 1000);
        }
    }
    
    /// Get audit logs
    pub fn get_audit_logs(&self) -> &[MpcAuditLog] {
        &self.audit_logs
    }
    
    /// Generate telemetry report
    pub fn generate_telemetry_report(&self) -> String {
        let mut report = String::from("MPC Network Status:\n");
        report.push_str(&format!("Total Participants: {}\n", self.participants.len()));
        report.push_str(&format!("Active Participants: {}\n", 
            self.participants.values().filter(|p| p.status == ParticipantStatus::Active).count()));
        report.push_str(&format!("Offline Participants: {}\n", 
            self.participants.values().filter(|p| p.status == ParticipantStatus::Offline).count()));
        report.push_str(&format!("Compromised Participants: {}\n", 
            self.participants.values().filter(|p| p.status == ParticipantStatus::Compromised).count()));
        report.push_str(&format!("Distributed Keys: {}\n", self.distributed_keys.len()));
        
        report.push_str(&format!("\nMPC Audit Logs:\n"));
        report.push_str(&format!("Total Audit Logs: {}\n", self.audit_logs.len()));
        
        let successful_operations = self.audit_logs.iter().filter(|log| log.success).count();
        report.push_str(&format!("Successful Operations: {}\n", successful_operations));
        report.push_str(&format!("Failed Operations: {}\n", self.audit_logs.len() - successful_operations));
        
        // Add recent audit logs
        report.push_str("\nRecent MPC Audit Logs:\n");
        let recent_logs = self.audit_logs.iter().rev().take(10);
        for log in recent_logs {
            report.push_str(&format!(
                "  {} - {} - {} - {} - {}\n",
                log.timestamp,
                log.principal,
                log.operation,
                log.key_id,
                if log.success { "SUCCESS" } else { "FAILED" }
            ));
        }
        
        report
    }
}

impl MpcInterface for MpcManager {
    fn generate_distributed_key(&mut self, participants: &[String], threshold: usize) -> Result<String, String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Check if enough participants are available
        if participants.len() < threshold {
            return Err("Not enough participants for threshold".to_string());
        }
        
        // Generate a unique key ID
        let key_id = format!("mpc-key-{}", rand::random::<u64>());
        
        // In a real implementation, this would generate an actual distributed key
        // For this implementation, we'll simulate the process
        let distributed_key = DistributedKey {
            key_id: key_id.clone(),
            threshold,
            participant_count: participants.len(),
            created_at: timestamp,
            metadata: KeyMetadata {
                key_id: key_id.clone(),
                creation_date: timestamp,
                last_rotation_date: timestamp,
                algorithm: "ECDSA-P256".to_string(),
                state: KeyState::Enabled,
                origin: "MPC".to_string(),
                owner: "system".to_string(),
            },
        };
        
        self.distributed_keys.insert(key_id.clone(), distributed_key);
        
        // Log audit entry
        self.log_audit(MpcAuditLog {
            timestamp,
            principal: "system".to_string(),
            operation: "GenerateDistributedKey".to_string(),
            key_id: key_id.clone(),
            participants: participants.to_vec(),
            success: true,
            error_message: None,
        });
        
        Ok(key_id)
    }
    
    fn distributed_sign(&self, key_id: &str, data: &[u8], participants: &[String]) -> Result<Vec<u8>, String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Check if key exists
        if !self.distributed_keys.contains_key(key_id) {
            return Err(format!("Key {} not found", key_id));
        }
        
        // Check if enough participants are available
        let key = self.distributed_keys.get(key_id).unwrap();
        if participants.len() < key.threshold {
            return Err("Not enough participants for threshold signing".to_string());
        }
        
        // In a real implementation, this would perform actual distributed signing
        // For this implementation, we'll simulate the process
        let signature = vec![0x06; 64]; // Simulated signature
        
        Ok(signature)
    }
    
    fn verify_distributed_signature(&self, key_id: &str, data: &[u8], signature: &[u8]) -> Result<bool, String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Check if key exists
        if !self.distributed_keys.contains_key(key_id) {
            return Err(format!("Key {} not found", key_id));
        }
        
        // In a real implementation, this would perform actual verification
        // For this implementation, we'll simulate the process
        Ok(signature.len() == 64 && signature[0] == 0x06)
    }
    
    fn add_participant(&mut self, participant: &MpcParticipant) -> Result<(), String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        self.participants.insert(participant.id.clone(), participant.clone());
        
        // Log audit entry
        self.log_audit(MpcAuditLog {
            timestamp,
            principal: "system".to_string(),
            operation: "AddParticipant".to_string(),
            key_id: "N/A".to_string(),
            participants: vec![participant.id.clone()],
            success: true,
            error_message: None,
        });
        
        Ok(())
    }
    
    fn remove_participant(&mut self, participant_id: &str) -> Result<(), String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        if self.participants.remove(participant_id).is_some() {
            // Log audit entry
            self.log_audit(MpcAuditLog {
                timestamp,
                principal: "system".to_string(),
                operation: "RemoveParticipant".to_string(),
                key_id: "N/A".to_string(),
                participants: vec![participant_id.to_string()],
                success: true,
                error_message: None,
            });
            
            Ok(())
        } else {
            Err(format!("Participant {} not found", participant_id))
        }
    }
}

impl Default for MpcManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kms_key_generation() {
        let mut kms = KmsManager::new();
        
        let key_spec = KeySpec {
            algorithm: "AES-256".to_string(),
            usage: "ENCRYPT_DECRYPT".to_string(),
            origin: "KMS".to_string(),
            description: "Test key".to_string(),
        };
        
        let key_id = kms.generate_key(&key_spec).unwrap();
        assert!(!key_id.is_empty());
        assert!(key_id.starts_with("kms-key-"));
        
        // Check that audit log was created
        assert_eq!(kms.get_audit_logs().len(), 1);
        let log = &kms.get_audit_logs()[0];
        assert_eq!(log.operation, "GenerateKey");
        assert_eq!(log.key_id, key_id);
        assert!(log.success);
    }

    #[test]
    fn test_kms_encrypt_decrypt() {
        let mut kms = KmsManager::new();
        
        let key_spec = KeySpec {
            algorithm: "AES-256".to_string(),
            usage: "ENCRYPT_DECRYPT".to_string(),
            origin: "KMS".to_string(),
            description: "Test key".to_string(),
        };
        
        let key_id = kms.generate_key(&key_spec).unwrap();
        let plaintext = b"Hello, World!";
        
        let ciphertext = kms.encrypt(&key_id, plaintext).unwrap();
        assert!(!ciphertext.is_empty());
        assert_ne!(ciphertext, plaintext);
        
        let decrypted = kms.decrypt(&key_id, &ciphertext).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_kms_sign_verify() {
        let mut kms = KmsManager::new();
        
        let key_spec = KeySpec {
            algorithm: "ECDSA-P256".to_string(),
            usage: "SIGN_VERIFY".to_string(),
            origin: "KMS".to_string(),
            description: "Test key".to_string(),
        };
        
        let key_id = kms.generate_key(&key_spec).unwrap();
        let data = b"Hello, World!";
        
        let signature = kms.sign(&key_id, data).unwrap();
        assert_eq!(signature.len(), 64);
        
        let verified = kms.verify(&key_id, data, &signature).unwrap();
        assert!(verified);
    }

    #[test]
    fn test_kms_key_rotation() {
        let mut kms = KmsManager::new();
        
        let key_spec = KeySpec {
            algorithm: "AES-256".to_string(),
            usage: "ENCRYPT_DECRYPT".to_string(),
            origin: "KMS".to_string(),
            description: "Test key".to_string(),
        };
        
        let key_id = kms.generate_key(&key_spec).unwrap();
        let new_key_id = kms.rotate_key(&key_id).unwrap();
        
        assert!(!new_key_id.is_empty());
        assert_ne!(key_id, new_key_id);
        assert!(new_key_id.starts_with("kms-key-"));
        
        // Check that audit log was created
        assert_eq!(kms.get_audit_logs().len(), 2);
        let log = &kms.get_audit_logs()[1];
        assert_eq!(log.operation, "RotateKey");
        assert_eq!(log.key_id, key_id);
        assert!(log.success);
    }

    #[test]
    fn test_kms_key_metadata() {
        let mut kms = KmsManager::new();
        
        let key_spec = KeySpec {
            algorithm: "AES-256".to_string(),
            usage: "ENCRYPT_DECRYPT".to_string(),
            origin: "KMS".to_string(),
            description: "Test key".to_string(),
        };
        
        let key_id = kms.generate_key(&key_spec).unwrap();
        let metadata = kms.get_key_metadata(&key_id).unwrap();
        
        assert_eq!(metadata.key_id, key_id);
        assert_eq!(metadata.algorithm, "AES-256");
        assert_eq!(metadata.origin, "KMS");
        assert_eq!(metadata.state, KeyState::Enabled);
    }

    #[test]
    fn test_hsm_key_generation() {
        let config = HsmConfig {
            vendor: "TestVendor".to_string(),
            model: "TestModel".to_string(),
            serial_number: "1234567890".to_string(),
            firmware_version: "1.0.0".to_string(),
            security_level: "FIPS 140-2 Level 3".to_string(),
        };
        
        let mut hsm = HsmManager::new(config);
        
        let key_spec = KeySpec {
            algorithm: "AES-256".to_string(),
            usage: "ENCRYPT_DECRYPT".to_string(),
            origin: "HSM".to_string(),
            description: "Test key".to_string(),
        };
        
        let key_id = hsm.generate_key_in_hsm(&key_spec).unwrap();
        assert!(!key_id.is_empty());
        assert!(key_id.starts_with("hsm-key-"));
        
        // Check that audit log was created
        assert_eq!(hsm.get_audit_logs().len(), 1);
        let log = &hsm.get_audit_logs()[0];
        assert_eq!(log.operation, "GenerateKeyInHsm");
        assert_eq!(log.key_id, key_id);
        assert!(log.success);
    }

    #[test]
    fn test_hsm_operations() {
        let config = HsmConfig {
            vendor: "TestVendor".to_string(),
            model: "TestModel".to_string(),
            serial_number: "1234567890".to_string(),
            firmware_version: "1.0.0".to_string(),
            security_level: "FIPS 140-2 Level 3".to_string(),
        };
        
        let mut hsm = HsmManager::new(config);
        
        let key_spec = KeySpec {
            algorithm: "AES-256".to_string(),
            usage: "ENCRYPT_DECRYPT".to_string(),
            origin: "HSM".to_string(),
            description: "Test key".to_string(),
        };
        
        let key_id = hsm.generate_key_in_hsm(&key_spec).unwrap();
        
        // Test encrypt operation
        let operation = HsmOperation {
            op_type: "encrypt".to_string(),
            key_id: key_id.clone(),
            input: b"Hello, World!".to_vec(),
            parameters: HashMap::new(),
        };
        
        let result = hsm.perform_operation(operation).unwrap();
        assert!(!result.is_empty());
        assert_ne!(result, b"Hello, World!");
        
        // Test decrypt operation
        let operation = HsmOperation {
            op_type: "decrypt".to_string(),
            key_id: key_id.clone(),
            input: result,
            parameters: HashMap::new(),
        };
        
        let result = hsm.perform_operation(operation).unwrap();
        assert_eq!(result, b"Hello, World!");
    }

    #[test]
    fn test_hsm_status() {
        let config = HsmConfig {
            vendor: "TestVendor".to_string(),
            model: "TestModel".to_string(),
            serial_number: "1234567890".to_string(),
            firmware_version: "1.0.0".to_string(),
            security_level: "FIPS 140-2 Level 3".to_string(),
        };
        
        let hsm = HsmManager::new(config);
        let status = hsm.get_status();
        
        assert!(status.online);
        assert_eq!(status.health, "OK");
        assert_eq!(status.active_sessions, 0);
    }

    #[test]
    fn test_mpc_key_generation() {
        let mut mpc = MpcManager::new();
        
        let participants = vec![
            "participant-1".to_string(),
            "participant-2".to_string(),
            "participant-3".to_string(),
        ];
        
        let key_id = mpc.generate_distributed_key(&participants, 2).unwrap();
        assert!(!key_id.is_empty());
        assert!(key_id.starts_with("mpc-key-"));
        
        // Check that audit log was created
        assert_eq!(mpc.get_audit_logs().len(), 1);
        let log = &mpc.get_audit_logs()[0];
        assert_eq!(log.operation, "GenerateDistributedKey");
        assert_eq!(log.key_id, key_id);
        assert_eq!(log.participants, participants);
        assert!(log.success);
    }

    #[test]
    fn test_mpc_distributed_signing() {
        let mut mpc = MpcManager::new();
        
        let participants = vec![
            "participant-1".to_string(),
            "participant-2".to_string(),
            "participant-3".to_string(),
        ];
        
        let key_id = mpc.generate_distributed_key(&participants, 2).unwrap();
        let data = b"Hello, World!";
        
        let signing_participants = vec![
            "participant-1".to_string(),
            "participant-2".to_string(),
        ];
        
        let signature = mpc.distributed_sign(&key_id, data, &signing_participants).unwrap();
        assert_eq!(signature.len(), 64);
        
        let verified = mpc.verify_distributed_signature(&key_id, data, &signature).unwrap();
        assert!(verified);
    }

    #[test]
    fn test_mpc_participant_management() {
        let mut mpc = MpcManager::new();
        
        let participant = MpcParticipant {
            id: "participant-1".to_string(),
            public_key: "test-public-key".to_string(),
            endpoint: "http://localhost:8080".to_string(),
            status: ParticipantStatus::Active,
            last_seen: 1234567890,
        };
        
        // Add participant
        assert!(mpc.add_participant(&participant).is_ok());
        assert_eq!(mpc.participants.len(), 1);
        
        // Check that audit log was created
        assert_eq!(mpc.get_audit_logs().len(), 1);
        let log = &mpc.get_audit_logs()[0];
        assert_eq!(log.operation, "AddParticipant");
        assert_eq!(log.participants, vec!["participant-1"]);
        assert!(log.success);
        
        // Remove participant
        assert!(mpc.remove_participant("participant-1").is_ok());
        assert_eq!(mpc.participants.len(), 0);
        
        // Check that audit log was created
        assert_eq!(mpc.get_audit_logs().len(), 2);
        let log = &mpc.get_audit_logs()[1];
        assert_eq!(log.operation, "RemoveParticipant");
        assert_eq!(log.participants, vec!["participant-1"]);
        assert!(log.success);
    }
}