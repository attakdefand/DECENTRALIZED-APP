//! Vendor authentication logging module
//!
//! This module provides specialized logging functionality for vendor authentication events.

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, warn, error};

/// Vendor authentication log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorAuthLog {
    /// Timestamp of the log entry
    pub timestamp: u64,
    /// Vendor identifier
    pub vendor_id: String,
    /// Authentication event type
    pub event_type: AuthEventType,
    /// IP address of the request
    pub ip_address: Option<String>,
    /// User agent of the request
    pub user_agent: Option<String>,
    /// Session ID if applicable
    pub session_id: Option<String>,
    /// Token ID if applicable
    pub token_id: Option<String>,
    /// Additional details
    pub details: Option<String>,
}

/// Authentication event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthEventType {
    LoginAttempt,
    LoginSuccess,
    LoginFailure(AuthFailureReason),
    Logout,
    TokenIssued,
    TokenRefreshed,
    TokenRevoked,
    SessionCreated,
    SessionExpired,
    SessionRevoked,
}

/// Authentication failure reasons
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthFailureReason {
    InvalidCredentials,
    ExpiredToken,
    RevokedToken,
    AccountLocked,
    RateLimited,
    Other(String),
}

/// Log a vendor authentication event
pub fn log_vendor_auth_event(log: VendorAuthLog) {
    match &log.event_type {
        AuthEventType::LoginAttempt => {
            info!(
                target: "vendor_auth",
                timestamp = log.timestamp,
                vendor_id = &log.vendor_id,
                event_type = "login_attempt",
                ip_address = &log.ip_address,
                user_agent = &log.user_agent,
                session_id = &log.session_id,
                token_id = &log.token_id,
                details = &log.details,
                "Vendor login attempt"
            );
        }
        AuthEventType::LoginSuccess => {
            info!(
                target: "vendor_auth",
                timestamp = log.timestamp,
                vendor_id = &log.vendor_id,
                event_type = "login_success",
                ip_address = &log.ip_address,
                user_agent = &log.user_agent,
                session_id = &log.session_id,
                token_id = &log.token_id,
                details = &log.details,
                "Vendor login successful"
            );
        }
        AuthEventType::LoginFailure(reason) => {
            warn!(
                target: "vendor_auth",
                timestamp = log.timestamp,
                vendor_id = &log.vendor_id,
                event_type = "login_failure",
                failure_reason = format!("{:?}", reason),
                ip_address = &log.ip_address,
                user_agent = &log.user_agent,
                session_id = &log.session_id,
                token_id = &log.token_id,
                details = &log.details,
                "Vendor login failed"
            );
        }
        AuthEventType::Logout => {
            info!(
                target: "vendor_auth",
                timestamp = log.timestamp,
                vendor_id = &log.vendor_id,
                event_type = "logout",
                ip_address = &log.ip_address,
                user_agent = &log.user_agent,
                session_id = &log.session_id,
                token_id = &log.token_id,
                details = &log.details,
                "Vendor logout"
            );
        }
        AuthEventType::TokenIssued => {
            info!(
                target: "vendor_auth",
                timestamp = log.timestamp,
                vendor_id = &log.vendor_id,
                event_type = "token_issued",
                ip_address = &log.ip_address,
                user_agent = &log.user_agent,
                session_id = &log.session_id,
                token_id = &log.token_id,
                details = &log.details,
                "Vendor token issued"
            );
        }
        AuthEventType::TokenRefreshed => {
            info!(
                target: "vendor_auth",
                timestamp = log.timestamp,
                vendor_id = &log.vendor_id,
                event_type = "token_refreshed",
                ip_address = &log.ip_address,
                user_agent = &log.user_agent,
                session_id = &log.session_id,
                token_id = &log.token_id,
                details = &log.details,
                "Vendor token refreshed"
            );
        }
        AuthEventType::TokenRevoked => {
            info!(
                target: "vendor_auth",
                timestamp = log.timestamp,
                vendor_id = &log.vendor_id,
                event_type = "token_revoked",
                ip_address = &log.ip_address,
                user_agent = &log.user_agent,
                session_id = &log.session_id,
                token_id = &log.token_id,
                details = &log.details,
                "Vendor token revoked"
            );
        }
        AuthEventType::SessionCreated => {
            info!(
                target: "vendor_auth",
                timestamp = log.timestamp,
                vendor_id = &log.vendor_id,
                event_type = "session_created",
                ip_address = &log.ip_address,
                user_agent = &log.user_agent,
                session_id = &log.session_id,
                token_id = &log.token_id,
                details = &log.details,
                "Vendor session created"
            );
        }
        AuthEventType::SessionExpired => {
            info!(
                target: "vendor_auth",
                timestamp = log.timestamp,
                vendor_id = &log.vendor_id,
                event_type = "session_expired",
                ip_address = &log.ip_address,
                user_agent = &log.user_agent,
                session_id = &log.session_id,
                token_id = &log.token_id,
                details = &log.details,
                "Vendor session expired"
            );
        }
        AuthEventType::SessionRevoked => {
            info!(
                target: "vendor_auth",
                timestamp = log.timestamp,
                vendor_id = &log.vendor_id,
                event_type = "session_revoked",
                ip_address = &log.ip_address,
                user_agent = &log.user_agent,
                session_id = &log.session_id,
                token_id = &log.token_id,
                details = &log.details,
                "Vendor session revoked"
            );
        }
    }
}

/// Create a vendor authentication log entry
pub fn create_vendor_auth_log(
    vendor_id: String,
    event_type: AuthEventType,
    ip_address: Option<String>,
    user_agent: Option<String>,
    session_id: Option<String>,
    token_id: Option<String>,
    details: Option<String>,
) -> VendorAuthLog {
    VendorAuthLog {
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs(),
        vendor_id,
        event_type,
        ip_address,
        user_agent,
        session_id,
        token_id,
        details,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vendor_auth_log() {
        let log = create_vendor_auth_log(
            "vendor1".to_string(),
            AuthEventType::LoginAttempt,
            Some("192.168.1.1".to_string()),
            Some("TestAgent/1.0".to_string()),
            Some("session1".to_string()),
            Some("token1".to_string()),
            Some("Test login".to_string()),
        );

        assert_eq!(log.vendor_id, "vendor1");
        assert!(matches!(log.event_type, AuthEventType::LoginAttempt));
        assert_eq!(log.ip_address, Some("192.168.1.1".to_string()));
        assert_eq!(log.user_agent, Some("TestAgent/1.0".to_string()));
        assert_eq!(log.session_id, Some("session1".to_string()));
        assert_eq!(log.token_id, Some("token1".to_string()));
        assert_eq!(log.details, Some("Test login".to_string()));
    }

    #[test]
    fn test_log_vendor_auth_event() {
        let log = create_vendor_auth_log(
            "vendor1".to_string(),
            AuthEventType::LoginSuccess,
            Some("192.168.1.1".to_string()),
            Some("TestAgent/1.0".to_string()),
            Some("session1".to_string()),
            Some("token1".to_string()),
            Some("Test successful login".to_string()),
        );

        // This test just ensures the function doesn't panic
        log_vendor_auth_event(log);
    }
}