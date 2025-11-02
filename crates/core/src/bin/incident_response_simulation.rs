//! Binary test runner for incident response simulation tests
//!
//! This binary allows running the incident response simulation tests as a standalone program.

use core::incident_response::{
    Backup, BackupStatus, BackupType, CommunicationPlan,
    IncidentResponseManager, PauseKillSwitch, RestoreJob, RestoreStatus,
};
use std::collections::HashMap;

fn main() {
    println!("Running Incident Response Simulation Tests");
    println!("========================================\n");

    // Run all simulation tests
    test_security_incident_response();
    test_disaster_recovery_procedure();
    test_incident_response_training();

    println!("All incident response simulation tests completed successfully!");
}

fn test_security_incident_response() {
    println!("1. Testing Security Incident Response...");

    let mut manager = IncidentResponseManager::new();

    // Configure emergency pause/kill switch
    let switch = PauseKillSwitch {
        id: "emergency-pause".to_string(),
        target: "smart-contracts".to_string(),
        active: false,
        reason: None,
        activated_at: None,
        expires_at: None,
        authorized_roles: vec!["admin".to_string()],
    };
    assert!(manager.add_pause_kill_switch(switch).is_ok());

    // Add backup
    let backup = Backup {
        id: "db-backup".to_string(),
        backup_type: BackupType::Database,
        created_at: 1234567890,
        location: "s3://backups/db.sql".to_string(),
        size: 1024000,
        status: BackupStatus::Completed,
        metadata: HashMap::new(),
    };
    assert!(manager.add_backup(backup).is_ok());

    // Configure communication plan
    let plan = CommunicationPlan {
        id: "security-plan".to_string(),
        incident_type: "security".to_string(),
        channels: vec![],
        recipients: vec![],
        templates: HashMap::new(),
        escalation_procedures: vec![],
    };
    assert!(manager.add_communication_plan(plan).is_ok());

    // Activate switch
    assert!(manager
        .activate_pause_kill_switch(
            "emergency-pause",
            "admin-user".to_string(),
            "Security incident".to_string()
        )
        .is_ok());

    println!("   ✓ Emergency pause/kill switch activated");

    // Add restore job
    let job = RestoreJob {
        id: "restore-1".to_string(),
        backup_id: "db-backup".to_string(),
        target: "database".to_string(),
        initiated_at: 1234567890,
        completed_at: None,
        status: RestoreStatus::Pending,
        error_message: None,
        metadata: HashMap::new(),
    };
    assert!(manager.add_restore_job(job).is_ok());

    println!("   ✓ Restore job added");

    // Validate configuration
    assert!(manager.validate_configuration().is_ok());

    println!("   ✓ Security incident response test passed\n");
}

fn test_disaster_recovery_procedure() {
    println!("2. Testing Disaster Recovery Procedure...");

    let mut manager = IncidentResponseManager::new();

    // Add required components
    let switch = PauseKillSwitch {
        id: "dr-pause".to_string(),
        target: "critical-systems".to_string(),
        active: false,
        reason: None,
        activated_at: None,
        expires_at: None,
        authorized_roles: vec!["admin".to_string()],
    };
    manager.add_pause_kill_switch(switch).unwrap();

    let backup = Backup {
        id: "dr-backup".to_string(),
        backup_type: BackupType::Database,
        created_at: 1234567890,
        location: "s3://dr/backups.db".to_string(),
        size: 2048000,
        status: BackupStatus::Completed,
        metadata: HashMap::new(),
    };
    manager.add_backup(backup).unwrap();

    let plan = CommunicationPlan {
        id: "dr-plan".to_string(),
        incident_type: "disaster-recovery".to_string(),
        channels: vec![],
        recipients: vec![],
        templates: HashMap::new(),
        escalation_procedures: vec![],
    };
    manager.add_communication_plan(plan).unwrap();

    // Test backup listing
    let backups = manager.list_backups_by_type(&BackupType::Database);
    assert!(!backups.is_empty());

    println!("   ✓ Disaster recovery backups verified");

    // Validate configuration
    assert!(manager.validate_configuration().is_ok());

    println!("   ✓ Disaster recovery procedure test passed\n");
}

fn test_incident_response_training() {
    println!("3. Testing Incident Response Training...");

    let mut manager = IncidentResponseManager::new();

    // Add training components
    let switch = PauseKillSwitch {
        id: "training-pause".to_string(),
        target: "training-environment".to_string(),
        active: false,
        reason: None,
        activated_at: None,
        expires_at: None,
        authorized_roles: vec!["trainer".to_string()],
    };
    manager.add_pause_kill_switch(switch).unwrap();

    let backup = Backup {
        id: "training-backup".to_string(),
        backup_type: BackupType::Configuration,
        created_at: 1234567890,
        location: "s3://training/config.tar.gz".to_string(),
        size: 102400,
        status: BackupStatus::Completed,
        metadata: HashMap::new(),
    };
    manager.add_backup(backup).unwrap();

    let plan = CommunicationPlan {
        id: "training-plan".to_string(),
        incident_type: "training-exercise".to_string(),
        channels: vec![],
        recipients: vec![],
        templates: HashMap::new(),
        escalation_procedures: vec![],
    };
    manager.add_communication_plan(plan).unwrap();

    // Test communication plan retrieval
    let plan = manager.get_communication_plan_by_type("training-exercise");
    assert!(plan.is_some());

    println!("   ✓ Training communication plan verified");

    // Validate configuration
    assert!(manager.validate_configuration().is_ok());

    println!("   ✓ Incident response training test passed\n");
}
