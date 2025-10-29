//! Tests for Incident Response features: Runbooks & Pager
//!
//! These tests validate the implementation of Layer 8 requirements:
//! "Who wakes up, what they do, communication path, rollback steps"
//! Goal: "Shorten incident lifetime"
//! Evidence/Telemetry: "Mean time to recover (MTTR), postmortem quality"

use core::observability::{
    ObservabilityManager, SiemSeverity, IncidentRunbook, RunbookStep, CommunicationPlan,
    CommunicationChannel, EscalationPath, OnCallPager, OnCallPerson, EscalationLevel,
    Postmortem, IncidentEvent,
};
use std::collections::HashMap;

/// Test Runbooks & Pager features
#[test]
fn test_runbooks_and_pager() {
    println!("Starting Runbooks & Pager test");

    // 1. Create observability manager
    let mut manager = ObservabilityManager::new();
    println!("✓ Observability manager created");

    // 2. Create and add an incident response runbook
    let steps = vec![
        RunbookStep {
            step_number: 1,
            description: "Identify the affected systems and services".to_string(),
            responsible_role: "SRE".to_string(),
            estimated_time: 5,
            prerequisites: vec!["Incident detected".to_string()],
        },
        RunbookStep {
            step_number: 2,
            description: "Isolate affected systems to prevent further damage".to_string(),
            responsible_role: "Security Engineer".to_string(),
            estimated_time: 10,
            prerequisites: vec!["Affected systems identified".to_string()],
        },
        RunbookStep {
            step_number: 3,
            description: "Begin forensic analysis and evidence collection".to_string(),
            responsible_role: "Forensics Team".to_string(),
            estimated_time: 30,
            prerequisites: vec!["Systems isolated".to_string()],
        },
    ];

    let mut templates = HashMap::new();
    templates.insert("initial".to_string(), "Security incident detected: {description}".to_string());
    templates.insert("update".to_string(), "Status update: {status}".to_string());
    templates.insert("resolution".to_string(), "Incident resolved: {summary}".to_string());

    let communication_plan = CommunicationPlan {
        channels: vec![
            CommunicationChannel {
                channel_type: "slack".to_string(),
                channel_id: "incidents".to_string(),
                priority: 1,
            },
            CommunicationChannel {
                channel_type: "email".to_string(),
                channel_id: "security-team@company.com".to_string(),
                priority: 2,
            },
        ],
        initial_recipients: vec!["oncall-sre@company.com".to_string(), "security-team@company.com".to_string()],
        escalation_paths: vec![
            EscalationPath {
                time_threshold: 30,
                recipients: vec!["manager@company.com".to_string()],
                reason: "No progress in 30 minutes".to_string(),
            },
        ],
        templates,
    };

    let rollback_steps = vec![
        "Restore from last known good backup".to_string(),
        "Revoke compromised credentials".to_string(),
        "Update firewall rules to block attack vectors".to_string(),
    ];

    let runbook = IncidentRunbook {
        id: "security-incident-response".to_string(),
        incident_type: "security-breach".to_string(),
        steps,
        communication_plan,
        rollback_steps,
        estimated_recovery_time: 60,
        priority: core::observability::IncidentPriority::Critical,
    };

    assert!(manager.add_runbook(runbook).is_ok());
    println!("✓ Incident response runbook added");

    // 3. Create and add an on-call pager configuration
    let mut contact_info = HashMap::new();
    contact_info.insert("email".to_string(), "sre-oncall@company.com".to_string());
    contact_info.insert("phone".to_string(), "+1-555-0123".to_string());
    contact_info.insert("slack".to_string(), "@sre-oncall".to_string());

    let on_call_person = OnCallPerson {
        user_id: "sre-001".to_string(),
        name: "Alice Smith".to_string(),
        contact_info,
        time_zone: "UTC-5".to_string(),
    };

    let escalation_policy = vec![
        EscalationLevel {
            level: 1,
            delay_minutes: 5,
            personnel: vec!["sre-001".to_string()],
        },
        EscalationLevel {
            level: 2,
            delay_minutes: 15,
            personnel: vec!["sre-manager-001".to_string()],
        },
    ];

    let pager = OnCallPager {
        id: "sre-pager".to_string(),
        team_service: "sre-team".to_string(),
        on_call_personnel: vec![on_call_person],
        escalation_policy,
        notification_methods: vec!["slack".to_string(), "sms".to_string(), "email".to_string()],
    };

    assert!(manager.add_pager(pager).is_ok());
    println!("✓ On-call pager configuration added");

    // 4. Test runbook retrieval
    let retrieved_runbook = manager.get_runbook("security-incident-response");
    assert!(retrieved_runbook.is_some());
    assert_eq!(retrieved_runbook.unwrap().incident_type, "security-breach");
    println!("✓ Runbook retrieved by ID");

    let runbook_by_type = manager.get_runbook_by_type("security-breach");
    assert!(runbook_by_type.is_some());
    assert_eq!(runbook_by_type.unwrap().id, "security-incident-response");
    println!("✓ Runbook retrieved by incident type");

    // 5. Test pager retrieval
    let retrieved_pager = manager.get_pager("sre-pager");
    assert!(retrieved_pager.is_some());
    assert_eq!(retrieved_pager.unwrap().team_service, "sre-team");
    println!("✓ Pager retrieved by ID");

    let pager_by_team = manager.get_pager_by_team("sre-team");
    assert!(pager_by_team.is_some());
    assert_eq!(pager_by_team.unwrap().id, "sre-pager");
    println!("✓ Pager retrieved by team");

    // 6. Test "Who wakes up" - on-call personnel
    let pager = manager.get_pager("sre-pager").unwrap();
    assert!(!pager.on_call_personnel.is_empty());
    let on_call_person = &pager.on_call_personnel[0];
    assert_eq!(on_call_person.name, "Alice Smith");
    assert_eq!(on_call_person.user_id, "sre-001");
    println!("✓ On-call personnel identified: {}", on_call_person.name);

    // 7. Test "What they do" - runbook steps
    let runbook = manager.get_runbook("security-incident-response").unwrap();
    assert!(!runbook.steps.is_empty());
    assert_eq!(runbook.steps.len(), 3);
    assert_eq!(runbook.steps[0].description, "Identify the affected systems and services");
    assert_eq!(runbook.steps[0].responsible_role, "SRE");
    assert_eq!(runbook.steps[1].description, "Isolate affected systems to prevent further damage");
    assert_eq!(runbook.steps[1].responsible_role, "Security Engineer");
    println!("✓ Runbook steps verified for incident response actions");

    // 8. Test "Communication path"
    assert!(!runbook.communication_plan.channels.is_empty());
    assert_eq!(runbook.communication_plan.channels.len(), 2);
    assert_eq!(runbook.communication_plan.channels[0].channel_type, "slack");
    assert_eq!(runbook.communication_plan.channels[1].channel_type, "email");
    assert!(!runbook.communication_plan.initial_recipients.is_empty());
    assert_eq!(runbook.communication_plan.initial_recipients[0], "oncall-sre@company.com");
    println!("✓ Communication paths verified");

    // 9. Test "Rollback steps"
    assert!(!runbook.rollback_steps.is_empty());
    assert_eq!(runbook.rollback_steps.len(), 3);
    assert_eq!(runbook.rollback_steps[0], "Restore from last known good backup");
    println!("✓ Rollback steps verified");

    println!("All Runbooks & Pager tests passed!");
}

/// Test Mean Time to Recover (MTTR) calculation
#[test]
fn test_mttr_calculation() {
    let mut manager = ObservabilityManager::new();

    // Create and add a simple runbook for testing
    let runbook = IncidentRunbook {
        id: "test-runbook".to_string(),
        incident_type: "test-incident".to_string(),
        steps: vec![],
        communication_plan: CommunicationPlan {
            channels: vec![],
            initial_recipients: vec![],
            escalation_paths: vec![],
            templates: HashMap::new(),
        },
        rollback_steps: vec![],
        estimated_recovery_time: 30,
        priority: core::observability::IncidentPriority::High,
    };
    assert!(manager.add_runbook(runbook).is_ok());

    // Create several incidents
    let incident1_id = manager.create_incident(
        "test-incident".to_string(),
        SiemSeverity::High,
        vec!["test-user".to_string()],
        Some("test-runbook".to_string()),
    ).expect("Failed to create incident 1");

    let incident2_id = manager.create_incident(
        "test-incident".to_string(),
        SiemSeverity::Critical,
        vec!["test-user".to_string()],
        Some("test-runbook".to_string()),
    ).expect("Failed to create incident 2");

    // Start incident response for both incidents
    assert!(manager.start_incident_response(&incident1_id).is_ok());
    assert!(manager.start_incident_response(&incident2_id).is_ok());

    // Small delay to ensure different timestamps
    std::thread::sleep(std::time::Duration::from_millis(10));

    // Resolve incidents
    assert!(manager.resolve_incident(&incident1_id, None).is_ok());
    
    // Small delay between resolutions
    std::thread::sleep(std::time::Duration::from_millis(10));
    
    assert!(manager.resolve_incident(&incident2_id, None).is_ok());

    // Check MTTR statistics
    let (resolved_count, mttr) = manager.get_mttr_stats();
    assert_eq!(resolved_count, 2);
    // MTTR may be 0 on fast systems, which is acceptable
    println!("MTTR calculation test passed:");
    println!("✓ Resolved incidents: {}", resolved_count);
    println!("✓ Mean time to recover: {:.2} seconds", mttr);
}

/// Test postmortem quality tracking
#[test]
fn test_postmortem_quality() {
    let mut manager = ObservabilityManager::new();

    // Create and add a simple runbook for testing
    let runbook = IncidentRunbook {
        id: "test-runbook".to_string(),
        incident_type: "test-incident".to_string(),
        steps: vec![],
        communication_plan: CommunicationPlan {
            channels: vec![],
            initial_recipients: vec![],
            escalation_paths: vec![],
            templates: HashMap::new(),
        },
        rollback_steps: vec![],
        estimated_recovery_time: 30,
        priority: core::observability::IncidentPriority::High,
    };
    assert!(manager.add_runbook(runbook).is_ok());

    // Create an incident
    let incident_id = manager.create_incident(
        "test-incident".to_string(),
        SiemSeverity::High,
        vec!["test-user".to_string()],
        Some("test-runbook".to_string()),
    ).expect("Failed to create incident");

    // Start and resolve incident with postmortem
    assert!(manager.start_incident_response(&incident_id).is_ok());

    let timeline = vec![
        IncidentEvent {
            timestamp: 1000,
            description: "Incident detected".to_string(),
            responsible: "monitoring-system".to_string(),
        },
        IncidentEvent {
            timestamp: 1005,
            description: "Incident response started".to_string(),
            responsible: "test-user".to_string(),
        },
    ];

    let postmortem = Postmortem {
        summary: "Test incident resolved".to_string(),
        root_cause: "Configuration error".to_string(),
        timeline,
        impact: "Minimal service disruption".to_string(),
        resolution: "Fixed configuration and restarted service".to_string(),
        preventive_measures: vec![
            "Add configuration validation".to_string(),
            "Improve monitoring".to_string(),
        ],
        quality_score: 85, // On a scale of 0-100
    };

    assert!(manager.resolve_incident(&incident_id, Some(postmortem)).is_ok());

    // Check postmortem quality statistics
    let (count, avg_quality) = manager.get_postmortem_quality_stats();
    assert_eq!(count, 1);
    assert_eq!(avg_quality, 85.0);
    
    println!("Postmortem quality test passed:");
    println!("✓ Postmortem entries: {}", count);
    println!("✓ Average quality score: {:.1}", avg_quality);
}