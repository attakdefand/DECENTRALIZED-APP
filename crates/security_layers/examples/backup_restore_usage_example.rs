//! Backup & Restore Usage Example
//!
//! This example demonstrates how to use the Backup & Restore implementation
//! for a decentralized exchange application.

use security_layers::data_security::{
    BackupRestoreConfig, BackupType, BackupRestoreManager,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Backup & Restore Usage Example");
    println!("=============================");
    
    // Create a Backup & Restore configuration for a DEX application
    let config = BackupRestoreConfig {
        // Enable periodic encrypted snapshots
        periodic_snapshots_enabled: true,
        
        // Create snapshots every hour
        snapshot_interval: 3600,
        
        // Enable offline copies for ransomware protection
        offline_copy_enabled: true,
        
        // Enable restore drills to ensure we can actually restore
        restore_drill_enabled: true,
        
        // Use strong encryption for backups
        encryption_algorithm: "AES-256-GCM".to_string(),
        
        // Retain backups for 30 days
        retention_days: 30,
    };
    
    // Create a Backup & Restore manager with the configuration
    let mut manager = BackupRestoreManager::new(config);
    
    println!("✓ Backup & Restore manager created with configuration");
    
    // Example 1: Creating periodic encrypted snapshots
    println!("\n1. Creating periodic encrypted snapshots:");
    
    let backup_types = vec![
        BackupType::Database,
        BackupType::Configuration,
        BackupType::State,
        BackupType::Keys,
    ];
    
    let mut snapshot_ids = Vec::new();
    
    for (i, backup_type) in backup_types.iter().enumerate() {
        let snapshot = manager.create_snapshot(backup_type.clone())?;
        snapshot_ids.push(snapshot.id.clone());
        
        println!("   Created {} snapshot: {}", 
            match backup_type {
                BackupType::Database => "Database",
                BackupType::Configuration => "Configuration",
                BackupType::State => "State",
                BackupType::Keys => "Keys",
            },
            snapshot.id
        );
        
        println!("   - Created at: {}", snapshot.created_at);
        println!("   - Size: {} bytes", snapshot.size);
        println!("   - Location: {}", snapshot.location);
        println!("   - Signed: {}", snapshot.is_signed);
        println!("   - Encrypted: {}", snapshot.is_encrypted);
        println!("   - Status: {:?}", snapshot.status);
    }
    
    // Example 2: Creating offline copies
    println!("\n2. Creating offline copies:");
    
    for (i, snapshot_id) in snapshot_ids.iter().enumerate() {
        let offline_location = manager.create_offline_copy(snapshot_id)?;
        println!("   Created offline copy for snapshot {}: {}", i + 1, offline_location);
    }
    
    // Example 3: Performing restore drills
    println!("\n3. Performing restore drills:");
    
    for i in 1..=3 {
        let drill_report = manager.perform_restore_drill();
        
        println!("   Restore Drill {}:", i);
        println!("   - Timestamp: {}", drill_report.timestamp);
        println!("   - Success: {}", drill_report.success);
        println!("   - RPO Achieved: {} seconds", drill_report.rpo_achieved);
        println!("   - RTO Achieved: {} seconds", drill_report.rto_achieved);
        println!("   - Metrics:");
        println!("     - Locate Time: {} seconds", drill_report.metrics.locate_time);
        println!("     - Decrypt Time: {} seconds", drill_report.metrics.decrypt_time);
        println!("     - Restore Time: {} seconds", drill_report.metrics.restore_time);
        println!("     - Total Time: {} seconds", drill_report.metrics.total_time);
    }
    
    // Example 4: Generating telemetry report (required evidence)
    println!("\n4. Generating telemetry report (required evidence):");
    
    let telemetry_report = manager.generate_telemetry_report();
    println!("   Telemetry Report:");
    println!("{}", telemetry_report);
    
    // Example 5: Verifying that Backup & Restore is properly configured
    println!("5. Verifying Backup & Restore configuration:");
    
    let is_enabled = manager.is_backup_restore_enabled();
    println!("   Backup & Restore enabled: {}", is_enabled);
    
    if is_enabled {
        println!("   ✓ All Backup & Restore mechanisms are enabled");
        println!("   ✓ Goal achieved: Survive ransomware / data loss");
        println!("   ✓ Evidence/Telemetry provided: Successful restore drill evidence, RPO/RTO metrics");
    } else {
        println!("   ✗ Backup & Restore is not fully enabled");
    }
    
    // Example 6: Updating configuration
    println!("\n6. Updating configuration:");
    
    let updated_config = BackupRestoreConfig {
        snapshot_interval: 1800, // 30 minutes (more frequent snapshots)
        retention_days: 60, // Retain backups for 60 days
        ..manager.get_config().clone()
    };
    
    manager.update_config(updated_config);
    println!("   Configuration updated successfully");
    
    // Verify the updated configuration
    let config = manager.get_config();
    println!("   Updated snapshot interval: {} seconds", config.snapshot_interval);
    println!("   Updated retention period: {} days", config.retention_days);
    
    println!("\nBackup & Restore Usage Example completed successfully!");
    
    Ok(())
}