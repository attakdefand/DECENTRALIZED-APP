//! Vendor key management module
//!
//! This module implements key rotation compliance tracking and health checks for third-party vendors.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

/// Vendor key management system
pub struct VendorKeyManager {
    /// Key rotation records by vendor
    pub key_rotation_records: HashMap<String, Vec<KeyRotationRecord>>,
    /// Key health check records by vendor
    pub health_check_records: HashMap<String, Vec<HealthCheckRecord>>,
    /// Vendor key policies compliance status
    pub policy_compliance: HashMap<String, bool>,
}

/// Key rotation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRotationRecord {
    /// Unique identifier for the rotation record
    pub id: String,
    /// Vendor identifier
    pub vendor_id: String,
    /// Key type
    pub key_type: KeyType,
    /// Rotation date
    pub rotation_date: u64,
    /// Next rotation due date
    pub next_rotation_due: u64,
    /// Rotation status
    pub status: RotationStatus,
    /// Compliance status
    pub compliant: bool,
}

/// Key health check record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckRecord {
    /// Unique identifier for the health check record
    pub id: String,
    /// Vendor identifier
    pub vendor_id: String,
    /// Check date
    pub check_date: u64,
    /// Health check type
    pub check_type: HealthCheckType,
    /// Check result
    pub result: HealthCheckResult,
    /// Details
    pub details: Option<String>,
}

/// Key type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyType {
    APIKey,
    DatabaseCredential,
    TLSCertificate,
    SSHKey,
    DataEncryptionKey,
    KeyEncryptionKey,
    ServiceAccountKey,
    Other(String),
}

/// Rotation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RotationStatus {
    Completed,
    Overdue,
    Scheduled,
    Failed,
}

/// Health check type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthCheckType {
    SystemStatus,
    KeyAccessibility,
    Performance,
    SecurityControl,
    Other(String),
}

/// Health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthCheckResult {
    Pass,
    Fail,
    Warning,
}

impl VendorKeyManager {
    /// Create a new vendor key manager
    pub fn new() -> Self {
        Self {
            key_rotation_records: HashMap::new(),
            health_check_records: HashMap::new(),
            policy_compliance: HashMap::new(),
        }
    }

    /// Record a key rotation
    pub fn record_key_rotation(&mut self, record: KeyRotationRecord) {
        self.key_rotation_records
            .entry(record.vendor_id.clone())
            .or_insert_with(Vec::new)
            .push(record);
    }

    /// Record a health check
    pub fn record_health_check(&mut self, record: HealthCheckRecord) {
        self.health_check_records
            .entry(record.vendor_id.clone())
            .or_insert_with(Vec::new)
            .push(record);
    }

    /// Set policy compliance status for a vendor
    pub fn set_policy_compliance(&mut self, vendor_id: String, compliant: bool) {
        self.policy_compliance.insert(vendor_id, compliant);
    }

    /// Get key rotation compliance percentage for a vendor
    pub fn get_key_rotation_compliance_pct(&self, vendor_id: &str) -> u32 {
        let records = match self.key_rotation_records.get(vendor_id) {
            Some(records) => records,
            None => return 100, // No records, assume 100% compliance
        };

        if records.is_empty() {
            return 100; // No records, assume 100% compliance
        }

        let compliant_count = records.iter().filter(|r| r.compliant).count();
        ((compliant_count * 100) / records.len()) as u32
    }

    /// Get health check pass count for a vendor
    pub fn get_health_checks_pass_count(&self, vendor_id: &str) -> u32 {
        let records = match self.health_check_records.get(vendor_id) {
            Some(records) => records,
            None => return 0, // No records
        };

        records.iter().filter(|r| matches!(r.result, HealthCheckResult::Pass)).count() as u32
    }

    /// Get total health check count for a vendor
    pub fn get_health_checks_total_count(&self, vendor_id: &str) -> u32 {
        match self.health_check_records.get(vendor_id) {
            Some(records) => records.len() as u32,
            None => 0, // No records
        }
    }

    /// Check if a vendor passes all health checks
    pub fn get_health_checks_pass(&self, vendor_id: &str) -> u32 {
        let pass_count = self.get_health_checks_pass_count(vendor_id);
        let total_count = self.get_health_checks_total_count(vendor_id);

        if total_count == 0 {
            return 1; // No checks performed, consider as pass
        }

        if pass_count == total_count {
            1 // All checks passed
        } else {
            0 // Some checks failed
        }
    }

    /// Get overall key management metrics for a vendor
    pub fn get_vendor_key_metrics(&self, vendor_id: &str) -> VendorKeyMetrics {
        VendorKeyMetrics {
            key_rotation_compliance_pct: self.get_key_rotation_compliance_pct(vendor_id),
            health_checks_pass: self.get_health_checks_pass(vendor_id),
            health_checks_pass_count: self.get_health_checks_pass_count(vendor_id),
            health_checks_total_count: self.get_health_checks_total_count(vendor_id),
        }
    }

    fn vendor_id_set(&self) -> HashSet<String> {
        let mut ids: HashSet<String> = self
            .key_rotation_records
            .keys()
            .cloned()
            .collect();
        ids.extend(self.health_check_records.keys().cloned());
        ids
    }

    /// Aggregate metrics across all vendors so CI/CD steps can gate on a single
    /// JSON document rather than enumerating every vendor identifier.
    pub fn get_overall_metrics(&self) -> VendorKeyMetrics {
        let vendor_ids = self.vendor_id_set();
        if vendor_ids.is_empty() {
            return VendorKeyMetrics {
                key_rotation_compliance_pct: 100,
                health_checks_pass: 1,
                health_checks_pass_count: 0,
                health_checks_total_count: 0,
            };
        }

        let mut total_compliance: u32 = 0;
        let mut aggregate_health_flag: u32 = 1;
        let mut aggregate_health_pass_count: u32 = 0;
        let mut aggregate_health_total_count: u32 = 0;

        for vendor_id in vendor_ids.iter() {
            let metrics = self.get_vendor_key_metrics(vendor_id);
            total_compliance += metrics.key_rotation_compliance_pct;
            if metrics.health_checks_pass == 0 {
                aggregate_health_flag = 0;
            }
            aggregate_health_pass_count += metrics.health_checks_pass_count;
            aggregate_health_total_count += metrics.health_checks_total_count;
        }

        let vendor_count = vendor_ids.len() as u32;
        let avg_compliance = if vendor_count == 0 {
            100
        } else {
            total_compliance / vendor_count
        };

        VendorKeyMetrics {
            key_rotation_compliance_pct: avg_compliance,
            health_checks_pass: aggregate_health_flag,
            health_checks_pass_count: aggregate_health_pass_count,
            health_checks_total_count: aggregate_health_total_count,
        }
    }

    /// Check if a vendor meets the key management requirements
    /// Returns true if key rotation compliance is 100% and all health checks pass
    pub fn is_vendor_compliant(&self, vendor_id: &str) -> bool {
        let rotation_compliance = self.get_key_rotation_compliance_pct(vendor_id);
        let health_checks_pass = self.get_health_checks_pass(vendor_id);

        rotation_compliance == 100 && health_checks_pass == 1
    }
}

/// Vendor key management metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorKeyMetrics {
    /// Key rotation compliance percentage
    pub key_rotation_compliance_pct: u32,
    /// Health checks pass (1 if all pass, 0 otherwise)
    pub health_checks_pass: u32,
    /// Number of health checks that passed
    pub health_checks_pass_count: u32,
    /// Total number of health checks
    pub health_checks_total_count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vendor_key_manager() {
        let mut manager = VendorKeyManager::new();

        // Test key rotation recording
        let rotation_record = KeyRotationRecord {
            id: "rotation1".to_string(),
            vendor_id: "vendor1".to_string(),
            key_type: KeyType::APIKey,
            rotation_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            next_rotation_due: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() + 90 * 24 * 60 * 60,
            status: RotationStatus::Completed,
            compliant: true,
        };

        manager.record_key_rotation(rotation_record);

        // Test health check recording
        let health_check_record = HealthCheckRecord {
            id: "health1".to_string(),
            vendor_id: "vendor1".to_string(),
            check_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            check_type: HealthCheckType::SystemStatus,
            result: HealthCheckResult::Pass,
            details: Some("System is healthy".to_string()),
        };

        manager.record_health_check(health_check_record);

        // Test metrics
        let metrics = manager.get_vendor_key_metrics("vendor1");
        assert_eq!(metrics.key_rotation_compliance_pct, 100);
        assert_eq!(metrics.health_checks_pass, 1);
        assert_eq!(metrics.health_checks_pass_count, 1);
        assert_eq!(metrics.health_checks_total_count, 1);

        // Test compliance
        assert!(manager.is_vendor_compliant("vendor1"));
    }

    #[test]
    fn test_non_compliant_vendor() {
        let mut manager = VendorKeyManager::new();

        // Test non-compliant key rotation
        let rotation_record = KeyRotationRecord {
            id: "rotation1".to_string(),
            vendor_id: "vendor2".to_string(),
            key_type: KeyType::APIKey,
            rotation_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            next_rotation_due: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() + 90 * 24 * 60 * 60,
            status: RotationStatus::Overdue,
            compliant: false,
        };

        manager.record_key_rotation(rotation_record);

        // Test failed health check
        let health_check_record = HealthCheckRecord {
            id: "health1".to_string(),
            vendor_id: "vendor2".to_string(),
            check_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            check_type: HealthCheckType::SystemStatus,
            result: HealthCheckResult::Fail,
            details: Some("System failure detected".to_string()),
        };

        manager.record_health_check(health_check_record);

        // Test metrics
        let metrics = manager.get_vendor_key_metrics("vendor2");
        assert_eq!(metrics.key_rotation_compliance_pct, 0);
        assert_eq!(metrics.health_checks_pass, 0);

        // Test compliance
        assert!(!manager.is_vendor_compliant("vendor2"));
    }
}
