//! Main vendor management module
//!
//! This module integrates all vendor management capabilities including risk assessment,
//! SLA monitoring, and access management.

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// Import submodules
pub mod vendor_management;
pub mod sla_monitoring;
pub mod vendor_access;

use vendor_management::{VendorRiskManager, VendorRiskMetrics};
use sla_monitoring::SLAMonitoringManager;
use vendor_access::{VendorAccessManager, ReviewMetrics};

/// Vendor management system
pub struct VendorManagementSystem {
    /// Vendor risk manager
    pub risk_manager: VendorRiskManager,
    /// SLA monitoring manager
    pub sla_manager: SLAMonitoringManager,
    /// Vendor access manager
    pub access_manager: VendorAccessManager,
}

impl VendorManagementSystem {
    /// Create a new vendor management system
    pub fn new() -> Self {
        Self {
            risk_manager: VendorRiskManager::new(),
            sla_manager: SLAMonitoringManager::new(),
            access_manager: VendorAccessManager::new(),
        }
    }

    /// Get overall vendor metrics
    pub fn get_vendor_metrics(&self) -> VendorMetrics {
        let risk_metrics = self.risk_manager.get_vendor_risk_metrics();
        let review_metrics = self.access_manager.get_review_metrics();
        
        VendorMetrics {
            risk_metrics,
            review_metrics,
            overdue_reviews: review_metrics.overdue_reviews,
            vendor_score_avg: risk_metrics.vendor_score_avg,
            last_updated: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
        }
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