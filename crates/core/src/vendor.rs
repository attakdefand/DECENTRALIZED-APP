//! Main vendor management module
//!
//! This module integrates all vendor management capabilities including risk assessment,
//! SLA monitoring, and access management.

use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::{
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

// Import submodules
pub mod vendor_management;
pub mod sla_monitoring;
pub mod vendor_access;
pub mod vendor_key_management;
pub mod vendor_policy_management;
pub mod vendor_auth;
pub mod vendor_logging;
pub mod vendor_monitoring;

use vendor_management::{VendorRiskManager, VendorRiskMetrics};
use sla_monitoring::SLAMonitoringManager;
use vendor_access::{VendorAccessManager, ReviewMetrics};
use vendor_key_management::VendorKeyManager;
use vendor_policy_management::VendorPolicyManager;
use vendor_auth::VendorAuthManager;
use vendor_logging::{VendorAuthLog, AuthEventType, create_vendor_auth_log, log_vendor_auth_event};
use vendor_monitoring::{VendorAuthMonitoring, MonitoringConfig};

/// Vendor management system
pub struct VendorManagementSystem {
    /// Vendor risk manager
    pub risk_manager: VendorRiskManager,
    /// SLA monitoring manager
    pub sla_manager: SLAMonitoringManager,
    /// Vendor access manager
    pub access_manager: VendorAccessManager,
    /// Vendor key manager
    pub key_manager: VendorKeyManager,
    /// Vendor policy manager
    pub policy_manager: VendorPolicyManager,
    /// Vendor authentication manager
    pub auth_manager: VendorAuthManager,
    /// Vendor authentication monitoring service
    pub auth_monitoring: Option<VendorAuthMonitoring>,
}

impl VendorManagementSystem {
    /// Create a new vendor management system
    pub fn new() -> Self {
        Self {
            risk_manager: VendorRiskManager::new(),
            sla_manager: SLAMonitoringManager::new(),
            access_manager: VendorAccessManager::new(),
            key_manager: VendorKeyManager::new(),
            policy_manager: VendorPolicyManager::new(),
            auth_manager: VendorAuthManager::new(),
            auth_monitoring: Some(VendorAuthMonitoring::new(MonitoringConfig::default())),
        }
    }

    /// Monitor for unusual authentication patterns for all vendors
    pub fn monitor_auth_patterns(&mut self) {
        if let Some(monitoring) = &mut self.auth_monitoring {
            // Get all vendors and check for unusual patterns
            let vendors = self.risk_manager.get_all_vendors();
            
            for vendor in vendors {
                let patterns = self.auth_manager.detect_unusual_patterns(&vendor.id);
                if !patterns.is_empty() {
                    monitoring.monitor_vendor_patterns(&vendor.id, patterns);
                }
            }
        }
    }

    /// Get overall vendor metrics aggregated across every onboarded vendor
    pub fn get_vendor_metrics(&self) -> VendorMetrics {
        self.collect_vendor_metrics(None)
    }

    /// Get metrics for a specific vendor identifier
    pub fn get_vendor_metrics_for_vendor(&self, vendor_id: &str) -> VendorMetrics {
        self.collect_vendor_metrics(Some(vendor_id))
    }

    fn collect_vendor_metrics(&self, vendor_id: Option<&str>) -> VendorMetrics {
        let risk_metrics = self.risk_manager.get_vendor_risk_metrics();
        let review_metrics = self.access_manager.get_review_metrics();

        let key_metrics = match vendor_id {
            Some(id) => self.key_manager.get_vendor_key_metrics(id),
            None => self.key_manager.get_overall_metrics(),
        };

        let policy_metrics = match vendor_id {
            Some(id) => self.policy_manager.get_vendor_policy_metrics(id),
            None => self.policy_manager.get_overall_metrics(),
        };

        let auth_success_rate = self.auth_manager.get_overall_success_rate();
        let auth_failure_rate = self.auth_manager.get_overall_failure_rate();

        VendorMetrics {
            risk_metrics,
            review_metrics: review_metrics.clone(),
            overdue_reviews: review_metrics.overdue_reviews,
            vendor_score_avg: risk_metrics.vendor_score_avg,
            last_updated: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            vendor_access_review_completion_pct: review_metrics.vendor_access_review_completion_pct,
            vendor_sod_violations: review_metrics.vendor_sod_violations,
            vendor_key_rotation_compliance_pct: key_metrics.key_rotation_compliance_pct,
            vendor_key_health_checks_pass: key_metrics.health_checks_pass,
            vendor_policy_coverage_pct: policy_metrics.policy_coverage_pct,
            vendor_policy_violations: policy_metrics.policy_violations,
            vendor_auth_success_rate: auth_success_rate,
            vendor_auth_failure_rate: auth_failure_rate,
        }
    }

    /// Export aggregated vendor metrics as prettified JSON so CI workflows and
    /// governance reviews can consume a single source of truth.
    pub fn export_metrics_to<P: AsRef<Path>>(&self, destination: P) -> std::io::Result<()> {
        let metrics = self.get_vendor_metrics();
        if let Some(parent) = destination.as_ref().parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        let payload = to_string_pretty(&metrics)?;
        fs::write(destination, payload)
    }

    /// Check if there are overdue reviews
    pub fn has_overdue_reviews(&self) -> bool {
        let metrics = self.get_vendor_metrics();
        metrics.overdue_reviews > 0
    }

    /// Generate due diligence report
    pub fn generate_due_diligence_report(&self, vendor_id: &str) -> String {
        if let Some(vendor) = self.risk_manager.get_vendor(vendor_id) {
            format!(
                "Due Diligence Report for {}\n\
                ==========================\n\
                Vendor ID: {}\n\
                Risk Tier: {:?}\n\
                Current Risk Score: {}\n\
                Last Assessment: {:?}\n\
                Number of Assessments: {}\n\
                CAIQ Controls: {}\n",
                vendor.name,
                vendor.id,
                vendor.risk_tier,
                vendor.risk_score,
                vendor.last_assessment,
                vendor.questionnaires.len(),
                vendor.caIQ_mappings.len()
            )
        } else {
            "Vendor not found".to_string()
        }
    }

    /// Generate access attestation
    pub fn generate_access_attestation(&self, access_id: &str) -> String {
        if let Some(access) = self.access_manager.get_vendor_access(access_id) {
            format!(
                "Access Attestation\n\
                ==================\n\
                Access ID: {}\n\
                Vendor ID: {}\n\
                Resource: {}\n\
                Access Type: {:?}\n\
                Access Level: {:?}\n\
                Status: {:?}\n\
                Granted Date: {}\n\
                Next Review Due: {}\n",
                access.id,
                access.vendor_id,
                access.resource,
                access.access_type,
                access.access_level,
                access.status,
                access.granted_date,
                access.next_review_due
            )
        } else {
            "Access record not found".to_string()
        }
    }

    /// Get vendors requiring attention (assessment or review)
    pub fn get_vendors_requiring_attention(&self) -> Vec<VendorAttentionItem> {
        let mut attention_items = Vec::new();
        
        // Add vendors requiring assessment
        for vendor in self.risk_manager.get_vendors_requiring_assessment() {
            attention_items.push(VendorAttentionItem {
                vendor_id: vendor.id.clone(),
                vendor_name: vendor.name.clone(),
                attention_type: AttentionType::AssessmentRequired,
                due_date: vendor.next_assessment_due,
            });
        }
        
        // Add accesses requiring review
        for access in self.access_manager.get_accesses_requiring_review() {
            attention_items.push(VendorAttentionItem {
                vendor_id: access.vendor_id.clone(),
                vendor_name: "Unknown".to_string(), // Would need to look up vendor name
                attention_type: AttentionType::AccessReviewRequired,
                due_date: access.next_review_due,
            });
        }
        
        attention_items
    }
}

/// Overall vendor metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorMetrics {
    /// Risk metrics
    pub risk_metrics: VendorRiskMetrics,
    /// Review metrics
    pub review_metrics: ReviewMetrics,
    /// Number of overdue reviews
    pub overdue_reviews: u32,
    /// Average vendor risk score
    pub vendor_score_avg: u32,
    /// Last updated timestamp
    pub last_updated: u64,
    /// Vendor access review completion percentage
    pub vendor_access_review_completion_pct: u32,
    /// Vendor SoD violations count
    pub vendor_sod_violations: u32,
    /// Vendor key rotation compliance percentage
    pub vendor_key_rotation_compliance_pct: u32,
    /// Vendor key health checks pass count
    pub vendor_key_health_checks_pass: u32,
    /// Vendor policy coverage percentage
    pub vendor_policy_coverage_pct: u32,
    /// Vendor policy violations count
    pub vendor_policy_violations: u32,
    /// Vendor authentication success rate (percentage)
    pub vendor_auth_success_rate: f64,
    /// Vendor authentication failure rate (percentage)
    pub vendor_auth_failure_rate: f64,
}

/// Vendor attention item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorAttentionItem {
    /// Vendor identifier
    pub vendor_id: String,
    /// Vendor name
    pub vendor_name: String,
    /// Type of attention required
    pub attention_type: AttentionType,
    /// Due date
    pub due_date: u64,
}

/// Attention type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttentionType {
    AssessmentRequired,
    AccessReviewRequired,
}

#[cfg(test)]
mod tests {
    use super::*;
    use vendor_management::{Vendor, VendorContact, RiskTier};
    use vendor_access::{VendorAccess, AccessType, AccessLevel, AccessStatus};

    #[test]
    fn test_vendor_management_system() {
        let mut system = VendorManagementSystem::new();
        
        // Register a vendor
        let contact = VendorContact {
            name: "John Doe".to_string(),
            email: "john@vendor.com".to_string(),
            phone: "+1234567890".to_string(),
            address: "123 Vendor St, City, Country".to_string(),
        };
        
        let vendor = Vendor {
            id: "vendor1".to_string(),
            name: "Test Vendor".to_string(),
            contact,
            services: vec!["Cloud Storage".to_string()],
            onboarding_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            contract_expiration: None,
            risk_tier: RiskTier::Medium,
            risk_score: 50,
            last_assessment: None,
            assessment_frequency: 90, // 90 days
            next_assessment_due: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() + 90 * 24 * 60 * 60,
            caIQ_mappings: vec![],
            questionnaires: vec![],
        };
        
        system.risk_manager.register_vendor(vendor);
        
        // Grant access to vendor
        let access = VendorAccess {
            id: "access1".to_string(),
            vendor_id: "vendor1".to_string(),
            resource: "Database".to_string(),
            access_type: AccessType::Database,
            access_level: AccessLevel::Read,
            granted_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            expiration_date: None,
            review_frequency: 90,
            next_review_due: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() + 90 * 24 * 60 * 60,
            justification: "Data analysis".to_string(),
            granted_by: "Security Team".to_string(),
            status: AccessStatus::Active,
        };
        
        system.access_manager.grant_access(access);
        
        // Check metrics
        let metrics = system.get_vendor_metrics();
        assert_eq!(metrics.risk_metrics.vendor_count, 1);
        assert_eq!(metrics.review_metrics.total_reviews, 0);
        
        // Generate reports
        let due_diligence_report = system.generate_due_diligence_report("vendor1");
        assert!(due_diligence_report.contains("Test Vendor"));
        
        let access_attestation = system.generate_access_attestation("access1");
        assert!(access_attestation.contains("Database"));
    }
}
