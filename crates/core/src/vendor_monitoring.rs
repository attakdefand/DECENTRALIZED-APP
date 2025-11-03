//! Vendor authentication monitoring module
//!
//! This module provides monitoring capabilities for detecting unusual authentication patterns.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{warn, error};
use crate::vendor_auth::{UnusualPattern, AuthFailureReason};

/// Vendor authentication monitoring service
pub struct VendorAuthMonitoring {
    /// Monitoring configuration
    config: MonitoringConfig,
    /// Alert history
    alert_history: HashMap<String, Vec<Alert>>,
}

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Enable/disable monitoring
    pub enabled: bool,
    /// Threshold for high failure rate alert (%)
    pub high_failure_rate_threshold: f64,
    /// Threshold for multiple failed attempts from same IP
    pub failed_attempts_threshold: u32,
    /// Time window for failed attempts (seconds)
    pub failed_attempts_time_window: u64,
    /// Enable alerting for unusual access hours
    pub monitor_unusual_hours: bool,
    /// Alert email recipients
    pub alert_recipients: Vec<String>,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            high_failure_rate_threshold: 50.0,
            failed_attempts_threshold: 5,
            failed_attempts_time_window: 3600, // 1 hour
            monitor_unusual_hours: true,
            alert_recipients: vec![],
        }
    }
}

/// Alert structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Alert ID
    pub id: String,
    /// Timestamp of the alert
    pub timestamp: u64,
    /// Vendor ID
    pub vendor_id: String,
    /// Alert type
    pub alert_type: AlertType,
    /// Alert message
    pub message: String,
    /// Severity level
    pub severity: Severity,
    /// Resolved status
    pub resolved: bool,
}

/// Alert types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    HighFailureRate,
    MultipleFailedAttempts,
    UnusualAccessHours,
    SuspiciousActivity,
}

/// Severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl VendorAuthMonitoring {
    /// Create a new vendor authentication monitoring service
    pub fn new(config: MonitoringConfig) -> Self {
        Self {
            config,
            alert_history: HashMap::new(),
        }
    }

    /// Monitor for unusual authentication patterns for a specific vendor
    pub fn monitor_vendor_patterns(&mut self, vendor_id: &str, patterns: Vec<UnusualPattern>) {
        if !self.config.enabled {
            return;
        }

        for pattern in patterns {
            self.handle_unusual_pattern(vendor_id, pattern);
        }
    }

    /// Handle detected unusual pattern
    fn handle_unusual_pattern(&mut self, vendor_id: &str, pattern: UnusualPattern) {
        match pattern {
            UnusualPattern::HighFailureRate { vendor_id, failure_rate } => {
                if failure_rate > self.config.high_failure_rate_threshold {
                    let message = format!(
                        "Vendor {} has a high authentication failure rate of {:.2}%",
                        vendor_id, failure_rate
                    );
                    
                    let alert = Alert {
                        id: format!("alert_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()),
                        timestamp: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards")
                            .as_secs(),
                        vendor_id: vendor_id.clone(),
                        alert_type: AlertType::HighFailureRate,
                        message: message.clone(),
                        severity: if failure_rate > 80.0 { Severity::Critical } else { Severity::High },
                        resolved: false,
                    };
                    
                    self.record_alert(alert);
                    self.send_alert(&message);
                }
            }
            UnusualPattern::MultipleFailedAttemptsFromIP { vendor_id, ip, count, time_window } => {
                if count > self.config.failed_attempts_threshold {
                    let message = format!(
                        "Vendor {} has {} failed authentication attempts from IP {} in {} seconds",
                        vendor_id, count, ip, time_window
                    );
                    
                    let alert = Alert {
                        id: format!("alert_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()),
                        timestamp: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards")
                            .as_secs(),
                        vendor_id: vendor_id.clone(),
                        alert_type: AlertType::MultipleFailedAttempts,
                        message: message.clone(),
                        severity: if count > self.config.failed_attempts_threshold * 2 { 
                            Severity::Critical 
                        } else { 
                            Severity::High 
                        },
                        resolved: false,
                    };
                    
                    self.record_alert(alert);
                    self.send_alert(&message);
                }
            }
            UnusualPattern::UnusualAccessHours { vendor_id, count } => {
                if self.config.monitor_unusual_hours {
                    let message = format!(
                        "Vendor {} has {} authentication attempts at unusual hours",
                        vendor_id, count
                    );
                    
                    let alert = Alert {
                        id: format!("alert_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()),
                        timestamp: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards")
                            .as_secs(),
                        vendor_id: vendor_id.clone(),
                        alert_type: AlertType::UnusualAccessHours,
                        message: message.clone(),
                        severity: Severity::Medium,
                        resolved: false,
                    };
                    
                    self.record_alert(alert);
                    self.send_alert(&message);
                }
            }
        }
    }

    /// Record an alert in the alert history
    fn record_alert(&mut self, alert: Alert) {
        self.alert_history
            .entry(alert.vendor_id.clone())
            .or_insert_with(Vec::new)
            .push(alert);
    }

    /// Send alert notification
    fn send_alert(&self, message: &str) {
        // Log the alert
        warn!(target: "vendor_monitoring", "UNUSUAL AUTHENTICATION PATTERN DETECTED: {}", message);
        
        // In a real implementation, this would send emails, Slack notifications, etc.
        // For now, we just log it
        println!("ALERT: {}", message);
        
        // If we had alert recipients configured, we would send notifications to them
        if !self.config.alert_recipients.is_empty() {
            println!("Alert recipients: {:?}", self.config.alert_recipients);
        }
    }

    /// Get alerts for a vendor
    pub fn get_vendor_alerts(&self, vendor_id: &str) -> Option<&Vec<Alert>> {
        self.alert_history.get(vendor_id)
    }

    /// Get all alerts
    pub fn get_all_alerts(&self) -> &HashMap<String, Vec<Alert>> {
        &self.alert_history
    }

    /// Resolve an alert
    pub fn resolve_alert(&mut self, alert_id: &str) -> bool {
        for alerts in self.alert_history.values_mut() {
            for alert in alerts {
                if alert.id == alert_id {
                    alert.resolved = true;
                    return true;
                }
            }
        }
        false
    }

    /// Get unresolved alerts
    pub fn get_unresolved_alerts(&self) -> Vec<&Alert> {
        self.alert_history
            .values()
            .flatten()
            .filter(|alert| !alert.resolved)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vendor_auth::UnusualPattern;

    #[test]
    fn test_vendor_auth_monitoring() {
        let config = MonitoringConfig::default();
        let mut monitoring = VendorAuthMonitoring::new(config);
        
        assert!(monitoring.config.enabled);
        assert_eq!(monitoring.config.high_failure_rate_threshold, 50.0);
        
        // Test with an empty alert history
        assert!(monitoring.get_unresolved_alerts().is_empty());
    }

    #[test]
    fn test_alert_creation() {
        let alert = Alert {
            id: "test_alert".to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            vendor_id: "vendor1".to_string(),
            alert_type: AlertType::HighFailureRate,
            message: "Test alert".to_string(),
            severity: Severity::High,
            resolved: false,
        };

        assert_eq!(alert.vendor_id, "vendor1");
        assert_eq!(alert.message, "Test alert");
        assert!(!alert.resolved);
    }

    #[test]
    fn test_high_failure_rate_detection() {
        let config = MonitoringConfig::default();
        let mut monitoring = VendorAuthMonitoring::new(config);
        
        // Create a high failure rate pattern
        let pattern = UnusualPattern::HighFailureRate {
            vendor_id: "vendor1".to_string(),
            failure_rate: 75.0, // Above the 50% threshold
        };
        
        // Monitor the pattern
        monitoring.monitor_vendor_patterns("vendor1", vec![pattern]);
        
        // Check that an alert was created
        let alerts = monitoring.get_vendor_alerts("vendor1").unwrap();
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].alert_type, AlertType::HighFailureRate);
        assert_eq!(alerts[0].severity, Severity::High);
    }
}