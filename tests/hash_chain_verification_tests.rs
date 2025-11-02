//! Tests for the hash chain verification functionality
//!
//! These tests validate the tamper-evidence capabilities of the hash chain implementation
//! used for audit trail integrity verification.

use core::hash_chain::{HashChain, HashChainError};

/// Test basic hash chain functionality
#[test]
fn test_hash_chain_basic_functionality() {
    let mut chain = HashChain::new();
    
    // Create genesis entry
    let genesis_result = chain.create_genesis_entry("Initial system state".to_string());
    assert!(genesis_result.is_ok());
    assert_eq!(chain.get_entries().len(), 1);
    
    // Add entries
    let entry1_result = chain.add_entry("config-change-001".to_string(), "Changed config parameter X".to_string());
    assert!(entry1_result.is_ok());
    
    let entry2_result = chain.add_entry("user-login-001".to_string(), "User admin logged in".to_string());
    assert!(entry2_result.is_ok());
    
    assert_eq!(chain.get_entries().len(), 3);
    
    // Verify chain integrity
    let verify_result = chain.verify_chain();
    assert!(verify_result.is_ok());
    assert!(verify_result.unwrap());
    
    println!("✅ Basic hash chain functionality test passed");
}

/// Test hash chain tamper detection
#[test]
fn test_hash_chain_tamper_detection() {
    let mut chain = HashChain::new();
    
    // Create genesis entry
    chain.create_genesis_entry("Initial system state".to_string()).unwrap();
    
    // Add entries
    chain.add_entry("config-change-001".to_string(), "Changed config parameter X".to_string()).unwrap();
    chain.add_entry("user-login-001".to_string(), "User admin logged in".to_string()).unwrap();
    
    // Verify chain is valid initially
    assert!(chain.verify_chain().unwrap());
    
    // Tamper with an entry
    let entry = chain.get_entries().get_mut(1).unwrap();
    entry.data = "Tampered data".to_string();
    
    // Verify tampering is detected
    let verify_result = chain.verify_chain();
    assert!(verify_result.is_err());
    
    match verify_result.unwrap_err() {
        HashChainError::IntegrityError(_) => {
            println!("✅ Hash chain tamper detection test passed");
        },
        _ => panic!("Expected IntegrityError"),
    }
}

/// Test individual entry verification
#[test]
fn test_individual_entry_verification() {
    let mut chain = HashChain::new();
    
    // Create genesis entry
    chain.create_genesis_entry("Initial system state".to_string()).unwrap();
    
    // Add entries
    chain.add_entry("config-change-001".to_string(), "Changed config parameter X".to_string()).unwrap();
    chain.add_entry("user-login-001".to_string(), "User admin logged in".to_string()).unwrap();
    
    // Verify individual entries
    assert!(chain.verify_entry("genesis").unwrap());
    assert!(chain.verify_entry("config-change-001").unwrap());
    assert!(chain.verify_entry("user-login-001").unwrap());
    
    // Try to verify non-existent entry
    let verify_result = chain.verify_entry("non-existent");
    assert!(verify_result.is_err());
    
    match verify_result.unwrap_err() {
        HashChainError::EntryNotFound(_) => {
            println!("✅ Individual entry verification test passed");
        },
        _ => panic!("Expected EntryNotFound error"),
    }
}

/// Test hash chain with many entries
#[test]
fn test_hash_chain_with_many_entries() {
    let mut chain = HashChain::new();
    
    // Create genesis entry
    chain.create_genesis_entry("Genesis entry".to_string()).unwrap();
    
    // Add many entries
    for i in 0..100 {
        let entry_id = format!("entry-{:03}", i);
        let entry_data = format!("Audit log entry #{}", i);
        chain.add_entry(entry_id, entry_data).unwrap();
    }
    
    assert_eq!(chain.get_entries().len(), 101);
    
    // Verify entire chain
    assert!(chain.verify_chain().unwrap());
    
    println!("✅ Hash chain with many entries test passed");
}

/// Test hash chain edge cases
#[test]
fn test_hash_chain_edge_cases() {
    let mut chain = HashChain::new();
    
    // Try to add entry to empty chain
    let result = chain.add_entry("entry-1".to_string(), "Some data".to_string());
    assert!(result.is_err());
    
    match result.unwrap_err() {
        HashChainError::ValidationError(_) => {
            println!("✅ Empty chain validation test passed");
        },
        _ => panic!("Expected ValidationError"),
    }
    
    // Create genesis and verify empty chain validation
    let result = chain.verify_chain();
    assert!(result.is_err());
    
    match result.unwrap_err() {
        HashChainError::ValidationError(_) => {
            println!("✅ Empty chain verification test passed");
        },
        _ => panic!("Expected ValidationError"),
    }
    
    // Create genesis entry
    chain.create_genesis_entry("Genesis".to_string()).unwrap();
    
    // Verify genesis entry
    assert!(chain.verify_entry("genesis").unwrap());
    
    println!("✅ Hash chain edge cases test passed");
}