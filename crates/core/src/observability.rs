//! Observability Module
//!
//! This module implements observability measures including OpenTelemetry collector integration,
//! Prometheus alerting rules, SIEM rules, admin audit logging, and comprehensive telemetry
//! for monitoring, logging, and tracing with centralized logs, metrics, traces, and span IDs.
//! It also includes security detection features for SIEM/IDS/anomaly alerts and forensics capabilities.
//! Additionally, it includes incident response features for runbooks and pager functionality.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
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

/// Severity levels for SIEM rules and alerts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

/// Represents a telemetry metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryMetric {
    /// Metric name
    pub name: String,
    /// Metric value
    pub value: f64,
    /// Timestamp
    pub timestamp: u64,
    /// Labels for the metric
    pub labels: HashMap<String, String>,
}

/// Represents a log entry with structured data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Unique identifier for the log entry
    pub id: String,
    /// Timestamp of the log
    pub timestamp: u64,
    /// Log level
    pub level: String,
    /// Log message
    pub message: String,
    /// Service name
    pub service: String,
    /// Span ID if available
    pub span_id: Option<String>,
    /// Trace ID if available
    pub trace_id: Option<String>,
    /// Additional structured data
    pub fields: HashMap<String, String>,
}

/// Represents a trace span
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceSpan {
    /// Unique identifier for the span
    pub id: String,
    /// Trace identifier
    pub trace_id: String,
    /// Parent span ID if exists
    pub parent_id: Option<String>,
    /// Span name
    pub name: String,
    /// Start timestamp
    pub start_time: u64,
    /// End timestamp
    pub end_time: Option<u64>,
    /// Service name
    pub service: String,
    /// Span attributes
    pub attributes: HashMap<String, String>,
}

/// Represents a SIEM alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiemAlert {
    /// Unique identifier for the alert
    pub id: String,
    /// Timestamp when the alert was generated
    pub timestamp: u64,
    /// Alert rule that triggered this alert
    pub rule_id: String,
    /// Severity level of the alert
    pub severity: SiemSeverity,
    /// Alert message/description
    pub message: String,
    /// Source service that generated the alert
    pub source_service: String,
    /// Additional alert data
    pub data: HashMap<String, String>,
    /// Resolution status
    pub resolved: bool,
}

/// Telemetry statistics for monitoring
#[derive(Debug, Clone)]
pub struct TelemetryStats {
    /// p95 latency tracking
    pub latency_p95: Arc<AtomicU64>,
    /// Error rate tracking
    pub error_count: Arc<AtomicU64>,
    /// Total request count
    pub total_requests: Arc<AtomicU64>,
    /// Authentication failure tracking
    pub auth_failures: Arc<AtomicU64>,
}

/// Security detection statistics
#[derive(Debug, Clone)]
pub struct SecurityDetectionStats {
    /// Total SIEM alerts generated
    pub total_alerts: Arc<AtomicU64>,
    /// SIEM alerts by severity
    pub alerts_by_severity: Arc<Mutex<HashMap<SiemSeverity, u64>>>,
    /// Timestamp of first alert for MTTD calculation
    pub first_alert_timestamp: Arc<AtomicU64>,
    /// Timestamp of last alert
    pub last_alert_timestamp: Arc<AtomicU64>,
}

/// Incident response statistics
#[derive(Debug, Clone)]
pub struct IncidentResponseStats {
    /// Total incidents
    pub total_incidents: Arc<AtomicU64>,
    /// Resolved incidents
    pub resolved_incidents: Arc<AtomicU64>,
    /// Total time to resolve all incidents (for MTTR calculation)
    pub total_resolution_time: Arc<AtomicU64>,
    /// Postmortem quality scores
    pub postmortem_quality_scores: Arc<Mutex<Vec<u8>>>,
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

/// Snapshot of telemetry statistics
#[derive(Debug, Clone)]
pub struct TelemetryStatsSnapshot {
    /// p95 latency in milliseconds
    pub latency_p95: u64,
    /// Error rate as percentage
    pub error_rate: f64,
    /// Total request count
    pub total_requests: u64,
    /// Authentication failure count
    pub auth_failures: u64,
}

/// Snapshot of security detection statistics
#[derive(Debug, Clone)]
pub struct SecurityDetectionStatsSnapshot {
    /// Total SIEM alerts generated
    pub total_alerts: u64,
    /// SIEM alerts by severity
    pub alerts_by_severity: HashMap<SiemSeverity, u64>,
    /// Mean time to detect (MTTD) in seconds
    pub mean_time_to_detect: u64,
}

/// Snapshot of incident response statistics
#[derive(Debug, Clone)]
pub struct IncidentResponseStatsSnapshot {
    /// Total incidents
    pub total_incidents: u64,
    /// Resolved incidents
    pub resolved_incidents: u64,
    /// Mean time to resolve (MTTR) in seconds
    pub mean_time_to_resolve: u64,
    /// Average postmortem quality score
    pub average_postmortem_quality_score: f64,
}

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
    /// Telemetry metrics
    metrics: Arc<Mutex<HashMap<String, TelemetryMetric>>>,
    /// Log entries
    logs: Arc<Mutex<Vec<LogEntry>>>,
    /// Trace spans
    spans: Arc<Mutex<HashMap<String, TraceSpan>>>,
    /// SIEM alerts
    siem_alerts: Arc<Mutex<Vec<SiemAlert>>>,
    /// Incident response runbooks
    runbooks: HashMap<String, IncidentRunbook>,
    /// On-call pagers
    pagers: HashMap<String, OnCallPager>,
    /// Incidents for tracking
    incidents: Arc<Mutex<Vec<Incident>>>,
    /// Telemetry statistics
    stats: TelemetryStats,
    /// Security detection statistics
    security_stats: SecurityDetectionStats,
    /// Incident response statistics
    incident_stats: IncidentResponseStats,
}

impl ObservabilityManager {
    /// Creates a new observability manager
    pub fn new() -> Self {
        Self {
            otel_collectors: HashMap::new(),
            prometheus_rules: HashMap::new(),
            siem_rules: HashMap::new(),
            audit_logs: Vec::new(),
            metrics: Arc::new(Mutex::new(HashMap::new())),
            logs: Arc::new(Mutex::new(Vec::new())),
            spans: Arc::new(Mutex::new(HashMap::new())),
            siem_alerts: Arc::new(Mutex::new(Vec::new())),
            runbooks: HashMap::new(),
            pagers: HashMap::new(),
            incidents: Arc::new(Mutex::new(Vec::new())),
            stats: TelemetryStats {
                latency_p95: Arc::new(AtomicU64::new(0)),
                error_count: Arc::new(AtomicU64::new(0)),
                total_requests: Arc::new(AtomicU64::new(0)),
                auth_failures: Arc::new(AtomicU64::new(0)),
            },
            security_stats: SecurityDetectionStats {
                total_alerts: Arc::new(AtomicU64::new(0)),
                alerts_by_severity: Arc::new(Mutex::new(HashMap::new())),
                first_alert_timestamp: Arc::new(AtomicU64::new(0)),
                last_alert_timestamp: Arc::new(AtomicU64::new(0)),
            },
            incident_stats: IncidentResponseStats {
                total_incidents: Arc::new(AtomicU64::new(0)),
                resolved_incidents: Arc::new(AtomicU64::new(0)),
                total_resolution_time: Arc::new(AtomicU64::new(0)),
                postmortem_quality_scores: Arc::new(Mutex::new(Vec::new())),
            },
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

    /// Records a telemetry metric
    pub fn record_metric(&self, name: String, value: f64, labels: HashMap<String, String>) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let metric = TelemetryMetric {
            name: name.clone(),
            value,
            timestamp,
            labels,
        };

        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.insert(name, metric);
        }
    }

    /// Gets all telemetry metrics
    pub fn get_metrics(&self) -> HashMap<String, TelemetryMetric> {
        if let Ok(metrics) = self.metrics.lock() {
            metrics.clone()
        } else {
            HashMap::new()
        }
    }

    /// Records a log entry
    pub fn record_log(
        &self,
        level: String,
        message: String,
        service: String,
        span_id: Option<String>,
        trace_id: Option<String>,
        fields: HashMap<String, String>,
    ) -> Result<String, ObservabilityError> {
        let id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| ObservabilityError::ExportError(format!("Time error: {}", e)))?
            .as_secs();

        let log_entry = LogEntry {
            id: id.clone(),
            timestamp,
            level,
            message,
            service,
            span_id,
            trace_id,
            fields,
        };

        if let Ok(mut logs) = self.logs.lock() {
            logs.push(log_entry);
        }

        Ok(id)
    }

    /// Gets log entries with optional filtering
    pub fn get_logs(&self, service_filter: Option<&str>) -> Vec<LogEntry> {
        if let Ok(logs) = self.logs.lock() {
            if let Some(service) = service_filter {
                logs.iter()
                    .filter(|log| log.service == service)
                    .cloned()
                    .collect()
            } else {
                logs.clone()
            }
        } else {
            Vec::new()
        }
    }

    /// Starts a trace span
    pub fn start_span(
        &self,
        name: String,
        service: String,
        parent_id: Option<String>,
        attributes: HashMap<String, String>,
    ) -> Result<String, ObservabilityError> {
        let id = Uuid::new_v4().to_string();
        let trace_id = if let Some(parent) = &parent_id {
            // If we have a parent, use its trace ID
            if let Ok(spans) = self.spans.lock() {
                if let Some(parent_span) = spans.get(parent) {
                    parent_span.trace_id.clone()
                } else {
                    Uuid::new_v4().to_string()
                }
            } else {
                Uuid::new_v4().to_string()
            }
        } else {
            // If no parent, create a new trace ID
            Uuid::new_v4().to_string()
        };

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| ObservabilityError::ExportError(format!("Time error: {}", e)))?
            .as_secs();

        let span = TraceSpan {
            id: id.clone(),
            trace_id,
            parent_id,
            name,
            start_time: timestamp,
            end_time: None,
            service,
            attributes,
        };

        if let Ok(mut spans) = self.spans.lock() {
            spans.insert(id.clone(), span);
        }

        Ok(id)
    }

    /// Ends a trace span
    pub fn end_span(&self, span_id: &str) -> Result<(), ObservabilityError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| ObservabilityError::ExportError(format!("Time error: {}", e)))?
            .as_secs();

        if let Ok(mut spans) = self.spans.lock() {
            if let Some(span) = spans.get_mut(span_id) {
                span.end_time = Some(timestamp);
            }
        }

        Ok(())
    }

    /// Gets all trace spans
    pub fn get_spans(&self) -> HashMap<String, TraceSpan> {
        if let Ok(spans) = self.spans.lock() {
            spans.clone()
        } else {
            HashMap::new()
        }
    }

    /// Records a request for telemetry statistics
    pub fn record_request(&self, latency_ms: u64, is_error: bool, is_auth_failure: bool) {
        // Update total requests
        self.stats.total_requests.fetch_add(1, Ordering::Relaxed);
        
        // Update error count if it's an error
        if is_error {
            self.stats.error_count.fetch_add(1, Ordering::Relaxed);
        }
        
        // Update auth failure count if it's an auth failure
        if is_auth_failure {
            self.stats.auth_failures.fetch_add(1, Ordering::Relaxed);
        }
        
        // Update latency p95 (simplified implementation)
        self.stats.latency_p95.store(latency_ms, Ordering::Relaxed);
    }

    /// Gets telemetry statistics
    pub fn get_telemetry_stats(&self) -> TelemetryStatsSnapshot {
        TelemetryStatsSnapshot {
            latency_p95: self.stats.latency_p95.load(Ordering::Relaxed),
            error_rate: if self.stats.total_requests.load(Ordering::Relaxed) > 0 {
                (self.stats.error_count.load(Ordering::Relaxed) as f64 / 
                 self.stats.total_requests.load(Ordering::Relaxed) as f64) * 100.0
            } else {
                0.0
            },
            total_requests: self.stats.total_requests.load(Ordering::Relaxed),
            auth_failures: self.stats.auth_failures.load(Ordering::Relaxed),
        }
    }

    /// Generates a SIEM alert
    pub fn generate_siem_alert(
        &self,
        rule_id: String,
        severity: SiemSeverity,
        message: String,
        source_service: String,
        data: HashMap<String, String>,
    ) -> Result<String, ObservabilityError> {
        let id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| ObservabilityError::ExportError(format!("Time error: {}", e)))?
            .as_secs();

        let alert = SiemAlert {
            id: id.clone(),
            timestamp,
            rule_id,
            severity: severity.clone(),
            message,
            source_service,
            data,
            resolved: false,
        };

        if let Ok(mut alerts) = self.siem_alerts.lock() {
            alerts.push(alert);
        }

        // Update security statistics
        self.security_stats.total_alerts.fetch_add(1, Ordering::Relaxed);
        
        // Update first alert timestamp if this is the first alert
        let first_alert = self.security_stats.first_alert_timestamp.load(Ordering::Relaxed);
        if first_alert == 0 {
            self.security_stats.first_alert_timestamp.store(timestamp, Ordering::Relaxed);
        }
        
        // Update last alert timestamp
        self.security_stats.last_alert_timestamp.store(timestamp, Ordering::Relaxed);
        
        // Update alerts by severity
        if let Ok(mut alerts_by_severity) = self.security_stats.alerts_by_severity.lock() {
            let count = alerts_by_severity.entry(severity.clone()).or_insert(0);
            *count += 1;
        }

        Ok(id)
    }

    /// Gets SIEM alerts with optional filtering
    pub fn get_siem_alerts(&self, severity_filter: Option<SiemSeverity>) -> Vec<SiemAlert> {
        if let Ok(alerts) = self.siem_alerts.lock() {
            if let Some(severity) = severity_filter {
                alerts.iter()
                    .filter(|alert| alert.severity == severity)
                    .cloned()
                    .collect()
            } else {
                alerts.clone()
            }
        } else {
            Vec::new()
        }
    }

    /// Resolves a SIEM alert
    pub fn resolve_siem_alert(&self, alert_id: &str) -> Result<(), ObservabilityError> {
        if let Ok(mut alerts) = self.siem_alerts.lock() {
            if let Some(alert) = alerts.iter_mut().find(|a| a.id == alert_id) {
                alert.resolved = true;
                Ok(())
            } else {
                Err(ObservabilityError::ValidationError(
                    "Alert not found".to_string(),
                ))
            }
        } else {
            Err(ObservabilityError::ExportError(
                "Failed to access alerts".to_string(),
            ))
        }
    }

    /// Gets security detection statistics
    pub fn get_security_detection_stats(&self) -> SecurityDetectionStatsSnapshot {
        let total_alerts = self.security_stats.total_alerts.load(Ordering::Relaxed);
        
        let alerts_by_severity = if let Ok(alerts_by_severity) = self.security_stats.alerts_by_severity.lock() {
            alerts_by_severity.clone()
        } else {
            HashMap::new()
        };
        
        let first_alert = self.security_stats.first_alert_timestamp.load(Ordering::Relaxed);
        let last_alert = self.security_stats.last_alert_timestamp.load(Ordering::Relaxed);
        
        let mean_time_to_detect = if first_alert > 0 && last_alert > 0 {
            last_alert - first_alert
        } else {
            0
        };

        SecurityDetectionStatsSnapshot {
            total_alerts,
            alerts_by_severity,
            mean_time_to_detect,
        }
    }

    /// Detects login anomalies
    pub fn detect_login_anomaly(
        &self,
        user_id: String,
        ip_address: String,
        service: String,
        failure_count: u32,
    ) -> Result<Option<String>, ObservabilityError> {
        // Check for suspicious login patterns
        if failure_count > 5 {
            let mut data = HashMap::new();
            data.insert("user_id".to_string(), user_id.clone());
            data.insert("ip_address".to_string(), ip_address.clone());
            data.insert("failure_count".to_string(), failure_count.to_string());
            
            let severity = if failure_count > 10 { 
                SiemSeverity::Critical 
            } else { 
                SiemSeverity::High 
            };
            
            let alert_id = self.generate_siem_alert(
                "login-anomaly".to_string(),
                severity,
                format!("Suspicious login activity detected for user {} from IP {}", user_id.clone(), ip_address),
                service,
                data,
            )?;
            
            Ok(Some(alert_id))
        } else {
            Ok(None)
        }
    }

    /// Detects data exfiltration attempts
    pub fn detect_data_exfiltration(
        &self,
        user_id: String,
        service: String,
        data_size: u64,
        threshold: u64,
    ) -> Result<Option<String>, ObservabilityError> {
        // Check for unusually large data transfers
        if data_size > threshold {
            let mut data = HashMap::new();
            data.insert("user_id".to_string(), user_id.clone());
            data.insert("data_size".to_string(), data_size.to_string());
            data.insert("threshold".to_string(), threshold.to_string());
            
            let severity = if data_size > threshold * 2 { 
                SiemSeverity::Critical 
            } else { 
                SiemSeverity::High 
            };
            
            let alert_id = self.generate_siem_alert(
                "data-exfiltration".to_string(),
                severity,
                format!("Potential data exfiltration detected: {} bytes transferred by user {}", data_size, user_id),
                service,
                data,
            )?;
            
            Ok(Some(alert_id))
        } else {
            Ok(None)
        }
    }

    /// Detects container breakout attempts
    pub fn detect_container_breakout(
        &self,
        container_id: String,
        service: String,
        suspicious_activity: String,
    ) -> Result<String, ObservabilityError> {
        let mut data = HashMap::new();
        data.insert("container_id".to_string(), container_id.clone());
        data.insert("suspicious_activity".to_string(), suspicious_activity);
        
        let alert_id = self.generate_siem_alert(
            "container-breakout".to_string(),
            SiemSeverity::Critical,
            format!("Potential container breakout detected in container {}", container_id),
            service,
            data,
        )?;
        
        Ok(alert_id)
    }

    /// Adds an incident response runbook
    pub fn add_runbook(&mut self, runbook: IncidentRunbook) -> Result<(), ObservabilityError> {
        if runbook.id.is_empty() {
            return Err(ObservabilityError::ConfigError(
                "Runbook ID cannot be empty".to_string(),
            ));
        }

        if runbook.incident_type.is_empty() {
            return Err(ObservabilityError::ConfigError(
                "Incident type cannot be empty".to_string(),
            ));
        }

        self.runbooks.insert(runbook.id.clone(), runbook);
        Ok(())
    }

    /// Gets a runbook by ID
    pub fn get_runbook(&self, id: &str) -> Option<&IncidentRunbook> {
        self.runbooks.get(id)
    }

    /// Gets a runbook by incident type
    pub fn get_runbook_by_type(&self, incident_type: &str) -> Option<&IncidentRunbook> {
        self.runbooks.values().find(|r| r.incident_type == incident_type)
    }

    /// Adds an on-call pager configuration
    pub fn add_pager(&mut self, pager: OnCallPager) -> Result<(), ObservabilityError> {
        if pager.id.is_empty() {
            return Err(ObservabilityError::ConfigError(
                "Pager ID cannot be empty".to_string(),
            ));
        }

        if pager.team_service.is_empty() {
            return Err(ObservabilityError::ConfigError(
                "Team/service cannot be empty".to_string(),
            ));
        }

        self.pagers.insert(pager.id.clone(), pager);
        Ok(())
    }

    /// Gets a pager by ID
    pub fn get_pager(&self, id: &str) -> Option<&OnCallPager> {
        self.pagers.get(id)
    }

    /// Gets a pager by team/service
    pub fn get_pager_by_team(&self, team_service: &str) -> Option<&OnCallPager> {
        self.pagers.values().find(|p| p.team_service == team_service)
    }

    /// Creates a new incident
    pub fn create_incident(
        &self,
        incident_type: String,
        severity: SiemSeverity,
        assigned_personnel: Vec<String>,
        runbook_id: Option<String>,
    ) -> Result<String, ObservabilityError> {
        let id = Uuid::new_v4().to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| ObservabilityError::ExportError(format!("Time error: {}", e)))?
            .as_secs();

        let incident = Incident {
            id: id.clone(),
            incident_type,
            detected_at: timestamp,
            response_started_at: None,
            resolved_at: None,
            severity,
            assigned_personnel,
            runbook_id,
            postmortem: None,
        };

        if let Ok(mut incidents) = self.incidents.lock() {
            incidents.push(incident);
        }

        // Update incident statistics
        self.incident_stats.total_incidents.fetch_add(1, Ordering::Relaxed);

        Ok(id)
    }

    /// Starts incident response
    pub fn start_incident_response(&self, incident_id: &str) -> Result<(), ObservabilityError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| ObservabilityError::ExportError(format!("Time error: {}", e)))?
            .as_secs();

        if let Ok(mut incidents) = self.incidents.lock() {
            if let Some(incident) = incidents.iter_mut().find(|i| i.id == incident_id) {
                incident.response_started_at = Some(timestamp);
                Ok(())
            } else {
                Err(ObservabilityError::ValidationError(
                    "Incident not found".to_string(),
                ))
            }
        } else {
            Err(ObservabilityError::ExportError(
                "Failed to access incidents".to_string(),
            ))
        }
    }

    /// Resolves an incident
    pub fn resolve_incident(
        &self,
        incident_id: &str,
        postmortem: Option<Postmortem>,
    ) -> Result<(), ObservabilityError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| ObservabilityError::ExportError(format!("Time error: {}", e)))?
            .as_secs();

        if let Ok(mut incidents) = self.incidents.lock() {
            if let Some(incident) = incidents.iter_mut().find(|i| i.id == incident_id) {
                incident.resolved_at = Some(timestamp);
                incident.postmortem = postmortem.clone();

                // Update statistics
                self.incident_stats.resolved_incidents.fetch_add(1, Ordering::Relaxed);
                
                if let Some(started_at) = incident.response_started_at {
                    let resolution_time = timestamp - started_at;
                    self.incident_stats.total_resolution_time.fetch_add(resolution_time, Ordering::Relaxed);
                }

                // Add postmortem quality score if provided
                if let Some(pm) = postmortem {
                    if let Ok(mut scores) = self.incident_stats.postmortem_quality_scores.lock() {
                        scores.push(pm.quality_score);
                    }
                }

                Ok(())
            } else {
                Err(ObservabilityError::ValidationError(
                    "Incident not found".to_string(),
                ))
            }
        } else {
            Err(ObservabilityError::ExportError(
                "Failed to access incidents".to_string(),
            ))
        }
    }

    /// Gets all incidents
    pub fn get_incidents(&self) -> Vec<Incident> {
        if let Ok(incidents) = self.incidents.lock() {
            incidents.clone()
        } else {
            Vec::new()
        }
    }

    /// Gets incident by ID
    pub fn get_incident(&self, id: &str) -> Option<Incident> {
        if let Ok(incidents) = self.incidents.lock() {
            incidents.iter().find(|i| i.id == id).cloned()
        } else {
            None
        }
    }

    /// Gets mean time to recover (MTTR) statistics
    pub fn get_mttr_stats(&self) -> (u64, f64) {
        let _total_incidents = self.incident_stats.total_incidents.load(Ordering::Relaxed);
        let resolved_incidents = self.incident_stats.resolved_incidents.load(Ordering::Relaxed);
        let total_resolution_time = self.incident_stats.total_resolution_time.load(Ordering::Relaxed);
        
        if resolved_incidents > 0 {
            let mttr = total_resolution_time as f64 / resolved_incidents as f64;
            (resolved_incidents, mttr)
        } else {
            (resolved_incidents, 0.0)
        }
    }

    /// Gets postmortem quality statistics
    pub fn get_postmortem_quality_stats(&self) -> (usize, f64) {
        if let Ok(scores) = self.incident_stats.postmortem_quality_scores.lock() {
            if !scores.is_empty() {
                let count = scores.len();
                let avg_quality: f64 = scores.iter().map(|&s| s as f64).sum::<f64>() / count as f64;
                (count, avg_quality)
            } else {
                (0, 0.0)
            }
        } else {
            (0, 0.0)
        }
    }
}

impl Default for ObservabilityManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents an incident response runbook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentRunbook {
    /// Unique identifier for the runbook
    pub id: String,
    /// Incident type the runbook addresses
    pub incident_type: String,
    /// Steps to take during the incident
    pub steps: Vec<RunbookStep>,
    /// Communication plan
    pub communication_plan: CommunicationPlan,
    /// Rollback procedures
    pub rollback_steps: Vec<String>,
    /// Estimated time to recovery
    pub estimated_recovery_time: u32, // in minutes
    /// Priority level
    pub priority: IncidentPriority,
}

/// Represents a step in a runbook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunbookStep {
    /// Step number
    pub step_number: u32,
    /// Description of what to do
    pub description: String,
    /// Role responsible for this step
    pub responsible_role: String,
    /// Estimated time to complete
    pub estimated_time: u32, // in minutes
    /// Prerequisites
    pub prerequisites: Vec<String>,
}

/// Represents a communication plan for incident response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationPlan {
    /// Communication channels to use
    pub channels: Vec<CommunicationChannel>,
    /// Initial recipients
    pub initial_recipients: Vec<String>,
    /// Escalation paths
    pub escalation_paths: Vec<EscalationPath>,
    /// Templates for different communication types
    pub templates: HashMap<String, String>,
}

/// Represents a communication channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationChannel {
    /// Type of channel (email, slack, sms, etc.)
    pub channel_type: String,
    /// Channel identifier
    pub channel_id: String,
    /// Priority level
    pub priority: u8,
}

/// Represents an escalation path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationPath {
    /// Time threshold before escalation (in minutes)
    pub time_threshold: u32,
    /// Recipients to escalate to
    pub recipients: Vec<String>,
    /// Reason for escalation
    pub reason: String,
}

/// Incident priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IncidentPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Represents an on-call pager configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnCallPager {
    /// Unique identifier for the pager
    pub id: String,
    /// Team or service this pager is for
    pub team_service: String,
    /// Current on-call personnel
    pub on_call_personnel: Vec<OnCallPerson>,
    /// Escalation policy
    pub escalation_policy: Vec<EscalationLevel>,
    /// Notification methods
    pub notification_methods: Vec<String>,
}

/// Represents an on-call person
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnCallPerson {
    /// User identifier
    pub user_id: String,
    /// User name
    pub name: String,
    /// Contact information
    pub contact_info: HashMap<String, String>, // email, phone, slack, etc.
    /// Time zone
    pub time_zone: String,
}

/// Represents an escalation level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationLevel {
    /// Level number
    pub level: u32,
    /// Time delay before escalation (in minutes)
    pub delay_minutes: u32,
    /// Personnel to contact at this level
    pub personnel: Vec<String>,
}

/// Represents an incident for tracking MTTR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Incident {
    /// Unique identifier for the incident
    pub id: String,
    /// Incident type
    pub incident_type: String,
    /// Timestamp when incident was detected
    pub detected_at: u64,
    /// Timestamp when incident response started
    pub response_started_at: Option<u64>,
    /// Timestamp when incident was resolved
    pub resolved_at: Option<u64>,
    /// Severity level
    pub severity: SiemSeverity,
    /// Assigned personnel
    pub assigned_personnel: Vec<String>,
    /// Runbook used
    pub runbook_id: Option<String>,
    /// Postmortem document
    pub postmortem: Option<Postmortem>,
}

/// Represents a postmortem document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Postmortem {
    /// Summary of the incident
    pub summary: String,
    /// Root cause analysis
    pub root_cause: String,
    /// Timeline of events
    pub timeline: Vec<IncidentEvent>,
    /// Impact assessment
    pub impact: String,
    /// Resolution steps
    pub resolution: String,
    /// Preventive measures
    pub preventive_measures: Vec<String>,
    /// Quality score (0-100)
    pub quality_score: u8,
}

/// Represents an event in the incident timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentEvent {
    /// Timestamp of the event
    pub timestamp: u64,
    /// Description of the event
    pub description: String,
    /// Person responsible
    pub responsible: String,
}
