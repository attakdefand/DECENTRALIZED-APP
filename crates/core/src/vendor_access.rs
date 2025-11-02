//! Vendor access management module
//!
//! This module implements access review and attestation workflows for third-party vendors.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Vendor access record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorAccess {
    /// Unique access identifier
    pub id: String,
    /// Vendor identifier
    pub vendor_id: String,
    /// System or resource being accessed
    pub resource: String,
    /// Access type
    pub access_type: AccessType,
    /// Access level
    pub access_level: AccessLevel,
    /// Granted date
    pub granted_date: u64,
    /// Expiration date
    pub expiration_date: Option<u64>,
    /// Review frequency in days
    pub review_frequency: u32,
    /// Next review due date
    pub next_review_due: u64,
    /// Access justification
    pub justification: String,
    /// Granting authority
    pub granted_by: String,
    /// Status
    pub status: AccessStatus,
}

/// Access type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessType {
    API,
    Database,
    FileSystem,
    Network,
    Application,
    Other(String),
}

/// Access level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessLevel {
    Read,
    Write,
    Admin,
    Root,
}

/// Access status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessStatus {
    Active,
    Inactive,
    Revoked,
    Expired,
}

/// Access review record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessReview {
    /// Unique review identifier
    pub id: String,
    /// Vendor access identifier
    pub access_id: String,
    /// Review date
    pub review_date: u64,
    /// Reviewer
    pub reviewer: String,
    /// Review outcome
    pub outcome: ReviewOutcome,
    /// Comments
    pub comments: Option<String>,
    /// Next review date
    pub next_review_date: u64,
    /// Attestation
    pub attestation: Option<Attestation>,
}

/// Review outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReviewOutcome {
    Approved,
    Revoked,
    Modified,
    Expired,
}

/// Attestation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attestation {
    /// Attestation identifier
    pub id: String,
    /// Attested by
    pub attested_by: String,
    /// Attestation date
    pub attestation_date: u64,
    /// Attestation statement
    pub statement: String,
    /// Signature (simplified)
    pub signature: String,
}

/// Vendor access manager
pub struct VendorAccessManager {
    /// Vendor access records
    pub vendor_accesses: HashMap<String, VendorAccess>,
    /// Access reviews
    pub access_reviews: Vec<AccessReview>,
    /// Attestations
    pub attestations: Vec<Attestation>,
}

impl VendorAccessManager {
    /// Create a new vendor access manager
    pub fn new() -> Self {
        Self {
            vendor_accesses: HashMap::new(),
            access_reviews: Vec::new(),
            attestations: Vec::new(),
        }
    }

    /// Grant access to a vendor
    pub fn grant_access(&mut self, access: VendorAccess) {
        self.vendor_accesses.insert(access.id.clone(), access);
    }

    /// Revoke vendor access
    pub fn revoke_access(&mut self, access_id: &str) -> Result<(), &'static str> {
        if let Some(access) = self.vendor_accesses.get_mut(access_id) {
            access.status = AccessStatus::Revoked;
            Ok(())
        } else {
            Err("Access record not found")
        }
    }

    /// Get vendor access by ID
    pub fn get_vendor_access(&self, access_id: &str) -> Option<&VendorAccess> {
        self.vendor_accesses.get(access_id)
    }

    /// Get all accesses for a vendor
    pub fn get_vendor_accesses(&self, vendor_id: &str) -> Vec<&VendorAccess> {
        self.vendor_accesses
            .values()
            .filter(|access| access.vendor_id == vendor_id)
            .collect()
    }

    /// Create access review
    pub fn create_access_review(&mut self, review: AccessReview) {
        self.access_reviews.push(review);
    }

    /// Get access reviews for an access record
    pub fn get_access_reviews(&self, access_id: &str) -> Vec<&AccessReview> {
        self.access_reviews
            .iter()
            .filter(|review| review.access_id == access_id)
            .collect()
    }

    /// Get overdue access reviews
    pub fn get_overdue_reviews(&self) -> Vec<&AccessReview> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        self.access_reviews
            .iter()
            .filter(|review| review.next_review_date <= current_time)
            .collect()
    }

    /// Create attestation
    pub fn create_attestation(&mut self, attestation: Attestation) {
        self.attestations.push(attestation);
    }

    /// Get attestation by ID
    pub fn get_attestation(&self, attestation_id: &str) -> Option<&Attestation> {
        self.attestations
            .iter()
            .find(|attestation| attestation.id == attestation_id)
    }

    /// Get accesses requiring review
    pub fn get_accesses_requiring_review(&self) -> Vec<&VendorAccess> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        self.vendor_accesses
            .values()
            .filter(|access| {
                access.status == AccessStatus::Active 
                && access.next_review_due <= current_time
            })
            .collect()
    }

    /// Get review metrics
    pub fn get_review_metrics(&self) -> ReviewMetrics {
        let total_reviews = self.access_reviews.len() as u32;
        let mut overdue_count = 0;
        let mut approved_count = 0;
        let mut revoked_count = 0;
        
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        
        for review in &self.access_reviews {
            if review.next_review_date <= current_time {
                overdue_count += 1;
            }
            
            match review.outcome {
                ReviewOutcome::Approved => approved_count += 1,
                ReviewOutcome::Revoked => revoked_count += 1,
                _ => (),
            }
        }
        
        ReviewMetrics {
            total_reviews,
            overdue_reviews: overdue_count,
            approved_reviews: approved_count,
            revoked_reviews: revoked_count,
        }
    }
}

/// Review metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewMetrics {
    /// Total number of reviews
    pub total_reviews: u32,
    /// Number of overdue reviews
    pub overdue_reviews: u32,
    /// Number of approved reviews
    pub approved_reviews: u32,
    /// Number of revoked reviews
    pub revoked_reviews: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grant_access() {
        let mut manager = VendorAccessManager::new();
        
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
        
        manager.grant_access(access);
        assert!(manager.get_vendor_access("access1").is_some());
    }

    #[test]
    fn test_revoke_access() {
        let mut manager = VendorAccessManager::new();
        
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
        
        manager.grant_access(access);
        assert!(manager.revoke_access("access1").is_ok());
        
        let revoked_access = manager.get_vendor_access("access1").unwrap();
        assert!(matches!(revoked_access.status, AccessStatus::Revoked));
    }

    #[test]
    fn test_access_review_metrics() {
        let mut manager = VendorAccessManager::new();
        
        let review = AccessReview {
            id: "review1".to_string(),
            access_id: "access1".to_string(),
            review_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            reviewer: "Security Officer".to_string(),
            outcome: ReviewOutcome::Approved,
            comments: None,
            next_review_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() + 90 * 24 * 60 * 60,
            attestation: None,
        };
        
        manager.create_access_review(review);
        let metrics = manager.get_review_metrics();
        
        assert_eq!(metrics.total_reviews, 1);
        assert_eq!(metrics.approved_reviews, 1);
    }
}