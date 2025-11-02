//! Supply Chain Security Module
//!
//! This module implements supply chain security measures including SBOM generation,
//! cryptographic signatures, provenance tracking, and dependency pinning.
//! It also includes artifact integrity features with Sigstore/cosign signing
//! and dependency trust features for SCA, pinning, and verification.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Represents a Software Bill of Materials (SBOM)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sbom {
    /// Unique identifier for the SBOM
    pub id: String,
    /// Name of the software component
    pub name: String,
    /// Version of the software component
    pub version: String,
    /// List of components in the SBOM
    pub components: Vec<Component>,
    /// Timestamp of SBOM creation
    pub created: u64,
    /// Hash of the SBOM content
    pub hash: String,
}

/// Represents a software component in an SBOM with additional trust information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    /// Unique identifier for the component
    pub id: String,
    /// Name of the component
    pub name: String,
    /// Version of the component
    pub version: String,
    /// Package URL (purl) of the component
    pub purl: String,
    /// Licenses associated with the component
    pub licenses: Vec<String>,
    /// Hash of the component
    pub hash: String,
    /// Whether this is a direct dependency
    pub is_direct: bool,
    /// Known vulnerabilities in the component
    pub vulnerabilities: Vec<Vulnerability>,
    /// Whether the component version is pinned
    pub is_pinned: bool,
    /// Whether the component checksum has been verified
    pub checksum_verified: bool,
    /// Whether the component is approved for use
    pub is_approved: bool,
}

/// Represents a vulnerability in a software component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    /// CVE or other identifier for the vulnerability
    pub id: String,
    /// Severity score (0.0 to 10.0)
    pub severity: f32,
    /// Description of the vulnerability
    pub description: String,
    /// Whether a fix is available
    pub fix_available: bool,
}

/// Represents source code information for provenance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfo {
    /// Repository URL
    pub repo_url: String,
    /// Commit hash
    pub commit_hash: String,
    /// Branch name
    pub branch: String,
    /// Tag name (if applicable)
    pub tag: Option<String>,
}

/// Represents build configuration information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Build script path
    pub build_script: String,
    /// Environment variables
    pub environment: HashMap<String, String>,
    /// Build tools and versions
    pub tools: HashMap<String, String>,
}

/// Represents a software artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    /// Name of the artifact
    pub name: String,
    /// URI of the artifact
    pub uri: String,
    /// Hash of the artifact
    pub hash: String,
    /// Size of the artifact in bytes
    pub size: u64,
    /// Digital signature (if signed)
    pub signature: Option<Signature>,
    /// SBOM attached to the artifact (if any)
    pub sbom: Option<Sbom>,
    /// Whether the artifact is signed
    pub is_signed: bool,
    /// Timestamp of artifact creation
    pub created: u64,
}

/// Represents a digital signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    /// Signature data
    pub data: String,
    /// Public key identifier
    pub key_id: String,
    /// Algorithm used for signing
    pub algorithm: String,
    /// Timestamp of signature creation
    pub timestamp: u64,
}

/// Represents build provenance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    /// Unique identifier for the provenance record
    pub id: String,
    /// Build identifier
    pub build_id: String,
    /// Source information
    pub source: SourceInfo,
    /// Build configuration
    pub build_config: BuildConfig,
    /// Artifacts produced by the build
    pub artifacts: Vec<Artifact>,
    /// Timestamp of provenance creation
    pub created: u64,
}

/// Represents a policy rule for supply chain governance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    /// Unique identifier for the rule
    pub id: String,
    /// Name of the rule
    pub name: String,
    /// Description of the rule
    pub description: String,
    /// Whether the rule is enabled
    pub enabled: bool,
    /// Severity level (low, medium, high, critical)
    pub severity: String,
    /// Category of the rule (security, compliance, etc.)
    pub category: String,
}

/// Represents a build record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Build {
    /// Unique identifier for the build
    pub id: String,
    /// Build number or version
    pub version: String,
    /// Status of the build (success, failed, in_progress)
    pub status: String,
    /// Timestamp of build start
    pub start_time: u64,
    /// Timestamp of build end
    pub end_time: Option<u64>,
    /// Commit hash associated with the build
    pub commit_hash: String,
    /// Branch associated with the build
    pub branch: String,
    /// Policy rules associated with this build
    pub policy_rules: Vec<PolicyRule>,
    /// Results of policy evaluations
    pub policy_results: HashMap<String, bool>,
    /// Whether all policies passed
    pub passed_policies: bool,
    /// Artifacts produced by the build
    pub artifacts: Vec<Artifact>,
}

/// Represents a security scan result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityScanResult {
    /// Unique identifier for the scan
    pub id: String,
    /// Build ID associated with this scan
    pub build_id: String,
    /// Type of scan performed
    pub scan_type: String,
    /// Artifact or component that was scanned
    pub target: String,
    /// Timestamp of the scan
    pub timestamp: u64,
    /// Vulnerabilities found
    pub vulnerabilities: Vec<Vulnerability>,
    /// Licenses found in the scanned components
    pub licenses: Vec<String>,
    /// Scan tool used
    pub tool: String,
    /// Scan result status
    pub status: String,
    /// Whether the scan passed all checks
    pub passed: bool,
    /// Timestamp when scan was completed
    pub completed: u64,
}

/// Represents a test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    /// Unique identifier for the test
    pub id: String,
    /// Build ID associated with this test
    pub build_id: String,
    /// Type of test performed
    pub test_type: String,
    /// Test name or description
    pub name: String,
    /// Test status (pass, fail, skip)
    pub status: String,
    /// Number of tests executed
    pub tests_executed: u64,
    /// Number of tests passed
    pub tests_passed: u64,
    /// Number of tests failed
    pub tests_failed: u64,
    /// Test coverage percentage
    pub coverage: f32,
    /// Timestamp of test execution
    pub timestamp: u64,
    /// Test duration in milliseconds
    pub duration: u64,
    /// Test output or log
    pub output: String,
    /// Whether the test passed
    pub passed: bool,
    /// Timestamp when test was completed
    pub completed: u64,
}

/// Represents an approved manifest for runtime drift control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovedManifest {
    /// Unique identifier for the manifest
    pub id: String,
    /// Name of the manifest
    pub name: String,
    /// Hash of the manifest content
    pub hash: String,
    /// Timestamp of manifest approval
    pub approved_at: u64,
    /// Approver information
    pub approved_by: String,
    /// Version of the manifest
    pub version: String,
    /// Expected processes that should be running
    pub expected_processes: Vec<String>,
    /// Expected file changes that are allowed
    pub expected_file_changes: Vec<String>,
    /// Expected network connections that are allowed
    pub expected_network_connections: Vec<String>,
    /// Expected environment variables
    pub expected_env_vars: HashMap<String, String>,
    /// Timestamp when manifest was created
    pub created: u64,
}

/// Represents a drift report for runtime drift control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftReport {
    /// Unique identifier for the report
    pub id: String,
    /// ID of the manifest this report is for
    pub manifest_id: String,
    /// Timestamp of the report
    pub timestamp: u64,
    /// Differences detected
    pub differences: Vec<String>,
    /// Severity of the drift
    pub severity: String,
    /// Actual processes running
    pub actual_processes: Vec<String>,
    /// Actual file changes
    pub actual_file_changes: Vec<String>,
    /// Actual network connections
    pub actual_network_connections: Vec<String>,
    /// Actual environment variables
    pub actual_env_vars: HashMap<String, String>,
    /// Process deviations detected
    pub process_deviations: Vec<String>,
    /// File deviations detected
    pub file_deviations: Vec<String>,
    /// Network deviations detected
    pub network_deviations: Vec<String>,
    /// Environment variable deviations detected
    pub env_var_deviations: Vec<String>,
    /// Whether this is a drift incident
    pub is_drift_incident: bool,
    /// Whether this is a sneaky container
    pub is_sneaky_container: bool,
    /// Timestamp when report was created
    pub created: u64,
}

/// Represents a running container for drift monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunningContainer {
    /// Container ID
    pub id: String,
    /// Container image
    pub image: String,
    /// Container name
    pub name: String,
    /// Timestamp when container started
    pub started_at: u64,
    /// Approved manifest for this container
    pub approved_manifest: Option<String>,
    /// Processes running in the container
    pub processes: Vec<String>,
    /// File changes in the container
    pub file_changes: Vec<String>,
    /// Network connections from the container
    pub network_connections: Vec<String>,
    /// Environment variables in the container
    pub env_vars: HashMap<String, String>,
    /// Timestamp when last checked
    pub last_checked: u64,
    /// ID of the manifest this container is associated with
    pub manifest_id: String,
}

/// Statistics for artifact integrity
#[derive(Debug, Clone)]
pub struct ArtifactIntegrityStats {
    /// Total number of artifacts processed
    pub total_artifacts: Arc<AtomicU64>,
    /// Number of unsigned artifacts blocked
    pub unsigned_blocked: Arc<AtomicU64>,
    /// Number of signed artifacts verified
    pub signed_verified: Arc<AtomicU64>,
    /// Number of signature verification failures
    pub signature_failures: Arc<AtomicU64>,
}

/// Statistics for dependency trust
#[derive(Debug, Clone)]
pub struct DependencyTrustStats {
    /// Total number of dependencies checked
    pub total_dependencies: Arc<AtomicU64>,
    /// Number of unapproved dependencies blocked
    pub unapproved_blocked: Arc<AtomicU64>,
    /// Number of typosquat packages detected
    pub typosquat_detected: Arc<AtomicU64>,
    /// Number of checksum mismatches
    pub checksum_failures: Arc<AtomicU64>,
}

/// Statistics for CI/CD gatekeeping
#[derive(Debug, Clone)]
pub struct CicdGatekeepingStats {
    /// Total number of builds processed
    pub total_builds: Arc<AtomicU64>,
    /// Number of builds blocked by policy
    pub builds_blocked: Arc<AtomicU64>,
    /// Number of policy violations detected
    pub policy_violations: Arc<AtomicU64>,
    /// Number of security scans performed
    pub security_scans: Arc<AtomicU64>,
    /// Number of tests executed
    pub tests_executed: Arc<AtomicU64>,
}

/// Statistics for runtime drift control
#[derive(Debug, Clone)]
pub struct RuntimeDriftStats {
    /// Total number of containers monitored
    pub total_containers: Arc<AtomicU64>,
    /// Number of drift incidents detected
    pub drift_incidents: Arc<AtomicU64>,
    /// Number of containers with approved manifests
    pub approved_containers: Arc<AtomicU64>,
    /// Number of containers with drift detected
    pub drifted_containers: Arc<AtomicU64>,
    /// Total number of drift checks performed
    pub total_drift_checks: Arc<AtomicU64>,
    /// Number of approved manifests
    pub approved_manifests: Arc<AtomicU64>,
    /// Number of sneaky containers detected
    pub sneaky_containers: Arc<AtomicU64>,
}

/// Error types for supply chain operations
#[derive(Debug)]
pub enum SupplyChainError {
    /// Error generating SBOM
    SbomGenerationFailed(String),
    /// Error storing SBOM
    SbomStorageFailed(String),
    /// Error creating signature
    SignatureCreationFailed(String),
    /// Error verifying signature
    SignatureVerificationFailed(String),
    /// Error creating provenance
    ProvenanceCreationFailed(String),
    /// Error storing provenance
    ProvenanceStorageFailed(String),
    /// Error scanning dependencies
    DependencyScanFailed(String),
    /// Error validating dependency trust
    DependencyTrustValidationFailed(String),
    /// Error validating dependency pinning
    DependencyPinningValidationFailed(String),
    /// Error with CI/CD policy validation
    CicdPolicyValidationFailed(String),
    /// Error with runtime drift detection
    RuntimeDriftDetectionFailed(String),
    /// Configuration error
    ConfigurationError(String),
}

impl fmt::Display for SupplyChainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SupplyChainError::SbomGenerationFailed(msg) => {
                write!(f, "SBOM generation failed: {}", msg)
            }
            SupplyChainError::SbomStorageFailed(msg) => {
                write!(f, "SBOM storage failed: {}", msg)
            }
            SupplyChainError::SignatureCreationFailed(msg) => {
                write!(f, "Signature creation failed: {}", msg)
            }
            SupplyChainError::SignatureVerificationFailed(msg) => {
                write!(f, "Signature verification failed: {}", msg)
            }
            SupplyChainError::ProvenanceCreationFailed(msg) => {
                write!(f, "Provenance creation failed: {}", msg)
            }
            SupplyChainError::ProvenanceStorageFailed(msg) => {
                write!(f, "Provenance storage failed: {}", msg)
            }
            SupplyChainError::DependencyScanFailed(msg) => {
                write!(f, "Dependency scan failed: {}", msg)
            }
            SupplyChainError::DependencyTrustValidationFailed(msg) => {
                write!(f, "Dependency trust validation failed: {}", msg)
            }
            SupplyChainError::DependencyPinningValidationFailed(msg) => {
                write!(f, "Dependency pinning validation failed: {}", msg)
            }
            SupplyChainError::CicdPolicyValidationFailed(msg) => {
                write!(f, "CI/CD policy validation failed: {}", msg)
            }
            SupplyChainError::RuntimeDriftDetectionFailed(msg) => {
                write!(f, "Runtime drift detection failed: {}", msg)
            }
            SupplyChainError::ConfigurationError(msg) => {
                write!(f, "Configuration error: {}", msg)
            }
        }
    }
}

impl std::error::Error for SupplyChainError {}

/// Manages supply chain security operations
pub struct SupplyChainManager {
    /// SBOM storage
    pub sboms: HashMap<String, Sbom>,
    /// Signature storage
    pub signatures: HashMap<String, Signature>,
    /// Provenance storage
    pub provenance: HashMap<String, Provenance>,
    /// Cosign signatures
    pub cosign_signatures: HashMap<String, String>,
    /// Artifact integrity statistics
    pub integrity_stats: ArtifactIntegrityStats,
    /// Dependency trust statistics
    pub trust_stats: DependencyTrustStats,
    /// CI/CD gatekeeping statistics
    pub cicd_stats: CicdGatekeepingStats,
    /// Runtime drift control statistics
    pub drift_stats: RuntimeDriftStats,
    /// Approved dependencies list
    pub approved_dependencies: HashMap<String, String>, // name -> version regex
    /// Known typosquat packages
    pub typosquat_packages: HashMap<String, String>, // suspicious name -> legitimate name
    /// Policy rules
    pub policy_rules: HashMap<String, PolicyRule>,
    /// Builds
    pub builds: HashMap<String, Build>,
    /// Security scan results
    pub security_scans: HashMap<String, SecurityScanResult>,
    /// Test results
    pub test_results: HashMap<String, TestResult>,
    /// Approved manifests for runtime drift control
    pub approved_manifests: HashMap<String, ApprovedManifest>,
    /// Drift reports
    pub drift_reports: HashMap<String, DriftReport>,
    /// Running containers for drift monitoring
    pub running_containers: HashMap<String, RunningContainer>,
}

impl SupplyChainManager {
    /// Create a new supply chain manager
    pub fn new() -> Self {
        let mut manager = Self {
            sboms: HashMap::new(),
            signatures: HashMap::new(),
            provenance: HashMap::new(),
            cosign_signatures: HashMap::new(),
            integrity_stats: ArtifactIntegrityStats {
                total_artifacts: Arc::new(AtomicU64::new(0)),
                unsigned_blocked: Arc::new(AtomicU64::new(0)),
                signed_verified: Arc::new(AtomicU64::new(0)),
                signature_failures: Arc::new(AtomicU64::new(0)),
            },
            trust_stats: DependencyTrustStats {
                total_dependencies: Arc::new(AtomicU64::new(0)),
                unapproved_blocked: Arc::new(AtomicU64::new(0)),
                typosquat_detected: Arc::new(AtomicU64::new(0)),
                checksum_failures: Arc::new(AtomicU64::new(0)),
            },
            cicd_stats: CicdGatekeepingStats {
                total_builds: Arc::new(AtomicU64::new(0)),
                builds_blocked: Arc::new(AtomicU64::new(0)),
                policy_violations: Arc::new(AtomicU64::new(0)),
                security_scans: Arc::new(AtomicU64::new(0)),
                tests_executed: Arc::new(AtomicU64::new(0)),
            },
            drift_stats: RuntimeDriftStats {
                total_containers: Arc::new(AtomicU64::new(0)),
                drift_incidents: Arc::new(AtomicU64::new(0)),
                approved_containers: Arc::new(AtomicU64::new(0)),
                drifted_containers: Arc::new(AtomicU64::new(0)),
                total_drift_checks: Arc::new(AtomicU64::new(0)),
                approved_manifests: Arc::new(AtomicU64::new(0)),
                sneaky_containers: Arc::new(AtomicU64::new(0)),
            },
            approved_dependencies: HashMap::new(),
            typosquat_packages: HashMap::new(),
            policy_rules: HashMap::new(),
            builds: HashMap::new(),
            security_scans: HashMap::new(),
            test_results: HashMap::new(),
            approved_manifests: HashMap::new(),
            drift_reports: HashMap::new(),
            running_containers: HashMap::new(),
        };

        // Initialize default approved dependencies
        manager.approved_dependencies.insert(
            "serde".to_string(),
            r"^1\.(0\.[0-9]+|[1-9][0-9]*\.[0-9]+)$".to_string(), // 1.x.x
        );
        manager.approved_dependencies.insert(
            "tokio".to_string(),
            r"^1\.(0\.[0-9]+|[1-9][0-9]*\.[0-9]+)$".to_string(), // 1.x.x
        );

        // Initialize known typosquat packages
        manager
            .typosquat_packages
            .insert("serede".to_string(), "serde".to_string());
        manager
            .typosquat_packages
            .insert("tokioo".to_string(), "tokio".to_string());

        // Initialize policy rules
        manager.initialize_policy_rules();

        manager
    }

    /// Get current timestamp
    fn current_timestamp(&self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }

    /// Initialize default policy rules
    fn initialize_policy_rules(&mut self) {
        // Artifact integrity policies
        self.policy_rules.insert(
            "artifact-signature-required".to_string(),
            PolicyRule {
                id: "artifact-signature-required".to_string(),
                name: "Artifact Signature Required".to_string(),
                description: "All artifacts must be signed".to_string(),
                enabled: true,
                severity: "high".to_string(),
                category: "artifact-integrity".to_string(),
            }
        );

        // Dependency trust policies
        self.policy_rules.insert(
            "dependency-trust-check".to_string(),
            PolicyRule {
                id: "dependency-trust-check".to_string(),
                name: "Dependency Trust Check".to_string(),
                description: "All dependencies must be trusted".to_string(),
                enabled: true,
                severity: "high".to_string(),
                category: "dependency-trust".to_string(),
            }
        );

        // CI/CD gatekeeping policies
        self.policy_rules.insert(
            "cicd-security-scan-required".to_string(),
            PolicyRule {
                id: "cicd-security-scan-required".to_string(),
                name: "CI/CD Security Scan Required".to_string(),
                description: "All builds must pass security scans".to_string(),
                enabled: true,
                severity: "critical".to_string(),
                category: "cicd-gatekeeping".to_string(),
            }
        );

        // Runtime drift control policies
        self.policy_rules.insert(
            "runtime-drift-detection".to_string(),
            PolicyRule {
                id: "runtime-drift-detection".to_string(),
                name: "Runtime Drift Detection".to_string(),
                description: "Detect and alert on runtime drift".to_string(),
                enabled: true,
                severity: "medium".to_string(),
                category: "runtime-drift".to_string(),
            }
        );

        // Build integrity policies
        self.policy_rules.insert(
            "reproducible-builds-required".to_string(),
            PolicyRule {
                id: "reproducible-builds-required".to_string(),
                name: "Reproducible Builds Required".to_string(),
                description: "All builds must be reproducible".to_string(),
                enabled: true,
                severity: "high".to_string(),
                category: "build-integrity".to_string(),
            }
        );

        // SBOM policies
        self.policy_rules.insert(
            "sbom-generation-required".to_string(),
            PolicyRule {
                id: "sbom-generation-required".to_string(),
                name: "SBOM Generation Required".to_string(),
                description: "All builds must generate SBOM".to_string(),
                enabled: true,
                severity: "medium".to_string(),
                category: "sbom".to_string(),
            }
        );
    }

    /// Generate an SBOM for a software component
    pub fn generate_sbom(&self, name: &str, version: &str) -> Result<Sbom, SupplyChainError> {
        // In a real implementation, this would scan the project dependencies
        // and generate a comprehensive SBOM. For this example, we'll create
        // a simple placeholder SBOM.

        let components = vec![
            Component {
                id: "component-1".to_string(),
                name: "serde".to_string(),
                version: "1.0.130".to_string(),
                purl: "pkg:cargo/serde@1.0.130".to_string(),
                licenses: vec!["MIT".to_string(), "Apache-2.0".to_string()],
                hash: "sha256:abcd1234...".to_string(),
                is_direct: true,
                vulnerabilities: vec![],
                is_pinned: true,
                checksum_verified: true,
                is_approved: true,
            },
            Component {
                id: "component-2".to_string(),
                name: "tokio".to_string(),
                version: "1.15.0".to_string(),
                purl: "pkg:cargo/tokio@1.15.0".to_string(),
                licenses: vec!["MIT".to_string()],
                hash: "sha256:efgh5678...".to_string(),
                is_direct: true,
                vulnerabilities: vec![],
                is_pinned: true,
                checksum_verified: true,
                is_approved: true,
            },
        ];

        let sbom = Sbom {
            id: format!("sbom-{}-{}", name, version),
            name: name.to_string(),
            version: version.to_string(),
            components,
            created: self.current_timestamp(),
            hash: "sha256:sbomhash123...".to_string(),
        };

        Ok(sbom)
    }

    /// Store an SBOM
    pub fn store_sbom(&mut self, sbom: Sbom) -> Result<(), SupplyChainError> {
        self.sboms.insert(sbom.id.clone(), sbom);
        Ok(())
    }

    /// Get an SBOM by ID
    pub fn get_sbom(&self, id: &str) -> Option<&Sbom> {
        self.sboms.get(id)
    }

    /// Create a digital signature for data
    pub fn create_signature(&self, data: &str) -> Result<Signature, SupplyChainError> {
        // In a real implementation, this would use actual cryptographic signing
        // For this example, we'll create a placeholder signature
        let signature = Signature {
            data: format!("signature-for-{}", data),
            key_id: "default-key-id".to_string(),
            algorithm: "SHA256-RSA".to_string(),
            timestamp: self.current_timestamp(),
        };

        Ok(signature)
    }

    /// Store a signature
    pub fn store_signature(&mut self, artifact_uri: &str, signature: Signature) -> Result<(), SupplyChainError> {
        self.signatures.insert(artifact_uri.to_string(), signature);
        Ok(())
    }

    /// Verify a signature
    pub fn verify_signature(&self, data: &str, signature: &Signature) -> Result<bool, SupplyChainError> {
        // In a real implementation, this would use actual cryptographic verification
        // For this example, we'll check if the signature matches our placeholder format
        let expected_signature = format!("signature-for-{}", data);
        Ok(signature.data == expected_signature)
    }

    /// Create build provenance
    pub fn create_provenance(
        &self,
        build_id: &str,
        source: SourceInfo,
        build_config: BuildConfig,
        artifacts: Vec<Artifact>,
    ) -> Result<Provenance, SupplyChainError> {
        let provenance = Provenance {
            id: format!("prov-{}", build_id),
            build_id: build_id.to_string(),
            source,
            build_config,
            artifacts,
            created: self.current_timestamp(),
        };

        Ok(provenance)
    }

    /// Store provenance
    pub fn store_provenance(&mut self, provenance: Provenance) -> Result<(), SupplyChainError> {
        self.provenance.insert(provenance.id.clone(), provenance);
        Ok(())
    }

    /// Get provenance by ID
    pub fn get_provenance(&self, id: &str) -> Option<&Provenance> {
        self.provenance.get(id)
    }

    /// Scan dependencies for vulnerabilities
    pub fn scan_dependencies(&self, sbom: &Sbom) -> Result<Vec<Vulnerability>, SupplyChainError> {
        // In a real implementation, this would use actual vulnerability databases
        // For this example, we'll return an empty list (no vulnerabilities found)
        let vulnerabilities = vec![];
        Ok(vulnerabilities)
    }

    /// Validate dependency trust
    pub fn validate_dependency_trust(&self, sbom: &Sbom) -> Result<(), SupplyChainError> {
        let mut unapproved_count = 0;
        let mut typosquat_count = 0;
        let mut checksum_mismatch_count = 0;

        for component in &sbom.components {
            // Check if component is approved
            if let Some(version_regex) = self.approved_dependencies.get(&component.name) {
                let regex = regex::Regex::new(version_regex).map_err(|e| {
                    SupplyChainError::DependencyTrustValidationFailed(format!(
                        "Invalid regex for {}: {}",
                        component.name, e
                    ))
                })?;
                
                if !regex.is_match(&component.version) {
                    unapproved_count += 1;
                }
            } else {
                unapproved_count += 1;
            }

            // Check for typosquat packages
            if self.typosquat_packages.contains_key(&component.name) {
                typosquat_count += 1;
            }

            // Check checksum verification
            if !component.checksum_verified {
                checksum_mismatch_count += 1;
            }
        }

        // Update statistics
        self.trust_stats
            .total_dependencies
            .fetch_add(sbom.components.len() as u64, Ordering::Relaxed);
        self.trust_stats
            .unapproved_blocked
            .fetch_add(unapproved_count, Ordering::Relaxed);
        self.trust_stats
            .typosquat_detected
            .fetch_add(typosquat_count, Ordering::Relaxed);
        self.trust_stats
            .checksum_failures
            .fetch_add(checksum_mismatch_count, Ordering::Relaxed);

        // Return error if any issues found
        if unapproved_count > 0 || typosquat_count > 0 || checksum_mismatch_count > 0 {
            let mut errors = vec![];
            if unapproved_count > 0 {
                errors.push(format!("{} unapproved dependencies", unapproved_count));
            }
            if typosquat_count > 0 {
                errors.push(format!("{} typosquat packages detected", typosquat_count));
            }
            if checksum_mismatch_count > 0 {
                errors.push(format!("{} checksum mismatches", checksum_mismatch_count));
            }
            
            return Err(SupplyChainError::DependencyTrustValidationFailed(
                format!("Dependency trust validation failed: {}", errors.join(", "))
            ));
        }

        Ok(())
    }

    /// Validate dependency pinning
    pub fn validate_dependency_pinning(&self, sbom: &Sbom) -> Result<(), SupplyChainError> {
        let unpinned_count = sbom
            .components
            .iter()
            .filter(|c| !c.is_pinned)
            .count() as u64;

        if unpinned_count > 0 {
            Err(SupplyChainError::DependencyPinningValidationFailed(
                format!("{} dependencies are not pinned", unpinned_count)
            ))
        } else {
            Ok(())
        }
    }

    /// Get trust statistics
    pub fn get_trust_stats(&self) -> (u64, u64, u64, u64) {
        (
            self.trust_stats.total_dependencies.load(Ordering::Relaxed),
            self.trust_stats.unapproved_blocked.load(Ordering::Relaxed),
            self.trust_stats.typosquat_detected.load(Ordering::Relaxed),
            self.trust_stats.checksum_failures.load(Ordering::Relaxed),
        )
    }

    /// Create a cosign signature
    pub fn create_cosign_signature(&mut self, artifact: &str, signature: &str) -> Result<(), SupplyChainError> {
        self.cosign_signatures.insert(artifact.to_string(), signature.to_string());
        Ok(())
    }

    /// Verify a cosign signature
    pub fn verify_cosign_signature(&self, artifact: &str) -> Result<bool, SupplyChainError> {
        // In a real implementation, this would verify the cosign signature
        // For this example, we'll just check if we have a signature stored
        Ok(self.cosign_signatures.contains_key(artifact))
    }

    /// Generate reproducible build hash
    pub fn generate_reproducible_build_hash(&self, build_config: &BuildConfig) -> Result<String, SupplyChainError> {
        // In a real implementation, this would generate a hash based on the build configuration
        // For this example, we'll create a placeholder hash
        Ok("sha256:reproducible-build-hash".to_string())
    }

    /// Verify reproducible build
    pub fn verify_reproducible_build(&self, build_config: &BuildConfig, expected_hash: &str) -> Result<bool, SupplyChainError> {
        let actual_hash = self.generate_reproducible_build_hash(build_config)?;
        Ok(actual_hash == expected_hash)
    }

    /// Attach SBOM to artifact
    pub fn attach_sbom_to_artifact(&mut self, artifact: &mut Artifact, sbom: Sbom) -> Result<(), SupplyChainError> {
        artifact.sbom = Some(sbom);
        Ok(())
    }

    /// Sign artifact with cosign
    pub fn sign_artifact_with_cosign(&mut self, artifact_uri: &str) -> Result<String, SupplyChainError> {
        // In a real implementation, this would use cosign to sign the artifact
        // For this example, we'll create a placeholder signature
        let signature = format!("cosign-signature-for-{}", artifact_uri);
        self.create_cosign_signature(artifact_uri, &signature)?;
        Ok(signature)
    }

    /// Sign artifact with cosign (with payload)
    pub fn sign_artifact_with_cosign_payload(&mut self, artifact_uri: &str, _payload: &str) -> Result<Signature, SupplyChainError> {
        // In a real implementation, this would use cosign to sign the artifact with payload
        // For this example, we'll create a placeholder signature
        let signature_data = format!("cosign-signature-for-{}", artifact_uri);
        let signature = Signature {
            data: signature_data,
            key_id: "cosign-key".to_string(),
            algorithm: "cosign".to_string(),
            timestamp: self.current_timestamp(),
        };
        self.create_cosign_signature(artifact_uri, &signature.data)?;
        Ok(signature)
    }

    /// Create an artifact
    pub fn create_artifact(
        &self,
        name: &str,
        uri: &str,
        hash: &str,
        size: u64,
        signature: Option<Signature>,
        sbom: Option<Sbom>,
    ) -> Result<Artifact, SupplyChainError> {
        let is_signed = signature.is_some();
        let artifact = Artifact {
            name: name.to_string(),
            uri: uri.to_string(),
            hash: hash.to_string(),
            size,
            signature,
            sbom,
            is_signed,
            created: self.current_timestamp(),
        };
        Ok(artifact)
    }

    /// Verify artifact integrity (Build Signing / Provenance)
    pub fn verify_artifact_integrity(&self, artifact: &Artifact) -> Result<bool, SupplyChainError> {
        // Update statistics
        self.integrity_stats
            .total_artifacts
            .fetch_add(1, Ordering::Relaxed);

        // Check if artifact is signed
        if !artifact.is_signed {
            self.integrity_stats
                .unsigned_blocked
                .fetch_add(1, Ordering::Relaxed);
            return Ok(false);
        }

        // Verify signature if present
        if let Some(signature) = &artifact.signature {
            if self.verify_signature(&artifact.uri, signature)? {
                self.integrity_stats
                    .signed_verified
                    .fetch_add(1, Ordering::Relaxed);
            } else {
                self.integrity_stats
                    .signature_failures
                    .fetch_add(1, Ordering::Relaxed);
                return Ok(false);
            }
        }

        // Verify cosign signature if present
        if self.cosign_signatures.contains_key(&artifact.uri) {
            if self.verify_cosign_signature(&artifact.uri)? {
                self.integrity_stats
                    .signed_verified
                    .fetch_add(1, Ordering::Relaxed);
            } else {
                self.integrity_stats
                    .signature_failures
                    .fetch_add(1, Ordering::Relaxed);
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Get artifact integrity statistics
    pub fn get_integrity_stats(&self) -> (u64, u64, u64, u64) {
        let total = self.integrity_stats.total_artifacts.load(Ordering::Relaxed);
        let unsigned = self.integrity_stats.unsigned_blocked.load(Ordering::Relaxed);
        let verified = self.integrity_stats.signed_verified.load(Ordering::Relaxed);
        let failures = self.integrity_stats.signature_failures.load(Ordering::Relaxed);
        (total, unsigned, verified, failures)
    }

    /// Add an approved dependency
    pub fn add_approved_dependency(&mut self, name: &str, version_regex: &str) {
        self.approved_dependencies.insert(name.to_string(), version_regex.to_string());
    }

    /// Remove an approved dependency
    pub fn remove_approved_dependency(&mut self, name: &str) {
        self.approved_dependencies.remove(name);
    }

    /// Add a typosquat package to detection list
    pub fn add_typosquat_package(&mut self, suspicious_name: &str, legitimate_name: &str) {
        self.typosquat_packages.insert(suspicious_name.to_string(), legitimate_name.to_string());
    }

    /// Create a build with CI/CD policy enforcement
    pub fn create_build(
        &mut self,
        build_number: &str,
        source: SourceInfo,
        _config: BuildConfig,
        artifacts: Vec<Artifact>,
    ) -> Result<Build, SupplyChainError> {
        // Update statistics
        self.cicd_stats
            .total_builds
            .fetch_add(1, Ordering::Relaxed);
            
        let build_id = format!("build-{}", build_number);
        
        // Get all policy rules
        let policy_rules: Vec<PolicyRule> = self.policy_rules.values().cloned().collect();
        
        // Create initial build with all policies marked as not evaluated
        let mut policy_results = HashMap::new();
        for rule in &policy_rules {
            policy_results.insert(rule.id.clone(), false);
        }
        
        let build = Build {
            id: build_id.clone(),
            version: build_number.to_string(),
            status: "pending".to_string(),
            start_time: self.current_timestamp(),
            end_time: None,
            commit_hash: source.commit_hash.clone(),
            branch: source.branch.clone(),
            policy_rules: self.policy_rules.values().cloned().collect(),
            policy_results: HashMap::new(),
            passed_policies: false,
            artifacts,
        };

        Ok(build)
    }

    /// Store a build
    pub fn store_build(&mut self, build: Build) -> Result<(), SupplyChainError> {
        self.builds.insert(build.id.clone(), build);
        Ok(())
    }

    /// Get a build by ID
    pub fn get_build(&self, id: &str) -> Option<&Build> {
        self.builds.get(id)
    }

    /// Perform security scan on a build
    pub fn perform_security_scan(
        &mut self,
        build_id: &str,
        scan_type: &str,
    ) -> Result<SecurityScanResult, SupplyChainError> {
        // Update statistics
        self.cicd_stats
            .security_scans
            .fetch_add(1, Ordering::Relaxed);
            
        let scan_id = format!("scan-{}-{}", build_id, self.current_timestamp());
        
        // In a real implementation, this would perform actual security scanning
        // For this example, we'll create a placeholder result
        let vulnerabilities = vec![];
        let licenses = vec!["MIT".to_string(), "Apache-2.0".to_string()];
        let passed = true; // For this example, we'll assume the scan passes
        
        let scan_result = SecurityScanResult {
            id: scan_id.clone(),
            build_id: build_id.to_string(),
            scan_type: scan_type.to_string(),
            target: "build".to_string(),
            timestamp: self.current_timestamp(),
            vulnerabilities,
            licenses,
            tool: "placeholder".to_string(),
            status: "completed".to_string(),
            completed: self.current_timestamp(),
            passed,
        };
        
        self.security_scans.insert(scan_id, scan_result.clone());
        Ok(scan_result)
    }

    /// Record test results for a build
    pub fn record_test_results(
        &mut self,
        build_id: &str,
        test_type: &str,
        tests_executed: u64,
        tests_passed: u64,
        tests_failed: u64,
        coverage: f32,
    ) -> Result<TestResult, SupplyChainError> {
        // Update statistics
        self.cicd_stats
            .tests_executed
            .fetch_add(1, Ordering::Relaxed);
            
        let test_id = format!("test-{}-{}", build_id, self.current_timestamp());
        
        let passed = tests_failed == 0;
        
        let test_result = TestResult {
            id: test_id.clone(),
            build_id: build_id.to_string(),
            test_type: test_type.to_string(),
            name: "test".to_string(),
            status: "completed".to_string(),
            tests_executed,
            tests_passed,
            tests_failed,
            coverage,
            timestamp: self.current_timestamp(),
            duration: 100,
            output: "test output".to_string(),
            completed: self.current_timestamp(),
            passed,
        };
        
        self.test_results.insert(test_id, test_result.clone());
        Ok(test_result)
    }

    /// Evaluate policy rules for a build
    pub fn evaluate_policies(&mut self, build_id: &str) -> Result<bool, SupplyChainError> {
        // Clone the necessary data to avoid borrowing issues
        let (policy_rules, enabled_rules): (Vec<PolicyRule>, Vec<bool>) = {
            let build = self.builds.get(build_id).ok_or_else(|| {
                SupplyChainError::ConfigurationError("Build not found".to_string())
            })?;
            let rules = build.policy_rules.clone();
            let enabled: Vec<bool> = rules.iter().map(|r| r.enabled).collect();
            (rules, enabled)
        };
        
        let mut policy_results = HashMap::new();
        let mut all_passed = true;
        
        // Evaluate each policy rule
        for (i, rule) in policy_rules.iter().enumerate() {
            if !enabled_rules[i] {
                // Skip disabled rules
                policy_results.insert(rule.id.clone(), true);
                continue;
            }
            
            let passed = match rule.category.as_str() {
                "security" => self.evaluate_security_policy(build_id, rule)?,
                "license" => self.evaluate_license_policy(build_id, rule)?,
                "quality" => self.evaluate_quality_policy(build_id, rule)?,
                "compliance" => self.evaluate_compliance_policy(build_id, rule)?,
                _ => {
                    // Unknown category, fail the policy
                    false
                }
            };
            
            policy_results.insert(rule.id.clone(), passed);
            
            if !passed && rule.severity == "critical" {
                // Critical policy failure blocks the build
                all_passed = false;
            }
        }
        
        // Update the build with results
        let build = self.builds.get_mut(build_id).ok_or_else(|| {
            SupplyChainError::ConfigurationError("Build not found".to_string())
        })?;
        build.policy_results = policy_results;
        build.passed_policies = all_passed;
        
        // Update statistics if build is blocked
        if !all_passed {
            self.cicd_stats
                .builds_blocked
                .fetch_add(1, Ordering::Relaxed);
        }
        
        Ok(all_passed)
    }

    /// Evaluate security policy rules
    fn evaluate_security_policy(&self, build_id: &str, rule: &PolicyRule) -> Result<bool, SupplyChainError> {
        match rule.id.as_str() {
            "security-scan-required" => {
                // Check if security scans were performed for this build
                for scan in self.security_scans.values() {
                    if scan.build_id == build_id {
                        return Ok(scan.passed);
                    }
                }
                // No security scan found
                Ok(false)
            }
            "no-critical-vulns" => {
                // Check if any critical vulnerabilities were found
                for scan in self.security_scans.values() {
                    if scan.build_id == build_id {
                        for vuln in &scan.vulnerabilities {
                            if vuln.severity >= 9.0 {
                                return Ok(false); // Critical vulnerability found
                            }
                        }
                    }
                }
                Ok(true) // No critical vulnerabilities found
            }
            _ => Ok(true) // Unknown security rule, pass by default
        }
    }

    /// Evaluate license policy rules
    fn evaluate_license_policy(&self, build_id: &str, rule: &PolicyRule) -> Result<bool, SupplyChainError> {
        match rule.id.as_str() {
            "approved-licenses-only" => {
                // Check if all licenses in security scans are approved
                for scan in self.security_scans.values() {
                    if scan.build_id == build_id {
                        for license in &scan.licenses {
                            // In a real implementation, we would check against approved licenses
                            // For this example, we'll assume MIT and Apache-2.0 are approved
                            if license != "MIT" && license != "Apache-2.0" {
                                return Ok(false); // Unapproved license found
                            }
                        }
                    }
                }
                Ok(true) // All licenses are approved
            }
            _ => Ok(true) // Unknown license rule, pass by default
        }
    }

    /// Evaluate quality policy rules
    fn evaluate_quality_policy(&self, build_id: &str, rule: &PolicyRule) -> Result<bool, SupplyChainError> {
        match rule.id.as_str() {
            "minimum-test-coverage" => {
                // Check if test coverage meets minimum requirements (e.g., 80%)
                for test in self.test_results.values() {
                    if test.build_id == build_id {
                        if test.coverage < 80.0 {
                            return Ok(false); // Coverage below minimum
                        }
                    }
                }
                Ok(true) // Coverage meets minimum requirements
            }
            "all-tests-must-pass" => {
                // Check if all tests passed
                for test in self.test_results.values() {
                    if test.build_id == build_id {
                        if !test.passed {
                            return Ok(false); // Some tests failed
                        }
                    }
                }
                Ok(true) // All tests passed
            }
            _ => Ok(true) // Unknown quality rule, pass by default
        }
    }

    /// Evaluate compliance policy rules
    fn evaluate_compliance_policy(&self, build_id: &str, rule: &PolicyRule) -> Result<bool, SupplyChainError> {
        match rule.id.as_str() {
            "dependency-trust-check" => {
                // Check if all artifacts in the build pass dependency trust validation
                let build = self.builds.get(build_id).ok_or_else(|| {
                    SupplyChainError::ConfigurationError("Build not found".to_string())
                })?;
                
                for artifact in &build.artifacts {
                    if let Some(sbom) = &artifact.sbom {
                        if self.validate_dependency_trust(sbom).is_err() {
                            return Ok(false); // Dependency trust validation failed
                        }
                    }
                }
                Ok(true) // All artifacts pass dependency trust validation
            }
            _ => Ok(true) // Unknown compliance rule, pass by default
        }
    }

    /// Get CI/CD gatekeeping statistics
    pub fn get_cicd_stats(&self) -> (u64, u64, u64, u64) {
        let total = self.cicd_stats.total_builds.load(Ordering::Relaxed);
        let blocked = self.cicd_stats.builds_blocked.load(Ordering::Relaxed);
        let scans = self.cicd_stats.security_scans.load(Ordering::Relaxed);
        let tests = self.cicd_stats.tests_executed.load(Ordering::Relaxed);
        (total, blocked, scans, tests)
    }

    /// Add a policy rule
    pub fn add_policy_rule(&mut self, rule: PolicyRule) {
        self.policy_rules.insert(rule.id.clone(), rule);
    }

    /// Remove a policy rule
    pub fn remove_policy_rule(&mut self, rule_id: &str) {
        self.policy_rules.remove(rule_id);
    }

    /// Enable a policy rule
    pub fn enable_policy_rule(&mut self, rule_id: &str) -> Result<(), SupplyChainError> {
        let rule = self.policy_rules.get_mut(rule_id).ok_or_else(|| {
            SupplyChainError::ConfigurationError("Policy rule not found".to_string())
        })?;
        rule.enabled = true;
        Ok(())
    }

    /// Disable a policy rule
    pub fn disable_policy_rule(&mut self, rule_id: &str) -> Result<(), SupplyChainError> {
        let rule = self.policy_rules.get_mut(rule_id).ok_or_else(|| {
            SupplyChainError::ConfigurationError("Policy rule not found".to_string())
        })?;
        rule.enabled = false;
        Ok(())
    }

    /// Create an approved manifest for runtime drift control
    pub fn create_approved_manifest(
        &mut self,
        name: &str,
        version: &str,
        expected_processes: Vec<String>,
        expected_file_changes: Vec<String>,
        expected_network_connections: Vec<String>,
        expected_env_vars: HashMap<String, String>,
    ) -> ApprovedManifest {
        let manifest = ApprovedManifest {
            id: format!("manifest-{}-{}", name, version),
            name: name.to_string(),
            hash: "sha256:manifesthash123...".to_string(), // In a real implementation, this would be a proper hash
            approved_at: self.current_timestamp(),
            approved_by: "system".to_string(),
            version: version.to_string(),
            expected_processes,
            expected_file_changes,
            expected_network_connections,
            expected_env_vars,
            created: self.current_timestamp(),
        };
        
        self.drift_stats.approved_manifests.fetch_add(1, Ordering::Relaxed);
        manifest
    }

    /// Store an approved manifest
    pub fn store_approved_manifest(&mut self, manifest: ApprovedManifest) -> Result<(), SupplyChainError> {
        self.approved_manifests.insert(manifest.id.clone(), manifest);
        Ok(())
    }

    /// Get an approved manifest by ID
    pub fn get_approved_manifest(&self, id: &str) -> Option<&ApprovedManifest> {
        self.approved_manifests.get(id)
    }

    /// Create a running container for drift monitoring
    pub fn create_running_container(
        &mut self,
        id: &str,
        name: &str,
        image: &str,
        processes: Vec<String>,
        file_changes: Vec<String>,
        network_connections: Vec<String>,
        env_vars: HashMap<String, String>,
        manifest_id: &str,
    ) -> RunningContainer {
        RunningContainer {
            id: id.to_string(),
            name: name.to_string(),
            image: image.to_string(),
            started_at: self.current_timestamp(),
            approved_manifest: Some(manifest_id.to_string()),
            processes,
            file_changes,
            network_connections,
            env_vars,
            last_checked: self.current_timestamp(),
            manifest_id: manifest_id.to_string(),
        }
    }

    /// Store a running container
    pub fn store_running_container(&mut self, container: RunningContainer) -> Result<(), SupplyChainError> {
        self.running_containers.insert(container.id.clone(), container);
        Ok(())
    }

    /// Get a running container by ID
    pub fn get_running_container(&self, id: &str) -> Option<&RunningContainer> {
        self.running_containers.get(id)
    }

    /// Check for drift between a running container and its approved manifest
    pub fn check_runtime_drift(&mut self, container_id: &str) -> Result<DriftReport, SupplyChainError> {
        // Update statistics
        self.drift_stats.total_drift_checks.fetch_add(1, Ordering::Relaxed);
        
        // Get the running container
        let container = self.running_containers.get(container_id).ok_or_else(|| {
            SupplyChainError::ConfigurationError("Container not found".to_string())
        })?;
        
        // Get the approved manifest
        let manifest = self.approved_manifests.get(&container.manifest_id).ok_or_else(|| {
            SupplyChainError::ConfigurationError("Approved manifest not found".to_string())
        })?;
        
        // Check for process deviations
        let mut process_deviations = Vec::new();
        for process in &container.processes {
            if !manifest.expected_processes.contains(process) {
                process_deviations.push(process.clone());
            }
        }
        
        // Check for file system deviations
        let mut file_deviations = Vec::new();
        for file_change in &container.file_changes {
            if !manifest.expected_file_changes.contains(file_change) {
                file_deviations.push(file_change.clone());
            }
        }
        
        // Check for network connection deviations
        let mut network_deviations = Vec::new();
        for connection in &container.network_connections {
            if !manifest.expected_network_connections.contains(connection) {
                network_deviations.push(connection.clone());
            }
        }
        
        // Check for environment variable deviations
        let mut env_var_deviations = Vec::new();
        for (key, value) in &container.env_vars {
            if let Some(expected_value) = manifest.expected_env_vars.get(key) {
                if expected_value != value {
                    env_var_deviations.push(format!("{}: expected '{}', got '{}'", key, expected_value, value));
                }
            } else {
                env_var_deviations.push(format!("{}: unexpected variable with value '{}'", key, value));
            }
        }
        
        // Check for missing expected environment variables
        for (key, expected_value) in &manifest.expected_env_vars {
            if !container.env_vars.contains_key(key) {
                env_var_deviations.push(format!("{}: missing expected variable with value '{}'", key, expected_value));
            }
        }
        
        // Determine if this is a drift incident or sneaky container
        let is_drift_incident = !process_deviations.is_empty() || 
                               !file_deviations.is_empty() || 
                               !network_deviations.is_empty() || 
                               !env_var_deviations.is_empty();
        
        let is_sneaky_container = process_deviations.len() > 2 || 
                                 file_deviations.len() > 5 || 
                                 network_deviations.len() > 3;
        
        // Update statistics if incidents detected
        if is_drift_incident {
            self.drift_stats.drift_incidents.fetch_add(1, Ordering::Relaxed);
        }
        
        if is_sneaky_container {
            self.drift_stats.sneaky_containers.fetch_add(1, Ordering::Relaxed);
        }
        
        let report = DriftReport {
            id: format!("drift-report-{}-{}", container_id, self.current_timestamp()),
            manifest_id: manifest.id.clone(),
            timestamp: self.current_timestamp(),
            differences: vec![],
            severity: "medium".to_string(),
            actual_processes: container.processes.clone(),
            actual_file_changes: container.file_changes.clone(),
            actual_network_connections: container.network_connections.clone(),
            actual_env_vars: container.env_vars.clone(),
            process_deviations,
            file_deviations,
            network_deviations,
            env_var_deviations,
            is_drift_incident,
            is_sneaky_container,
            created: self.current_timestamp(),
        };
        
        // Store the drift report
        self.drift_reports.insert(report.id.clone(), report.clone());
        
        Ok(report)
    }

    /// Get a drift report by ID
    pub fn get_drift_report(&self, id: &str) -> Option<&DriftReport> {
        self.drift_reports.get(id)
    }

    /// Get runtime drift statistics
    pub fn get_drift_stats(&self) -> (u64, u64, u64, u64) {
        let total_checks = self.drift_stats.total_drift_checks.load(Ordering::Relaxed);
        let incidents = self.drift_stats.drift_incidents.load(Ordering::Relaxed);
        let sneaky = self.drift_stats.sneaky_containers.load(Ordering::Relaxed);
        let manifests = self.drift_stats.approved_manifests.load(Ordering::Relaxed);
        (total_checks, incidents, sneaky, manifests)
    }

    /// Get drift reports for a specific manifest
    pub fn get_drift_reports_for_manifest(&self, manifest_id: &str) -> Vec<&DriftReport> {
        self.drift_reports
            .values()
            .filter(|report| report.manifest_id == manifest_id)
            .collect()
    }

    /// Get drift incidents per week for reporting
    pub fn get_drift_incidents_per_week(&self) -> u64 {
        // In a real implementation, this would calculate incidents per week
        // For now, we'll just return the total incidents
        self.drift_stats.drift_incidents.load(Ordering::Relaxed)
    }
}

impl Default for SupplyChainManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supply_chain_manager_creation() {
        let manager = SupplyChainManager::new();
        assert_eq!(manager.sboms.len(), 0);
        assert_eq!(manager.signatures.len(), 0);
        assert_eq!(manager.provenance.len(), 0);
        assert_eq!(manager.cosign_signatures.len(), 0);
    }

    #[test]
    fn test_sbom_generation() {
        let manager = SupplyChainManager::new();
        let sbom = manager.generate_sbom("test-component", "1.0.0").unwrap();
        assert_eq!(sbom.name, "test-component");
        assert_eq!(sbom.version, "1.0.0");
        assert_eq!(sbom.components.len(), 2);
    }

    #[test]
    fn test_sbom_storage_and_retrieval() {
        let mut manager = SupplyChainManager::new();
        let sbom = manager.generate_sbom("test-component", "1.0.0").unwrap();
        let sbom_id = sbom.id.clone();

        assert!(manager.store_sbom(sbom).is_ok());
        assert_eq!(manager.sboms.len(), 1);
        assert!(manager.get_sbom(&sbom_id).is_some());
    }

    #[test]
    fn test_signature_creation_and_verification() {
        let manager = SupplyChainManager::new();
        let artifact_hash = "sha256:test123";

        let signature = manager.create_signature(artifact_hash).unwrap();
        assert!(manager.verify_signature(artifact_hash, &signature).unwrap());

        // Test with wrong hash
        assert!(!manager
            .verify_signature("sha256:wrong", &signature)
            .unwrap());
    }

    #[test]
    fn test_signature_storage() {
        let mut manager = SupplyChainManager::new();
        let artifact_uri = "artifact://test";
        let signature = manager.create_signature("sha256:test123").unwrap();

        assert!(manager.store_signature(artifact_uri, signature).is_ok());
        assert_eq!(manager.signatures.len(), 1);
    }

    #[test]
    fn test_provenance_creation_storage_and_retrieval() {
        let mut manager = SupplyChainManager::new();

        let source = SourceInfo {
            repo_url: "https://github.com/example/repo".to_string(),
            commit_hash: "abc123".to_string(),
            branch: "main".to_string(),
            tag: Some("v1.0.0".to_string()),
        };

        let build_config = BuildConfig {
            build_script: "build.sh".to_string(),
            environment: HashMap::new(),
            tools: HashMap::new(),
        };

        let artifacts = vec![Artifact {
            name: "test-artifact".to_string(),
            uri: "artifact://test".to_string(),
            hash: "sha256:test123".to_string(),
            size: 1024,
            signature: None,
            sbom: None,
            is_signed: false,
            created: 0,
        }];

        let provenance = manager
            .create_provenance("build-123", source, build_config, artifacts)
            .unwrap();
        let provenance_id = provenance.id.clone();

        assert!(manager.store_provenance(provenance).is_ok());
        assert_eq!(manager.provenance.len(), 1);
        assert!(manager.get_provenance(&provenance_id).is_some());
    }

    #[test]
    fn test_dependency_scanning() {
        let manager = SupplyChainManager::new();
        let sbom = manager.generate_sbom("test-component", "1.0.0").unwrap();
        let vulnerabilities = manager.scan_dependencies(&sbom).unwrap();
        assert_eq!(vulnerabilities.len(), 0);
    }

    #[test]
    fn test_dependency_pinning_validation() {
        let manager = SupplyChainManager::new();
        let sbom = manager.generate_sbom("test-component", "1.0.0").unwrap();
        let result = manager.validate_dependency_pinning(&sbom);
        assert!(result.is_ok());
    }

    #[test]
    fn test_artifact_creation_with_integrity() {
        let manager = SupplyChainManager::new();
        let signature = manager.create_signature("sha256:test123").unwrap();
        let sbom = manager.generate_sbom("test-component", "1.0.0").unwrap();

        let artifact = manager
            .create_artifact(
                "test-artifact",
                "artifact://test",
                "sha256:test123",
                1024,
                Some(signature),
                Some(sbom),
            )
            .unwrap();

        assert_eq!(artifact.name, "test-artifact");
        assert_eq!(artifact.uri, "artifact://test");
        assert_eq!(artifact.hash, "sha256:test123");
        assert_eq!(artifact.size, 1024);
        assert!(artifact.signature.is_some());
        assert!(artifact.sbom.is_some());
        assert!(artifact.is_signed);
    }

    #[test]
    fn test_cosign_signing_and_verification() {
        let mut manager = SupplyChainManager::new();
        let artifact_uri = "artifact://test-container";

        let signature = manager
            .sign_artifact_with_cosign(artifact_uri)
            .unwrap();
        assert_eq!(signature, "cosign-signature-for-artifact://test-container");

        assert!(manager.cosign_signatures.len() == 1);
        assert!(manager
            .verify_cosign_signature(artifact_uri)
            .unwrap());
    }

    #[test]
    fn test_artifact_integrity_verification() {
        let manager = SupplyChainManager::new();

        // Create a signed artifact
        let signature = manager.create_signature("sha256:test123").unwrap();
        let sbom = manager.generate_sbom("test-component", "1.0.0").unwrap();

        let artifact = manager
            .create_artifact(
                "test-artifact",
                "artifact://test",
                "sha256:test123",
                1024,
                Some(signature),
                Some(sbom),
            )
            .unwrap();

        // Verify integrity of signed artifact
        assert!(manager.verify_artifact_integrity(&artifact).unwrap());

        // Create an unsigned artifact
        let unsigned_artifact = manager
            .create_artifact("unsigned-artifact", "artifact://unsigned", "sha256:unsigned", 512, None, None)
            .unwrap();

        // Verify integrity of unsigned artifact (should fail)
        assert!(!manager.verify_artifact_integrity(&unsigned_artifact).unwrap());

        // Check statistics
        let (total, unsigned, verified, failures) = manager.get_integrity_stats();
        assert_eq!(total, 2); // Two artifacts checked
        assert_eq!(unsigned, 1); // One unsigned artifact blocked
        assert_eq!(verified, 1); // One signed artifact verified
        assert_eq!(failures, 0); // No signature failures
    }

    #[test]
    fn test_sbom_attachment_to_artifact() {
        let mut manager = SupplyChainManager::new();
        let mut artifact = manager
            .create_artifact("test-artifact", "artifact://test", "sha256:test123", 1024, None, None)
            .unwrap();

        let sbom = manager.generate_sbom("attached-component", "2.0.0").unwrap();
        assert!(manager.attach_sbom_to_artifact(&mut artifact, sbom).is_ok());
        assert!(artifact.sbom.is_some());
        assert_eq!(artifact.sbom.as_ref().unwrap().name, "attached-component");
    }

    #[test]
    fn test_dependency_trust_validation() {
        let manager = SupplyChainManager::new();
        let sbom = manager.generate_sbom("test-component", "1.0.0").unwrap();
        
        // Test that the generated SBOM passes trust validation
        let result = manager.validate_dependency_trust(&sbom);
        assert!(result.is_ok());
    }

    #[test]
    fn test_dependency_trust_with_unapproved_dependency() {
        let mut manager = SupplyChainManager::new();
        
        // Create an SBOM with an unapproved dependency
        let unapproved_component = Component {
            id: "unapproved-component".to_string(),
            name: "malicious-dep".to_string(),
            version: "1.0.0".to_string(),
            purl: "pkg:cargo/malicious-dep@1.0.0".to_string(),
            licenses: vec!["MIT".to_string()],
            hash: "sha256:malicioushash".to_string(),
            is_direct: true,
            vulnerabilities: vec![],
            is_pinned: true,
            checksum_verified: true,
            is_approved: false, // Not approved
        };

        let sbom = Sbom {
            id: "test-sbom-unapproved".to_string(),
            name: "test-app".to_string(),
            version: "1.0.0".to_string(),
            components: vec![unapproved_component],
            created: 1234567890,
            hash: "sha256:sbomhash".to_string(),
        };

        // Validation should fail due to unapproved dependency
        let result = manager.validate_dependency_trust(&sbom);
        assert!(result.is_err());
        
        // Check that the error is the expected type
        match result {
            Err(SupplyChainError::DependencyTrustValidationFailed(msg)) => {
                assert!(msg.contains("Unapproved dependencies detected"));
            }
            _ => panic!("Expected DependencyTrustValidationFailed error"),
        }
        
        // Check statistics
        let (total, unapproved, typosquat, checksum) = manager.get_trust_stats();
        assert_eq!(total, 1); // One dependency checked
        assert_eq!(unapproved, 1); // One unapproved dependency blocked
        assert_eq!(typosquat, 0);
        assert_eq!(checksum, 0);
    }

    #[test]
    fn test_dependency_trust_with_typosquat_package() {
        let mut manager = SupplyChainManager::new();
        
        // Add a typosquat package to detection list
        manager.add_typosquat_package("serede", "serde");
        
        // Create an SBOM with a typosquat package
        let typosquat_component = Component {
            id: "typosquat-component".to_string(),
            name: "serede".to_string(), // Typosquat for "serde"
            version: "1.0.0".to_string(),
            purl: "pkg:cargo/serede@1.0.0".to_string(),
            licenses: vec!["MIT".to_string()],
            hash: "sha256:typosquathash".to_string(),
            is_direct: true,
            vulnerabilities: vec![],
            is_pinned: true,
            checksum_verified: true,
            is_approved: true,
        };

        let sbom = Sbom {
            id: "test-sbom-typosquat".to_string(),
            name: "test-app".to_string(),
            version: "1.0.0".to_string(),
            components: vec![typosquat_component],
            created: 1234567890,
            hash: "sha256:sbomhash".to_string(),
        };

        // Validation should fail due to typosquat package
        let result = manager.validate_dependency_trust(&sbom);
        assert!(result.is_err());
        
        // Check that the error is the expected type
        match result {
            Err(SupplyChainError::DependencyTrustValidationFailed(msg)) => {
                assert!(msg.contains("Typosquat packages detected"));
                assert!(msg.contains("serede"));
            }
            _ => panic!("Expected DependencyTrustValidationFailed error"),
        }
        
        // Check statistics
        let (total, unapproved, typosquat, checksum) = manager.get_trust_stats();
        assert_eq!(total, 1); // One dependency checked
        assert_eq!(unapproved, 0);
        assert_eq!(typosquat, 1); // One typosquat package detected
        assert_eq!(checksum, 0);
    }

    #[test]
    fn test_dependency_trust_with_unpinned_dependency() {
        let manager = SupplyChainManager::new();
        
        // Create an SBOM with an unpinned dependency
        let unpinned_component = Component {
            id: "unpinned-component".to_string(),
            name: "unpinned-dep".to_string(),
            version: "1.0.0".to_string(),
            purl: "pkg:cargo/unpinned-dep@1.0.0".to_string(),
            licenses: vec!["MIT".to_string()],
            hash: "sha256:unpinnedhash".to_string(),
            is_direct: true,
            vulnerabilities: vec![],
            is_pinned: false, // Not pinned
            checksum_verified: true,
            is_approved: true,
        };

        let sbom = Sbom {
            id: "test-sbom-unpinned".to_string(),
            name: "test-app".to_string(),
            version: "1.0.0".to_string(),
            components: vec![unpinned_component],
            created: 1234567890,
            hash: "sha256:sbomhash".to_string(),
        };

        // Validation should fail due to unpinned dependency
        let result = manager.validate_dependency_trust(&sbom);
        assert!(result.is_err());
        
        // Check that the error is the expected type
        match result {
            Err(SupplyChainError::DependencyTrustValidationFailed(msg)) => {
                assert!(msg.contains("Dependencies are not properly pinned"));
            }
            _ => panic!("Expected DependencyTrustValidationFailed error"),
        }
    }

    #[test]
    fn test_dependency_trust_with_checksum_failure() {
        let manager = SupplyChainManager::new();
        
        // Create an SBOM with a component that failed checksum verification
        let checksum_component = Component {
            id: "checksum-component".to_string(),
            name: "checksum-dep".to_string(),
            version: "1.0.0".to_string(),
            purl: "pkg:cargo/checksum-dep@1.0.0".to_string(),
            licenses: vec!["MIT".to_string()],
            hash: "sha256:checksumhash".to_string(),
            is_direct: true,
            vulnerabilities: vec![],
            is_pinned: true,
            checksum_verified: false, // Checksum verification failed
            is_approved: true,
        };

        let sbom = Sbom {
            id: "test-sbom-checksum".to_string(),
            name: "test-app".to_string(),
            version: "1.0.0".to_string(),
            components: vec![checksum_component],
            created: 1234567890,
            hash: "sha256:sbomhash".to_string(),
        };

        // Validation should fail due to checksum verification failure
        let result = manager.validate_dependency_trust(&sbom);
        assert!(result.is_err());
        
        // Check that the error is the expected type
        match result {
            Err(SupplyChainError::DependencyTrustValidationFailed(msg)) => {
                assert!(msg.contains("Component checksum verification failed"));
            }
            _ => panic!("Expected DependencyTrustValidationFailed error"),
        }
    }

    #[test]
    fn test_approved_dependency_management() {
        let mut manager = SupplyChainManager::new();
        
        // Add an approved dependency
        manager.add_approved_dependency("test-dep", r"^1\.[0-9]+\.[0-9]+$");
        assert!(manager.approved_dependencies.contains_key("test-dep"));
        
        // Remove an approved dependency
        manager.remove_approved_dependency("test-dep");
        assert!(!manager.approved_dependencies.contains_key("test-dep"));
    }

    #[test]
    fn test_typosquat_detection_management() {
        let mut manager = SupplyChainManager::new();
        
        // Add a typosquat package to detection list
        manager.add_typosquat_package("suspicious-name", "legitimate-name");
        assert!(manager.typosquat_packages.contains_key("suspicious-name"));
    }

    #[test]
    fn test_get_trust_stats() {
        let manager = SupplyChainManager::new();
        let (total, unapproved, typosquat, checksum) = manager.get_trust_stats();
        assert_eq!(total, 0);
        assert_eq!(unapproved, 0);
        assert_eq!(typosquat, 0);
        assert_eq!(checksum, 0);
    }

    #[test]
    fn test_cicd_gatekeeping_initialization() {
        let manager = SupplyChainManager::new();
        assert_eq!(manager.builds.len(), 0);
        assert_eq!(manager.security_scans.len(), 0);
        assert_eq!(manager.test_results.len(), 0);
        assert!(!manager.policy_rules.is_empty());
        
        // Check that default policies are loaded
        assert!(manager.policy_rules.contains_key("security-scan-required"));
        assert!(manager.policy_rules.contains_key("no-critical-vulns"));
        assert!(manager.policy_rules.contains_key("approved-licenses-only"));
        assert!(manager.policy_rules.contains_key("minimum-test-coverage"));
        assert!(manager.policy_rules.contains_key("all-tests-must-pass"));
        assert!(manager.policy_rules.contains_key("dependency-trust-check"));
    }

    #[test]
    fn test_build_creation_and_storage() {
        let mut manager = SupplyChainManager::new();
        
        let source = SourceInfo {
            repo_url: "https://github.com/example/test-app".to_string(),
            commit_hash: "a1b2c3d4e5f".to_string(),
            branch: "main".to_string(),
            tag: Some("v1.0.0".to_string()),
        };

        let build_config = BuildConfig {
            build_script: "build.sh".to_string(),
            environment: std::collections::HashMap::new(),
            tools: std::collections::HashMap::new(),
        };

        let artifacts = vec![Artifact {
            name: "test-app-binary".to_string(),
            uri: "artifact://test-app-v1.0.0".to_string(),
            hash: "sha256:test123".to_string(),
            size: 2048,
            signature: None,
            sbom: None,
            is_signed: false,
            created: 0,
        }];
        
        let build = manager.create_build("123", source, build_config, artifacts).unwrap();
        assert_eq!(build.version, "123");
        assert_eq!(build.artifacts.len(), 1);
        assert!(!build.passed_policies); // Should be false initially
        
        // Store the build
        assert!(manager.store_build(build.clone()).is_ok());
        assert_eq!(manager.builds.len(), 1);
        
        // Retrieve the build
        let retrieved_build = manager.get_build(&build.id).unwrap();
        assert_eq!(retrieved_build.version, "123");
    }

    #[test]
    fn test_security_scan() {
        let mut manager = SupplyChainManager::new();
        let scan_result = manager.perform_security_scan("build-123", "sast").unwrap();
        assert_eq!(scan_result.build_id, "build-123");
        assert_eq!(scan_result.scan_type, "sast");
        assert!(scan_result.passed);
        assert_eq!(manager.security_scans.len(), 1);
    }

    #[test]
    fn test_test_results_recording() {
        let mut manager = SupplyChainManager::new();
        let test_result = manager.record_test_results("build-123", "unit", 100, 95, 5, 95.0).unwrap();
        assert_eq!(test_result.build_id, "build-123");
        assert_eq!(test_result.test_type, "unit");
        assert_eq!(test_result.tests_executed, 100);
        assert_eq!(test_result.tests_passed, 95);
        assert_eq!(test_result.tests_failed, 5);
        assert_eq!(test_result.coverage, 95.0);
        assert!(!test_result.passed); // Because some tests failed
        assert_eq!(manager.test_results.len(), 1);
    }

    #[test]
    fn test_policy_rule_management() {
        let mut manager = SupplyChainManager::new();
        
        // Add a new policy rule
        let new_rule = PolicyRule {
            id: "custom-rule".to_string(),
            name: "Custom Rule".to_string(),
            description: "A custom policy rule".to_string(),
            enabled: true,
            severity: "medium".to_string(),
            category: "security".to_string(),
        };
        
        manager.add_policy_rule(new_rule);
        assert!(manager.policy_rules.contains_key("custom-rule"));
        
        // Disable the rule
        assert!(manager.disable_policy_rule("custom-rule").is_ok());
        assert!(!manager.policy_rules.get("custom-rule").unwrap().enabled);
        
        // Enable the rule
        assert!(manager.enable_policy_rule("custom-rule").is_ok());
        assert!(manager.policy_rules.get("custom-rule").unwrap().enabled);
        
        // Remove the rule
        manager.remove_policy_rule("custom-rule");
        assert!(!manager.policy_rules.contains_key("custom-rule"));
    }

    #[test]
    fn test_policy_evaluation() {
        let mut manager = SupplyChainManager::new();
        
        let source = SourceInfo {
            repo_url: "https://github.com/example/test-app".to_string(),
            commit_hash: "a1b2c3d4e5f".to_string(),
            branch: "main".to_string(),
            tag: Some("v1.0.0".to_string()),
        };

        let build_config = BuildConfig {
            build_script: "build.sh".to_string(),
            environment: std::collections::HashMap::new(),
            tools: std::collections::HashMap::new(),
        };

        let artifacts = vec![Artifact {
            name: "test-app-binary".to_string(),
            uri: "artifact://test-app-v1.0.0".to_string(),
            hash: "sha256:test123".to_string(),
            size: 2048,
            signature: None,
            sbom: None,
            is_signed: false,
            created: 0,
        }];
        
        let build = manager.create_build("123", source, build_config, artifacts).unwrap();
        manager.store_build(build.clone()).unwrap();
        
        // Perform a security scan
        manager.perform_security_scan(&build.id, "sast").unwrap();
        
        // Record test results
        manager.record_test_results(&build.id, "unit", 100, 100, 0, 95.0).unwrap();
        
        // Evaluate policies
        let result = manager.evaluate_policies(&build.id).unwrap();
        assert!(result); // Should pass with default policies
        
        // Check statistics
        let (total, blocked, scans, tests) = manager.get_cicd_stats();
        assert_eq!(total, 1); // One build processed
        assert_eq!(blocked, 0); // No builds blocked since all policies passed
        assert_eq!(scans, 1); // One security scan performed
        assert_eq!(tests, 1); // One test executed
    }
}