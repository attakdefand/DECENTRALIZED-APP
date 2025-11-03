//! Vendor authentication module
//!
//! This module implements authentication tracking and metrics for third-party vendors.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::vendor_logging::{VendorAuthLog, AuthEventType, AuthFailureReason as LogAuthFailureReason, create_vendor_auth_log, log_vendor_auth_event};

/// Vendor authentication manager
pub struct VendorAuthManager {
    /// Authentication events by vendor
    pub auth_events: HashMap<String, Vec<AuthEvent>>,
    /// Authentication metrics by vendor
    pub auth_metrics: HashMap<String, AuthMetrics>,
}

/// Authentication event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthEvent {
    /// Unique identifier for the auth event
    pub id: String,
    /// Vendor identifier
    pub vendor_id: String,
    /// Timestamp of the event
    pub timestamp: u64,
    /// Authentication result
    pub result: AuthResult,
    /// IP address of the request
    pub ip_address: Option<String>,
    /// User agent of the request
    pub user_agent: Option<String>,
    /// Session ID if applicable
    pub session_id: Option<String>,
    /// Token ID if applicable
    pub token_id: Option<String>,
}

/// Authentication result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthResult {
    Success,
    Failure(AuthFailureReason),
}

/// Authentication failure reason
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthFailureReason {
    InvalidCredentials,
    ExpiredToken,
    RevokedToken,
    AccountLocked,
    RateLimited,
    Other(String),
}

impl From<AuthFailureReason> for LogAuthFailureReason {
    fn from(reason: AuthFailureReason) -> Self {
        match reason {
            AuthFailureReason::InvalidCredentials => LogAuthFailureReason::InvalidCredentials,
            AuthFailureReason::ExpiredToken => LogAuthFailureReason::ExpiredToken,
            AuthFailureReason::RevokedToken => LogAuthFailureReason::RevokedToken,
            AuthFailureReason::AccountLocked => LogAuthFailureReason::AccountLocked,
            AuthFailureReason::RateLimited => LogAuthFailureReason::RateLimited,
            AuthFailureReason::Other(s) => LogAuthFailureReason::Other(s),
        }
    }
}

/// Authentication metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthMetrics {
    /// Vendor identifier
    pub vendor_id: String,
    /// Total authentication attempts
    pub total_attempts: u64,
    /// Successful authentications
    pub success_count: u64,
    /// Failed authentications
    pub failure_count: u64,
    /// Authentication success rate (percentage)
    pub success_rate: f64,
    /// Authentication failure rate (percentage)
    pub failure_rate: f64,
    /// Last update timestamp
    pub last_updated: u64,
}

impl VendorAuthManager {
    /// Create a new vendor authentication manager
    pub fn new() -> Self {
        Self {
            auth_events: HashMap::new(),
            auth_metrics: HashMap::new(),
        }
    }

    /// Record an authentication event
    pub fn record_auth_event(&mut self, event: AuthEvent) {
        // Log the authentication event
        let log_event_type = match &event.result {
            AuthResult::Success => AuthEventType::LoginSuccess,
            AuthResult::Failure(reason) => AuthEventType::LoginFailure(reason.clone().into()),
        };

        let log = create_vendor_auth_log(
            event.vendor_id.clone(),
            log_event_type,
            event.ip_address.clone(),
            event.user_agent.clone(),
            event.session_id.clone(),
            event.token_id.clone(),
            None,
        );
        
        log_vendor_auth_event(log);

        // Store the event
        self.auth_events
            .entry(event.vendor_id.clone())
            .or_insert_with(Vec::new)
            .push(event.clone());

        // Update metrics
        self.update_auth_metrics(&event.vendor_id);
    }

    /// Record a login attempt
    pub fn record_login_attempt(
        &mut self,
        vendor_id: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
        session_id: Option<String>,
        token_id: Option<String>,
    ) {
        // Log the login attempt
        let log = create_vendor_auth_log(
            vendor_id.clone(),
            AuthEventType::LoginAttempt,
            ip_address.clone(),
            user_agent.clone(),
            session_id.clone(),
            token_id.clone(),
            None,
        );
        
        log_vendor_auth_event(log);

        // Store the event
        let event = AuthEvent {
            id: format!("attempt_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()),
            vendor_id: vendor_id.clone(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            result: AuthResult::Failure(AuthFailureReason::Other("Attempt recorded".to_string())),
            ip_address,
            user_agent,
            session_id,
            token_id,
        };

        self.auth_events
            .entry(vendor_id)
            .or_insert_with(Vec::new)
            .push(event);

        // Note: We don't update metrics for login attempts as they're not completed authentications
    }

    /// Record a successful login
    pub fn record_login_success(
        &mut self,
        vendor_id: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
        session_id: Option<String>,
        token_id: Option<String>,
    ) {
        // Log the successful login
        let log = create_vendor_auth_log(
            vendor_id.clone(),
            AuthEventType::LoginSuccess,
            ip_address.clone(),
            user_agent.clone(),
            session_id.clone(),
            token_id.clone(),
            None,
        );
        
        log_vendor_auth_event(log);

        // Store the event
        let event = AuthEvent {
            id: format!("success_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()),
            vendor_id: vendor_id.clone(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            result: AuthResult::Success,
            ip_address,
            user_agent,
            session_id,
            token_id,
        };

        self.auth_events
            .entry(vendor_id.clone())
            .or_insert_with(Vec::new)
            .push(event);

        // Update metrics
        self.update_auth_metrics(&vendor_id);
    }

    /// Record a failed login
    pub fn record_login_failure(
        &mut self,
        vendor_id: String,
        reason: AuthFailureReason,
        ip_address: Option<String>,
        user_agent: Option<String>,
        session_id: Option<String>,
        token_id: Option<String>,
    ) {
        // Log the failed login
        let log = create_vendor_auth_log(
            vendor_id.clone(),
            AuthEventType::LoginFailure(reason.clone().into()),
            ip_address.clone(),
            user_agent.clone(),
            session_id.clone(),
            token_id.clone(),
            None,
        );
        
        log_vendor_auth_event(log);

        // Store the event
        let event = AuthEvent {
            id: format!("failure_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()),
            vendor_id: vendor_id.clone(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            result: AuthResult::Failure(reason),
            ip_address,
            user_agent,
            session_id,
            token_id,
        };

        self.auth_events
            .entry(vendor_id.clone())
            .or_insert_with(Vec::new)
            .push(event);

        // Update metrics
        self.update_auth_metrics(&vendor_id);
    }

    /// Record token issuance
    pub fn record_token_issuance(
        &self,
        vendor_id: String,
        token_id: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
        session_id: Option<String>,
    ) {
        // Log the token issuance
        let log = create_vendor_auth_log(
            vendor_id,
            AuthEventType::TokenIssued,
            ip_address,
            user_agent,
            session_id,
            Some(token_id),
            None,
        );
        
        log_vendor_auth_event(log);
    }

    /// Record session creation
    pub fn record_session_creation(
        &self,
        vendor_id: String,
        session_id: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
        token_id: Option<String>,
    ) {
        // Log the session creation
        let log = create_vendor_auth_log(
            vendor_id,
            AuthEventType::SessionCreated,
            ip_address,
            user_agent,
            Some(session_id),
            token_id,
            None,
        );
        
        log_vendor_auth_event(log);
    }

    /// Update authentication metrics for a vendor
    fn update_auth_metrics(&mut self, vendor_id: &str) {
        let events = match self.auth_events.get(vendor_id) {
            Some(events) => events,
            None => return,
        };

        // Only count actual authentication results (success/failure), not attempts
        let auth_results: Vec<_> = events
            .iter()
            .filter(|e| matches!(e.result, AuthResult::Success | AuthResult::Failure(_)))
            .collect();

        let total_attempts = auth_results.len() as u64;
        let success_count = auth_results
            .iter()
            .filter(|e| matches!(e.result, AuthResult::Success))
            .count() as u64;
        let failure_count = total_attempts - success_count;

        let success_rate = if total_attempts > 0 {
            (success_count as f64 / total_attempts as f64) * 100.0
        } else {
            0.0
        };

        let failure_rate = if total_attempts > 0 {
            (failure_count as f64 / total_attempts as f64) * 100.0
        } else {
            0.0
        };

        let metrics = AuthMetrics {
            vendor_id: vendor_id.to_string(),
            total_attempts,
            success_count,
            failure_count,
            success_rate,
            failure_rate,
            last_updated: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
        };

        self.auth_metrics.insert(vendor_id.to_string(), metrics);
    }

    /// Get authentication metrics for a vendor
    pub fn get_auth_metrics(&self, vendor_id: &str) -> Option<&AuthMetrics> {
        self.auth_metrics.get(vendor_id)
    }

    /// Get overall authentication success rate across all vendors
    pub fn get_overall_success_rate(&self) -> f64 {
        let total_vendors = self.auth_metrics.len();
        if total_vendors == 0 {
            return 0.0;
        }

        let sum: f64 = self.auth_metrics.values().map(|m| m.success_rate).sum();
        sum / total_vendors as f64
    }

    /// Get overall authentication failure rate across all vendors
    pub fn get_overall_failure_rate(&self) -> f64 {
        let total_vendors = self.auth_metrics.len();
        if total_vendors == 0 {
            return 0.0;
        }

        let sum: f64 = self.auth_metrics.values().map(|m| m.failure_rate).sum();
        sum / total_vendors as f64
    }

    /// Get authentication events for a vendor
    pub fn get_auth_events(&self, vendor_id: &str) -> Option<&Vec<AuthEvent>> {
        self.auth_events.get(vendor_id)
    }

    /// Detect unusual authentication patterns for a vendor
    pub fn detect_unusual_patterns(&self, vendor_id: &str) -> Vec<UnusualPattern> {
        let mut patterns = Vec::new();
        
        let events = match self.auth_events.get(vendor_id) {
            Some(events) => events,
            None => return patterns,
        };

        // Check for high failure rate (>50%)
        if let Some(metrics) = self.auth_metrics.get(vendor_id) {
            if metrics.failure_rate > 50.0 {
                patterns.push(UnusualPattern::HighFailureRate {
                    vendor_id: vendor_id.to_string(),
                    failure_rate: metrics.failure_rate,
                });
            }
        }

        // Check for multiple failed attempts from different IPs in short time
        let mut ip_attempts: HashMap<String, Vec<&AuthEvent>> = HashMap::new();
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        for event in events {
            if let AuthResult::Failure(_) = &event.result {
                if let Some(ip) = &event.ip_address {
                    ip_attempts.entry(ip.clone()).or_insert_with(Vec::new).push(event);
                }
            }
        }

        for (ip, attempts) in &ip_attempts {
            if attempts.len() > 5 {
                // Check if these attempts happened in a short time window (last hour)
                let recent_attempts: Vec<_> = attempts
                    .iter()
                    .filter(|e| current_time - e.timestamp < 3600) // Last hour
                    .collect();

                if recent_attempts.len() > 5 {
                    patterns.push(UnusualPattern::MultipleFailedAttemptsFromIP {
                        vendor_id: vendor_id.to_string(),
                        ip: ip.clone(),
                        count: recent_attempts.len() as u32,
                        time_window: 3600, // 1 hour
                    });
                }
            }
        }

        // Check for authentication attempts at unusual hours (outside business hours)
        let unusual_hour_attempts: Vec<_> = events
            .iter()
            .filter(|e| {
                // Convert timestamp to hour (UTC)
                let hour = (e.timestamp % 86400) / 3600;
                // Consider unusual hours as outside 7 AM to 7 PM
                hour < 7 || hour > 19
            })
            .collect();

        if unusual_hour_attempts.len() > 3 {
            patterns.push(UnusualPattern::UnusualAccessHours {
                vendor_id: vendor_id.to_string(),
                count: unusual_hour_attempts.len() as u32,
            });
        }

        patterns
    }
}

/// Unusual authentication pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnusualPattern {
    HighFailureRate {
        vendor_id: String,
        failure_rate: f64,
    },
    MultipleFailedAttemptsFromIP {
        vendor_id: String,
        ip: String,
        count: u32,
        time_window: u64, // seconds
    },
    UnusualAccessHours {
        vendor_id: String,
        count: u32,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_record_auth_event() {
        let mut manager = VendorAuthManager::new();
        
        let event = AuthEvent {
            id: "event1".to_string(),
            vendor_id: "vendor1".to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            result: AuthResult::Success,
            ip_address: Some("192.168.1.1".to_string()),
            user_agent: Some("TestAgent/1.0".to_string()),
            session_id: Some("session1".to_string()),
            token_id: Some("token1".to_string()),
        };
        
        manager.record_auth_event(event);
        
        assert!(manager.get_auth_events("vendor1").is_some());
        assert_eq!(manager.get_auth_events("vendor1").unwrap().len(), 1);
    }

    #[test]
    fn test_record_login_functions() {
        let mut manager = VendorAuthManager::new();
        
        // Record login attempt
        manager.record_login_attempt(
            "vendor1".to_string(),
            Some("192.168.1.1".to_string()),
            Some("TestAgent/1.0".to_string()),
            Some("session1".to_string()),
            Some("token1".to_string()),
        );
        
        // Record successful login
        manager.record_login_success(
            "vendor1".to_string(),
            Some("192.168.1.1".to_string()),
            Some("TestAgent/1.0".to_string()),
            Some("session2".to_string()),
            Some("token2".to_string()),
        );
        
        // Record failed login
        manager.record_login_failure(
            "vendor1".to_string(),
            AuthFailureReason::InvalidCredentials,
            Some("192.168.1.1".to_string()),
            Some("TestAgent/1.0".to_string()),
            Some("session3".to_string()),
            Some("token3".to_string()),
        );
        
        // Check that we have events
        assert!(manager.get_auth_events("vendor1").is_some());
        // Should have 2 events (success and failure, not the attempt)
        assert_eq!(manager.get_auth_events("vendor1").unwrap().len(), 2);
        
        // Check metrics
        let metrics = manager.get_auth_metrics("vendor1").unwrap();
        assert_eq!(metrics.total_attempts, 2);
        assert_eq!(metrics.success_count, 1);
        assert_eq!(metrics.failure_count, 1);
    }

    #[test]
    fn test_record_token_and_session() {
        let manager = VendorAuthManager::new();
        
        // These functions should not panic
        manager.record_token_issuance(
            "vendor1".to_string(),
            "token1".to_string(),
            Some("192.168.1.1".to_string()),
            Some("TestAgent/1.0".to_string()),
            Some("session1".to_string()),
        );
        
        manager.record_session_creation(
            "vendor1".to_string(),
            "session1".to_string(),
            Some("192.168.1.1".to_string()),
            Some("TestAgent/1.0".to_string()),
            Some("token1".to_string()),
        );
    }

    #[test]
    fn test_auth_metrics() {
        let mut manager = VendorAuthManager::new();
        
        // Record successful auth
        manager.record_login_success(
            "vendor1".to_string(),
            Some("192.168.1.1".to_string()),
            Some("TestAgent/1.0".to_string()),
            Some("session1".to_string()),
            Some("token1".to_string()),
        );
        
        // Record failed auth
        manager.record_login_failure(
            "vendor1".to_string(),
            AuthFailureReason::InvalidCredentials,
            Some("192.168.1.1".to_string()),
            Some("TestAgent/1.0".to_string()),
            Some("session2".to_string()),
            Some("token2".to_string()),
        );
        
        // Check metrics
        let metrics = manager.get_auth_metrics("vendor1").unwrap();
        assert_eq!(metrics.total_attempts, 2);
        assert_eq!(metrics.success_count, 1);
        assert_eq!(metrics.failure_count, 1);
        assert_eq!(metrics.success_rate, 50.0);
        assert_eq!(metrics.failure_rate, 50.0);
    }

    #[test]
    fn test_overall_rates() {
        let mut manager = VendorAuthManager::new();
        
        // Vendor 1: 50% success rate
        manager.record_login_success(
            "vendor1".to_string(),
            Some("192.168.1.1".to_string()),
            Some("TestAgent/1.0".to_string()),
            Some("session1".to_string()),
            Some("token1".to_string()),
        );
        
        manager.record_login_failure(
            "vendor1".to_string(),
            AuthFailureReason::InvalidCredentials,
            Some("192.168.1.1".to_string()),
            Some("TestAgent/1.0".to_string()),
            Some("session2".to_string()),
            Some("token2".to_string()),
        );
        
        // Vendor 2: 100% success rate
        manager.record_login_success(
            "vendor2".to_string(),
            Some("192.168.1.2".to_string()),
            Some("TestAgent/1.0".to_string()),
            Some("session3".to_string()),
            Some("token3".to_string()),
        );
        
        // Check overall rates
        let overall_success_rate = manager.get_overall_success_rate();
        let overall_failure_rate = manager.get_overall_failure_rate();
        
        // Vendor 1: 50% success rate
        // Vendor 2: 100% success rate
        // Overall: (50 + 100) / 2 = 75%
        assert_eq!(overall_success_rate, 75.0);
        assert_eq!(overall_failure_rate, 25.0);
    }

    #[test]
    fn test_unusual_patterns() {
        let mut manager = VendorAuthManager::new();
        
        // Record multiple failed attempts from the same IP
        let base_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        
        for i in 0..7 {
            manager.record_login_failure(
                "vendor1".to_string(),
                AuthFailureReason::InvalidCredentials,
                Some("192.168.1.100".to_string()),
                Some("TestAgent/1.0".to_string()),
                Some(format!("session{}", i)),
                Some(format!("token{}", i)),
            );
        }
        
        // Check for unusual patterns
        let patterns = manager.detect_unusual_patterns("vendor1");
        assert!(!patterns.is_empty());
        
        // Should detect multiple failed attempts from IP
        let has_ip_pattern = patterns.iter().any(|p| matches!(p, UnusualPattern::MultipleFailedAttemptsFromIP { .. }));
        assert!(has_ip_pattern);
    }
}