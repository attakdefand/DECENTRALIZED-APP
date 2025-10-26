//! Data Security Backup & Restore Validation Tests
//!
//! This module contains tests that validate the Backup & Restore functionality
//! as defined in the web3_protection_layers.csv file for Layer 5.

use security_layers::data_security::{
    BackupRestoreConfig, BackupSnapshot, RestoreDrillReport, BackupType, BackupStatus, 
    BackupRestoreManager,
};

/// Test BackupRestoreConfig creation and validation
#[test]
fn test_backup_restore_config_creation_and_validation() {
    let config = BackupRestoreConfig {
        periodic_snapshots_enabled: true,
        snapshot_interval: 3600, // 1 hour
        offline_copy_enabled: true,
        restore_drill_enabled: true,
        encryption_algorithm: "AES-256-GCM".to_string(),
        retention_days: 30,
    };
    
    assert!(config.periodic_snapshots_enabled);
    assert_eq!(config.snapshot_interval, 3600);
    assert!(config.offline_copy_enabled);
    assert!(config.restore_drill_enabled);
    assert_eq!(config.encryption_algorithm, "AES-256-GCM");
    assert_eq!(config.retention_days, 30);
}

/// Test BackupSnapshot functionality
#[test]
fn test_backup_snapshot() {
    let snapshot = BackupSnapshot {
        id: "test-snapshot-123".to_string(),
        created_at: 1234567890,
        location: "s3://backups/test-snapshot".to_string(),
        size: 1024000,
        is_signed: true,
        is_encrypted: true,
        backup_type: BackupType::Database,
        status: BackupStatus::Completed,
    };
    
    assert_eq!(snapshot.id, "test-snapshot-123");
    assert_eq!(snapshot.created_at, 1234567890);
    assert_eq!(snapshot.location, "s3://backups/test-snapshot");
    assert_eq!(snapshot.size, 1024000);
    assert!(snapshot.is_signed);
    assert!(snapshot.is_encrypted);
    assert_eq!(snapshot.backup_type, BackupType::Database);
    assert_eq!(snapshot.status, BackupStatus::Completed);
}

/// Test RestoreDrillReport functionality
#[test]
fn test_restore_drill_report() {
    let metrics = security_layers::data_security::RestoreMetrics {
        locate_time: 300,
        decrypt_time: 600,
        restore_time: 1800,
        total_time: 2700,
    };
    
    let report = RestoreDrillReport {
        timestamp: 1234567890,
        success: true,
        rpo_achieved: 1800,
        rto_achieved: 3600,
        error_message: None,
        metrics,
    };
    
    assert_eq!(report.timestamp, 1234567890);
    assert!(report.success);
    assert_eq!(report.rpo_achieved, 1800);
    assert_eq!(report.rto_achieved, 3600);
    assert_eq!(report.error_message, None);
    assert_eq!(report.metrics.locate_time, 300);
    assert_eq!(report.metrics.decrypt_time, 600);
    assert_eq!(report.metrics.restore_time, 1800);
    assert_eq!(report.metrics.total_time, 2700);
}

/// Test BackupRestoreManager functionality
#[test]
fn test_backup_restore_manager() {
    let config = BackupRestoreConfig {
        periodic_snapshots_enabled: true,
        snapshot_interval: 3600,
        offline_copy_enabled: true,
        restore_drill_enabled: true,
        encryption_algorithm: "AES-256-GCM".to_string(),
        retention_days: 30,
    };
    
    let mut manager = BackupRestoreManager::new(config.clone());
    
    // Test configuration access
    let manager_config = manager.get_config();
    assert_eq!(manager_config.snapshot_interval, 3600);
    
    // Test configuration update
    let new_config = BackupRestoreConfig {
        snapshot_interval: 7200, // 2 hours
        ..config.clone()
    };
    
    manager.update_config(new_config.clone());
    let updated_config = manager.get_config();
    assert_eq!(updated_config.snapshot_interval, 7200);
    
    // Test creating snapshots
    let snapshot = manager.create_snapshot(BackupType::Database).unwrap();
    assert!(snapshot.is_signed);
    assert!(snapshot.is_encrypted);
    assert_eq!(snapshot.backup_type, BackupType::Database);
    assert_eq!(snapshot.status, BackupStatus::Completed);
    
    // Test retrieving snapshots
    let snapshots = manager.get_snapshots();
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0].id, snapshot.id);
    
    // Test creating offline copy
    let offline_location = manager.create_offline_copy(&snapshot.id).unwrap();
    assert!(offline_location.starts_with("offline://"));
    
    // Test performing restore drill
    let drill_report = manager.perform_restore_drill();
    assert!(drill_report.timestamp > 0);
    assert!(drill_report.rpo_achieved >= 0);
    assert!(drill_report.rto_achieved >= 0);
    
    // Test retrieving drill reports
    let drill_reports = manager.get_drill_reports();
    assert_eq!(drill_reports.len(), 1);
    assert_eq!(drill_reports[0].timestamp, drill_report.timestamp);
    
    // Test telemetry report generation
    let telemetry_report = manager.generate_telemetry_report();
    assert!(telemetry_report.contains("Backup & Restore Report:"));
    assert!(telemetry_report.contains("Periodic snapshots enabled: true"));
    assert!(telemetry_report.contains("Offline copies enabled: true"));
    assert!(telemetry_report.contains("Restore drills enabled: true"));
    assert!(telemetry_report.contains("Total Snapshots: 1"));
    assert!(telemetry_report.contains("Total Restore Drill Reports: 1"));
    
    // Test backup restore enabled check
    assert!(manager.is_backup_restore_enabled());
    
    // Test with disabled features
    let disabled_config = BackupRestoreConfig {
        periodic_snapshots_enabled: false,
        offline_copy_enabled: false,
        restore_drill_enabled: false,
        ..config.clone()
    };
    
    let mut disabled_manager = BackupRestoreManager::new(disabled_config);
    assert!(!disabled_manager.is_backup_restore_enabled());
    
    // Test that creating snapshots fails when disabled
    let snapshot_result = disabled_manager.create_snapshot(BackupType::Database);
    assert!(snapshot_result.is_err());
    assert_eq!(snapshot_result.unwrap_err(), "Periodic snapshots are not enabled");
    
    // Test that creating offline copies fails when disabled
    let offline_result = disabled_manager.create_offline_copy("test-id");
    assert!(offline_result.is_err());
    assert_eq!(offline_result.unwrap_err(), "Offline copies are not enabled");
}

/// Test the specific requirement from the CSV: "Periodic encrypted snapshots, offline copy, tested restore drill"
#[test]
fn test_csv_requirement_mechanisms() {
    let config = BackupRestoreConfig {
        periodic_snapshots_enabled: true, // Periodic encrypted snapshots
        snapshot_interval: 3600,
        offline_copy_enabled: true, // Offline copy
        restore_drill_enabled: true, // Tested restore drill
        encryption_algorithm: "AES-256-GCM".to_string(), // Encryption
        retention_days: 30,
    };
    
    let manager = BackupRestoreManager::new(config);
    
    // Verify the configuration meets the requirements
    let config = manager.get_config();
    assert!(config.periodic_snapshots_enabled); // Periodic encrypted snapshots
    assert!(config.offline_copy_enabled); // Offline copy
    assert!(config.restore_drill_enabled); // Tested restore drill
    
    // Verify backup restore is enabled
    assert!(manager.is_backup_restore_enabled());
}

/// Test the specific requirement from the CSV: "Successful restore drill evidence, RPO/RTO metrics"
#[test]
fn test_csv_requirement_telemetry() {
    let config = BackupRestoreConfig {
        periodic_snapshots_enabled: true,
        snapshot_interval: 3600,
        offline_copy_enabled: true,
        restore_drill_enabled: true,
        encryption_algorithm: "AES-256-GCM".to_string(),
        retention_days: 30,
    };
    
    let mut manager = BackupRestoreManager::new(config);
    
    // Perform a restore drill to generate the required evidence/telemetry
    let report = manager.perform_restore_drill();
    
    // Verify the evidence/telemetry requirement is met
    assert!(report.success);
    assert!(report.rpo_achieved >= 0); // RPO metrics
    assert!(report.rto_achieved >= 0); // RTO metrics
    
    // Generate the required evidence/telemetry: "Successful restore drill evidence, RPO/RTO metrics"
    let telemetry_report = manager.generate_telemetry_report();
    
    // Verify the evidence/telemetry requirement is met
    assert!(telemetry_report.contains("Backup & Restore Report:"));
    assert!(telemetry_report.contains("Total Restore Drill Reports: 1"));
    assert!(telemetry_report.contains("Recent Restore Drill Reports:"));
    assert!(telemetry_report.contains(&format!("RPO: {}s, RTO: {}s", report.rpo_achieved, report.rto_achieved)));
}

/// Integration test showing how the Backup & Restore system works
#[test]
fn test_backup_restore_integration() {
    // Create a Backup & Restore configuration for a DEX application
    let config = BackupRestoreConfig {
        periodic_snapshots_enabled: true,
        snapshot_interval: 3600, // 1 hour
        offline_copy_enabled: true,
        restore_drill_enabled: true,
        encryption_algorithm: "AES-256-GCM".to_string(),
        retention_days: 30,
    };
    
    let mut manager = BackupRestoreManager::new(config);
    
    // Simulate creating periodic encrypted snapshots
    let db_snapshot = manager.create_snapshot(BackupType::Database).unwrap();
    let config_snapshot = manager.create_snapshot(BackupType::Configuration).unwrap();
    let state_snapshot = manager.create_snapshot(BackupType::State).unwrap();
    
    // Verify snapshots are encrypted and signed
    assert!(db_snapshot.is_signed);
    assert!(db_snapshot.is_encrypted);
    assert_eq!(db_snapshot.backup_type, BackupType::Database);
    
    assert!(config_snapshot.is_signed);
    assert!(config_snapshot.is_encrypted);
    assert_eq!(config_snapshot.backup_type, BackupType::Configuration);
    
    assert!(state_snapshot.is_signed);
    assert!(state_snapshot.is_encrypted);
    assert_eq!(state_snapshot.backup_type, BackupType::State);
    
    // Verify we have 3 snapshots
    let snapshots = manager.get_snapshots();
    assert_eq!(snapshots.len(), 3);
    
    // Simulate creating offline copies
    let db_offline = manager.create_offline_copy(&db_snapshot.id).unwrap();
    let config_offline = manager.create_offline_copy(&config_snapshot.id).unwrap();
    
    // Verify offline copies are created
    assert!(db_offline.starts_with("offline://"));
    assert!(config_offline.starts_with("offline://"));
    
    // Simulate performing restore drills
    let drill_report1 = manager.perform_restore_drill();
    let drill_report2 = manager.perform_restore_drill();
    
    // Verify drill reports
    assert!(drill_report1.success);
    assert!(drill_report1.rpo_achieved >= 0);
    assert!(drill_report1.rto_achieved >= 0);
    
    assert!(drill_report2.success);
    assert!(drill_report2.rpo_achieved >= 0);
    assert!(drill_report2.rto_achieved >= 0);
    
    // Verify we have 2 drill reports
    let drill_reports = manager.get_drill_reports();
    assert_eq!(drill_reports.len(), 2);
    
    // Verify the Backup & Restore configuration meets security requirements
    assert!(manager.is_backup_restore_enabled());
    
    // Generate the required evidence/telemetry
    let telemetry_report = manager.generate_telemetry_report();
    println!("Telemetry Report:\n{}", telemetry_report);
    
    // Verify that we have the required evidence
    assert!(telemetry_report.contains("Backup & Restore Report:"));
    assert!(telemetry_report.contains("Periodic snapshots enabled: true"));
    assert!(telemetry_report.contains("Offline copies enabled: true"));
    assert!(telemetry_report.contains("Restore drills enabled: true"));
    assert!(telemetry_report.contains("Total Snapshots: 3"));
    assert!(telemetry_report.contains("Total Restore Drill Reports: 2"));
    assert!(telemetry_report.contains("Recent Snapshots:"));
    assert!(telemetry_report.contains("Recent Restore Drill Reports:"));
    
    // Verify the goal: "Survive ransomware / data loss"
    // By having periodic encrypted snapshots, offline copies, and tested restore drills,
    // we can survive ransomware / data loss
    assert!(manager.is_backup_restore_enabled());
}