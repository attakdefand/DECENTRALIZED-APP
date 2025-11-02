//! SLA/SLO monitoring module for third-party services
//!
//! This module implements monitoring and tracking of Service Level Agreements
//! and Service Level Objectives for third-party vendors.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// SLA definition for a vendor service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLA {
    /// Unique SLA identifier
    pub id: String,
    /// Vendor identifier
    pub vendor_id: String,
    /// Service name
    pub service_name: String,
    /// SLA metrics
    pub metrics: Vec<SLAMetric>,
    /// Reporting period in days
    pub reporting_period: u32,
    /// Effective date
    pub effective_date: u64,
    /// Expiration date
    pub expiration_date: Option<u64>,
    /// Penalty clauses
    pub penalties: Vec<PenaltyClause>,
}

/// SLA metric definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLAMetric {
    /// Metric name
    pub name: String,
    /// Metric description
    pub description: String,
    /// Target value
    pub target: f64,
    /// Unit of measurement
    pub unit: String,
    /// Measurement type (e.g., percentage, milliseconds)
    pub measurement_type: MeasurementType,
    /// Reporting frequency
    pub reporting_frequency: u32, // in hours
}

/// Measurement type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MeasurementType {
    Percentage,
    Milliseconds,
    Seconds,
    Count,
    Boolean,
}

/// Penalty clause in SLA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PenaltyClause {
    /// Clause description
    pub description: String,
    /// Breach threshold
    pub threshold: f64,
    /// Penalty type
    pub penalty_type: PenaltyType,
    /// Penalty value
    pub penalty_value: f64,
}

/// Penalty type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PenaltyType {
    ServiceCredit,
    Termination,
    Financial,
}

/// SLA measurement record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLAMeasurement {
    /// SLA identifier
    pub sla_id: String,
    /// Metric name
    pub metric_name: String,
    /// Measurement timestamp
    pub timestamp: u64,
    /// Actual value
    pub actual_value: f64,
    /// Target value
    pub target_value: f64,
    /// Compliance status
    pub is_compliant: bool,
    /// Notes
    pub notes: Option<String>,
}

/// SLA dashboard data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLADashboard {
    /// Vendor identifier
    pub vendor_id: String,
    /// Vendor name
    pub vendor_name: String,
    /// SLA compliance summary
    pub compliance_summary: SLAComplianceSummary,
    /// Recent measurements
    pub recent_measurements: Vec<SLAMeasurement>,
    /// Breach history
    pub breach_history: Vec<SLABreach>,
}

/// SLA compliance summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLAComplianceSummary {
    /// Total metrics monitored
    pub total_metrics: u32,
    /// Compliant metrics
    pub compliant_metrics: u32,
    /// Non-compliant metrics
    pub non_compliant_metrics: u32,
    /// Overall compliance percentage
    pub compliance_percentage: f64,
    /// Last updated timestamp
    pub last_updated: u64,
}

/// SLA breach record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLABreach {
    /// Breach identifier
    pub id: String,
    /// SLA identifier
    pub sla_id: String,
    /// Metric name
    pub metric_name: String,
    /// Breach timestamp
    pub timestamp: u64,
    /// Actual value
    pub actual_value: f64,
    /// Target value
    pub target_value: f64,
    /// Breach severity
    pub severity: BreachSeverity,
    /// Resolved status
    pub resolved: bool,
    /// Resolution timestamp
    pub resolution_timestamp: Option<u64>,
    /// Notes
    pub notes: Option<String>,
}

/// Breach severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BreachSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// SLA monitoring manager
pub struct SLAMonitoringManager {
    /// Registered SLAs
    pub slas: HashMap<String, SLA>,
    /// SLA measurements
    pub measurements: Vec<SLAMeasurement>,
    /// SLA breaches
    pub breaches: Vec<SLABreach>,
    /// Vendor dashboard data
    pub dashboards: HashMap<String, SLADashboard>,
}

impl SLAMonitoringManager {
    /// Create a new SLA monitoring manager
    pub fn new() -> Self {
        Self {
            slas: HashMap::new(),
            measurements: Vec::new(),
            breaches: Vec::new(),
            dashboards: HashMap::new(),
        }
    }

    /// Register a new SLA
    pub fn register_sla(&mut self, sla: SLA) {
        self.slas.insert(sla.id.clone(), sla);
    }

    /// Record SLA measurement
    pub fn record_measurement(&mut self, measurement: SLAMeasurement) {
        let is_compliant = measurement.actual_value <= measurement.target_value;
        
        let updated_measurement = SLAMeasurement {
            is_compliant,
            ..measurement
        };
        
        // Check for breach
        if !is_compliant {
            let breach = SLABreach {
                id: format!("breach_{}", 
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_secs()
                ),
                sla_id: updated_measurement.sla_id.clone(),
                metric_name: updated_measurement.metric_name.clone(),
                timestamp: updated_measurement.timestamp,
                actual_value: updated_measurement.actual_value,
                target_value: updated_measurement.target_value,
                severity: self.calculate_breach_severity(
                    updated_measurement.actual_value,
                    updated_measurement.target_value
                ),
                resolved: false,
                resolution_timestamp: None,
                notes: None,
            };
            
            self.breaches.push(breach);
        }
        
        self.measurements.push(updated_measurement);
        self.update_dashboard(&updated_measurement);
    }

    /// Calculate breach severity based on deviation from target
    fn calculate_breach_severity(&self, actual: f64, target: f64) -> BreachSeverity {
        let deviation = (actual - target) / target;
        
        if deviation > 0.5 {
            BreachSeverity::Critical
        } else if deviation > 0.2 {
            BreachSeverity::High
        } else if deviation > 0.1 {
            BreachSeverity::Medium
        } else {
            BreachSeverity::Low
        }
    }

    /// Update dashboard with new measurement
    fn update_dashboard(&mut self, measurement: &SLAMeasurement) {
        // In a real implementation, this would update the dashboard data
        // For now, we'll just ensure the dashboard exists
        if !self.dashboards.contains_key(&measurement.sla_id) {
            self.dashboards.insert(measurement.sla_id.clone(), SLADashboard {
                vendor_id: measurement.sla_id.clone(),
                vendor_name: "Vendor".to_string(),
                compliance_summary: SLAComplianceSummary {
                    total_metrics: 0,
                    compliant_metrics: 0,
                    non_compliant_metrics: 0,
                    compliance_percentage: 0.0,
                    last_updated: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_secs(),
                },
                recent_measurements: vec![measurement.clone()],
                breach_history: vec![],
            });
        }
    }

    /// Get SLA by ID
    pub fn get_sla(&self, sla_id: &str) -> Option<&SLA> {
        self.slas.get(sla_id)
    }

    /// Get measurements for a specific SLA
    pub fn get_sla_measurements(&self, sla_id: &str) -> Vec<&SLAMeasurement> {
        self.measurements
            .iter()
            .filter(|m| m.sla_id == sla_id)
            .collect()
    }

    /// Get breaches for a specific SLA
    pub fn get_sla_breaches(&self, sla_id: &str) -> Vec<&SLABreach> {
        self.breaches
            .iter()
            .filter(|b| b.sla_id == sla_id)
            .collect()
    }

    /// Get dashboard for a vendor
    pub fn get_vendor_dashboard(&self, vendor_id: &str) -> Option<&SLADashboard> {
        self.dashboards.get(vendor_id)
    }

    /// Get all unresolved breaches
    pub fn get_unresolved_breaches(&self) -> Vec<&SLABreach> {
        self.breaches
            .iter()
            .filter(|b| !b.resolved)
            .collect()
    }

    /// Resolve a breach
    pub fn resolve_breach(&mut self, breach_id: &str) -> Result<(), &'static str> {
        for breach in &mut self.breaches {
            if breach.id == breach_id {
                breach.resolved = true;
                breach.resolution_timestamp = Some(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_secs()
                );
                return Ok(());
            }
        }
        Err("Breach not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sla_registration() {
        let mut manager = SLAMonitoringManager::new();
        
        let metric = SLAMetric {
            name: "Uptime".to_string(),
            description: "System uptime percentage".to_string(),
            target: 99.9,
            unit: "%".to_string(),
            measurement_type: MeasurementType::Percentage,
            reporting_frequency: 24,
        };
        
        let sla = SLA {
            id: "sla1".to_string(),
            vendor_id: "vendor1".to_string(),
            service_name: "Cloud Service".to_string(),
            metrics: vec![metric],
            reporting_period: 30,
            effective_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            expiration_date: None,
            penalties: vec![],
        };
        
        manager.register_sla(sla);
        assert!(manager.get_sla("sla1").is_some());
    }

    #[test]
    fn test_sla_measurement() {
        let mut manager = SLAMonitoringManager::new();
        
        let measurement = SLAMeasurement {
            sla_id: "sla1".to_string(),
            metric_name: "Uptime".to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            actual_value: 99.5,
            target_value: 99.9,
            is_compliant: false, // 99.5 < 99.9, but we'll let the system calculate this
            notes: None,
        };
        
        manager.record_measurement(measurement);
        assert_eq!(manager.measurements.len(), 1);
        assert_eq!(manager.breaches.len(), 1);
    }

    #[test]
    fn test_breach_resolution() {
        let mut manager = SLAMonitoringManager::new();
        
        let breach = SLABreach {
            id: "breach1".to_string(),
            sla_id: "sla1".to_string(),
            metric_name: "Uptime".to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            actual_value: 99.0,
            target_value: 99.9,
            severity: BreachSeverity::High,
            resolved: false,
            resolution_timestamp: None,
            notes: None,
        };
        
        manager.breaches.push(breach);
        
        assert!(manager.resolve_breach("breach1").is_ok());
        
        let unresolved = manager.get_unresolved_breaches();
        assert_eq!(unresolved.len(), 0);
    }
}