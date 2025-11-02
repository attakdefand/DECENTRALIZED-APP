//! Evidence generation module for Proof of Reserves and treasury management
//!
//! This module provides functionality for generating, storing, and validating
//! evidence related to Proof of Reserves reports and limit breaches.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use md5;

/// Evidence types for different security controls
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceType {
    ProofOfReserves,
    LimitBreach,
    TreasuryManagement,
    RiskAssessment,
}

/// Evidence record for audit trail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceRecord {
    /// Unique identifier for the evidence
    pub id: String,
    /// Type of evidence
    pub evidence_type: EvidenceType,
    /// Timestamp when evidence was generated
    pub timestamp: u64,
    /// Evidence content in JSON format
    pub content: String,
    /// Hash of the evidence for integrity verification
    pub hash: String,
    /// Signatures from validators (if applicable)
    pub signatures: Vec<String>,
    /// Retention period in days
    pub retention_days: u64,
}

impl EvidenceRecord {
    pub fn new(
        id: String,
        evidence_type: EvidenceType,
        content: String,
        signatures: Vec<String>,
        retention_days: u64,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        // Simple hash calculation (in practice, use a proper cryptographic hash)
        let hash = format!("{:x}", md5::compute(&content));
        
        Self {
            id,
            evidence_type,
            timestamp,
            content,
            hash,
            signatures,
            retention_days,
        }
    }
    
    /// Check if evidence is still within retention period
    pub fn is_within_retention(&self) -> bool {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let age_days = (current_time - self.timestamp) / 86400;
        age_days < self.retention_days
    }
}

/// Evidence manager for handling evidence lifecycle
pub struct EvidenceManager {
    /// Storage for evidence records
    pub evidence_records: HashMap<String, EvidenceRecord>,
    /// Evidence retention policy in days
    pub retention_policy_days: u64,
}

impl EvidenceManager {
    pub fn new(retention_policy_days: u64) -> Self {
        Self {
            evidence_records: HashMap::new(),
            retention_policy_days,
        }
    }
    
    /// Add new evidence record
    pub fn add_evidence(&mut self, evidence: EvidenceRecord) {
        self.evidence_records.insert(evidence.id.clone(), evidence);
    }
    
    /// Generate Proof of Reserves evidence
    pub fn generate_por_evidence(
        &mut self,
        reserves_data: HashMap<String, u128>,
        total_value_usd: f64,
        validator_signature: String,
    ) -> String {
        let evidence_content = format!(
            "{{\
            \"reserves_data\": {:?},\
            \"total_value_usd\": {},\
            \"validator_signature\": \"{}\",\
            \"timestamp\": {}\
            }}",
            reserves_data,
            total_value_usd,
            validator_signature,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs()
        );
        
        let evidence_id = format!(
            "por_{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs()
        );
        
        let evidence = EvidenceRecord::new(
            evidence_id.clone(),
            EvidenceType::ProofOfReserves,
            evidence_content.clone(),
            vec![validator_signature],
            self.retention_policy_days,
        );
        
        self.add_evidence(evidence);
        evidence_content
    }
    
    /// Generate limit breach evidence
    pub fn generate_limit_breach_evidence(&mut self, breach_count: u64) -> String {
        let evidence_content = format!(
            "{{\
            \"breach_count\": {},\
            \"timestamp\": {}\
            }}",
            breach_count,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs()
        );
        
        let evidence_id = format!(
            "limit_breach_{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs()
        );
        
        let evidence = EvidenceRecord::new(
            evidence_id.clone(),
            EvidenceType::LimitBreach,
            evidence_content.clone(),
            vec![],
            self.retention_policy_days,
        );
        
        self.add_evidence(evidence);
        evidence_content
    }
    
    /// Get evidence by ID
    pub fn get_evidence(&self, id: &str) -> Option<&EvidenceRecord> {
        self.evidence_records.get(id)
    }
    
    /// Get all evidence of a specific type
    pub fn get_evidence_by_type(&self, evidence_type: EvidenceType) -> Vec<&EvidenceRecord> {
        self.evidence_records
            .values()
            .filter(|record| std::mem::discriminant(&record.evidence_type) == std::mem::discriminant(&evidence_type))
            .collect()
    }
    
    /// Clean up expired evidence based on retention policy
    pub fn cleanup_expired_evidence(&mut self) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        self.evidence_records.retain(|_, record| {
            let age_days = (current_time - record.timestamp) / 86400;
            age_days < record.retention_days
        });
    }
    
    /// Generate evidence report
    pub fn generate_evidence_report(&self) -> String {
        let mut report = "Evidence Report\n===============\n\n".to_string();
        
        for (id, record) in &self.evidence_records {
            report.push_str(&format!(
                "ID: {}\nType: {:?}\nTimestamp: {}\nContent: {}\nHash: {}\nSignatures: {:?}\n\n",
                id, record.evidence_type, record.timestamp, record.content, record.hash, record.signatures
            ));
        }
        
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_evidence_record_creation() {
        let evidence = EvidenceRecord::new(
            "test_id".to_string(),
            EvidenceType::ProofOfReserves,
            "test_content".to_string(),
            vec!["signature1".to_string()],
            365,
        );
        
        assert_eq!(evidence.id, "test_id");
        assert!(matches!(evidence.evidence_type, EvidenceType::ProofOfReserves));
        assert_eq!(evidence.content, "test_content");
        assert_eq!(evidence.signatures.len(), 1);
        assert_eq!(evidence.retention_days, 365);
    }
    
    #[test]
    fn test_evidence_manager() {
        let mut manager = EvidenceManager::new(365);
        
        // Test generating POR evidence
        let mut reserves = HashMap::new();
        reserves.insert("ETH".to_string(), 100000000000000000000);
        reserves.insert("USDC".to_string(), 2000000000000);
        
        let por_evidence = manager.generate_por_evidence(
            reserves,
            500000.0,
            "validator_signature".to_string(),
        );
        
        assert!(!por_evidence.is_empty());
        assert_eq!(manager.evidence_records.len(), 1);
        
        // Test generating limit breach evidence
        let breach_evidence = manager.generate_limit_breach_evidence(5);
        assert!(!breach_evidence.is_empty());
        assert_eq!(manager.evidence_records.len(), 2);
        
        // Test getting evidence by type
        let por_evidence_list = manager.get_evidence_by_type(EvidenceType::ProofOfReserves);
        assert_eq!(por_evidence_list.len(), 1);
        
        let breach_evidence_list = manager.get_evidence_by_type(EvidenceType::LimitBreach);
        assert_eq!(breach_evidence_list.len(), 1);
        
        // Test evidence report generation
        let report = manager.generate_evidence_report();
        assert!(report.contains("Evidence Report"));
        assert!(report.contains("ProofOfReserves"));
        assert!(report.contains("LimitBreach"));
    }
}