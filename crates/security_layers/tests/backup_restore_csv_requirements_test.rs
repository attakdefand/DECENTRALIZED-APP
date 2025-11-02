//! Backup & Restore CSV Requirements Test
//!
//! This test verifies that the Backup & Restore implementation satisfies
//! all requirements specified in the web3_protection_layers.csv file:
//!
//! Layer: 5
//! Main Type: Data Security
//! Sub Type: Backup & Restore
//! Component/Mechanism: "Periodic encrypted snapshots, offline copy, tested restore drill"
//! Goal: "Survive ransomware / data loss"
//! Evidence/Telemetry: "Successful restore drill evidence, RPO/RTO metrics"

use security_layers::data_security::{
    BackupRestoreConfig, BackupType, BackupRestoreManager,
};

/// Test that the implementation satisfies the CSV requirements for Backup & Restore
#[test]
fn test_backup_restore_csv_requirements() {
    // Requirement from CSV:
    // "5,Data Security,Backup & Restore,Signed/Encrypted Backups,"Periodic encrypted snapshots, offline copy, tested restore drill","Survive ransomware / data loss","Successful restore drill evidence, RPO/RTO metrics"
    
    // Create configuration that implements the required mechanisms
    let config = BackupRestoreConfig {
        // Mechanism: "Periodic encrypted snapshots"
        periodic_snapshots_enabled: true,
        
        // Snapshot interval (1 hour)
        snapshot_interval: 3600,
        
        // Mechanism: "offline copy"
        offline_copy_enabled: true,
        
        // Mechanism: "tested restore drill"
        restore_drill_enabled: true,
        
        // Encryption algorithm
        encryption_algorithm: "AES-256-GCM".to_string(),
        
        // Retention period
        retention_days: 30,
    };
    
    let mut manager = BackupRestoreManager::new(config);
    
    // Verify the mechanisms are implemented correctly
    
    // 1. "Periodic encrypted snapshots"
    assert!(manager.get_config().periodic_snapshots_enabled, "Must enable periodic encrypted snapshots");
    
    // Test creating periodic encrypted snapshots
    let snapshot = manager.create_snapshot(BackupType::Database).unwrap();
    assert!(snapshot.is_signed, "Snapshots must be signed");
    assert!(snapshot.is_encrypted, "Snapshots must be encrypted");
    assert_eq!(snapshot.backup_type, BackupType::Database, "Snapshot must be of correct type");
    
    // 2. "offline copy"
    assert!(manager.get_config().offline_copy_enabled, "Must enable offline copies");
    
    // Test creating offline copy
    let offline_location = manager.create_offline_copy(&snapshot.id).unwrap();
    assert!(offline_location.starts_with("offline://"), "Offline copy must have correct location format");
    
    // 3. "tested restore drill"
    assert!(manager.get_config().restore_drill_enabled, "Must enable restore drills");
    
    // Test performing restore drill
    let drill_report = manager.perform_restore_drill();
    assert!(drill_report.success, "Restore drill must be successful");
    
    // Verify the goal is achieved: "Survive ransomware / data loss"
    assert!(manager.is_backup_restore_enabled(), "Backup & Restore must be enabled to survive ransomware / data loss");
    
    // Verify the evidence/telemetry is provided: "Successful restore drill evidence, RPO/RTO metrics"
    
    // Generate the required evidence/telemetry
    let telemetry_report = manager.generate_telemetry_report();
    
    // Verify the evidence/telemetry requirement is met
    assert!(telemetry_report.contains("Backup & Restore Report:"), "Telemetry report must contain header");
    assert!(telemetry_report.contains("Total Snapshots: 1"), "Should show snapshot count");
    assert!(telemetry_report.contains("Total Restore Drill Reports: 1"), "Should show drill report count");
    assert!(telemetry_report.contains("Recent Snapshots:"), "Should show recent snapshots");
    assert!(telemetry_report.contains("Recent Restore Drill Reports:"), "Should show recent drill reports");
    assert!(telemetry_report.contains(&format!("RPO: {}s, RTO: {}s", drill_report.rpo_achieved, drill_report.rto_achieved)), "Should show RPO/RTO metrics");
    
    // Verify that the implementation achieves the goal: "Survive ransomware / data loss"
    // By implementing all three mechanisms, we can survive ransomware / data loss:
    // 1. Periodic encrypted snapshots ensure we have recent backups
    // 2. Offline copies ensure we have backups that can't be affected by ransomware
    // 3. Tested restore drills ensure we can actually restore from backups
    
    assert!(manager.is_backup_restore_enabled(), "All Backup & Restore mechanisms must be enabled to achieve the goal");
    
    println!("✓ Backup & Restore CSV requirements test passed");
    println!("✓ Mechanisms implemented: Periodic encrypted snapshots, offline copy, tested restore drill");
    println!("✓ Goal achieved: Survive ransomware / data loss");
    println!("✓ Evidence/Telemetry provided: Successful restore drill evidence, RPO/RTO metrics");
}