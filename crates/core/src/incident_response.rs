//! Incident Response Module
//!
//! This module implements incident response measures including pause/kill switch functionality,
//! backup and snapshot management, restore jobs, and communication plans.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a pause/kill switch configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PauseKillSwitch {
    /// Unique identifier for the switch
    pub id: String,
    /// Target component (contract, service, etc.)
    pub target: String,
    /// Current state (active/inactive)
    pub active: bool,
    /// Reason for activation
    pub reason: Option<String>,
    /// Timestamp of last activation
    pub activated_at: Option<u64>,
    /// Expiration timestamp (if applicable)
    pub expires_at: Option<u64>,
    /// Authorized roles for activation
    pub authorized_roles: Vec<String>,
}

/// Represents a backup configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Backup {
    /// Unique identifier for the backup
    pub id: String,
    /// Backup type (database, config, state, etc.)
    pub backup_type: BackupType,
    /// Timestamp of creation
    pub created_at: u64,
    /// Backup location
    pub location: String,
    /// Backup size in bytes
    pub size: u64,
    /// Backup status
    pub status: BackupStatus,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

/// Types of backups
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BackupType {
    Database,
    Configuration,
    State,
    Keys,
    Archive,
}

/// Status of a backup
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BackupStatus {
    Created,
    InProgress,
    Completed,
    Failed,
    Expired,
}

/// Represents a restore job
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RestoreJob {
    /// Unique identifier for the job
    pub id: String,
    /// Backup to restore from
    pub backup_id: String,
    /// Target component
    pub target: String,
    /// Timestamp of initiation
    pub initiated_at: u64,
    /// Timestamp of completion
    pub completed_at: Option<u64>,
    /// Job status
    pub status: RestoreStatus,
    /// Error message (if failed)
    pub error_message: Option<String>,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

/// Status of a restore job
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RestoreStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

/// Represents a communication plan
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommunicationPlan {
    /// Unique identifier for the plan
    pub id: String,
    /// Incident type
    pub incident_type: String,
    /// Communication channels
    pub channels: Vec<CommunicationChannel>,
    /// Recipients
    pub recipients: Vec<String>,
    /// Template messages
    pub templates: HashMap<String, String>,
    /// Escalation procedures
    pub escalation_procedures: Vec<EscalationStep>,
}

/// Communication channel
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommunicationChannel {
    /// Channel type (email, slack, etc.)
    pub channel_type: String,
    /// Channel identifier
    pub channel_id: String,
    /// Priority level
    pub priority: u8,
}

/// Escalation step
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EscalationStep {
    /// Step number
    pub step: u8,
    /// Time threshold (minutes)
    pub time_threshold: u32,
    /// Recipients to notify
    pub recipients: Vec<String>,
    /// Action to take
    pub action: String,
}

/// Error types for incident response operations
#[derive(Debug)]
pub enum IncidentResponseError {
    /// Configuration error
    ConfigError(String),
    /// Operation error
    OperationError(String),
    /// Authorization error
    AuthError(String),
    /// Validation error
    ValidationError(String),
}

impl fmt::Display for IncidentResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IncidentResponseError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            IncidentResponseError::OperationError(msg) => write!(f, "Operation error: {}", msg),
            IncidentResponseError::AuthError(msg) => write!(f, "Authorization error: {}", msg),
            IncidentResponseError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for IncidentResponseError {}

/// Manages incident response components
pub struct IncidentResponseManager {
    /// Pause/kill switches
    pause_kill_switches: HashMap<String, PauseKillSwitch>,
    /// Backups
    backups: HashMap<String, Backup>,
    /// Restore jobs
    restore_jobs: HashMap<String, RestoreJob>,
    /// Communication plans
    communication_plans: HashMap<String, CommunicationPlan>,
    /// Audit log of actions
    audit_log: Vec<IncidentAction>,
}

/// Represents an incident action in the audit log
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IncidentAction {
    /// Unique identifier for the action
    pub id: String,
    /// Action type
    pub action_type: String,
    /// User who performed the action
    pub user: String,
    /// Timestamp of the action
    pub timestamp: u64,
    /// Details of the action
    pub details: HashMap<String, String>,
}

impl IncidentResponseManager {
    /// Creates a new incident response manager
    pub fn new() -> Self {
        Self {
            pause_kill_switches: HashMap::new(),
            backups: HashMap::new(),
            restore_jobs: HashMap::new(),
            communication_plans: HashMap::new(),
            audit_log: Vec::new(),
        }
    }

    /// Adds a pause/kill switch
    pub fn add_pause_kill_switch(&mut self, switch: PauseKillSwitch) -> Result<(), IncidentResponseError> {
        if switch.id.is_empty() {
            return Err(IncidentResponseError::ConfigError("Switch ID cannot be empty".to_string()));
        }
        
        if switch.target.is_empty() {
            return Err(IncidentResponseError::ConfigError("Target cannot be empty".to_string()));
        }
        
        if switch.authorized_roles.is_empty() {
            return Err(IncidentResponseError::ConfigError("At least one authorized role is required".to_string()));
        }
        
        self.pause_kill_switches.insert(switch.id.clone(), switch);
        Ok(())
    }

    /// Gets a pause/kill switch by ID
    pub fn get_pause_kill_switch(&self, id: &str) -> Option<&PauseKillSwitch> {
        self.pause_kill_switches.get(id)
    }

    /// Activates a pause/kill switch
    pub fn activate_pause_kill_switch(
        &mut self,
        id: &str,
        user: String,
        reason: String,
    ) -> Result<(), IncidentResponseError> {
        let switch = self.pause_kill_switches.get_mut(id).ok_or_else(|| {
            IncidentResponseError::OperationError("Switch not found".to_string())
        })?;
        
        // In a real implementation, we would check user authorization here
        // For now, we'll just log the action
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| IncidentResponseError::OperationError(format!("Time error: {}", e)))?
            .as_secs();
        
        switch.active = true;
        switch.reason = Some(reason.clone());
        switch.activated_at = Some(timestamp);
        
        // Log the action
        let mut details = HashMap::new();
        details.insert("switch_id".to_string(), id.to_string());
        details.insert("reason".to_string(), reason);
        
        let action = IncidentAction {
            id: uuid::Uuid::new_v4().to_string(),
            action_type: "activate_pause_kill_switch".to_string(),
            user,
            timestamp,
            details,
        };
        
        self.audit_log.push(action);
        
        Ok(())
    }

    /// Deactivates a pause/kill switch
    pub fn deactivate_pause_kill_switch(
        &mut self,
        id: &str,
        user: String,
    ) -> Result<(), IncidentResponseError> {
        let switch = self.pause_kill_switches.get_mut(id).ok_or_else(|| {
            IncidentResponseError::OperationError("Switch not found".to_string())
        })?;
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| IncidentResponseError::OperationError(format!("Time error: {}", e)))?
            .as_secs();
        
        switch.active = false;
        switch.reason = None;
        switch.activated_at = None;
        switch.expires_at = None;
        
        // Log the action
        let mut details = HashMap::new();
        details.insert("switch_id".to_string(), id.to_string());
        
        let action = IncidentAction {
            id: uuid::Uuid::new_v4().to_string(),
            action_type: "deactivate_pause_kill_switch".to_string(),
            user,
            timestamp,
            details,
        };
        
        self.audit_log.push(action);
        
        Ok(())
    }

    /// Adds a backup
    pub fn add_backup(&mut self, backup: Backup) -> Result<(), IncidentResponseError> {
        if backup.id.is_empty() {
            return Err(IncidentResponseError::ConfigError("Backup ID cannot be empty".to_string()));
        }
        
        if backup.location.is_empty() {
            return Err(IncidentResponseError::ConfigError("Backup location cannot be empty".to_string()));
        }
        
        self.backups.insert(backup.id.clone(), backup);
        Ok(())
    }

    /// Gets a backup by ID
    pub fn get_backup(&self, id: &str) -> Option<&Backup> {
        self.backups.get(id)
    }

    /// Lists backups by type
    pub fn list_backups_by_type(&self, backup_type: &BackupType) -> Vec<&Backup> {
        self.backups
            .values()
            .filter(|backup| &backup.backup_type == backup_type)
            .collect()
    }

    /// Adds a restore job
    pub fn add_restore_job(&mut self, job: RestoreJob) -> Result<(), IncidentResponseError> {
        if job.id.is_empty() {
            return Err(IncidentResponseError::ConfigError("Job ID cannot be empty".to_string()));
        }
        
        if job.backup_id.is_empty() {
            return Err(IncidentResponseError::ConfigError("Backup ID cannot be empty".to_string()));
        }
        
        self.restore_jobs.insert(job.id.clone(), job);
        Ok(())
    }

    /// Gets a restore job by ID
    pub fn get_restore_job(&self, id: &str) -> Option<&RestoreJob> {
        self.restore_jobs.get(id)
    }

    /// Updates restore job status
    pub fn update_restore_job_status(
        &mut self,
        id: &str,
        status: RestoreStatus,
        error_message: Option<String>,
    ) -> Result<(), IncidentResponseError> {
        let job = self.restore_jobs.get_mut(id).ok_or_else(|| {
            IncidentResponseError::OperationError("Restore job not found".to_string())
        })?;
        
        // Check if status indicates completion before moving it
        let is_completed = status == RestoreStatus::Completed || status == RestoreStatus::Failed;
        
        job.status = status;
        
        if is_completed {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| IncidentResponseError::OperationError(format!("Time error: {}", e)))?
                .as_secs();
            job.completed_at = Some(timestamp);
        }
        
        job.error_message = error_message;
        
        Ok(())
    }

    /// Adds a communication plan
    pub fn add_communication_plan(&mut self, plan: CommunicationPlan) -> Result<(), IncidentResponseError> {
        if plan.id.is_empty() {
            return Err(IncidentResponseError::ConfigError("Plan ID cannot be empty".to_string()));
        }
        
        if plan.incident_type.is_empty() {
            return Err(IncidentResponseError::ConfigError("Incident type cannot be empty".to_string()));
        }
        
        self.communication_plans.insert(plan.id.clone(), plan);
        Ok(())
    }

    /// Gets a communication plan by ID
    pub fn get_communication_plan(&self, id: &str) -> Option<&CommunicationPlan> {
        self.communication_plans.get(id)
    }

    /// Gets communication plan by incident type
    pub fn get_communication_plan_by_type(&self, incident_type: &str) -> Option<&CommunicationPlan> {
        self.communication_plans
            .values()
            .find(|plan| plan.incident_type == incident_type)
    }

    /// Gets audit log entries
    pub fn get_audit_log(&self, limit: Option<usize>) -> Vec<&IncidentAction> {
        let mut log_entries: Vec<&IncidentAction> = self.audit_log.iter().collect();
        log_entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        if let Some(limit) = limit {
            log_entries.truncate(limit);
        }
        
        log_entries
    }

    /// Validates the incident response configuration
    pub fn validate_configuration(&self) -> Result<(), IncidentResponseError> {
        // Validate that we have at least one pause/kill switch
        if self.pause_kill_switches.is_empty() {
            return Err(IncidentResponseError::ValidationError("At least one pause/kill switch is required".to_string()));
        }
        
        // Validate that we have at least one backup
        if self.backups.is_empty() {
            return Err(IncidentResponseError::ValidationError("At least one backup is required".to_string()));
        }
        
        // Validate that we have at least one communication plan
        if self.communication_plans.is_empty() {
            return Err(IncidentResponseError::ValidationError("At least one communication plan is required".to_string()));
        }
        
        Ok(())
    }
}

impl Default for IncidentResponseManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pause_kill_switch_creation() {
        let switch = PauseKillSwitch {
            id: "test-switch".to_string(),
            target: "smart-contract".to_string(),
            active: false,
            reason: None,
            activated_at: None,
            expires_at: None,
            authorized_roles: vec!["admin".to_string(), "security".to_string()],
        };
        
        assert_eq!(switch.id, "test-switch");
        assert_eq!(switch.authorized_roles.len(), 2);
    }

    #[test]
    fn test_backup_creation() {
        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), "1.0.0".to_string());
        
        let backup = Backup {
            id: "backup-123".to_string(),
            backup_type: BackupType::Database,
            created_at: 1234567890,
            location: "/backups/db-2023-01-01.sql".to_string(),
            size: 1024000,
            status: BackupStatus::Completed,
            metadata,
        };
        
        assert_eq!(backup.id, "backup-123");
        assert!(matches!(backup.backup_type, BackupType::Database));
        assert_eq!(backup.metadata.len(), 1);
    }

    #[test]
    fn test_restore_job_creation() {
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), "production".to_string());
        
        let job = RestoreJob {
            id: "restore-456".to_string(),
            backup_id: "backup-123".to_string(),
            target: "database".to_string(),
            initiated_at: 1234567890,
            completed_at: None,
            status: RestoreStatus::Pending,
            error_message: None,
            metadata,
        };
        
        assert_eq!(job.id, "restore-456");
        assert!(matches!(job.status, RestoreStatus::Pending));
    }

    #[test]
    fn test_communication_plan_creation() {
        let channels = vec![
            CommunicationChannel {
                channel_type: "email".to_string(),
                channel_id: "team@company.com".to_string(),
                priority: 1,
            },
            CommunicationChannel {
                channel_type: "slack".to_string(),
                channel_id: "incident-channel".to_string(),
                priority: 2,
            }
        ];
        
        let mut templates = HashMap::new();
        templates.insert("initial".to_string(), "Incident detected: {description}".to_string());
        templates.insert("update".to_string(), "Update: {status}".to_string());
        
        let escalation_steps = vec![
            EscalationStep {
                step: 1,
                time_threshold: 30,
                recipients: vec!["oncall@company.com".to_string()],
                action: "Notify on-call team".to_string(),
            }
        ];
        
        let plan = CommunicationPlan {
            id: "security-incident-plan".to_string(),
            incident_type: "security".to_string(),
            channels,
            recipients: vec!["security-team@company.com".to_string()],
            templates,
            escalation_procedures: escalation_steps,
        };
        
        assert_eq!(plan.id, "security-incident-plan");
        assert_eq!(plan.channels.len(), 2);
        assert_eq!(plan.templates.len(), 2);
    }

    #[test]
    fn test_incident_response_manager() {
        let mut manager = IncidentResponseManager::new();
        
        // Test adding pause/kill switch
        let switch = PauseKillSwitch {
            id: "contract-pause".to_string(),
            target: "smart-contract".to_string(),
            active: false,
            reason: None,
            activated_at: None,
            expires_at: None,
            authorized_roles: vec!["admin".to_string()],
        };
        
        assert!(manager.add_pause_kill_switch(switch).is_ok());
        assert!(manager.get_pause_kill_switch("contract-pause").is_some());
        
        // Test adding backup
        let backup = Backup {
            id: "db-backup-1".to_string(),
            backup_type: BackupType::Database,
            created_at: 1234567890,
            location: "/backups/db.sql".to_string(),
            size: 1024000,
            status: BackupStatus::Completed,
            metadata: HashMap::new(),
        };
        
        assert!(manager.add_backup(backup).is_ok());
        assert!(manager.get_backup("db-backup-1").is_some());
        
        // Test adding restore job
        let job = RestoreJob {
            id: "restore-1".to_string(),
            backup_id: "db-backup-1".to_string(),
            target: "database".to_string(),
            initiated_at: 1234567890,
            completed_at: None,
            status: RestoreStatus::Pending,
            error_message: None,
            metadata: HashMap::new(),
        };
        
        assert!(manager.add_restore_job(job).is_ok());
        assert!(manager.get_restore_job("restore-1").is_some());
        
        // Test adding communication plan
        let plan = CommunicationPlan {
            id: "security-plan".to_string(),
            incident_type: "security".to_string(),
            channels: vec![],
            recipients: vec![],
            templates: HashMap::new(),
            escalation_procedures: vec![],
        };
        
        assert!(manager.add_communication_plan(plan).is_ok());
        assert!(manager.get_communication_plan("security-plan").is_some());
        
        // Test validation
        assert!(manager.validate_configuration().is_ok());
    }

    #[test]
    fn test_pause_kill_switch_activation() {
        let mut manager = IncidentResponseManager::new();
        
        let switch = PauseKillSwitch {
            id: "test-switch".to_string(),
            target: "smart-contract".to_string(),
            active: false,
            reason: None,
            activated_at: None,
            expires_at: None,
            authorized_roles: vec!["admin".to_string()],
        };
        
        assert!(manager.add_pause_kill_switch(switch).is_ok());
        
        // Activate the switch
        assert!(manager.activate_pause_kill_switch(
            "test-switch",
            "admin-user".to_string(),
            "Security incident detected".to_string()
        ).is_ok());
        
        let switch = manager.get_pause_kill_switch("test-switch").unwrap();
        assert!(switch.active);
        assert!(switch.reason.is_some());
        assert!(switch.activated_at.is_some());
        
        // Check audit log
        let audit_log = manager.get_audit_log(None);
        assert_eq!(audit_log.len(), 1);
        assert_eq!(audit_log[0].action_type, "activate_pause_kill_switch");
        
        // Deactivate the switch
        assert!(manager.deactivate_pause_kill_switch(
            "test-switch",
            "admin-user".to_string()
        ).is_ok());
        
        let switch = manager.get_pause_kill_switch("test-switch").unwrap();
        assert!(!switch.active);
        assert!(switch.reason.is_none());
        assert!(switch.activated_at.is_none());
        
        // Check audit log again
        let audit_log = manager.get_audit_log(None);
        assert_eq!(audit_log.len(), 2);
    }

    #[test]
    fn test_invalid_configurations() {
        let mut manager = IncidentResponseManager::new();
        
        // Test invalid pause/kill switch (empty ID)
        let switch = PauseKillSwitch {
            id: "".to_string(),
            target: "smart-contract".to_string(),
            active: false,
            reason: None,
            activated_at: None,
            expires_at: None,
            authorized_roles: vec!["admin".to_string()],
        };
        
        assert!(manager.add_pause_kill_switch(switch).is_err());
        
        // Test invalid backup (empty location)
        let backup = Backup {
            id: "backup-1".to_string(),
            backup_type: BackupType::Database,
            created_at: 1234567890,
            location: "".to_string(),
            size: 1024000,
            status: BackupStatus::Completed,
            metadata: HashMap::new(),
        };
        
        assert!(manager.add_backup(backup).is_err());
        
        // Test validation with empty manager
        let empty_manager = IncidentResponseManager::new();
        assert!(empty_manager.validate_configuration().is_err());
    }
}