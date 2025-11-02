//! Client Protection Module
//!
//! This module implements client-side security features including:
//! - Transaction confirmation flows
//! - Phishing resistance (domain verification, visual security cues)
//! - Fraud alert systems

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use regex::Regex;

/// Transaction confirmation flow
pub struct TransactionConfirmation {
    /// Transaction details
    pub transaction: TransactionDetails,
    /// Confirmation status
    pub status: ConfirmationStatus,
    /// Security checks performed
    pub security_checks: Vec<SecurityCheck>,
    /// Timestamp of confirmation
    pub timestamp: u64,
}

/// Transaction details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionDetails {
    /// Transaction ID
    pub id: String,
    /// Amount being transferred
    pub amount: f64,
    /// Token type
    pub token: String,
    /// Destination address
    pub to_address: String,
    /// Origin address
    pub from_address: String,
    /// Gas limit
    pub gas_limit: u64,
    /// Gas price
    pub gas_price: u64,
    /// Network chain ID
    pub chain_id: u64,
}

/// Confirmation status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConfirmationStatus {
    /// Pending user confirmation
    Pending,
    /// Confirmed by user
    Confirmed,
    /// Rejected by user
    Rejected,
    /// Timed out
    Timeout,
}

/// Security check performed during transaction confirmation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCheck {
    /// Check type
    pub check_type: SecurityCheckType,
    /// Check result
    pub result: SecurityCheckResult,
    /// Details about the check
    pub details: String,
    /// Timestamp when check was performed
    pub timestamp: u64,
}

/// Types of security checks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityCheckType {
    /// Domain verification
    DomainVerification,
    /// Address verification
    AddressVerification,
    /// Amount verification
    AmountVerification,
    /// Gas analysis
    GasAnalysis,
    /// Phishing detection
    PhishingDetection,
    /// Risk assessment
    RiskAssessment,
}

/// Security check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityCheckResult {
    /// Check passed
    Passed,
    /// Check failed
    Failed,
    /// Warning issued
    Warning,
}

/// Phishing resistance system
pub struct PhishingResistance {
    /// Known legitimate domains
    legitimate_domains: Vec<String>,
    /// Known phishing domains
    phishing_domains: Vec<String>,
    /// Suspicious patterns
    suspicious_patterns: Vec<Regex>,
    /// Visual security cues configuration
    visual_cues: VisualSecurityCues,
}

/// Visual security cues configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualSecurityCues {
    /// Show security badges for verified domains
    pub show_verification_badges: bool,
    /// Highlight suspicious elements
    pub highlight_suspicious: bool,
    /// Show warnings for unknown domains
    pub warn_unknown_domains: bool,
    /// Show security status indicators
    pub show_status_indicators: bool,
}

impl PhishingResistance {
    /// Create a new phishing resistance system
    pub fn new() -> Self {
        Self {
            legitimate_domains: vec![
                "app.decentralized-app.com".to_string(),
                "wallet.decentralized-app.com".to_string(),
                "api.decentralized-app.com".to_string(),
            ],
            phishing_domains: vec![
                "decentralized-app.login.com".to_string(),
                "decentralized-app.secure-login.com".to_string(),
                "my-decentralized-app.com".to_string(),
            ],
            suspicious_patterns: vec![
                Regex::new(r"login").unwrap(),
                Regex::new(r"secure").unwrap(),
                Regex::new(r"account").unwrap(),
                Regex::new(r"update").unwrap(),
            ],
            visual_cues: VisualSecurityCues {
                show_verification_badges: true,
                highlight_suspicious: true,
                warn_unknown_domains: true,
                show_status_indicators: true,
            },
        }
    }

    /// Verify if a domain is legitimate
    pub fn verify_domain(&self, domain: &str) -> DomainVerificationResult {
        // Check if domain is in legitimate list
        if self.legitimate_domains.contains(&domain.to_string()) {
            return DomainVerificationResult::Legitimate {
                confidence: 100,
                details: "Domain is in known legitimate list".to_string(),
            };
        }

        // Check if domain is in phishing list
        if self.phishing_domains.contains(&domain.to_string()) {
            return DomainVerificationResult::Phishing {
                confidence: 100,
                details: "Domain is in known phishing list".to_string(),
            };
        }

        // Check for suspicious patterns
        for pattern in &self.suspicious_patterns {
            if pattern.is_match(domain) {
                return DomainVerificationResult::Suspicious {
                    confidence: 80,
                    details: format!("Domain matches suspicious pattern: {}", pattern.as_str()),
                };
            }
        }

        // Domain not recognized
        DomainVerificationResult::Unknown {
            confidence: 0,
            details: "Domain is not recognized".to_string(),
        }
    }

    /// Get visual security cues configuration
    pub fn get_visual_cues(&self) -> &VisualSecurityCues {
        &self.visual_cues
    }
}

impl Default for PhishingResistance {
    fn default() -> Self {
        Self::new()
    }
}

/// Domain verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomainVerificationResult {
    /// Legitimate domain
    Legitimate {
        /// Confidence level (0-100)
        confidence: u8,
        /// Details about verification
        details: String,
    },
    /// Phishing domain
    Phishing {
        /// Confidence level (0-100)
        confidence: u8,
        /// Details about verification
        details: String,
    },
    /// Suspicious domain
    Suspicious {
        /// Confidence level (0-100)
        confidence: u8,
        /// Details about verification
        details: String,
    },
    /// Unknown domain
    Unknown {
        /// Confidence level (0-100)
        confidence: u8,
        /// Details about verification
        details: String,
    },
}

/// Fraud alert system
pub struct FraudAlertSystem {
    /// Active alerts
    alerts: HashMap<String, FraudAlert>,
    /// Alert thresholds
    thresholds: FraudAlertThresholds,
    /// Notification channels
    notification_channels: Vec<NotificationChannel>,
}

/// Fraud alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudAlert {
    /// Alert ID
    pub id: String,
    /// Alert type
    pub alert_type: FraudAlertType,
    /// Severity level
    pub severity: AlertSeverity,
    /// Description of the alert
    pub description: String,
    /// Affected user ID
    pub user_id: String,
    /// Timestamp when alert was generated
    pub timestamp: u64,
    /// Whether alert has been acknowledged
    pub acknowledged: bool,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Types of fraud alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FraudAlertType {
    /// Suspicious transaction detected
    SuspiciousTransaction,
    /// Large value transfer
    LargeTransfer,
    /// Rapid transactions
    RapidTransactions,
    /// Unusual location
    UnusualLocation,
    /// Failed login attempts
    FailedLogins,
    /// New device login
    NewDeviceLogin,
    /// Address mismatch
    AddressMismatch,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum AlertSeverity {
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity
    High,
    /// Critical severity
    Critical,
}

/// Fraud alert thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudAlertThresholds {
    /// Large transfer threshold in USD
    pub large_transfer_threshold: f64,
    /// Rapid transaction threshold (transactions per minute)
    pub rapid_transaction_threshold: u32,
    /// Failed login threshold
    pub failed_login_threshold: u32,
    /// Time window for rapid transactions (in seconds)
    pub rapid_transaction_window: u64,
}

/// Notification channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationChannel {
    /// Email notification
    Email {
        /// Email address
        address: String,
    },
    /// SMS notification
    SMS {
        /// Phone number
        number: String,
    },
    /// Push notification
    Push {
        /// Device token
        device_token: String,
    },
    /// In-app notification
    InApp,
}

impl FraudAlertSystem {
    /// Create a new fraud alert system
    pub fn new() -> Self {
        Self {
            alerts: HashMap::new(),
            thresholds: FraudAlertThresholds {
                large_transfer_threshold: 10000.0, // $10,000
                rapid_transaction_threshold: 10,   // 10 transactions per minute
                failed_login_threshold: 5,         // 5 failed logins
                rapid_transaction_window: 60,      // 60 seconds
            },
            notification_channels: vec![
                NotificationChannel::InApp,
                NotificationChannel::Email {
                    address: "user@example.com".to_string(),
                },
            ],
        }
    }

    /// Generate a fraud alert
    pub fn generate_alert(&mut self, alert: FraudAlert) -> Result<(), String> {
        if self.alerts.contains_key(&alert.id) {
            return Err("Alert with this ID already exists".to_string());
        }

        self.alerts.insert(alert.id.clone(), alert);
        Ok(())
    }

    /// Acknowledge an alert
    pub fn acknowledge_alert(&mut self, alert_id: &str) -> Result<(), String> {
        match self.alerts.get_mut(alert_id) {
            Some(alert) => {
                alert.acknowledged = true;
                Ok(())
            }
            None => Err("Alert not found".to_string()),
        }
    }

    /// Get unacknowledged alerts for a user
    pub fn get_unacknowledged_alerts(&self, user_id: &str) -> Vec<&FraudAlert> {
        self.alerts
            .values()
            .filter(|alert| !alert.acknowledged && alert.user_id == user_id)
            .collect()
    }

    /// Check for large transfer alert
    pub fn check_large_transfer(&self, amount: f64, user_id: &str) -> Option<FraudAlert> {
        if amount > self.thresholds.large_transfer_threshold {
            Some(FraudAlert {
                id: format!("large-transfer-{}-{}", user_id, current_timestamp()),
                alert_type: FraudAlertType::LargeTransfer,
                severity: if amount > self.thresholds.large_transfer_threshold * 2.0 {
                    AlertSeverity::Critical
                } else {
                    AlertSeverity::High
                },
                description: format!("Large transfer of ${:.2} detected", amount),
                user_id: user_id.to_string(),
                timestamp: current_timestamp(),
                acknowledged: false,
                metadata: {
                    let mut map = HashMap::new();
                    map.insert("amount".to_string(), amount.to_string());
                    map.insert("threshold".to_string(), self.thresholds.large_transfer_threshold.to_string());
                    map
                },
            })
        } else {
            None
        }
    }

    /// Check for rapid transactions
    pub fn check_rapid_transactions(&self, transaction_count: u32, time_window: u64, user_id: &str) -> Option<FraudAlert> {
        if time_window <= self.thresholds.rapid_transaction_window 
            && transaction_count > self.thresholds.rapid_transaction_threshold {
            Some(FraudAlert {
                id: format!("rapid-transactions-{}-{}", user_id, current_timestamp()),
                alert_type: FraudAlertType::RapidTransactions,
                severity: if transaction_count > self.thresholds.rapid_transaction_threshold * 2 {
                    AlertSeverity::High
                } else {
                    AlertSeverity::Medium
                },
                description: format!("{} rapid transactions detected in {} seconds", transaction_count, time_window),
                user_id: user_id.to_string(),
                timestamp: current_timestamp(),
                acknowledged: false,
                metadata: {
                    let mut map = HashMap::new();
                    map.insert("count".to_string(), transaction_count.to_string());
                    map.insert("window".to_string(), time_window.to_string());
                    map.insert("threshold".to_string(), self.thresholds.rapid_transaction_threshold.to_string());
                    map
                },
            })
        } else {
            None
        }
    }

    /// Check for failed logins
    pub fn check_failed_logins(&self, failed_count: u32, user_id: &str) -> Option<FraudAlert> {
        if failed_count >= self.thresholds.failed_login_threshold {
            Some(FraudAlert {
                id: format!("failed-logins-{}-{}", user_id, current_timestamp()),
                alert_type: FraudAlertType::FailedLogins,
                severity: if failed_count > self.thresholds.failed_login_threshold * 2 {
                    AlertSeverity::Critical
                } else {
                    AlertSeverity::High
                },
                description: format!("{} failed login attempts detected", failed_count),
                user_id: user_id.to_string(),
                timestamp: current_timestamp(),
                acknowledged: false,
                metadata: {
                    let mut map = HashMap::new();
                    map.insert("count".to_string(), failed_count.to_string());
                    map.insert("threshold".to_string(), self.thresholds.failed_login_threshold.to_string());
                    map
                },
            })
        } else {
            None
        }
    }

    /// Send notifications for alerts
    pub fn send_notifications(&self, alert: &FraudAlert) {
        for channel in &self.notification_channels {
            match channel {
                NotificationChannel::Email { address } => {
                    // In a real implementation, this would send an email
                    println!("Sending email to {} about alert: {}", address, alert.description);
                }
                NotificationChannel::SMS { number } => {
                    // In a real implementation, this would send an SMS
                    println!("Sending SMS to {} about alert: {}", number, alert.description);
                }
                NotificationChannel::Push { device_token } => {
                    // In a real implementation, this would send a push notification
                    println!("Sending push notification to device {} about alert: {}", device_token, alert.description);
                }
                NotificationChannel::InApp => {
                    // In a real implementation, this would show an in-app notification
                    println!("Showing in-app notification about alert: {}", alert.description);
                }
            }
        }
    }
}

impl Default for FraudAlertSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to get current timestamp
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phishing_resistance_domain_verification() {
        let phishing_resistance = PhishingResistance::new();
        
        // Test legitimate domain
        let result = phishing_resistance.verify_domain("app.decentralized-app.com");
        match result {
            DomainVerificationResult::Legitimate { confidence, .. } => {
                assert_eq!(confidence, 100);
            }
            _ => panic!("Expected legitimate domain result"),
        }
        
        // Test phishing domain
        let result = phishing_resistance.verify_domain("decentralized-app.login.com");
        match result {
            DomainVerificationResult::Phishing { confidence, .. } => {
                assert_eq!(confidence, 100);
            }
            _ => panic!("Expected phishing domain result"),
        }
        
        // Test suspicious domain
        let result = phishing_resistance.verify_domain("secure-login.decentralized-app.com");
        match result {
            DomainVerificationResult::Suspicious { confidence, .. } => {
                assert!(confidence >= 80);
            }
            _ => panic!("Expected suspicious domain result"),
        }
        
        // Test unknown domain
        let result = phishing_resistance.verify_domain("unknown-domain.com");
        match result {
            DomainVerificationResult::Unknown { confidence, .. } => {
                assert_eq!(confidence, 0);
            }
            _ => panic!("Expected unknown domain result"),
        }
    }
    
    #[test]
    fn test_fraud_alert_system_large_transfer() {
        let fraud_alerts = FraudAlertSystem::new();
        
        // Test normal transfer (should not generate alert)
        let alert = fraud_alerts.check_large_transfer(5000.0, "user123");
        assert!(alert.is_none());
        
        // Test large transfer (should generate alert)
        let alert = fraud_alerts.check_large_transfer(15000.0, "user123");
        assert!(alert.is_some());
        
        let alert = alert.unwrap();
        assert_eq!(alert.alert_type, FraudAlertType::LargeTransfer);
        assert_eq!(alert.severity, AlertSeverity::High);
        assert_eq!(alert.user_id, "user123");
    }
    
    #[test]
    fn test_fraud_alert_system_rapid_transactions() {
        let fraud_alerts = FraudAlertSystem::new();
        
        // Test normal transaction rate (should not generate alert)
        let alert = fraud_alerts.check_rapid_transactions(5, 60, "user123");
        assert!(alert.is_none());
        
        // Test rapid transactions (should generate alert)
        let alert = fraud_alerts.check_rapid_transactions(15, 60, "user123");
        assert!(alert.is_some());
        
        let alert = alert.unwrap();
        assert_eq!(alert.alert_type, FraudAlertType::RapidTransactions);
        assert_eq!(alert.severity, AlertSeverity::Medium);
        assert_eq!(alert.user_id, "user123");
    }
    
    #[test]
    fn test_fraud_alert_system_failed_logins() {
        let fraud_alerts = FraudAlertSystem::new();
        
        // Test normal login attempts (should not generate alert)
        let alert = fraud_alerts.check_failed_logins(3, "user123");
        assert!(alert.is_none());
        
        // Test excessive failed logins (should generate alert)
        let alert = fraud_alerts.check_failed_logins(7, "user123");
        assert!(alert.is_some());
        
        let alert = alert.unwrap();
        assert_eq!(alert.alert_type, FraudAlertType::FailedLogins);
        assert_eq!(alert.severity, AlertSeverity::High);
        assert_eq!(alert.user_id, "user123");
    }
    
    #[test]
    fn test_fraud_alert_system_alert_management() {
        let mut fraud_alerts = FraudAlertSystem::new();
        
        // Create a test alert
        let alert = FraudAlert {
            id: "test-alert-1".to_string(),
            alert_type: FraudAlertType::SuspiciousTransaction,
            severity: AlertSeverity::Medium,
            description: "Test alert".to_string(),
            user_id: "user123".to_string(),
            timestamp: current_timestamp(),
            acknowledged: false,
            metadata: HashMap::new(),
        };
        
        // Generate the alert
        assert!(fraud_alerts.generate_alert(alert).is_ok());
        
        // Try to generate the same alert again (should fail)
        let alert2 = FraudAlert {
            id: "test-alert-1".to_string(),
            alert_type: FraudAlertType::SuspiciousTransaction,
            severity: AlertSeverity::Medium,
            description: "Test alert".to_string(),
            user_id: "user123".to_string(),
            timestamp: current_timestamp(),
            acknowledged: false,
            metadata: HashMap::new(),
        };
        assert!(fraud_alerts.generate_alert(alert2).is_err());
        
        // Acknowledge the alert
        assert!(fraud_alerts.acknowledge_alert("test-alert-1").is_ok());
        
        // Try to acknowledge non-existent alert (should fail)
        assert!(fraud_alerts.acknowledge_alert("non-existent-alert").is_err());
        
        // Check unacknowledged alerts (should be empty)
        let unacknowledged = fraud_alerts.get_unacknowledged_alerts("user123");
        assert!(unacknowledged.is_empty());
    }
}