//! Security Layers Implementation
//!
//! This crate implements all 9 security layers from the Web3 protection layers matrix.

pub mod governance_policy;
pub mod identity_access;
pub mod application_security;
pub mod api_gateway;
pub mod data_security;
pub mod network_infra;
pub mod resilience;
pub mod observability;
pub mod supply_chain;

// Re-export key types
// For now, we're only re-exporting from the modules we've fully implemented
pub use governance_policy::{
    AuditIssue, ExceptionRegister, PolicyCatalog, RiskAcceptanceWorkflow,
};
pub use identity_access::{
    AuthNManager, AuthZManager, JwtToken, MfaChallenge, PasswordHash, RbacPolicy, SecretManager,
    SessionManager, TokenLifecycle,
};

/// Common error types used across security layers
#[derive(Debug)]
pub enum SecurityError {
    Io(std::io::Error),
    Json(serde_json::Error),
    Network(reqwest::Error),
    Ethereum(ethers::providers::ProviderError),
    Custom(String),
}

impl std::fmt::Display for SecurityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityError::Io(e) => write!(f, "IO error: {}", e),
            SecurityError::Json(e) => write!(f, "JSON error: {}", e),
            SecurityError::Network(e) => write!(f, "Network error: {}", e),
            SecurityError::Ethereum(e) => write!(f, "Ethereum error: {}", e),
            SecurityError::Custom(e) => write!(f, "Custom error: {}", e),
        }
    }
}

impl std::error::Error for SecurityError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SecurityError::Io(e) => Some(e),
            SecurityError::Json(e) => Some(e),
            SecurityError::Network(e) => Some(e),
            SecurityError::Ethereum(e) => Some(e),
            SecurityError::Custom(_) => None,
        }
    }
}

impl From<std::io::Error> for SecurityError {
    fn from(error: std::io::Error) -> Self {
        SecurityError::Io(error)
    }
}

impl From<serde_json::Error> for SecurityError {
    fn from(error: serde_json::Error) -> Self {
        SecurityError::Json(error)
    }
}

impl From<reqwest::Error> for SecurityError {
    fn from(error: reqwest::Error) -> Self {
        SecurityError::Network(error)
    }
}

impl From<ethers::providers::ProviderError> for SecurityError {
    fn from(error: ethers::providers::ProviderError) -> Self {
        SecurityError::Ethereum(error)
    }
}

/// Result type alias for convenience
pub type SecurityResult<T> = std::result::Result<T, SecurityError>;

/// Common types used across security layers
pub mod types {
    use serde::{Deserialize, Serialize};

    /// Represents a blockchain address
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub struct Address(pub String);

    /// Represents a token amount with decimals
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TokenAmount {
        pub value: u128,
        pub decimals: u8,
    }

    /// Represents a security layer
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SecurityLayer {
        pub layer_number: u8,
        pub layer_name: String,
        pub main_type: String,
        pub sub_type: String,
        pub component_mechanism: String,
        pub goal: String,
        pub evidence_telemetry: String,
    }
}