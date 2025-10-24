//! Core components for the decentralized application
//!
//! This crate contains shared components used across different modules of the dApp.

pub mod risk;
pub mod mev_mitigation;
pub mod aa_security;
pub mod tx_routing;
pub mod rate_limiting;
pub mod data_protection;
pub mod data_integrity;
pub mod rpc_resilience;
pub mod container_hardening;
pub mod supply_chain;
pub mod observability;
pub mod incident_response;
pub mod logging;

// Re-export key types
pub use risk::{CollateralFactor, FeeRouter, InsuranceFund};
pub use mev_mitigation::{PrivateRelayer, BatchAuctionEngine, Order, ExecutedTrade};
pub use aa_security::{SessionKeyManager, PaymasterSecurityManager, UserOperation, SessionKey, Paymaster};
pub use tx_routing::{TxRoutingManager, DeadlineHandler, Transaction, Permit, PrivateTxRelay, SubmissionResult, TxRoutingError};
pub use rate_limiting::{TokenBucket, RateLimiter, IdempotencyManager, JobGuard, WAFRules, Request, WAFError, JobError};
pub use data_protection::{FieldEncryption, PiiMap, DsrErasureManager, PiiField, PiiClassification, DsrRequest, DsrRequestType, DsrRequestStatus, EncryptionError};
pub use data_integrity::{ContentItem, PinningService, HashAnchor, ContentSafetyPolicy, DataIntegrityError, DataIntegrityManager};
pub use rpc_resilience::{RpcProvider, ProviderHealth, TlsConfig, FailoverPolicy, CircuitBreaker, CircuitBreakerState, RpcResilienceError, RpcResilienceManager};
pub use container_hardening::{AdmissionPolicy, SeccompProfile, AppArmorProfile, SecretsManagement, SecretBackend, ContainerHardeningError, ContainerHardeningManager, ContainerConfig, PolicyViolation, ViolationSeverity};
pub use supply_chain::{Sbom, Component, Vulnerability, Signature, Provenance, SupplyChainError, SupplyChainManager};
pub use observability::{OtelCollector, PrometheusRule, SiemRule, SiemSeverity, AdminAuditLog, ObservabilityError, ObservabilityManager};
pub use incident_response::{PauseKillSwitch, Backup, BackupType, BackupStatus, RestoreJob, RestoreStatus, CommunicationPlan, CommunicationChannel, EscalationStep, IncidentResponseError, IncidentResponseManager, IncidentAction};

/// Common error types used across the application
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Json(serde_json::Error),
    Network(reqwest::Error),
    Ethereum(ethers::providers::ProviderError),
    Custom(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Json(e) => write!(f, "JSON error: {}", e),
            Error::Network(e) => write!(f, "Network error: {}", e),
            Error::Ethereum(e) => write!(f, "Ethereum error: {}", e),
            Error::Custom(e) => write!(f, "Custom error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Json(e) => Some(e),
            Error::Network(e) => Some(e),
            Error::Ethereum(e) => Some(e),
            Error::Custom(_) => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Json(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Network(error)
    }
}

impl From<ethers::providers::ProviderError> for Error {
    fn from(error: ethers::providers::ProviderError) -> Self {
        Error::Ethereum(error)
    }
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, Error>;

/// Common types used across the application
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
}