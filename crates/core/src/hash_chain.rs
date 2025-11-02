//! Hash Chain Verification Module
//!
//! This module implements hash chain verification for tamper-evidence in audit trails.
//! It provides functionality to create and verify hash chains that can detect
//! any modification to the audit log entries.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents an entry in a hash chain
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HashChainEntry {
    /// Unique identifier for the entry
    pub id: String,
    /// Data that is hashed
    pub data: String,
    /// Hash of this entry's data
    pub data_hash: String,
    /// Hash of the previous entry in the chain
    pub previous_hash: String,
    /// Combined hash of data_hash and previous_hash
    pub entry_hash: String,
    /// Timestamp of the entry
    pub timestamp: u64,
}

/// Error types for hash chain operations
#[derive(Debug)]
pub enum HashChainError {
    /// Validation error
    ValidationError(String),
    /// Chain integrity error
    IntegrityError(String),
    /// Entry not found error
    EntryNotFound(String),
}

impl fmt::Display for HashChainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashChainError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            HashChainError::IntegrityError(msg) => write!(f, "Integrity error: {}", msg),
            HashChainError::EntryNotFound(msg) => write!(f, "Entry not found: {}", msg),
        }
    }
}

impl std::error::Error for HashChainError {}

/// Manages a hash chain for tamper-evidence
pub struct HashChain {
    /// Chain entries
    entries: Vec<HashChainEntry>,
}

impl HashChain {
    /// Creates a new hash chain
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Creates the genesis entry in the hash chain
    pub fn create_genesis_entry(&mut self, data: String) -> Result<String, HashChainError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| HashChainError::ValidationError(format!("Time error: {}", e)))?
            .as_secs();

        let data_hash = Self::hash_data(&data);
        let previous_hash = "0".repeat(64); // Genesis entry has no previous hash
        let entry_hash = Self::hash_combined(&data_hash, &previous_hash);

        let entry = HashChainEntry {
            id: "genesis".to_string(),
            data,
            data_hash,
            previous_hash: previous_hash.clone(),
            entry_hash: entry_hash.clone(),
            timestamp,
        };

        self.entries.push(entry);
        Ok(entry_hash)
    }

    /// Adds a new entry to the hash chain
    pub fn add_entry(&mut self, id: String, data: String) -> Result<String, HashChainError> {
        if self.entries.is_empty() {
            return Err(HashChainError::ValidationError(
                "Cannot add entry to empty chain. Create genesis entry first.".to_string(),
            ));
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| HashChainError::ValidationError(format!("Time error: {}", e)))?
            .as_secs();

        let data_hash = Self::hash_data(&data);
        let previous_hash = self.entries.last().unwrap().entry_hash.clone();
        let entry_hash = Self::hash_combined(&data_hash, &previous_hash);

        let entry = HashChainEntry {
            id,
            data,
            data_hash,
            previous_hash,
            entry_hash: entry_hash.clone(),
            timestamp,
        };

        self.entries.push(entry);
        Ok(entry_hash)
    }

    /// Verifies the integrity of the entire hash chain
    pub fn verify_chain(&self) -> Result<bool, HashChainError> {
        if self.entries.is_empty() {
            return Err(HashChainError::ValidationError(
                "Cannot verify empty chain".to_string(),
            ));
        }

        // Verify genesis entry
        let genesis = &self.entries[0];
        let expected_genesis_hash = Self::hash_combined(&genesis.data_hash, &genesis.previous_hash);
        if genesis.entry_hash != expected_genesis_hash {
            return Err(HashChainError::IntegrityError(
                "Genesis entry hash mismatch".to_string(),
            ));
        }

        // Verify all subsequent entries
        for i in 1..self.entries.len() {
            let current = &self.entries[i];
            let previous = &self.entries[i - 1];

            // Check that previous_hash matches the previous entry's entry_hash
            if current.previous_hash != previous.entry_hash {
                return Err(HashChainError::IntegrityError(format!(
                    "Hash chain broken at entry {}",
                    current.id
                )));
            }

            // Check that entry_hash is correctly calculated
            let expected_hash = Self::hash_combined(&current.data_hash, &current.previous_hash);
            if current.entry_hash != expected_hash {
                return Err(HashChainError::IntegrityError(format!(
                    "Entry hash mismatch at entry {}",
                    current.id
                )));
            }
        }

        Ok(true)
    }

    /// Verifies a specific entry in the chain
    pub fn verify_entry(&self, entry_id: &str) -> Result<bool, HashChainError> {
        let entry_index = self
            .entries
            .iter()
            .position(|e| e.id == entry_id)
            .ok_or_else(|| HashChainError::EntryNotFound(entry_id.to_string()))?;

        let entry = &self.entries[entry_index];

        // Check that entry_hash is correctly calculated
        let expected_hash = Self::hash_combined(&entry.data_hash, &entry.previous_hash);
        if entry.entry_hash != expected_hash {
            return Err(HashChainError::IntegrityError(format!(
                "Entry hash mismatch for entry {}",
                entry_id
            )));
        }

        // If not the genesis entry, check linkage to previous entry
        if entry_index > 0 {
            let previous_entry = &self.entries[entry_index - 1];
            if entry.previous_hash != previous_entry.entry_hash {
                return Err(HashChainError::IntegrityError(format!(
                    "Link to previous entry broken for entry {}",
                    entry_id
                )));
            }
        }

        Ok(true)
    }

    /// Gets an entry by ID
    pub fn get_entry(&self, entry_id: &str) -> Option<&HashChainEntry> {
        self.entries.iter().find(|e| e.id == entry_id)
    }

    /// Gets all entries
    pub fn get_entries(&self) -> &Vec<HashChainEntry> {
        &self.entries
    }

    /// Gets all entries as mutable
    pub fn get_entries_mut(&mut self) -> &mut Vec<HashChainEntry> {
        &mut self.entries
    }

    /// Hashes data using SHA-256
    fn hash_data(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Hashes two strings combined
    fn hash_combined(first: &str, second: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(first.as_bytes());
        hasher.update(second.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_chain_creation() {
        let mut chain = HashChain::new();
        assert_eq!(chain.get_entries().len(), 0);
    }

    #[test]
    fn test_genesis_entry() {
        let mut chain = HashChain::new();
        let result = chain.create_genesis_entry("genesis data".to_string());
        assert!(result.is_ok());
        assert_eq!(chain.get_entries().len(), 1);
        
        let genesis = chain.get_entries().first().unwrap();
        assert_eq!(genesis.id, "genesis");
        assert_eq!(genesis.data, "genesis data");
        assert_eq!(genesis.previous_hash, "0".repeat(64));
    }

    #[test]
    fn test_add_entry() {
        let mut chain = HashChain::new();
        chain.create_genesis_entry("genesis data".to_string()).unwrap();
        
        let result = chain.add_entry("entry-1".to_string(), "entry data".to_string());
        assert!(result.is_ok());
        assert_eq!(chain.get_entries().len(), 2);
        
        let entry = chain.get_entries().get(1).unwrap();
        assert_eq!(entry.id, "entry-1");
        assert_eq!(entry.data, "entry data");
        assert_eq!(entry.previous_hash, chain.get_entries().first().unwrap().entry_hash);
    }

    #[test]
    fn test_verify_chain() {
        let mut chain = HashChain::new();
        chain.create_genesis_entry("genesis data".to_string()).unwrap();
        chain.add_entry("entry-1".to_string(), "entry data 1".to_string()).unwrap();
        chain.add_entry("entry-2".to_string(), "entry data 2".to_string()).unwrap();
        
        let result = chain.verify_chain();
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_verify_entry() {
        let mut chain = HashChain::new();
        chain.create_genesis_entry("genesis data".to_string()).unwrap();
        chain.add_entry("entry-1".to_string(), "entry data 1".to_string()).unwrap();
        
        let result = chain.verify_entry("entry-1");
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_chain_integrity_detection() {
        let mut chain = HashChain::new();
        chain.create_genesis_entry("genesis data".to_string()).unwrap();
        chain.add_entry("entry-1".to_string(), "entry data 1".to_string()).unwrap();
        
        // Tamper with an entry (modify data_hash without updating entry_hash)
        let entries = chain.get_entries_mut();
        let entry = entries.get_mut(1).unwrap();
        entry.data_hash = "tampered_hash".to_string();
        // Note: We don't update entry.entry_hash, which should cause verification to fail
        
        let result = chain.verify_chain();
        assert!(result.is_err());
        match result.unwrap_err() {
            HashChainError::IntegrityError(_) => assert!(true),
            _ => assert!(false, "Expected IntegrityError"),
        }
    }

    #[test]
    fn test_entry_not_found() {
        let chain = HashChain::new();
        let result = chain.verify_entry("non-existent");
        assert!(result.is_err());
        match result.unwrap_err() {
            HashChainError::EntryNotFound(_) => assert!(true),
            _ => assert!(false, "Expected EntryNotFound"),
        }
    }
}