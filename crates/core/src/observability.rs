//! Observability Module
//!
//! This module implements observability measures including OpenTelemetry collector integration,
//! Prometheus alerting rules, SIEM rules, and admin audit logging.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Represents an OpenTelemetry collector configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtelCollector {
    /// Unique identifier for the collector
    pub id: String,
    /// Endpoint URL for the collector
    pub endpoint: String,
    /// Enabled telemetry types (traces, metrics, logs)
    pub telemetry_types: Vec<String>,
    /// Sampling configuration
    pub sampling_rate: f64,
    /// Export interval in seconds
    pub export_interval: u64,
}

/// Represents a Prometheus alerting rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrometheusRule {
    /// Unique identifier for the rule
    pub id: String,
    /// Rule expression
    pub expr: String,
    /// Alert duration threshold
    pub for_duration: String,
    /// Alert labels
    pub labels: HashMap<String, String>,
    /// Alert annotations
    pub annotations: HashMap<String, String>,
}

/// Represents a SIEM (Security Information and Event Management) rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiemRule {
    /// Unique identifier for the rule
    pub id: String,
    /// Rule description
    pub description: String,
    /// Detection criteria
    pub criteria: String,
    /// Severity level
    pub severity: SiemSeverity,
    /// Enabled status
    pub enabled: bool,
}

/// Severity levels for SIEM rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SiemSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Represents an administrative audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminAuditLog {
    /// Unique identifier for the log entry
    pub id: String,
    /// Timestamp of the action
    pub timestamp: u64,
    /// User who performed the action
    pub user: String,
    /// Action performed
    pub action: String,
    /// Target of the action
    pub target: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    /// IP address of the requester
    pub ip_address: Option<String>,
}

/// Error types for observability operations
#[derive(Debug)]
pub enum ObservabilityError {
    /// Configuration error
    ConfigError(String),
    /// Export error
    ExportError(String),
    /// Validation error
    ValidationError(String),
}

impl fmt::Display for ObservabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObservabilityError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            ObservabilityError::ExportError(msg) => write!(f, "Export error: {}", msg),
            ObservabilityError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for ObservabilityError {}

/// Manages observability components
pub struct ObservabilityManager {
    /// OpenTelemetry collectors
    otel_collectors: HashMap<String, OtelCollector>,
    /// Prometheus rules
    prometheus_rules: HashMap<String, PrometheusRule>,
    /// SIEM rules
    siem_rules: HashMap<String, SiemRule>,
    /// Admin audit logs
    audit_logs: Vec<AdminAuditLog>,
}

impl ObservabilityManager {
    /// Creates a new observability manager
    pub fn new() -> Self {
        Self {
            otel_collectors: HashMap::new(),
            prometheus_rules: HashMap::new(),
            siem_rules: HashMap::new(),
            audit_logs: Vec::new(),
        }
    }

    /// Adds an OpenTelemetry collector
    pub fn add_otel_collector(
        &mut self,
        collector: OtelCollector,
    ) -> Result<(), ObservabilityError> {
        if collector.endpoint.is_empty() {
            return Err(ObservabilityError::ConfigError(
                "Endpoint cannot be empty".to_string(),
            ));
        }

        if collector.sampling_rate < 0.0 || collector.sampling_rate > 1.0 {
            return Err(ObservabilityError::ConfigError(
                "Sampling rate must be between 0.0 and 1.0".to_string(),
            ));
        }

        self.otel_collectors.insert(collector.id.clone(), collector);
        Ok(())
    }

    /// Gets an OpenTelemetry collector by ID
    pub fn get_otel_collector(&self, id: &str) -> Option<&OtelCollector> {
        self.otel_collectors.get(id)
    }

    /// Adds a Prometheus rule
    pub fn add_prometheus_rule(&mut self, rule: PrometheusRule) -> Result<(), ObservabilityError> {
        if rule.expr.is_empty() {
            return Err(ObservabilityError::ConfigError(
                "Expression cannot be empty".to_string(),
            ));
        }

        self.prometheus_rules.insert(rule.id.clone(), rule);
        Ok(())
    }

    /// Gets a Prometheus rule by ID
    pub fn get_prometheus_rule(&self, id: &str) -> Option<&PrometheusRule> {
        self.prometheus_rules.get(id)
    }

    /// Adds a SIEM rule
    pub fn add_siem_rule(&mut self, rule: SiemRule) -> Result<(), ObservabilityError> {
        self.siem_rules.insert(rule.id.clone(), rule);
        Ok(())
    }

    /// Gets a SIEM rule by ID
    pub fn get_siem_rule(&self, id: &str) -> Option<&SiemRule> {
        self.siem_rules.get(id)
    }

    /// Logs an administrative action
    pub fn log_admin_action(
        &mut self,
        user: String,
        action: String,
        target: String,
        metadata: HashMap<String, String>,
        ip_address: Option<String>,
    ) -> Result<String, ObservabilityError> {
        let id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| ObservabilityError::ExportError(format!("Time error: {}", e)))?
            .as_secs();

        let log_entry = AdminAuditLog {
            id: id.clone(),
            timestamp,
            user,
            action,
            target,
            metadata,
            ip_address,
        };

        self.audit_logs.push(log_entry);
        Ok(id)
    }

    /// Gets audit logs with optional filtering
    pub fn get_audit_logs(&self, user_filter: Option<&str>) -> Vec<&AdminAuditLog> {
        if let Some(user) = user_filter {
            self.audit_logs
                .iter()
                .filter(|log| log.user == user)
                .collect()
        } else {
            self.audit_logs.iter().collect()
        }
    }

    /// Validates the observability configuration
    pub fn validate_configuration(&self) -> Result<(), ObservabilityError> {
        // Validate that we have at least one collector
        if self.otel_collectors.is_empty() {
            return Err(ObservabilityError::ValidationError(
                "At least one OpenTelemetry collector is required".to_string(),
            ));
        }

        // Validate that we have at least one alerting rule
        if self.prometheus_rules.is_empty() {
            return Err(ObservabilityError::ValidationError(
                "At least one Prometheus rule is required".to_string(),
            ));
        }

        // Validate that we have at least one SIEM rule
        if self.siem_rules.is_empty() {
            return Err(ObservabilityError::ValidationError(
                "At least one SIEM rule is required".to_string(),
            ));
        }

        Ok(())
    }
}

impl Default for ObservabilityManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_otel_collector_creation() {
        let collector = OtelCollector {
            id: "test-collector".to_string(),
            endpoint: "http://localhost:4317".to_string(),
            telemetry_types: vec!["traces".to_string(), "metrics".to_string()],
            sampling_rate: 0.1,
            export_interval: 30,
        };

        assert_eq!(collector.id, "test-collector");
        assert_eq!(collector.telemetry_types.len(), 2);
    }

    #[test]
    fn test_prometheus_rule_creation() {
        let mut labels = HashMap::new();
        labels.insert("severity".to_string(), "critical".to_string());

        let mut annotations = HashMap::new();
        annotations.insert("summary".to_string(), "High latency detected".to_string());

        let rule = PrometheusRule {
            id: "high-latency".to_string(),
            expr: "rate(http_request_duration_seconds_sum[5m]) / rate(http_request_duration_seconds_count[5m]) > 1".to_string(),
            for_duration: "5m".to_string(),
            labels,
            annotations,
        };

        assert_eq!(rule.id, "high-latency");
        assert_eq!(rule.labels.len(), 1);
    }

    #[test]
    fn test_siem_rule_creation() {
        let rule = SiemRule {
            id: "suspicious-login".to_string(),
            description: "Detects suspicious login attempts".to_string(),
            criteria: "failed_logins > 5".to_string(),
            severity: SiemSeverity::High,
            enabled: true,
        };

        assert_eq!(rule.id, "suspicious-login");
        assert!(matches!(rule.severity, SiemSeverity::High));
    }

    #[test]
    fn test_observability_manager() {
        let mut manager = ObservabilityManager::new();

        // Test adding collector
        let collector = OtelCollector {
            id: "test-collector".to_string(),
            endpoint: "http://localhost:4317".to_string(),
            telemetry_types: vec!["traces".to_string(), "metrics".to_string()],
            sampling_rate: 0.1,
            export_interval: 30,
        };

        assert!(manager.add_otel_collector(collector).is_ok());
        assert!(manager.get_otel_collector("test-collector").is_some());

        // Test adding Prometheus rule
        let rule = PrometheusRule {
            id: "high-latency".to_string(),
            expr: "rate(http_request_duration_seconds_sum[5m]) / rate(http_request_duration_seconds_count[5m]) > 1".to_string(),
            for_duration: "5m".to_string(),
            labels: HashMap::new(),
            annotations: HashMap::new(),
        };

        assert!(manager.add_prometheus_rule(rule).is_ok());
        assert!(manager.get_prometheus_rule("high-latency").is_some());

        // Test adding SIEM rule
        let siem_rule = SiemRule {
            id: "suspicious-login".to_string(),
            description: "Detects suspicious login attempts".to_string(),
            criteria: "failed_logins > 5".to_string(),
            severity: SiemSeverity::High,
            enabled: true,
        };

        assert!(manager.add_siem_rule(siem_rule).is_ok());
        assert!(manager.get_siem_rule("suspicious-login").is_some());

        // Test logging admin action
        let mut metadata = HashMap::new();
        metadata.insert("resource".to_string(), "user_database".to_string());

        let log_result = manager.log_admin_action(
            "admin_user".to_string(),
            "database_access".to_string(),
            "user_table".to_string(),
            metadata,
            Some("192.168.1.1".to_string()),
        );

        assert!(log_result.is_ok());
        assert_eq!(manager.get_audit_logs(None).len(), 1);

        // Test validation
        assert!(manager.validate_configuration().is_ok());
    }

    #[test]
    fn test_invalid_otel_collector() {
        let mut manager = ObservabilityManager::new();

        // Test invalid endpoint
        let collector = OtelCollector {
            id: "invalid-collector".to_string(),
            endpoint: "".to_string(), // Empty endpoint
            telemetry_types: vec!["traces".to_string()],
            sampling_rate: 0.1,
            export_interval: 30,
        };

        assert!(manager.add_otel_collector(collector).is_err());

        // Test invalid sampling rate
        let collector = OtelCollector {
            id: "invalid-collector-2".to_string(),
            endpoint: "http://localhost:4317".to_string(),
            telemetry_types: vec!["traces".to_string()],
            sampling_rate: 1.5, // Invalid sampling rate
            export_interval: 30,
        };

        assert!(manager.add_otel_collector(collector).is_err());
    }
}
