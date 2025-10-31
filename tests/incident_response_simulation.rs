//! Simulation tests for the incident response module
//!
//! These tests simulate real-world scenarios for incident response including
//! security incidents, system failures, and disaster recovery procedures.

use core::incident_response::{
    IncidentResponseManager, PauseKillSwitch, Backup, BackupType, BackupStatus, 
    RestoreJob, RestoreStatus, CommunicationPlan, CommunicationChannel, 
    EscalationStep, IncidentAction
};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

/// Simulation: Security Incident Response
///
/// This test simulates a security incident where the pause/kill switch is activated,
/// backups are verified, and the communication plan is executed.
#[test]
fn simulate_security_incident_response() {
    println!("=== Security Incident Response Simulation ===");
    
    let mut manager = IncidentResponseManager::new();
    
    // Configure emergency pause/kill switch
    let switch = PauseKillSwitch {
        id: "emergency-contract-pause".to_string(),
        target: "core-smart-contracts".to_string(),
        active: false,
        reason: None,
        activated_at: None,
        expires_at: Some(1234567890 + 3600), // Expires in 1 hour
        authorized_roles: vec!["admin".to_string(), "security".to_string()],
    };
    assert!(manager.add_pause_kill_switch(switch).is_ok());
    
    // Add critical backups
    let backup_types = vec![
        (BackupType::Database, "db-backup-critical", 2147483648), // 2GB
        (BackupType::Configuration, "config-backup-critical", 2097152), // 2MB
        (BackupType::State, "state-backup-critical", 104857600), // 100MB
    ];
    
    for (backup_type, id, size) in backup_types {
        let mut metadata = HashMap::new();
        metadata.insert("criticality".to_string(), "high".to_string());
        metadata.insert("retention".to_string(), "30 days".to_string());
        
        let backup = Backup {
            id: id.to_string(),
            backup_type,
            created_at: 1234567890,
            location: format!("s3://critical-backups/production/{}", id),
            size,
            status: BackupStatus::Completed,
            metadata,
        };
        
        assert!(manager.add_backup(backup).is_ok());
        thread::sleep(Duration::from_millis(10)); // Simulate processing time
    }
    
    println!("✓ Configured emergency pause/kill switch and critical backups");
    
    // Configure comprehensive communication plan
    let channels = vec![
        CommunicationChannel {
            channel_type: "email".to_string(),
            channel_id: "security-team@company.com".to_string(),
            priority: 1,
        },
        CommunicationChannel {
            channel_type: "slack".to_string(),
            channel_id: "incident-response-channel".to_string(),
            priority: 1,
        },
        CommunicationChannel {
            channel_type: "sms".to_string(),
            channel_id: "+1-555-SECURITY".to_string(),
            priority: 2,
        },
        CommunicationChannel {
            channel_type: "phone".to_string(),
            channel_id: "+1-555-EXECUTIVE".to_string(),
            priority: 3,
        }
    ];
    
    let mut templates = HashMap::new();
    templates.insert("initial".to_string(), "🚨 SECURITY INCIDENT DETECTED 🚨\n\nDescription: {description}\nSeverity: {severity}\nAffected Systems: {systems}\n\nAction Required: {action_required}".to_string());
    templates.insert("update".to_string(), "🔄 INCIDENT UPDATE 🔄\n\nStatus: {status}\nTimeline: {timeline}\nNext Steps: {next_steps}".to_string());
    templates.insert("resolution".to_string(), "✅ INCIDENT RESOLVED ✅\n\nResolution: {resolution}\nImpact: {impact}\nPost-Mortem: {post_mortem_date}".to_string());
    
    let escalation_procedures = vec![
        EscalationStep {
            step: 1,
            time_threshold: 15, // 15 minutes
            recipients: vec!["oncall-security@company.com".to_string()],
            action: "Immediate investigation and containment".to_string(),
        },
        EscalationStep {
            step: 2,
            time_threshold: 30, // 30 minutes
            recipients: vec!["security-lead@company.com".to_string()],
            action: "Escalate to security leadership team".to_string(),
        },
        EscalationStep {
            step: 3,
            time_threshold: 60, // 1 hour
            recipients: vec!["cto@company.com".to_string(), "ciso@company.com".to_string()],
            action: "Executive notification and stakeholder briefing".to_string(),
        }
    ];
    
    let communication_plan = CommunicationPlan {
        id: "critical-security-incident-plan".to_string(),
        incident_type: "critical-security".to_string(),
        channels: channels.clone(),
        recipients: vec![
            "security-team@company.com".to_string(),
            "engineering-leads@company.com".to_string(),
            "product-team@company.com".to_string(),
        ],
        templates: templates.clone(),
        escalation_procedures,
    };
    
    assert!(manager.add_communication_plan(communication_plan).is_ok());
    println!("✓ Configured comprehensive security communication plan");
    
    // Validate configuration
    assert!(manager.validate_configuration().is_ok());
    println!("✓ Incident response configuration validated");
    
    // Simulate normal system operation
    println!("System operating normally...");
    thread::sleep(Duration::from_millis(100));
    
    // Simulate security incident detection
    println!("⚠️  Security incident detected!");
    
    // Activate emergency pause/kill switch
    assert!(manager.activate_pause_kill_switch(
        "emergency-contract-pause",
        "security-admin".to_string(),
        "Critical security vulnerability detected in smart contracts".to_string()
    ).is_ok());
    
    let switch = manager.get_pause_kill_switch("emergency-contract-pause").unwrap();
    assert!(switch.active);
    println!("  Emergency pause/kill switch activated for: {}", switch.target);
    
    // Verify critical backups are available
    let critical_backups = manager.list_backups_by_type(&BackupType::Database);
    assert!(!critical_backups.is_empty());
    println!("  Verified {} critical database backups available", critical_backups.len());
    
    // Execute communication plan
    let security_plan = manager.get_communication_plan_by_type("critical-security").unwrap();
    println!("  Executing communication plan for {} channels", security_plan.channels.len());
    
    // Simulate sending initial notification
    let initial_template = security_plan.templates.get("initial").unwrap();
    let notification = initial_template.replace("{description}", "Critical security vulnerability detected in smart contracts")
        .replace("{severity}", "CRITICAL")
        .replace("{systems}", "Core smart contracts, User funds")
        .replace("{action_required}", "Immediate system pause, investigation initiated");
    
    println!("  Sent initial notification: {}", notification.lines().next().unwrap());
    
    // Simulate incident investigation and resolution
    println!("  Investigating incident...");
    thread::sleep(Duration::from_millis(200));
    
    // Add restore job for system recovery
    let mut restore_metadata = HashMap::new();
    restore_metadata.insert("incident_id".to_string(), "SEC-2023-001".to_string());
    restore_metadata.insert("investigator".to_string(), "security-team".to_string());
    
    let restore_job = RestoreJob {
        id: "restore-post-incident-001".to_string(),
        backup_id: "db-backup-critical".to_string(),
        target: "database-primary".to_string(),
        initiated_at: 1234567890 + 300, // 5 minutes after incident
        completed_at: None,
        status: RestoreStatus::Pending,
        error_message: None,
        metadata: restore_metadata,
    };
    
    assert!(manager.add_restore_job(restore_job).is_ok());
    println!("  Created restore job from pre-incident backup");
    
    // Update restore job status
    assert!(manager.update_restore_job_status(
        "restore-post-incident-001",
        RestoreStatus::InProgress,
        None
    ).is_ok());
    
    thread::sleep(Duration::from_millis(50));
    
    assert!(manager.update_restore_job_status(
        "restore-post-incident-001",
        RestoreStatus::Completed,
        None
    ).is_ok());
    
    println!("  Restore job completed successfully");
    
    // Deactivate pause/kill switch after resolution
    assert!(manager.deactivate_pause_kill_switch(
        "emergency-contract-pause",
        "security-lead".to_string()
    ).is_ok());
    
    let switch = manager.get_pause_kill_switch("emergency-contract-pause").unwrap();
    assert!(!switch.active);
    println!("  Emergency pause/kill switch deactivated");
    
    // Send resolution notification
    let resolution_template = security_plan.templates.get("resolution").unwrap();
    let resolution_notification = resolution_template.replace("{resolution}", "Vulnerability patched, system restored from clean backup")
        .replace("{impact}", "No user funds affected, 0.5 hours downtime")
        .replace("{post_mortem_date}", "2023-01-02 10:00 UTC");
    
    println!("  Sent resolution notification: {}", resolution_notification.lines().next().unwrap());
    
    // Verify audit log
    let audit_log = manager.get_audit_log(Some(5));
    assert!(!audit_log.is_empty());
    println!("  Audit log contains {} recent actions", audit_log.len());
    
    println!("✓ Security incident response simulation completed successfully");
    println!("=== Security Incident Response Simulation Complete ===\n");
}

/// Simulation: Disaster Recovery Procedure
///
/// This test simulates a disaster recovery scenario where multiple systems fail
/// and the incident response procedures are executed to restore operations.
#[test]
fn simulate_disaster_recovery_procedure() {
    println!("=== Disaster Recovery Procedure Simulation ===");
    
    let mut manager = IncidentResponseManager::new();
    
    // Configure multiple pause/kill switches for different systems
    let switches_data = vec![
        ("api-pause", "api-gateway", vec!["admin".to_string(), "ops".to_string()]),
        ("db-kill", "database-cluster", vec!["admin".to_string(), "dba".to_string()]),
        ("indexer-pause", "indexer-services", vec!["admin".to_string(), "dev".to_string()]),
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
        thread::sleep(Duration::from_millis(5));
    }
    
    println!("✓ Configured system pause/kill switches");
    
    // Add comprehensive backup system
    let backup_data = vec![
        (BackupType::Database, "db-backup-full", 10737418240, "s3://backups/production/full-db-2023-01-01.sql"), // 10GB
        (BackupType::Database, "db-backup-incremental-1", 1073741824, "s3://backups/production/incr-db-2023-01-01-01.sql"), // 1GB
        (BackupType::Database, "db-backup-incremental-2", 1073741824, "s3://backups/production/incr-db-2023-01-01-02.sql"), // 1GB
        (BackupType::Configuration, "config-backup-api", 1048576, "s3://backups/production/config-api-2023-01-01.tar.gz"), // 1MB
        (BackupType::Configuration, "config-backup-db", 524288, "s3://backups/production/config-db-2023-01-01.tar.gz"), // 0.5MB
        (BackupType::State, "contract-state-backup", 20971520, "s3://backups/production/contract-state-2023-01-01.dat"), // 20MB
        (BackupType::Keys, "key-backup-master", 4096, "s3://backups/production/key-master-2023-01-01.pem"), // 4KB
    ];
    
    for (backup_type, id, size, location) in backup_data {
        let mut metadata = HashMap::new();
        metadata.insert("backup_strategy".to_string(), "daily_full_weekly_incr".to_string());
        metadata.insert("retention_policy".to_string(), "30_days".to_string());
        metadata.insert("encryption".to_string(), "AES-256".to_string());
        
        let backup = Backup {
            id: id.to_string(),
            backup_type,
            created_at: 1234567890,
            location: location.to_string(),
            size,
            status: BackupStatus::Completed,
            metadata,
        };
        
        assert!(manager.add_backup(backup).is_ok());
        thread::sleep(Duration::from_millis(5));
    }
    
    println!("✓ Configured comprehensive backup system");
    
    // Configure disaster recovery communication plan
    let channels = vec![
        CommunicationChannel {
            channel_type: "email".to_string(),
            channel_id: "disaster-recovery@company.com".to_string(),
            priority: 1,
        },
        CommunicationChannel {
            channel_type: "slack".to_string(),
            channel_id: "disaster-recovery-channel".to_string(),
            priority: 1,
        },
        CommunicationChannel {
            channel_type: "phone".to_string(),
            channel_id: "+1-555-DR-TEAM".to_string(),
            priority: 2,
        }
    ];
    
    let mut templates = HashMap::new();
    templates.insert("dr-initiation".to_string(), "🚨 DISASTER RECOVERY INITIATED 🚨\n\nEvent: {event}\nSeverity: {severity}\nAffected Systems: {systems}\n\nRecovery Team: {recovery_team}".to_string());
    templates.insert("dr-progress".to_string(), "🔄 DISASTER RECOVERY PROGRESS 🔄\n\nPhase: {phase}\nStatus: {status}\nETA: {eta}".to_string());
    templates.insert("dr-completion".to_string(), "✅ DISASTER RECOVERY COMPLETED ✅\n\nRestored Systems: {systems}\nDowntime: {downtime}\nVerification Status: {verification}".to_string());
    
    let communication_plan = CommunicationPlan {
        id: "disaster-recovery-plan".to_string(),
        incident_type: "disaster-recovery".to_string(),
        channels,
        recipients: vec![
            "dr-team@company.com".to_string(),
            "executives@company.com".to_string(),
            "customers@company.com".to_string(),
        ],
        templates,
        escalation_procedures: vec![],
    };
    
    assert!(manager.add_communication_plan(communication_plan).is_ok());
    println!("✓ Configured disaster recovery communication plan");
    
    // Validate configuration
    assert!(manager.validate_configuration().is_ok());
    println!("✓ Disaster recovery configuration validated");
    
    // Simulate normal operations
    println!("Systems operating normally...");
    thread::sleep(Duration::from_millis(100));
    
    // Simulate disaster scenario (multiple system failures)
    println!("⚠️  DISASTER SCENARIO: Multiple system failures detected!");
    
    // Activate all pause/kill switches
    let switches_to_activate = vec!["api-pause", "db-kill", "indexer-pause"];
    for switch_id in switches_to_activate {
        assert!(manager.activate_pause_kill_switch(
            switch_id,
            "dr-coordinator".to_string(),
            "Disaster recovery initiated - system failures detected".to_string()
        ).is_ok());
        
        let switch = manager.get_pause_kill_switch(switch_id).unwrap();
        assert!(switch.active);
        println!("  Activated {} for {}", switch_id, switch.target);
        thread::sleep(Duration::from_millis(20));
    }
    
    // Execute disaster recovery communication plan
    let dr_plan = manager.get_communication_plan_by_type("disaster-recovery").unwrap();
    let initiation_template = dr_plan.templates.get("dr-initiation").unwrap();
    let dr_notification = initiation_template.replace("{event}", "Cascading system failures across API, database, and indexing services")
        .replace("{severity}", "CRITICAL")
        .replace("{systems}", "API Gateway, Database Cluster, Indexer Services")
        .replace("{recovery_team}", "DR Team activated - see runbook");
    
    println!("  Sent DR initiation notification: {}", dr_notification.lines().next().unwrap());
    
    // Begin recovery process
    println!("  Initiating disaster recovery process...");
    
    // Create restore jobs for each critical system
    let restore_jobs_data = vec![
        ("restore-db-full", "db-backup-full", "database-primary", "Restoring primary database from full backup"),
        ("restore-api-config", "config-backup-api", "api-gateway", "Restoring API gateway configuration"),
        ("restore-contract-state", "contract-state-backup", "smart-contracts", "Restoring contract state from backup"),
    ];
    
    for (id, backup_id, target, description) in restore_jobs_data {
        let mut metadata = HashMap::new();
        metadata.insert("dr_event_id".to_string(), "DR-2023-001".to_string());
        metadata.insert("description".to_string(), description.to_string());
        
        let job = RestoreJob {
            id: id.to_string(),
            backup_id: backup_id.to_string(),
            target: target.to_string(),
            initiated_at: 1234567890 + 600, // 10 minutes after disaster
            completed_at: None,
            status: RestoreStatus::Pending,
            error_message: None,
            metadata,
        };
        
        assert!(manager.add_restore_job(job).is_ok());
        println!("    Created restore job: {}", id);
        thread::sleep(Duration::from_millis(15));
    }
    
    // Simulate restore job execution
    let jobs_to_execute = vec!["restore-db-full", "restore-api-config", "restore-contract-state"];
    for job_id in jobs_to_execute {
        // Update to in-progress
        assert!(manager.update_restore_job_status(
            job_id,
            RestoreStatus::InProgress,
            None
        ).is_ok());
        
        println!("    Executing restore job: {}", job_id);
        thread::sleep(Duration::from_millis(30));
        
        // Update to completed
        assert!(manager.update_restore_job_status(
            job_id,
            RestoreStatus::Completed,
            None
        ).is_ok());
        
        println!("    Completed restore job: {}", job_id);
        thread::sleep(Duration::from_millis(20));
    }
    
    // Verify all backups are available for restoration
    let all_backups = manager.list_backups_by_type(&BackupType::Database);
    assert!(!all_backups.is_empty());
    println!("  Verified {} database backups available for restoration", all_backups.len());
    
    // Deactivate pause/kill switches after recovery
    for switch_id in switches_to_activate {
        assert!(manager.deactivate_pause_kill_switch(
            switch_id,
            "dr-coordinator".to_string()
        ).is_ok());
        
        let switch = manager.get_pause_kill_switch(switch_id).unwrap();
        assert!(!switch.active);
        println!("  Deactivated {}", switch_id);
        thread::sleep(Duration::from_millis(10));
    }
    
    // Send DR completion notification
    let completion_template = dr_plan.templates.get("dr-completion").unwrap();
    let completion_notification = completion_template.replace("{systems}", "API Gateway, Database Cluster, Indexer Services")
        .replace("{downtime}", "2.5 hours")
        .replace("{verification}", "All systems verified operational");
    
    println!("  Sent DR completion notification: {}", completion_notification.lines().next().unwrap());
    
    // Verify audit trail
    let audit_log = manager.get_audit_log(Some(10));
    assert!(!audit_log.is_empty());
    println!("  Audit trail contains {} recent actions", audit_log.len());
    
    // Verify all restore jobs completed
    let completed_jobs = manager.restore_jobs.values()
        .filter(|job| matches!(job.status, RestoreStatus::Completed))
        .count();
    assert_eq!(completed_jobs, 3);
    println!("  Verified all {} restore jobs completed successfully", completed_jobs);
    
    println!("✓ Disaster recovery procedure simulation completed successfully");
    println!("=== Disaster Recovery Procedure Simulation Complete ===\n");
}

/// Simulation: Incident Response Training Exercise
///
/// This test simulates a training exercise for the incident response team
/// to validate procedures and identify areas for improvement.
#[test]
fn simulate_incident_response_training() {
    println!("=== Incident Response Training Exercise Simulation ===");
    
    let mut manager = IncidentResponseManager::new();
    
    // Set up training environment with realistic configurations
    let training_switches = vec![
        ("training-contract-pause", "training-smart-contracts", vec!["trainer".to_string(), "trainee".to_string()]),
        ("training-service-kill", "training-api-service", vec!["trainer".to_string(), "trainee".to_string()]),
    ];
    
    for (id, target, roles) in training_switches {
        let switch = PauseKillSwitch {
            id: id.to_string(),
            target: target.to_string(),
            active: false,
            reason: None,
            activated_at: None,
            expires_at: Some(1234567890 + 7200), // 2 hours
            authorized_roles: roles,
        };
        
        assert!(manager.add_pause_kill_switch(switch).is_ok());
    }
    
    println!("✓ Configured training environment pause/kill switches");
    
    // Add training backups with different scenarios
    let training_backups = vec![
        (BackupType::Database, "training-db-backup-clean", 536870912, "s3://training-backups/clean-db.sql"), // 512MB
        (BackupType::Database, "training-db-backup-corrupted", 536870912, "s3://training-backups/corrupted-db.sql"), // 512MB
        (BackupType::Configuration, "training-config-backup", 262144, "s3://training-backups/config.tar.gz"), // 256KB
    ];
    
    for (backup_type, id, size, location) in training_backups {
        let mut metadata = HashMap::new();
        metadata.insert("scenario".to_string(), "training".to_string());
        metadata.insert("purpose".to_string(), "exercise".to_string());
        
        let backup = Backup {
            id: id.to_string(),
            backup_type,
            created_at: 1234567890,
            location: location.to_string(),
            size,
            status: BackupStatus::Completed,
            metadata,
        };
        
        assert!(manager.add_backup(backup).is_ok());
    }
    
    println!("✓ Configured training scenario backups");
    
    // Configure training communication plan
    let channels = vec![
        CommunicationChannel {
            channel_type: "slack".to_string(),
            channel_id: "training-exercise-channel".to_string(),
            priority: 1,
        },
        CommunicationChannel {
            channel_type: "email".to_string(),
            channel_id: "training-team@company.com".to_string(),
            priority: 2,
        }
    ];
    
    let mut templates = HashMap::new();
    templates.insert("exercise-start".to_string(), "🎓 TRAINING EXERCISE STARTED 🎓\n\nScenario: {scenario}\nObjectives: {objectives}\nDuration: {duration}".to_string());
    templates.insert("exercise-update".to_string(), "🔄 TRAINING EXERCISE UPDATE 🔄\n\nCurrent Phase: {phase}\nTasks: {tasks}\nTime Remaining: {time_remaining}".to_string());
    templates.insert("exercise-end".to_string(), "✅ TRAINING EXERCISE COMPLETED ✅\n\nScenario: {scenario}\nResults: {results}\nLessons Learned: {lessons}".to_string());
    
    let communication_plan = CommunicationPlan {
        id: "training-exercise-plan".to_string(),
        incident_type: "training-exercise".to_string(),
        channels,
        recipients: vec![
            "training-team@company.com".to_string(),
            "security-team@company.com".to_string(),
        ],
        templates,
        escalation_procedures: vec![],
    };
    
    assert!(manager.add_communication_plan(communication_plan).is_ok());
    println!("✓ Configured training communication plan");
    
    // Validate configuration
    assert!(manager.validate_configuration().is_ok());
    println!("✓ Training environment configuration validated");
    
    // Begin training exercise
    println!("🎓 Starting incident response training exercise...");
    
    // Send exercise start notification
    let training_plan = manager.get_communication_plan_by_type("training-exercise").unwrap();
    let start_template = training_plan.templates.get("exercise-start").unwrap();
    let start_notification = start_template.replace("{scenario}", "Simulated security breach with database corruption")
        .replace("{objectives}", "Test pause/kill procedures, backup restoration, communication protocols")
        .replace("{duration}", "2 hours");
    
    println!("  {}", start_notification.lines().next().unwrap());
    
    // Exercise 1: Activate pause/kill switch
    println!("  Exercise 1: Activate pause/kill switch");
    assert!(manager.activate_pause_kill_switch(
        "training-contract-pause",
        "trainee-1".to_string(),
        "Training exercise - activating emergency pause".to_string()
    ).is_ok());
    
    let switch = manager.get_pause_kill_switch("training-contract-pause").unwrap();
    assert!(switch.active);
    println!("    ✓ Successfully activated emergency pause");
    
    // Exercise 2: Verify backups and create restore job
    println!("  Exercise 2: Verify backups and create restore job");
    let clean_backups = manager.list_backups_by_type(&BackupType::Database);
    assert!(!clean_backups.is_empty());
    println!("    ✓ Verified {} database backups available", clean_backups.len());
    
    let mut restore_metadata = HashMap::new();
    restore_metadata.insert("exercise_id".to_string(), "TRAIN-2023-001".to_string());
    restore_metadata.insert("scenario".to_string(), "database_corruption".to_string());
    
    let restore_job = RestoreJob {
        id: "training-restore-001".to_string(),
        backup_id: "training-db-backup-clean".to_string(),
        target: "training-database".to_string(),
        initiated_at: 1234567890 + 300,
        completed_at: None,
        status: RestoreStatus::Pending,
        error_message: None,
        metadata: restore_metadata,
    };
    
    assert!(manager.add_restore_job(restore_job).is_ok());
    println!("    ✓ Created restore job from clean backup");
    
    // Exercise 3: Execute restore job
    println!("  Exercise 3: Execute restore job");
    assert!(manager.update_restore_job_status(
        "training-restore-001",
        RestoreStatus::InProgress,
        None
    ).is_ok());
    
    thread::sleep(Duration::from_millis(50));
    
    assert!(manager.update_restore_job_status(
        "training-restore-001",
        RestoreStatus::Completed,
        None
    ).is_ok());
    
    let job = manager.get_restore_job("training-restore-001").unwrap();
    assert!(matches!(job.status, RestoreStatus::Completed));
    println!("    ✓ Successfully completed restore job");
    
    // Exercise 4: Deactivate pause/kill switch
    println!("  Exercise 4: Deactivate pause/kill switch");
    assert!(manager.deactivate_pause_kill_switch(
        "training-contract-pause",
        "trainee-1".to_string()
    ).is_ok());
    
    let switch = manager.get_pause_kill_switch("training-contract-pause").unwrap();
    assert!(!switch.active);
    println!("    ✓ Successfully deactivated emergency pause");
    
    // Send exercise completion notification
    let end_template = training_plan.templates.get("exercise-end").unwrap();
    let end_notification = end_template.replace("{scenario}", "Simulated security breach with database corruption")
        .replace("{results}", "All team members successfully executed procedures")
        .replace("{lessons}", "Improve communication timing, streamline restore verification");
    
    println!("  {}", end_notification.lines().next().unwrap());
    
    // Generate training exercise report
    let audit_log = manager.get_audit_log(None);
    println!("  Training exercise audit log contains {} actions", audit_log.len());
    
    let restore_jobs = manager.restore_jobs.len();
    println!("  Executed {} restore jobs during training", restore_jobs);
    
    let switches = manager.pause_kill_switches.len();
    println!("  Managed {} pause/kill switches during training", switches);
    
    println!("✓ Incident response training exercise completed successfully");
    println!("=== Incident Response Training Exercise Simulation Complete ===\n");
}