//! Core components for the decentralized application
//!
//! This crate contains shared components used across different modules of the dApp.

pub mod aa_security;
pub mod container_hardening;
pub mod data_integrity;
pub mod data_protection;
pub mod incident_response;
pub mod logging;
pub mod mev_mitigation;
pub mod observability;
pub mod rate_limiting;
pub mod resilience_availability;
pub mod risk;
pub mod rpc_resilience;
pub mod supply_chain;
pub mod tx_routing;

// Re-export key types
pub use aa_security::{
    Paymaster, PaymasterSecurityManager, SessionKey, SessionKeyManager, UserOperation,
};
pub use container_hardening::{
    AdmissionPolicy, AppArmorProfile, ContainerConfig, ContainerHardeningError,
    ContainerHardeningManager, PolicyViolation, SeccompProfile, SecretBackend, SecretsManagement,
    ViolationSeverity,
};
pub use data_integrity::{
    ContentItem, ContentSafetyPolicy, DataIntegrityError, DataIntegrityManager, HashAnchor,
    PinningService,
};
pub use data_protection::{
    DsrErasureManager, DsrRequest, DsrRequestStatus, DsrRequestType, EncryptionError,
    FieldEncryption, PiiClassification, PiiField, PiiMap,
};
pub use incident_response::{
    Backup, BackupStatus, BackupType, CommunicationChannel, CommunicationPlan, EscalationStep,
    IncidentAction, IncidentResponseError, IncidentResponseManager, PauseKillSwitch, RestoreJob,
    RestoreStatus,
};
pub use mev_mitigation::{BatchAuctionEngine, ExecutedTrade, Order, PrivateRelayer};
pub use observability::{
    AdminAuditLog, ObservabilityError, ObservabilityManager, OtelCollector, PrometheusRule,
    SiemRule, SiemSeverity,
};
pub use rate_limiting::{
    IdempotencyManager, JobError, JobGuard, RateLimiter, Request, TokenBucket, WAFError, WAFRules,
};
pub use resilience_availability::{
    Bulkhead, CircuitBreakerState as ResilienceCircuitBreakerState, DisasterRecoveryConfig,
    GracefulDegradationConfig, HaFailoverConfig, RateShapingConfig, ResilienceAvailabilityError,
    ResilienceAvailabilityManager, ServiceHealth, ServiceInstance, TrafficProtectionConfig,
};
pub use risk::{CollateralFactor, FeeRouter, InsuranceFund};
pub use rpc_resilience::{
    CircuitBreaker, CircuitBreakerState, FailoverPolicy, ProviderHealth, RpcProvider,
    RpcResilienceError, RpcResilienceManager, TlsConfig,
};
pub use supply_chain::{
    Component, Provenance, Sbom, Signature, SupplyChainError, SupplyChainManager, Vulnerability,
};
pub use tx_routing::{
    DeadlineHandler, Permit, PrivateTxRelay, SubmissionResult, Transaction, TxRoutingError,
    TxRoutingManager,
};

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
