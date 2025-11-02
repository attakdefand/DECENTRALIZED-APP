//! Vendor risk management module
//!
//! This module implements vendor risk assessment, due diligence workflows,
//! and third-party monitoring capabilities for the DECENTRALIZED-APP protocol.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Vendor risk assessment questionnaire
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorQuestionnaire {
    /// Unique identifier for the questionnaire
    pub id: String,
    /// Vendor name
    pub vendor_name: String,
    /// Assessment date
    pub assessment_date: u64,
    /// Assessor name
    pub assessor: String,
    /// Questionnaire sections
    pub sections: Vec<QuestionnaireSection>,
    /// Overall risk score (0-100)
    pub overall_score: u32,
    /// Assessment status
    pub status: AssessmentStatus,
}

/// Questionnaire section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionnaireSection {
    /// Section name
    pub name: String,
    /// Section weight in overall score
    pub weight: f64,
    /// Questions in this section
    pub questions: Vec<Question>,
}

/// Question in a questionnaire
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    /// Question text
    pub text: String,
    /// Question weight
    pub weight: f64,
    /// Answer provided
    pub answer: Option<String>,
    /// Score for this question (0-100)
    pub score: Option<u32>,
    /// Comments on the answer
    pub comments: Option<String>,
}

/// Assessment status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssessmentStatus {
    Draft,
    InProgress,
    Completed,
    Expired,
}

/// CAIQ (Consensus Assessments Initiative Questionnaire) mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CAIQMapping {
    /// CAIQ control ID
    pub control_id: String,
    /// Control name
    pub control_name: String,
    /// Control description
    pub description: String,
    /// Implementation status
    pub implementation_status: ImplementationStatus,
    /// Evidence provided
    pub evidence: Option<String>,
    /// Last reviewed date
    pub last_reviewed: u64,
}

/// Implementation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplementationStatus {
    NotApplicable,
    NotImplemented,
    PartiallyImplemented,
    FullyImplemented,
}

/// Vendor record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vendor {
    /// Unique vendor identifier
    pub id: String,
    /// Vendor name
    pub name: String,
    /// Vendor contact information
    pub contact: VendorContact,
    /// Services provided by vendor
    pub services: Vec<String>,
    /// Onboarding date
    pub onboarding_date: u64,
    /// Contract expiration date
    pub contract_expiration: Option<u64>,
    /// Vendor risk tier (High, Medium, Low)
    pub risk_tier: RiskTier,
    /// Current risk score
    pub risk_score: u32,
    /// Last assessment date
    pub last_assessment: Option<u64>,
    /// Assessment frequency in days
    pub assessment_frequency: u64,
    /// Next assessment due date
    pub next_assessment_due: u64,
    /// CAIQ mappings
    pub caIQ_mappings: Vec<CAIQMapping>,
    /// Questionnaires
    pub questionnaires: Vec<VendorQuestionnaire>,
}

/// Vendor contact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorContact {
    /// Primary contact name
    pub name: String,
    /// Primary contact email
    pub email: String,
    /// Primary contact phone
    pub phone: String,
    /// Address
    pub address: String,
}

/// Risk tier classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskTier {
    High,
    Medium,
    Low,
}

/// Vendor risk manager
pub struct VendorRiskManager {
    /// Registered vendors
    pub vendors: HashMap<String, Vendor>,
    /// Questionnaire templates
    pub questionnaire_templates: HashMap<String, VendorQuestionnaire>,
    /// CAIQ controls repository
    pub caIQ_controls: HashMap<String, CAIQMapping>,
}

impl VendorRiskManager {
    /// Create a new vendor risk manager
    pub fn new() -> Self {
        Self {
            vendors: HashMap::new(),
            questionnaire_templates: HashMap::new(),
            caIQ_controls: HashMap::new(),
        }
    }

    /// Register a new vendor
    pub fn register_vendor(&mut self, vendor: Vendor) {
        self.vendors.insert(vendor.id.clone(), vendor);
    }

    /// Get vendor by ID
    pub fn get_vendor(&self, vendor_id: &str) -> Option<&Vendor> {
        self.vendors.get(vendor_id)
    }

    /// Update vendor risk score
    pub fn update_vendor_risk_score(&mut self, vendor_id: &str, score: u32) -> Result<(), &'static str> {
        if let Some(vendor) = self.vendors.get_mut(vendor_id) {
            vendor.risk_score = score;
            vendor.last_assessment = Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs()
            );
            Ok(())
        } else {
            Err("Vendor not found")
        }
    }

    /// Add questionnaire template
    pub fn add_questionnaire_template(&mut self, template: VendorQuestionnaire) {
        self.questionnaire_templates.insert(template.id.clone(), template);
    }

    /// Create vendor assessment from template
    pub fn create_vendor_assessment(&self, vendor_id: &str, template_id: &str) -> Option<VendorQuestionnaire> {
        if let (Some(_vendor), Some(template)) = (self.vendors.get(vendor_id), self.questionnaire_templates.get(template_id)) {
            let mut assessment = template.clone();
            assessment.id = format!("assessment_{}_{}", vendor_id, 
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs()
            );
            assessment.assessment_date = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();
            assessment.status = AssessmentStatus::InProgress;
            Some(assessment)
        } else {
            None
        }
    }

    /// Complete vendor assessment
    pub fn complete_vendor_assessment(&mut self, mut assessment: VendorQuestionnaire) -> Result<(), &'static str> {
        // Calculate overall score
        let mut total_score = 0.0;
        let mut total_weight = 0.0;
        
        for section in &assessment.sections {
            let mut section_score = 0.0;
            let mut section_weight = 0.0;
            
            for question in &section.questions {
                if let Some(score) = question.score {
                    section_score += score as f64 * question.weight;
                    section_weight += question.weight;
                }
            }
            
            if section_weight > 0.0 {
                total_score += (section_score / section_weight) * section.weight;
                total_weight += section.weight;
            }
        }
        
        if total_weight > 0.0 {
            assessment.overall_score = (total_score / total_weight) as u32;
        }
        
        assessment.status = AssessmentStatus::Completed;
        
        // Update vendor with completed assessment
        if let Some(vendor) = self.vendors.get_mut(&assessment.id.split('_').nth(1).unwrap_or("")) {
            vendor.questionnaires.push(assessment.clone());
            vendor.risk_score = assessment.overall_score;
            vendor.last_assessment = Some(assessment.assessment_date);
        }
        
        Ok(())
    }

    /// Add CAIQ control
    pub fn add_caiq_control(&mut self, control: CAIQMapping) {
        self.caIQ_controls.insert(control.control_id.clone(), control);
    }

    /// Get vendors requiring assessment
    pub fn get_vendors_requiring_assessment(&self) -> Vec<&Vendor> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
            
        self.vendors
            .values()
            .filter(|vendor| vendor.next_assessment_due <= current_time)
            .collect()
    }

    /// Get vendor risk metrics
    pub fn get_vendor_risk_metrics(&self) -> VendorRiskMetrics {
        let mut high_risk_count = 0;
        let mut medium_risk_count = 0;
        let mut low_risk_count = 0;
        let mut total_score = 0;
        
        for vendor in self.vendors.values() {
            match vendor.risk_tier {
                RiskTier::High => high_risk_count += 1,
                RiskTier::Medium => medium_risk_count += 1,
                RiskTier::Low => low_risk_count += 1,
            }
            total_score += vendor.risk_score;
        }
        
        let vendor_count = self.vendors.len() as u32;
        let avg_score = if vendor_count > 0 {
            total_score / vendor_count
        } else {
            0
        };
        
        VendorRiskMetrics {
            vendor_count,
            high_risk_count,
            medium_risk_count,
            low_risk_count,
            vendor_score_avg: avg_score,
        }
    }
}

/// Vendor risk metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorRiskMetrics {
    /// Total number of vendors
    pub vendor_count: u32,
    /// Number of high-risk vendors
    pub high_risk_count: u32,
    /// Number of medium-risk vendors
    pub medium_risk_count: u32,
    /// Number of low-risk vendors
    pub low_risk_count: u32,
    /// Average vendor risk score
    pub vendor_score_avg: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vendor_registration() {
        let mut manager = VendorRiskManager::new();
        
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
        
        manager.register_vendor(vendor.clone());
        let retrieved_vendor = manager.get_vendor("vendor1");
        
        assert!(retrieved_vendor.is_some());
        assert_eq!(retrieved_vendor.unwrap().name, "Test Vendor");
    }

    #[test]
    fn test_questionnaire_template() {
        let mut manager = VendorRiskManager::new();
        
        let question = Question {
            text: "Does the vendor have SOC 2 certification?".to_string(),
            weight: 1.0,
            answer: None,
            score: None,
            comments: None,
        };
        
        let section = QuestionnaireSection {
            name: "Compliance".to_string(),
            weight: 1.0,
            questions: vec![question],
        };
        
        let template = VendorQuestionnaire {
            id: "compliance_template".to_string(),
            vendor_name: "Template".to_string(),
            assessment_date: 0,
            assessor: "System".to_string(),
            sections: vec![section],
            overall_score: 0,
            status: AssessmentStatus::Draft,
        };
        
        manager.add_questionnaire_template(template);
        assert!(manager.questionnaire_templates.contains_key("compliance_template"));
    }

    #[test]
    fn test_vendor_risk_metrics() {
        let mut manager = VendorRiskManager::new();
        
        // Add test vendors
        let contact = VendorContact {
            name: "John Doe".to_string(),
            email: "john@vendor.com".to_string(),
            phone: "+1234567890".to_string(),
            address: "123 Vendor St, City, Country".to_string(),
        };
        
        let vendor1 = Vendor {
            id: "vendor1".to_string(),
            name: "High Risk Vendor".to_string(),
            contact: contact.clone(),
            services: vec!["Cloud Storage".to_string()],
            onboarding_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            contract_expiration: None,
            risk_tier: RiskTier::High,
            risk_score: 80,
            last_assessment: None,
            assessment_frequency: 90,
            next_assessment_due: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() + 90 * 24 * 60 * 60,
            caIQ_mappings: vec![],
            questionnaires: vec![],
        };
        
        let vendor2 = Vendor {
            id: "vendor2".to_string(),
            name: "Low Risk Vendor".to_string(),
            contact,
            services: vec!["Support Services".to_string()],
            onboarding_date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            contract_expiration: None,
            risk_tier: RiskTier::Low,
            risk_score: 20,
            last_assessment: None,
            assessment_frequency: 180,
            next_assessment_due: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() + 180 * 24 * 60 * 60,
            caIQ_mappings: vec![],
            questionnaires: vec![],
        };
        
        manager.register_vendor(vendor1);
        manager.register_vendor(vendor2);
        
        let metrics = manager.get_vendor_risk_metrics();
        assert_eq!(metrics.vendor_count, 2);
        assert_eq!(metrics.high_risk_count, 1);
        assert_eq!(metrics.low_risk_count, 1);
        assert_eq!(metrics.vendor_score_avg, 50); // (80 + 20) / 2
    }
}