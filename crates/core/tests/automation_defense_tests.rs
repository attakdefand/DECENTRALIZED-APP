//! Tests for Automation & Continuous Defense features
//!
//! This file validates the implementation of Layer 7 requirements:
//! "Automate safe ops; reduce human error"
//! Goal: "Automate safe ops; reduce human error"
//! Evidence/Telemetry: Automated workflows, policy enforcement systems

#[cfg(test)]
mod tests {
    use core::{
        automated_remediation::*,
        policy_enforcement::*,
    };
    use std::collections::HashMap;

    /// Test Automated Remediation features
    #[test]
    fn test_automated_remediation_features() {
        println!("Starting Automated Remediation test");

        // 1. Create automated remediation manager
        let mut manager = AutomatedRemediationManager::new();
        println!("✓ Automated remediation manager created");

        // 2. Create remediation actions
        let password_reset_action = RemediationAction {
            id: "action-1".to_string(),
            name: "Password Reset".to_string(),
            description: "Reset user password".to_string(),
            action_type: RemediationActionType::PasswordReset,
            config: {
                let mut config = HashMap::new();
                config.insert("reset_method".to_string(), "email".to_string());
                config.insert("notification_template".to_string(), "password_reset".to_string());
                config
            },
            enabled: true,
            priority: 8,
        };

        let session_invalidation_action = RemediationAction {
            id: "action-2".to_string(),
            name: "Session Invalidation".to_string(),
            description: "Invalidate all user sessions".to_string(),
            action_type: RemediationActionType::SessionInvalidation,
            config: {
                let mut config = HashMap::new();
                config.insert("invalidate_all".to_string(), "true".to_string());
                config
            },
            enabled: true,
            priority: 9,
        };

        println!("✓ Remediation actions created");

        // 3. Create remediation workflow
        let workflow = RemediationWorkflow {
            id: "workflow-1".to_string(),
            name: "Security Incident Response".to_string(),
            description: "Automated response to security incidents".to_string(),
            triggers: vec![],
            actions: vec![password_reset_action, session_invalidation_action],
            enabled: true,
            max_retries: 3,
        };

        // 4. Register workflow
        assert!(manager.register_workflow(workflow).is_ok());
        println!("✓ Remediation workflow registered");

        // 5. Test workflow retrieval
        let retrieved_workflow = manager.get_workflow("workflow-1").unwrap();
        assert_eq!(retrieved_workflow.name, "Security Incident Response");
        assert_eq!(retrieved_workflow.actions.len(), 2);
        println!("✓ Workflow retrieval verified");

        // 6. Execute workflow
        let mut context = HashMap::new();
        context.insert("user_id".to_string(), "user-123".to_string());
        context.insert("incident_type".to_string(), "suspicious_login".to_string());
        context.insert("ip_address".to_string(), "192.168.1.100".to_string());

        let execution_result = manager.execute_workflow("workflow-1", context);
        assert!(execution_result.is_ok());
        println!("✓ Workflow execution successful");

        // 7. Check execution history
        let execution_history = manager.get_execution_history();
        assert_eq!(execution_history.len(), 1);
        assert_eq!(execution_history[0].workflow_id, "workflow-1");
        assert_eq!(execution_history[0].status, RemediationStatus::Success);
        println!("✓ Execution history verified");

        // 8. Check statistics
        let (total, successful, failed, _avg_time) = manager.get_stats();
        assert_eq!(total, 1);
        assert_eq!(successful, 1);
        assert_eq!(failed, 0);
        println!("✓ Statistics verified");

        // 9. Test workflow enable/disable
        assert!(manager.disable_workflow("workflow-1").is_ok());
        let disabled_workflow = manager.get_workflow("workflow-1").unwrap();
        assert!(!disabled_workflow.enabled);

        assert!(manager.enable_workflow("workflow-1").is_ok());
        let enabled_workflow = manager.get_workflow("workflow-1").unwrap();
        assert!(enabled_workflow.enabled);
        println!("✓ Workflow enable/disable functionality verified");

        println!("✓ All automated remediation tests passed");
    }

    /// Test Policy Enforcement Pipeline features
    #[test]
    fn test_policy_enforcement_pipeline_features() {
        println!("Starting Policy Enforcement Pipeline test");

        // 1. Create policy enforcement manager
        let mut manager = PolicyEnforcementManager::new();
        println!("✓ Policy enforcement manager created");

        // 2. Create policy stages
        let evaluation_stage = PolicyStage {
            id: "stage-1".to_string(),
            name: "Policy Evaluation".to_string(),
            description: "Evaluate policy rules".to_string(),
            stage_type: PolicyStageType::PolicyEvaluation,
            config: {
                let mut config = HashMap::new();
                config.insert("engine".to_string(), "cedar".to_string());
                config.insert("timeout_ms".to_string(), "5000".to_string());
                config
            },
            enabled: true,
            order: 1,
        };

        let enforcement_stage = PolicyStage {
            id: "stage-2".to_string(),
            name: "Policy Enforcement".to_string(),
            description: "Enforce policy decisions".to_string(),
            stage_type: PolicyStageType::PolicyEnforcement,
            config: {
                let mut config = HashMap::new();
                config.insert("enforcement_mode".to_string(), "strict".to_string());
                config
            },
            enabled: true,
            order: 2,
        };

        println!("✓ Policy stages created");

        // 3. Create policy enforcement pipeline
        let pipeline = PolicyEnforcementPipeline {
            id: "pipeline-1".to_string(),
            name: "Access Control Pipeline".to_string(),
            description: "Enforce access control policies".to_string(),
            stages: vec![evaluation_stage, enforcement_stage],
            enabled: true,
            priority: 7,
        };

        // 4. Register pipeline
        assert!(manager.register_pipeline(pipeline).is_ok());
        println!("✓ Policy enforcement pipeline registered");

        // 5. Test pipeline retrieval
        let retrieved_pipeline = manager.get_pipeline("pipeline-1").unwrap();
        assert_eq!(retrieved_pipeline.name, "Access Control Pipeline");
        assert_eq!(retrieved_pipeline.stages.len(), 2);
        println!("✓ Pipeline retrieval verified");

        // 6. Execute pipeline
        let mut context = HashMap::new();
        context.insert("principal".to_string(), "user-456".to_string());
        context.insert("action".to_string(), "read".to_string());
        context.insert("resource".to_string(), "document-789".to_string());

        let execution_result = manager.execute_pipeline("pipeline-1", context);
        assert!(execution_result.is_ok());
        println!("✓ Pipeline execution successful");

        // 7. Check execution history
        let execution_history = manager.get_execution_history();
        assert_eq!(execution_history.len(), 1);
        assert_eq!(execution_history[0].pipeline_id, "pipeline-1");
        assert_eq!(execution_history[0].status, ExecutionStatus::Success);
        println!("✓ Execution history verified");

        // 8. Record policy decisions
        let allow_decision = PolicyDecision {
            id: "decision-1".to_string(),
            pipeline_id: "pipeline-1".to_string(),
            stage_id: "stage-1".to_string(),
            resource: "document-789".to_string(),
            principal: "user-456".to_string(),
            action: "read".to_string(),
            decision: DecisionOutcome::Allow,
            reason: "User has read access to document".to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            metadata: {
                let mut metadata = HashMap::new();
                metadata.insert("policy_id".to_string(), "access-policy-001".to_string());
                metadata.insert("rule_id".to_string(), "read-rule-001".to_string());
                metadata
            },
        };

        let deny_decision = PolicyDecision {
            id: "decision-2".to_string(),
            pipeline_id: "pipeline-1".to_string(),
            stage_id: "stage-1".to_string(),
            resource: "admin-panel".to_string(),
            principal: "user-456".to_string(),
            action: "delete".to_string(),
            decision: DecisionOutcome::Deny,
            reason: "User does not have delete permissions".to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            metadata: {
                let mut metadata = HashMap::new();
                metadata.insert("policy_id".to_string(), "access-policy-001".to_string());
                metadata.insert("rule_id".to_string(), "delete-rule-001".to_string());
                metadata
            },
        };

        assert!(manager.record_decision(allow_decision).is_ok());
        assert!(manager.record_decision(deny_decision).is_ok());
        println!("✓ Policy decisions recorded");

        // 9. Check decisions
        let decisions = manager.get_decisions();
        assert_eq!(decisions.len(), 2);
        println!("✓ Policy decisions verified");

        // 10. Check statistics
        let (total, successful, failed, violations, _avg_time) = manager.get_stats();
        assert_eq!(total, 1);
        assert_eq!(successful, 1);
        assert_eq!(failed, 0);
        assert_eq!(violations, 1); // One deny decision counts as violation
        println!("✓ Statistics verified");

        // 11. Test pipeline enable/disable
        assert!(manager.disable_pipeline("pipeline-1").is_ok());
        let disabled_pipeline = manager.get_pipeline("pipeline-1").unwrap();
        assert!(!disabled_pipeline.enabled);

        assert!(manager.enable_pipeline("pipeline-1").is_ok());
        let enabled_pipeline = manager.get_pipeline("pipeline-1").unwrap();
        assert!(enabled_pipeline.enabled);
        println!("✓ Pipeline enable/disable functionality verified");

        println!("✓ All policy enforcement pipeline tests passed");
    }

    /// Test integration between automated remediation and policy enforcement
    #[test]
    fn test_automation_defense_integration() {
        println!("Starting Automation & Defense integration test");

        // 1. Create both managers
        let mut remediation_manager = AutomatedRemediationManager::new();
        let mut policy_manager = PolicyEnforcementManager::new();
        println!("✓ Both managers created");

        // 2. Create a remediation workflow triggered by policy violations
        let block_ip_action = RemediationAction {
            id: "action-1".to_string(),
            name: "Block IP Address".to_string(),
            description: "Block suspicious IP address".to_string(),
            action_type: RemediationActionType::IpBlocking,
            config: {
                let mut config = HashMap::new();
                config.insert("duration_minutes".to_string(), "60".to_string());
                config
            },
            enabled: true,
            priority: 10,
        };

        let notify_security_team_action = RemediationAction {
            id: "action-2".to_string(),
            name: "Notify Security Team".to_string(),
            description: "Send alert to security team".to_string(),
            action_type: RemediationActionType::Notification,
            config: {
                let mut config = HashMap::new();
                config.insert("channel".to_string(), "slack-security".to_string());
                config.insert("template".to_string(), "policy_violation".to_string());
                config
            },
            enabled: true,
            priority: 5,
        };

        let remediation_workflow = RemediationWorkflow {
            id: "policy-violation-response".to_string(),
            name: "Policy Violation Response".to_string(),
            description: "Automated response to policy violations".to_string(),
            triggers: vec![],
            actions: vec![block_ip_action, notify_security_team_action],
            enabled: true,
            max_retries: 2,
        };

        assert!(remediation_manager.register_workflow(remediation_workflow).is_ok());
        println!("✓ Remediation workflow for policy violations registered");

        // 3. Create a policy enforcement pipeline
        let policy_stage = PolicyStage {
            id: "policy-eval".to_string(),
            name: "Policy Evaluation".to_string(),
            description: "Evaluate access control policies".to_string(),
            stage_type: PolicyStageType::PolicyEvaluation,
            config: HashMap::new(),
            enabled: true,
            order: 1,
        };

        let enforcement_stage = PolicyStage {
            id: "policy-enforce".to_string(),
            name: "Policy Enforcement".to_string(),
            description: "Enforce policy decisions".to_string(),
            stage_type: PolicyStageType::PolicyEnforcement,
            config: HashMap::new(),
            enabled: true,
            order: 2,
        };

        let pipeline = PolicyEnforcementPipeline {
            id: "access-control-pipeline".to_string(),
            name: "Access Control Pipeline".to_string(),
            description: "Enforce access control policies".to_string(),
            stages: vec![policy_stage, enforcement_stage],
            enabled: true,
            priority: 8,
        };

        assert!(policy_manager.register_pipeline(pipeline).is_ok());
        println!("✓ Policy enforcement pipeline registered");

        // 4. Simulate a policy violation that triggers automated remediation
        let deny_decision = PolicyDecision {
            id: "decision-violation-1".to_string(),
            pipeline_id: "access-control-pipeline".to_string(),
            stage_id: "policy-eval".to_string(),
            resource: "sensitive-data".to_string(),
            principal: "user-789".to_string(),
            action: "delete".to_string(),
            decision: DecisionOutcome::Deny,
            reason: "Unauthorized access attempt".to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            metadata: {
                let mut metadata = HashMap::new();
                metadata.insert("threat_level".to_string(), "high".to_string());
                metadata.insert("ip_address".to_string(), "203.0.113.45".to_string());
                metadata
            },
        };

        assert!(policy_manager.record_decision(deny_decision).is_ok());
        println!("✓ Policy violation recorded");

        // 5. Execute automated remediation based on the policy violation
        let mut remediation_context = HashMap::new();
        remediation_context.insert("violation_id".to_string(), "decision-violation-1".to_string());
        remediation_context.insert("ip_address".to_string(), "203.0.113.45".to_string());
        remediation_context.insert("threat_level".to_string(), "high".to_string());
        remediation_context.insert("resource".to_string(), "sensitive-data".to_string());

        let remediation_result = remediation_manager.execute_workflow("policy-violation-response", remediation_context);
        assert!(remediation_result.is_ok());
        println!("✓ Automated remediation executed for policy violation");

        // 6. Verify both systems tracked the interaction
        let (rem_total, rem_successful, _rem_failed, _rem_avg_time) = remediation_manager.get_stats();
        assert_eq!(rem_total, 1);
        assert_eq!(rem_successful, 1);

        let (pol_total, _pol_successful, _pol_failed, pol_violations, _pol_avg_time) = policy_manager.get_stats();
        assert_eq!(pol_total, 0); // No pipeline executions yet
        assert_eq!(pol_violations, 1); // But we did record a violation

        println!("✓ Integration between systems verified");

        println!("✓ All automation & defense integration tests passed");
    }
}