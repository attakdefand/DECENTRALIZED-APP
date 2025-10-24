//! Integration tests for the incident response module
//!
//! These tests verify the complete incident response workflow including
//! pause/kill switch functionality, backup management, restore jobs,
//! and communication plans.

use core::incident_response::{
    IncidentResponseManager, PauseKillSwitch, Backup, BackupType, BackupStatus, 
    RestoreJob, RestoreStatus, CommunicationPlan, CommunicationChannel, 
    EscalationStep
};
use std::collections::HashMap;

/// Integration test for the complete incident response workflow
#[test]
fn test_complete_incident_response_workflow() {
    println!("Starting complete incident response workflow test");
    
    // 1. Create incident response manager
    let mut manager = IncidentResponseManager::new();
    println!("✓ Incident response manager created");
    
    // 2. Configure pause/kill switch
    let switch = PauseKillSwitch {
        id: "contract-emergency-pause".to_string(),
        target: "core-contracts".to_string(),
        active: false,
        reason: None,
        activated_at: None,
        expires_at: None,
        authorized_roles: vec!["admin".to_string(), "security".to_string()],
    };
    
    assert!(manager.add_pause_kill_switch(switch).is_ok());
    println!("✓ Pause/kill switch configured");
    
    // 3. Add database backup
    let mut backup_metadata = HashMap::new();
    backup_metadata.insert("environment".to_string(), "production".to_string());
    backup_metadata.insert("version".to_string(), "1.2.0".to_string());
    
    let backup = Backup {
        id: "db-backup-2023-01-01".to_string(),
        backup_type: BackupType::Database,
        created_at: 1234567890,
        location: "s3://backups/production/db-2023-01-01.sql".to_string(),
        size: 1073741824, // 1GB
        status: BackupStatus::Completed,
        metadata: backup_metadata,
    };
    
    assert!(manager.add_backup(backup).is_ok());
    println!("✓ Database backup added");
    
    // 4. Add configuration backup
    let mut config_metadata = HashMap::new();
    config_metadata.insert("service".to_string(), "api-server".to_string());
    
    let config_backup = Backup {
        id: "config-backup-2023-01-01".to_string(),
        backup_type: BackupType::Configuration,
        created_at: 1234567890,
        location: "s3://backups/production/config-2023-01-01.tar.gz".to_string(),
        size: 1048576, // 1MB
        status: BackupStatus::Completed,
        metadata: config_metadata,
    };
    
    assert!(manager.add_backup(config_backup).is_ok());
    println!("✓ Configuration backup added");
    
    // 5. Add restore job
    let mut restore_metadata = HashMap::new();
    restore_metadata.insert("initiator".to_string(), "admin-user".to_string());
    restore_metadata.insert("reason".to_string(), "database corruption".to_string());
    
    let restore_job = RestoreJob {
        id: "restore-job-123".to_string(),
        backup_id: "db-backup-2023-01-01".to_string(),
        target: "database-primary".to_string(),
        initiated_at: 1234567890,
        completed_at: None,
        status: RestoreStatus::Pending,
        error_message: None,
        metadata: restore_metadata,
    };
    
    assert!(manager.add_restore_job(restore_job).is_ok());
    println!("✓ Restore job added");
    
    // 6. Configure communication plan
    let channels = vec![
        CommunicationChannel {
            channel_type: "email".to_string(),
            channel_id: "security-team@company.com".to_string(),
            priority: 1,
        },
        CommunicationChannel {
            channel_type: "slack".to_string(),
            channel_id: "incident-response".to_string(),
            priority: 2,
        },
        CommunicationChannel {
            channel_type: "sms".to_string(),
            channel_id: "+1-555-0123".to_string(),
            priority: 3,
        }
    ];
    
    let mut templates = HashMap::new();
    templates.insert("initial".to_string(), "Security incident detected: {description}. Severity: {severity}".to_string());
    templates.insert("update".to_string(), "Incident update: {status}. Affected systems: {systems}".to_string());
    templates.insert("resolution".to_string(), "Incident resolved: {resolution}. Post-mortem scheduled for {date}".to_string());
    
    let escalation_steps = vec![
        EscalationStep {
            step: 1,
            time_threshold: 30, // 30 minutes
            recipients: vec!["oncall@company.com".to_string()],
            action: "Notify on-call security team".to_string(),
        },
        EscalationStep {
            step: 2,
            time_threshold: 60, // 1 hour
            recipients: vec!["security-lead@company.com".to_string(), "cto@company.com".to_string()],
            action: "Escalate to security leadership and CTO".to_string(),
        }
    ];
    
    let communication_plan = CommunicationPlan {
        id: "security-incident-plan".to_string(),
        incident_type: "security".to_string(),
        channels,
        recipients: vec![
            "security-team@company.com".to_string(),
            "engineering-team@company.com".to_string(),
            "executives@company.com".to_string(),
        ],
        templates,
        escalation_procedures: escalation_steps,
    };
    
    assert!(manager.add_communication_plan(communication_plan).is_ok());
    println!("✓ Communication plan configured");
    
    // 7. Verify configuration
    assert!(manager.validate_configuration().is_ok());
    println!("✓ Incident response configuration validated");
    
    // 8. Retrieve and verify components
    assert!(manager.get_pause_kill_switch("contract-emergency-pause").is_some());
    assert!(manager.get_backup("db-backup-2023-01-01").is_some());
    assert!(manager.get_restore_job("restore-job-123").is_some());
    assert!(manager.get_communication_plan("security-incident-plan").is_some());
    
    // 9. Test backup listing by type
    let db_backups = manager.list_backups_by_type(&BackupType::Database);
    assert_eq!(db_backups.len(), 1);
    println!("✓ Backup listing by type verified");
    
    // 10. Test communication plan retrieval by type
    let security_plan = manager.get_communication_plan_by_type("security");
    assert!(security_plan.is_some());
    assert_eq!(security_plan.unwrap().channels.len(), 3);
    println!("✓ Communication plan retrieval by type verified");
    
    println!("All incident response workflow tests passed!");
}

/// Test pause/kill switch management
#[test]
fn test_pause_kill_switch_management() {
    let mut manager = IncidentResponseManager::new();
    
    // Add required components for validation
    let backup = Backup {
        id: "test-backup".to_string(),
        backup_type: BackupType::Database,
        created_at: 1234567890,
        location: "test-location".to_string(),
        size: 1024,
        status: BackupStatus::Completed,
        metadata: HashMap::new(),
    };
    manager.add_backup(backup).unwrap();
    
    let communication_plan = CommunicationPlan {
        id: "test-plan".to_string(),
        incident_type: "test".to_string(),
        channels: vec![],
        recipients: vec![],
        templates: HashMap::new(),
        escalation_procedures: vec![],
    };
    manager.add_communication_plan(communication_plan).unwrap();
    
    // Test adding multiple switches
    let switches_data = vec![
        ("contract-pause", "smart-contracts", vec!["admin".to_string()]),
        ("service-kill", "api-service", vec!["admin".to_string(), "ops".to_string()]),
        ("network-shutdown", "network-infrastructure", vec!["admin".to_string(), "security".to_string()]),
    ];
    
    for (id, target, roles) in switches_data {
        let switch = PauseKillSwitch {
            id: id.to_string(),
            target: target.to_string(),
            active: false,
            reason: None,
            activated_at: None,
            expires_at: None,
            authorized_roles: roles,
        };
        
        assert!(manager.add_pause_kill_switch(switch).is_ok());
    }
    
    // Verify all switches were added
    assert!(manager.get_pause_kill_switch("contract-pause").is_some());
    assert!(manager.get_pause_kill_switch("service-kill").is_some());
    assert!(manager.get_pause_kill_switch("network-shutdown").is_some());
    
    // Test switch activation and deactivation
    assert!(manager.activate_pause_kill_switch(
        "contract-pause",
        "admin-user".to_string(),
        "Testing activation".to_string()
    ).is_ok());
    
    let switch = manager.get_pause_kill_switch("contract-pause").unwrap();
    assert!(switch.active);
    assert!(switch.reason.is_some());
    
    assert!(manager.deactivate_pause_kill_switch(
        "contract-pause",
        "admin-user".to_string()
    ).is_ok());
    
    let switch = manager.get_pause_kill_switch("contract-pause").unwrap();
    assert!(!switch.active);
    
    // Validate configuration
    assert!(manager.validate_configuration().is_ok());
}

/// Test backup and restore operations
#[test]
fn test_backup_restore_operations() {
    let mut manager = IncidentResponseManager::new();
    
    // Add required components for validation
    let switch = PauseKillSwitch {
        id: "test-switch".to_string(),
        target: "test".to_string(),
        active: false,
        reason: None,
        activated_at: None,
        expires_at: None,
        authorized_roles: vec!["admin".to_string()],
    };
    manager.add_pause_kill_switch(switch).unwrap();
    
    let communication_plan = CommunicationPlan {
        id: "test-plan".to_string(),
        incident_type: "test".to_string(),
        channels: vec![],
        recipients: vec![],
        templates: HashMap::new(),
        escalation_procedures: vec![],
    };
    manager.add_communication_plan(communication_plan).unwrap();
    
    // Test adding different types of backups
    let backup_types = vec![
        (BackupType::Database, "db-backup", 1073741824), // 1GB
        (BackupType::Configuration, "config-backup", 1048576), // 1MB
        (BackupType::State, "state-backup", 52428800), // 50MB
        (BackupType::Keys, "keys-backup", 2048), // 2KB
    ];
    
    for (backup_type, id, size) in backup_types {
        let backup = Backup {
            id: id.to_string(),
            backup_type: backup_type.clone(),
            created_at: 1234567890,
            location: format!("s3://backups/{}", id),
            size,
            status: BackupStatus::Completed,
            metadata: HashMap::new(),
        };
        
        assert!(manager.add_backup(backup).is_ok());
    }
    
    // Verify all backups were added
    assert!(manager.get_backup("db-backup").is_some());
    assert!(manager.get_backup("config-backup").is_some());
    assert!(manager.get_backup("state-backup").is_some());
    assert!(manager.get_backup("keys-backup").is_some());
    
    // Test backup listing by type
    let db_backups = manager.list_backups_by_type(&BackupType::Database);
    assert_eq!(db_backups.len(), 1);
    
    let config_backups = manager.list_backups_by_type(&BackupType::Configuration);
    assert_eq!(config_backups.len(), 1);
    
    // Test adding restore jobs
    let restore_jobs_data = vec![
        ("restore-db", "db-backup", "database-primary"),
        ("restore-config", "config-backup", "api-server"),
        ("restore-state", "state-backup", "contract-state"),
    ];
    
    for (id, backup_id, target) in restore_jobs_data {
        let job = RestoreJob {
            id: id.to_string(),
            backup_id: backup_id.to_string(),
            target: target.to_string(),
            initiated_at: 1234567890,
            completed_at: None,
            status: RestoreStatus::Pending,
            error_message: None,
            metadata: HashMap::new(),
        };
        
        assert!(manager.add_restore_job(job).is_ok());
    }
    
    // Test updating restore job status
    assert!(manager.update_restore_job_status(
        "restore-db",
        RestoreStatus::InProgress,
        None
    ).is_ok());
    
    let job = manager.get_restore_job("restore-db").unwrap();
    assert!(matches!(job.status, RestoreStatus::InProgress));
    
    assert!(manager.update_restore_job_status(
        "restore-db",
        RestoreStatus::Completed,
        None
    ).is_ok());
    
    let job = manager.get_restore_job("restore-db").unwrap();
    assert!(matches!(job.status, RestoreStatus::Completed));
    assert!(job.completed_at.is_some());
    
    // Validate configuration
    assert!(manager.validate_configuration().is_ok());
}

/// Test communication plan management
#[test]
fn test_communication_plan_management() {
    let mut manager = IncidentResponseManager::new();
    
    // Add required components for validation
    let switch = PauseKillSwitch {
        id: "test-switch".to_string(),
        target: "test".to_string(),
        active: false,
        reason: None,
        activated_at: None,
        expires_at: None,
        authorized_roles: vec!["admin".to_string()],
    };
    manager.add_pause_kill_switch(switch).unwrap();
    
    let backup = Backup {
        id: "test-backup".to_string(),
        backup_type: BackupType::Database,
        created_at: 1234567890,
        location: "test-location".to_string(),
        size: 1024,
        status: BackupStatus::Completed,
        metadata: HashMap::new(),
    };
    manager.add_backup(backup).unwrap();
    
    // Test adding multiple communication plans for different incident types
    let incident_types = vec![
        ("security", "Security Incident Response Plan"),
        ("availability", "Service Availability Incident Response Plan"),
        ("data-breach", "Data Breach Incident Response Plan"),
    ];
    
    for (incident_type, description) in incident_types {
        let plan = CommunicationPlan {
            id: format!("{}-plan", incident_type),
            incident_type: incident_type.to_string(),
            channels: vec![
                CommunicationChannel {
                    channel_type: "email".to_string(),
                    channel_id: format!("{}@company.com", incident_type),
                    priority: 1,
                }
            ],
            recipients: vec![format!("{}-team@company.com", incident_type)],
            templates: {
                let mut templates = HashMap::new();
                templates.insert("initial".to_string(), format!("{}: {{description}}", description));
                templates
            },
            escalation_procedures: vec![
                EscalationStep {
                    step: 1,
                    time_threshold: 30,
                    recipients: vec![format!("oncall-{}@company.com", incident_type)],
                    action: format!("Notify on-call {} team", incident_type),
                }
            ],
        };
        
        assert!(manager.add_communication_plan(plan).is_ok());
    }
    
    // Verify all plans were added
    assert!(manager.get_communication_plan("security-plan").is_some());
    assert!(manager.get_communication_plan("availability-plan").is_some());
    assert!(manager.get_communication_plan("data-breach-plan").is_some());
    
    // Test plan retrieval by incident type
    let security_plan = manager.get_communication_plan_by_type("security");
    assert!(security_plan.is_some());
    assert_eq!(security_plan.unwrap().recipients[0], "security-team@company.com");
    
    // Validate configuration
    assert!(manager.validate_configuration().is_ok());
}