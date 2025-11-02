//! Tamper Evidence Tests
//!
//! These tests validate the tamper-evidence capabilities of the audit trail system
//! using hash chain verification to detect any modifications to audit logs.

use core::hash_chain::{HashChain, HashChainEntry};
use core::observability::ObservabilityManager;
use std::collections::HashMap;

/// Test tamper evidence with audit logs
#[test]
fn test_tamper_evidence_with_audit_logs() {
    let mut manager = ObservabilityManager::new();
    let mut hash_chain = HashChain::new();
    
    // Create genesis entry for the hash chain
    hash_chain.create_genesis_entry("Audit trail genesis".to_string()).unwrap();
    
    // Simulate audit log entries
    let audit_entries = vec![
        ("user-login-admin", "User admin logged in from 192.168.1.100"),
        ("config-change-001", "Changed database connection timeout to 30s"),
        ("user-action-001", "User admin created new API key"),
        ("system-event-001", "Scheduled backup completed successfully"),
        ("user-logout-admin", "User admin logged out"),
    ];
    
    // Add audit entries to both the observability manager and hash chain
    for (id, description) in &audit_entries {
        // Add to observability manager
        let mut metadata = HashMap::new();
        metadata.insert("description".to_string(), description.to_string());
        
        manager.log_admin_action(
            "system".to_string(),
            "audit_entry".to_string(),
            id.to_string(),
            metadata,
            Some("127.0.0.1".to_string()),
        ).unwrap();
        
        // Add to hash chain
        hash_chain.add_entry(id.to_string(), description.to_string()).unwrap();
    }
    
    // Verify hash chain integrity
    assert!(hash_chain.verify_chain().unwrap());
    
    // Verify individual entries
    for (id, _) in &audit_entries {
        assert!(hash_chain.verify_entry(id).unwrap());
    }
    
    println!("✅ Tamper evidence with audit logs test passed");
}

/// Test tamper detection in audit trail
#[test]
fn test_tamper_detection_in_audit_trail() {
    let mut hash_chain = HashChain::new();
    
    // Create genesis entry
    hash_chain.create_genesis_entry("Security audit trail start".to_string()).unwrap();
    
    // Add legitimate entries
    hash_chain.add_entry("login-001".to_string(), "User john_doe logged in".to_string()).unwrap();
    hash_chain.add_entry("file-access-001".to_string(), "User john_doe accessed confidential_file.pdf".to_string()).unwrap();
    hash_chain.add_entry("file-modify-001".to_string(), "User john_doe modified confidential_file.pdf".to_string()).unwrap();
    hash_chain.add_entry("logout-001".to_string(), "User john_doe logged out".to_string()).unwrap();
    
    // Verify chain integrity before tampering
    assert!(hash_chain.verify_chain().unwrap());
    
    // Simulate tampering by modifying an entry
    let entry = hash_chain.get_entries().get_mut(2).unwrap(); // Modify the file access entry
    entry.data = "User admin accessed confidential_file.pdf".to_string(); // Changed user
    
    // Verify tampering is detected
    let verify_result = hash_chain.verify_chain();
    assert!(verify_result.is_err());
    
    println!("✅ Tamper detection in audit trail test passed");
}

/// Test tamper evidence with large audit trails
#[test]
fn test_tamper_evidence_with_large_audit_trails() {
    let mut hash_chain = HashChain::new();
    
    // Create genesis entry
    hash_chain.create_genesis_entry("Large audit trail genesis".to_string()).unwrap();
    
    // Add many entries to simulate a large audit trail
    for i in 0..1000 {
        let entry_id = format!("audit-{:04}", i);
        let entry_data = format!("Audit event #{} at timestamp {}", i, 1000000 + i);
        hash_chain.add_entry(entry_id, entry_data).unwrap();
    }
    
    // Verify the entire chain
    assert!(hash_chain.verify_chain().unwrap());
    assert_eq!(hash_chain.get_entries().len(), 1001); // Genesis + 1000 entries
    
    // Verify random entries
    assert!(hash_chain.verify_entry("audit-0000").unwrap());
    assert!(hash_chain.verify_entry("audit-0500").unwrap());
    assert!(hash_chain.verify_entry("audit-0999").unwrap());
    
    println!("✅ Tamper evidence with large audit trails test passed");
}

/// Test tamper evidence performance
#[test]
fn test_tamper_evidence_performance() {
    let mut hash_chain = HashChain::new();
    
    // Create genesis entry
    hash_chain.create_genesis_entry("Performance test genesis".to_string()).unwrap();
    
    // Measure time to add entries
    let start_time = std::time::Instant::now();
    
    // Add entries
    for i in 0..100 {
        let entry_id = format!("perf-{}", i);
        let entry_data = format!("Performance test entry #{}", i);
        hash_chain.add_entry(entry_id, entry_data).unwrap();
    }
    
    let add_duration = start_time.elapsed();
    
    // Measure time to verify chain
    let verify_start = std::time::Instant::now();
    assert!(hash_chain.verify_chain().unwrap());
    let verify_duration = verify_start.elapsed();
    
    // Measure time to verify individual entries
    let individual_start = std::time::Instant::now();
    for i in 0..100 {
        let entry_id = format!("perf-{}", i);
        assert!(hash_chain.verify_entry(&entry_id).unwrap());
    }
    let individual_duration = individual_start.elapsed();
    
    println!("Performance results:");
    println!("  - Adding 100 entries: {:?}", add_duration);
    println!("  - Verifying chain: {:?}", verify_duration);
    println!("  - Verifying 100 individual entries: {:?}", individual_duration);
    
    // Ensure reasonable performance (less than 1 second for all operations)
    assert!(add_duration.as_millis() < 1000);
    assert!(verify_duration.as_millis() < 1000);
    assert!(individual_duration.as_millis() < 1000);
    
    println!("✅ Tamper evidence performance test passed");
}

/// Test tamper evidence with complex data structures
#[test]
fn test_tamper_evidence_with_complex_data() {
    let mut hash_chain = HashChain::new();
    
    // Create genesis entry
    hash_chain.create_genesis_entry("Complex data genesis".to_string()).unwrap();
    
    // Create complex audit entries with JSON-like data
    let complex_entries = vec![
        (
            "user-session-001",
            r#"{"user_id": "user-123", "session_id": "sess-456", "login_time": "2023-01-01T10:00:00Z", "ip_address": "192.168.1.100", "user_agent": "Mozilla/5.0"}"#
        ),
        (
            "transaction-001",
            r#"{"transaction_id": "tx-789", "from": "addr-123", "to": "addr-456", "amount": "1.5", "currency": "ETH", "timestamp": "2023-01-01T10:05:00Z", "gas_used": "21000"}"#
        ),
        (
            "contract-deploy-001",
            r#"{"contract_address": "0x1234...", "deployer": "addr-789", "bytecode_hash": "0xabcd...", "gas_used": "1500000", "timestamp": "2023-01-01T10:10:00Z"}"#
        ),
    ];
    
    // Add complex entries
    for (id, data) in &complex_entries {
        hash_chain.add_entry(id.to_string(), data.to_string()).unwrap();
    }
    
    // Verify chain integrity
    assert!(hash_chain.verify_chain().unwrap());
    
    // Verify individual complex entries
    for (id, _) in &complex_entries {
        assert!(hash_chain.verify_entry(id).unwrap());
    }
    
    println!("✅ Tamper evidence with complex data test passed");
}

/// Test tamper evidence recovery after detection
#[test]
fn test_tamper_evidence_recovery() {
    let mut hash_chain = HashChain::new();
    
    // Create genesis entry
    hash_chain.create_genesis_entry("Recovery test genesis".to_string()).unwrap();
    
    // Add entries
    hash_chain.add_entry("event-001".to_string(), "System event 1".to_string()).unwrap();
    hash_chain.add_entry("event-002".to_string(), "System event 2".to_string()).unwrap();
    hash_chain.add_entry("event-003".to_string(), "System event 3".to_string()).unwrap();
    
    // Verify initial integrity
    assert!(hash_chain.verify_chain().unwrap());
    
    // Simulate tampering
    let original_data = hash_chain.get_entries()[2].data.clone();
    let entry = hash_chain.get_entries().get_mut(2).unwrap();
    entry.data = "Tampered data".to_string();
    
    // Verify tampering is detected
    assert!(hash_chain.verify_chain().is_err());
    
    // Simulate recovery by restoring original data
    let entry = hash_chain.get_entries().get_mut(2).unwrap();
    entry.data = original_data;
    
    // Verify integrity is restored
    assert!(hash_chain.verify_chain().unwrap());
    
    println!("✅ Tamper evidence recovery test passed");
}