//! Application Security Module
//!
//! This module implements security layers 3: Application Security
//!
//! Line 10 from web3_protection_layers.csv:
//! 3,Application Security,Input Protection,Validation & Sanitization,"Strict type validation, regex allowlists, length limits, unicode normalization","Block injection, XSS, deserialization attacks","Rejected request counts by rule"
//! Line 11 from web3_protection_layers.csv:
//! 3,Application Security,Output Protection,Encoding/Escaping,"HTML encode, JSON encode, header encode","Stop stored/reflective XSS","CSP violation reports, browser security reports"

use std::collections::HashMap;
use regex::Regex;

/// Input validation and sanitization for application security
pub struct InputProtection {
    /// Regex patterns for validation
    validation_patterns: HashMap<String, Regex>,
    /// Maximum allowed lengths for different input types
    max_lengths: HashMap<String, usize>,
}

impl InputProtection {
    /// Create a new InputProtection instance
    pub fn new() -> Self {
        let mut validation_patterns = HashMap::new();
        
        // Email validation pattern
        validation_patterns.insert(
            "email".to_string(),
            Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
        );
        
        // Username validation pattern (alphanumeric and underscore, 3-20 chars)
        validation_patterns.insert(
            "username".to_string(),
            Regex::new(r"^[a-zA-Z0-9_]{3,20}$").unwrap()
        );
        
        // Phone number validation pattern
        validation_patterns.insert(
            "phone".to_string(),
            Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap()
        );
        
        let mut max_lengths = HashMap::new();
        max_lengths.insert("username".to_string(), 20);
        max_lengths.insert("email".to_string(), 254);
        max_lengths.insert("password".to_string(), 128);
        max_lengths.insert("comment".to_string(), 1000);
        max_lengths.insert("title".to_string(), 100);
        
        Self {
            validation_patterns,
            max_lengths,
        }
    }
    
    /// Validate input against a specific pattern
    pub fn validate_input(&self, input_type: &str, value: &str) -> Result<(), String> {
        // Check length limits
        if let Some(&max_length) = self.max_lengths.get(input_type) {
            if value.len() > max_length {
                return Err(format!("Input exceeds maximum length of {}", max_length));
            }
        }
        
        // Validate against regex patterns
        if let Some(pattern) = self.validation_patterns.get(input_type) {
            if !pattern.is_match(value) {
                return Err(format!("Input does not match {} pattern", input_type));
            }
        }
        
        // Unicode normalization check
        if !self.is_normalized_unicode(value) {
            return Err("Input contains non-normalized unicode".to_string());
        }
        
        Ok(())
    }
    
    /// Sanitize input by removing dangerous characters
    pub fn sanitize_input(&self, input: &str) -> String {
        // Remove null bytes which can be used in injection attacks
        let sanitized = input.replace('\0', "");
        
        // Normalize unicode
        let normalized = self.normalize_unicode(&sanitized);
        
        normalized
    }
    
    /// Check if string is normalized unicode
    fn is_normalized_unicode(&self, input: &str) -> bool {
        // Simple check - in a real implementation, we would use a proper unicode normalization library
        // This is a basic check to demonstrate the concept
        !input.chars().any(|c| c as u32 > 0x10FFFF)
    }
    
    /// Normalize unicode string
    fn normalize_unicode(&self, input: &str) -> String {
        // In a real implementation, we would use a proper unicode normalization library
        // For now, we'll just return the input as-is to demonstrate the concept
        input.to_string()
    }
    
    /// Validate and sanitize input with strict type validation
    pub fn validate_and_sanitize(&self, input_type: &str, value: &str) -> Result<String, String> {
        // First validate
        self.validate_input(input_type, value)?;
        
        // Then sanitize
        let sanitized = self.sanitize_input(value);
        
        Ok(sanitized)
    }
    
    /// Add a custom validation pattern
    pub fn add_validation_pattern(&mut self, name: &str, pattern: &str) -> Result<(), String> {
        match Regex::new(pattern) {
            Ok(regex) => {
                self.validation_patterns.insert(name.to_string(), regex);
                Ok(())
            },
            Err(e) => Err(format!("Invalid regex pattern: {}", e))
        }
    }
    
    /// Add a length limit
    pub fn add_length_limit(&mut self, input_type: &str, max_length: usize) {
        self.max_lengths.insert(input_type.to_string(), max_length);
    }
}

impl Default for InputProtection {
    fn default() -> Self {
        Self::new()
    }
}

/// Rejection statistics for monitoring rejected requests
pub struct RejectionStats {
    /// Count of rejected requests by rule
    rejection_counts: HashMap<String, u64>,
}

impl RejectionStats {
    /// Create a new RejectionStats instance
    pub fn new() -> Self {
        Self {
            rejection_counts: HashMap::new(),
        }
    }
    
    /// Record a rejected request
    pub fn record_rejection(&mut self, rule: &str) {
        *self.rejection_counts.entry(rule.to_string()).or_insert(0) += 1;
    }
    
    /// Get rejection count for a specific rule
    pub fn get_rejection_count(&self, rule: &str) -> u64 {
        *self.rejection_counts.get(rule).unwrap_or(&0)
    }
    
    /// Get all rejection counts
    pub fn get_all_rejection_counts(&self) -> &HashMap<String, u64> {
        &self.rejection_counts
    }
}

impl Default for RejectionStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Output encoding and escaping for application security
pub struct OutputProtection {
    /// Context-specific encoders
    encoders: HashMap<String, Box<dyn Encoder>>,
}

impl OutputProtection {
    /// Create a new OutputProtection instance
    pub fn new() -> Self {
        let mut encoders: HashMap<String, Box<dyn Encoder>> = HashMap::new();
        
        // Register default encoders
        encoders.insert("html".to_string(), Box::new(HTMLEncoder::new()));
        encoders.insert("json".to_string(), Box::new(JSONEncoder::new()));
        encoders.insert("header".to_string(), Box::new(HeaderEncoder::new()));
        
        Self {
            encoders,
        }
    }
    
    /// Encode content for HTML context
    pub fn html_encode(&self, content: &str) -> String {
        if let Some(encoder) = self.encoders.get("html") {
            encoder.encode(content)
        } else {
            // Fallback encoding
            content.replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('"', "&quot;")
                .replace('\'', "&#x27;")
        }
    }
    
    /// Encode content for JSON context
    pub fn json_encode(&self, content: &str) -> String {
        if let Some(encoder) = self.encoders.get("json") {
            encoder.encode(content)
        } else {
            // Fallback encoding
            format!("\"{}\"", content
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\r', "\\r")
                .replace('\t', "\\t"))
        }
    }
    
    /// Encode content for HTTP header context
    pub fn header_encode(&self, content: &str) -> String {
        if let Some(encoder) = self.encoders.get("header") {
            encoder.encode(content)
        } else {
            // Fallback encoding - only allow printable ASCII characters
            content.chars()
                .filter(|c| c.is_ascii() && !c.is_control())
                .collect()
        }
    }
    
    /// Encode content for a specific context
    pub fn encode_for_context(&self, context: &str, content: &str) -> String {
        if let Some(encoder) = self.encoders.get(context) {
            encoder.encode(content)
        } else {
            // Return original content if no encoder found
            content.to_string()
        }
    }
    
    /// Add a custom encoder
    pub fn add_encoder(&mut self, context: &str, encoder: Box<dyn Encoder>) {
        self.encoders.insert(context.to_string(), encoder);
    }
}

impl Default for OutputProtection {
    fn default() -> Self {
        Self::new()
    }
}

/// Encoder trait for context-specific encoding
pub trait Encoder {
    /// Encode content for a specific context
    fn encode(&self, content: &str) -> String;
}

/// HTML encoder implementation
pub struct HTMLEncoder;

impl HTMLEncoder {
    /// Create a new HTMLEncoder instance
    pub fn new() -> Self {
        Self
    }
}

impl Encoder for HTMLEncoder {
    fn encode(&self, content: &str) -> String {
        content.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
    }
}

/// JSON encoder implementation
pub struct JSONEncoder;

impl JSONEncoder {
    /// Create a new JSONEncoder instance
    pub fn new() -> Self {
        Self
    }
}

impl Encoder for JSONEncoder {
    fn encode(&self, content: &str) -> String {
        format!("\"{}\"", content
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t"))
    }
}

/// HTTP header encoder implementation
pub struct HeaderEncoder;

impl HeaderEncoder {
    /// Create a new HeaderEncoder instance
    pub fn new() -> Self {
        Self
    }
}

impl Encoder for HeaderEncoder {
    fn encode(&self, content: &str) -> String {
        // Only allow printable ASCII characters in HTTP headers
        content.chars()
            .filter(|c| c.is_ascii() && !c.is_control())
            .collect()
    }
}

/// Business logic controls for rate and velocity limiting
pub struct BusinessLogicControls {
    /// Rate limiters for different operations
    rate_limiters: HashMap<String, RateLimiter>,
    /// User-specific counters for throttling
    user_counters: HashMap<String, UserCounter>,
    /// Lockout manager for abusive users
    lockout_manager: LockoutManager,
}

impl BusinessLogicControls {
    /// Create a new BusinessLogicControls instance
    pub fn new() -> Self {
        Self {
            rate_limiters: HashMap::new(),
            user_counters: HashMap::new(),
            lockout_manager: LockoutManager::new(),
        }
    }
    
    /// Configure OTP retry limits
    pub fn configure_otp_retry_limits(&mut self, max_attempts: u32, window_seconds: u64) {
        let limiter = RateLimiter::new(max_attempts, window_seconds);
        self.rate_limiters.insert("otp_retry".to_string(), limiter);
    }
    
    /// Configure withdrawal limits
    pub fn configure_withdrawal_limits(&mut self, max_amount: f64, window_seconds: u64) {
        let limiter = RateLimiter::new(max_amount as u32, window_seconds);
        self.rate_limiters.insert("withdrawal".to_string(), limiter);
    }
    
    /// Configure anti-bruteforce counters
    pub fn configure_bruteforce_protection(&mut self, max_attempts: u32, window_seconds: u64, lockout_duration: u64) {
        let limiter = RateLimiter::new(max_attempts, window_seconds);
        self.rate_limiters.insert("bruteforce".to_string(), limiter);
        self.lockout_manager.set_lockout_duration(lockout_duration);
    }
    
    /// Configure anti-spam throttles
    pub fn configure_spam_throttles(&mut self, max_requests: u32, window_seconds: u64) {
        let limiter = RateLimiter::new(max_requests, window_seconds);
        self.rate_limiters.insert("spam".to_string(), limiter);
    }
    
    /// Check if OTP retry is allowed for a user
    pub fn check_otp_retry_allowed(&mut self, user_id: &str) -> Result<(), String> {
        if let Some(limiter) = self.rate_limiters.get_mut("otp_retry") {
            if limiter.is_allowed(user_id) {
                Ok(())
            } else {
                // Record throttle hit
                self.record_throttle_hit(user_id, "otp_retry");
                Err("OTP retry limit exceeded".to_string())
            }
        } else {
            // No limit configured, allow by default
            Ok(())
        }
    }
    
    /// Check if withdrawal is allowed for a user
    pub fn check_withdrawal_allowed(&mut self, user_id: &str, amount: f64) -> Result<(), String> {
        if let Some(limiter) = self.rate_limiters.get_mut("withdrawal") {
            if limiter.is_allowed_with_value(user_id, amount as u32) {
                Ok(())
            } else {
                // Record throttle hit
                self.record_throttle_hit(user_id, "withdrawal");
                Err("Withdrawal limit exceeded".to_string())
            }
        } else {
            // No limit configured, allow by default
            Ok(())
        }
    }
    
    /// Check if login attempt is allowed (anti-bruteforce)
    pub fn check_login_allowed(&mut self, user_id: &str) -> Result<(), String> {
        // Check if user is locked out
        if self.lockout_manager.is_locked_out(user_id) {
            return Err("Account temporarily locked due to suspicious activity".to_string());
        }
        
        if let Some(limiter) = self.rate_limiters.get_mut("bruteforce") {
            if limiter.is_allowed(user_id) {
                Ok(())
            } else {
                // Record throttle hit and potentially lock out user
                self.record_throttle_hit(user_id, "bruteforce");
                self.lockout_manager.lockout_user(user_id);
                Err("Too many failed login attempts".to_string())
            }
        } else {
            // No limit configured, allow by default
            Ok(())
        }
    }
    
    /// Check if request is allowed (anti-spam)
    pub fn check_request_allowed(&mut self, user_id: &str) -> Result<(), String> {
        if let Some(limiter) = self.rate_limiters.get_mut("spam") {
            if limiter.is_allowed(user_id) {
                Ok(())
            } else {
                // Record throttle hit
                self.record_throttle_hit(user_id, "spam");
                Err("Request rate limit exceeded".to_string())
            }
        } else {
            // No limit configured, allow by default
            Ok(())
        }
    }
    
    /// Record a throttle hit for telemetry
    fn record_throttle_hit(&mut self, user_id: &str, rule: &str) {
        let key = format!("{}_{}", user_id, rule);
        let counter = self.user_counters.entry(key).or_insert_with(UserCounter::new);
        counter.increment();
    }
    
    /// Get throttle hit count for a user and rule
    pub fn get_throttle_hit_count(&self, user_id: &str, rule: &str) -> u64 {
        let key = format!("{}_{}", user_id, rule);
        if let Some(counter) = self.user_counters.get(&key) {
            counter.get_count()
        } else {
            0
        }
    }
    
    /// Get all throttle hit counts for a user
    pub fn get_user_throttle_hits(&self, user_id: &str) -> HashMap<String, u64> {
        let mut result = HashMap::new();
        for (key, counter) in &self.user_counters {
            if key.starts_with(user_id) {
                // Extract the rule name from the key (format: user_id_rule)
                if let Some(rule) = key.strip_prefix(&format!("{}_", user_id)) {
                    result.insert(rule.to_string(), counter.get_count());
                }
            }
        }
        result
    }
    
    /// Check if a user is locked out
    pub fn is_user_locked_out(&self, user_id: &str) -> bool {
        self.lockout_manager.is_locked_out(user_id)
    }
    
    /// Reset counters for a user (e.g., after successful authentication)
    pub fn reset_user_counters(&mut self, user_id: &str) {
        // Reset rate limiter counters
        for (_, limiter) in &mut self.rate_limiters {
            limiter.reset_user(user_id);
        }
        
        // Reset user counters
        self.user_counters.retain(|key, _| !key.starts_with(user_id));
        
        // Remove lockout if present
        self.lockout_manager.unlock_user(user_id);
    }
}

impl Default for BusinessLogicControls {
    fn default() -> Self {
        Self::new()
    }
}

/// Rate limiter for controlling request rates
pub struct RateLimiter {
    /// Maximum allowed attempts/values per window
    max_allowed: u32,
    /// Time window in seconds
    window_seconds: u64,
    /// User-specific counters
    user_counts: HashMap<String, WindowedCounter>,
}

impl RateLimiter {
    /// Create a new RateLimiter
    pub fn new(max_allowed: u32, window_seconds: u64) -> Self {
        Self {
            max_allowed,
            window_seconds,
            user_counts: HashMap::new(),
        }
    }
    
    /// Check if an action is allowed for a user
    pub fn is_allowed(&mut self, user_id: &str) -> bool {
        let counter = self.user_counts.entry(user_id.to_string()).or_insert_with(|| {
            WindowedCounter::new(self.window_seconds)
        });
        
        if counter.get_count() < self.max_allowed {
            counter.increment();
            true
        } else {
            false
        }
    }
    
    /// Check if an action with a value is allowed for a user
    pub fn is_allowed_with_value(&mut self, user_id: &str, value: u32) -> bool {
        let counter = self.user_counts.entry(user_id.to_string()).or_insert_with(|| {
            WindowedCounter::new(self.window_seconds)
        });
        
        if counter.get_count() + value <= self.max_allowed {
            counter.increment_by(value);
            true
        } else {
            false
        }
    }
    
    /// Reset counters for a user
    pub fn reset_user(&mut self, user_id: &str) {
        self.user_counts.remove(user_id);
    }
}

/// Windowed counter that tracks counts within a time window
pub struct WindowedCounter {
    /// Count of events
    count: u32,
    /// Timestamp of last reset
    last_reset: std::time::SystemTime,
    /// Window duration in seconds
    window_seconds: u64,
}

impl WindowedCounter {
    /// Create a new WindowedCounter
    pub fn new(window_seconds: u64) -> Self {
        Self {
            count: 0,
            last_reset: std::time::SystemTime::now(),
            window_seconds,
        }
    }
    
    /// Increment the counter
    pub fn increment(&mut self) {
        if self.should_reset() {
            self.reset();
        }
        self.count += 1;
    }
    
    /// Increment the counter by a specific value
    pub fn increment_by(&mut self, value: u32) {
        if self.should_reset() {
            self.reset();
        }
        self.count += value;
    }
    
    /// Get the current count
    pub fn get_count(&self) -> u32 {
        if self.should_reset() {
            0
        } else {
            self.count
        }
    }
    
    /// Check if the counter should be reset
    fn should_reset(&self) -> bool {
        if let Ok(elapsed) = self.last_reset.elapsed() {
            elapsed.as_secs() >= self.window_seconds
        } else {
            false
        }
    }
    
    /// Reset the counter
    fn reset(&mut self) {
        self.count = 0;
        self.last_reset = std::time::SystemTime::now();
    }
}

/// User counter for tracking throttle hits
pub struct UserCounter {
    /// Count of throttle hits
    count: u64,
}

impl UserCounter {
    /// Create a new UserCounter
    pub fn new() -> Self {
        Self { count: 0 }
    }
    
    /// Increment the counter
    pub fn increment(&mut self) {
        self.count += 1;
    }
    
    /// Get the current count
    pub fn get_count(&self) -> u64 {
        self.count
    }
}

/// Lockout manager for temporarily blocking abusive users
pub struct LockoutManager {
    /// Locked out users and their lockout expiration times
    locked_users: HashMap<String, std::time::SystemTime>,
    /// Lockout duration in seconds
    lockout_duration: u64,
}

impl LockoutManager {
    /// Create a new LockoutManager
    pub fn new() -> Self {
        Self {
            locked_users: HashMap::new(),
            lockout_duration: 300, // 5 minutes default
        }
    }
    
    /// Set the lockout duration
    pub fn set_lockout_duration(&mut self, duration_seconds: u64) {
        self.lockout_duration = duration_seconds;
    }
    
    /// Check if a user is locked out
    pub fn is_locked_out(&self, user_id: &str) -> bool {
        if let Some(lockout_time) = self.locked_users.get(user_id) {
            if let Ok(elapsed) = lockout_time.elapsed() {
                if elapsed.as_secs() < self.lockout_duration {
                    return true;
                }
            }
        }
        false
    }
    
    /// Lock out a user
    pub fn lockout_user(&mut self, user_id: &str) {
        self.locked_users.insert(user_id.to_string(), std::time::SystemTime::now());
    }
    
    /// Unlock a user
    pub fn unlock_user(&mut self, user_id: &str) {
        self.locked_users.remove(user_id);
    }
}

/// Dependency safety for SAST/SCA scanning
pub struct DependencySafety {
    /// Vulnerability database
    vulnerability_db: HashMap<String, Vulnerability>,
    /// License policy database
    license_policy: HashMap<String, LicensePolicy>,
    /// Scan results
    scan_results: HashMap<String, ScanResult>,
    /// SBOM (Software Bill of Materials)
    sbom: HashMap<String, Component>,
}

impl DependencySafety {
    /// Create a new DependencySafety instance
    pub fn new() -> Self {
        Self {
            vulnerability_db: HashMap::new(),
            license_policy: HashMap::new(),
            scan_results: HashMap::new(),
            sbom: HashMap::new(),
        }
    }
    
    /// Add a vulnerability to the database
    pub fn add_vulnerability(&mut self, vuln: Vulnerability) {
        self.vulnerability_db.insert(vuln.id.clone(), vuln);
    }
    
    /// Add a license policy
    pub fn add_license_policy(&mut self, policy: LicensePolicy) {
        self.license_policy.insert(policy.license_id.clone(), policy);
    }
    
    /// Add a component to the SBOM
    pub fn add_component(&mut self, component: Component) {
        self.sbom.insert(component.id.clone(), component);
    }
    
    /// Perform static code analysis
    pub fn perform_static_analysis(&self, code_path: &str) -> Vec<Issue> {
        // In a real implementation, this would integrate with actual SAST tools
        // For this example, we'll simulate some findings
        let mut issues = Vec::new();
        
        // Simulate finding some common security issues
        issues.push(Issue {
            id: "SAST-001".to_string(),
            severity: Severity::High,
            description: "Hardcoded credentials detected".to_string(),
            file: code_path.to_string(),
            line: 42,
        });
        
        issues.push(Issue {
            id: "SAST-002".to_string(),
            severity: Severity::Medium,
            description: "SQL injection vulnerability detected".to_string(),
            file: code_path.to_string(),
            line: 120,
        });
        
        issues
    }
    
    /// Scan dependencies for vulnerabilities
    pub fn scan_dependencies(&self, dependencies: &[Dependency]) -> Vec<VulnerabilityFinding> {
        let mut findings = Vec::new();
        
        for dependency in dependencies {
            // Check if this dependency has any known vulnerabilities
            for (_, vuln) in &self.vulnerability_db {
                if vuln.package_name == dependency.name && 
                   self.is_version_affected(&dependency.version, &vuln.affected_versions) {
                    findings.push(VulnerabilityFinding {
                        dependency: dependency.clone(),
                        vulnerability: vuln.clone(),
                        severity: vuln.severity.clone(),
                    });
                }
            }
        }
        
        findings
    }
    
    /// Check if a version is affected by a vulnerability
    fn is_version_affected(&self, version: &str, affected_versions: &[String]) -> bool {
        // In a real implementation, this would use proper version comparison logic
        // For this example, we'll do a simple check
        affected_versions.contains(&version.to_string())
    }
    
    /// Generate Software Bill of Materials (SBOM)
    pub fn generate_sbom(&self, project_name: &str) -> SBOM {
        SBOM {
            project_name: project_name.to_string(),
            components: self.sbom.values().cloned().collect(),
            generated_at: std::time::SystemTime::now(),
        }
    }
    
    /// Scan licenses for compliance
    pub fn scan_licenses(&self, dependencies: &[Dependency]) -> Vec<LicenseFinding> {
        let mut findings = Vec::new();
        
        for dependency in dependencies {
            // Check if this dependency's license is allowed
            if let Some(license_id) = &dependency.license {
                if let Some(policy) = self.license_policy.get(license_id) {
                    if !policy.allowed {
                        findings.push(LicenseFinding {
                            dependency: dependency.clone(),
                            license_id: license_id.clone(),
                            policy: policy.clone(),
                            violation: true,
                        });
                    }
                }
            }
        }
        
        findings
    }
    
    /// Get critical vulnerability count
    pub fn get_critical_vuln_count(&self, findings: &[VulnerabilityFinding]) -> usize {
        findings.iter()
            .filter(|finding| finding.severity == Severity::Critical)
            .count()
    }
    
    /// Get unresolved vulnerability age statistics
    pub fn get_unresolved_vuln_age(&self, findings: &[VulnerabilityFinding]) -> Vec<u64> {
        let mut ages = Vec::new();
        let now = std::time::SystemTime::now();
        
        for finding in findings {
            if let Ok(age) = now.duration_since(finding.vulnerability.published_date) {
                ages.push(age.as_secs() / 86400); // Convert to days
            }
        }
        
        ages
    }
    
    /// Record scan results for telemetry
    pub fn record_scan_results(&mut self, scan_id: &str, results: ScanResult) {
        self.scan_results.insert(scan_id.to_string(), results);
    }
    
    /// Get scan results
    pub fn get_scan_results(&self, scan_id: &str) -> Option<&ScanResult> {
        self.scan_results.get(scan_id)
    }
}

impl Default for DependencySafety {
    fn default() -> Self {
        Self::new()
    }
}

/// Vulnerability information
#[derive(Clone, Debug)]
pub struct Vulnerability {
    /// Vulnerability ID (e.g., CVE-2023-12345)
    pub id: String,
    /// Package name affected
    pub package_name: String,
    /// Affected versions
    pub affected_versions: Vec<String>,
    /// Severity level
    pub severity: Severity,
    /// Description of the vulnerability
    pub description: String,
    /// Publication date
    pub published_date: std::time::SystemTime,
}

/// License policy information
#[derive(Clone, Debug)]
pub struct LicensePolicy {
    /// License identifier
    pub license_id: String,
    /// Whether this license is allowed
    pub allowed: bool,
    /// Description of the policy
    pub description: String,
}

/// Component information for SBOM
#[derive(Clone, Debug)]
pub struct Component {
    /// Component ID
    pub id: String,
    /// Component name
    pub name: String,
    /// Component version
    pub version: String,
    /// Component license
    pub license: Option<String>,
    /// Component supplier
    pub supplier: Option<String>,
}

/// Dependency information
#[derive(Clone, Debug)]
pub struct Dependency {
    /// Dependency name
    pub name: String,
    /// Dependency version
    pub version: String,
    /// Dependency license
    pub license: Option<String>,
}

/// Issue found during static analysis
#[derive(Clone, Debug)]
pub struct Issue {
    /// Issue ID
    pub id: String,
    /// Severity level
    pub severity: Severity,
    /// Description of the issue
    pub description: String,
    /// File where the issue was found
    pub file: String,
    /// Line number where the issue was found
    pub line: u32,
}

/// Vulnerability finding
#[derive(Clone, Debug)]
pub struct VulnerabilityFinding {
    /// The dependency with the vulnerability
    pub dependency: Dependency,
    /// The vulnerability information
    pub vulnerability: Vulnerability,
    /// Severity of the finding
    pub severity: Severity,
}

/// License finding
#[derive(Clone, Debug)]
pub struct LicenseFinding {
    /// The dependency with the license issue
    pub dependency: Dependency,
    /// The license ID
    pub license_id: String,
    /// The license policy
    pub policy: LicensePolicy,
    /// Whether this is a violation
    pub violation: bool,
}

/// Software Bill of Materials
#[derive(Clone, Debug)]
pub struct SBOM {
    /// Project name
    pub project_name: String,
    /// Components in the project
    pub components: Vec<Component>,
    /// Generation timestamp
    pub generated_at: std::time::SystemTime,
}

/// Scan result for telemetry
#[derive(Clone, Debug)]
pub struct ScanResult {
    /// Scan timestamp
    pub timestamp: std::time::SystemTime,
    /// Number of vulnerabilities found
    pub vuln_count: usize,
    /// Number of license violations
    pub license_violations: usize,
    /// Critical vulnerability count
    pub critical_vuln_count: usize,
}

/// Severity levels
#[derive(Clone, Debug, PartialEq)]
pub enum Severity {
    /// Critical severity
    Critical,
    /// High severity
    High,
    /// Medium severity
    Medium,
    /// Low severity
    Low,
}

/// Web Application Firewall (WAF) for runtime protection
pub struct WebApplicationFirewall {
    /// WAF rules organized by category
    rules: HashMap<String, Vec<WafRule>>,
    /// Block events for telemetry
    block_events: HashMap<String, u64>,
    /// Rule hit statistics
    rule_hits: HashMap<String, u64>,
}

impl WebApplicationFirewall {
    /// Create a new WebApplicationFirewall instance
    pub fn new() -> Self {
        let mut waf = Self {
            rules: HashMap::new(),
            block_events: HashMap::new(),
            rule_hits: HashMap::new(),
        };
        
        // Initialize with OWASP Top 10 rules
        waf.initialize_owasp_rules();
        
        waf
    }
    
    /// Initialize with OWASP Top 10 rules
    fn initialize_owasp_rules(&mut self) {
        // Injection prevention rules
        self.add_rule("injection".to_string(), WafRule {
            id: "INJ-001".to_string(),
            name: "SQL Injection Detection".to_string(),
            pattern: Regex::new(r#"(?i)(\bunion\b|\bselect\b|\binsert\b|\bupdate\b|\bdelete\b|\bdrop\b|\bcreate\b|\balter\b|\bexec\b|\bexecute\b).*?['"].*?;"#).unwrap(),
            severity: Severity::High,
            enabled: true,
            action: WafAction::Block,
        });
        
        self.add_rule("injection".to_string(), WafRule {
            id: "INJ-002".to_string(),
            name: "Command Injection Detect".to_string(),
            pattern: Regex::new(r"(?i)\b(system|exec|popen|eval|assert)\b.*\(").unwrap(),
            severity: Severity::Critical,
            enabled: true,
            action: WafAction::Block,
        });
        
        // XSS prevention rules
        self.add_rule("xss".to_string(), WafRule {
            id: "XSS-001".to_string(),
            name: "Script Tag Detect".to_string(),
            pattern: Regex::new(r"(?i)<script[^>]*?>").unwrap(),
            severity: Severity::High,
            enabled: true,
            action: WafAction::Block,
        });
        
        self.add_rule("xss".to_string(), WafRule {
            id: "XSS-002".to_string(),
            name: "Event Handler Detect".to_string(),
            pattern: Regex::new(r#"(?i)on(load|click|error|focus|blur|change|submit|reset|select|abort|unload|resize|scroll)\s*=\s*["']"#).unwrap(),
            severity: Severity::High,
            enabled: true,
            action: WafAction::Block,
        });
        
        // Path traversal prevention rules
        self.add_rule("path_traversal".to_string(), WafRule {
            id: "PT-001".to_string(),
            name: "Path Traversal Detect".to_string(),
            pattern: Regex::new(r#"(?i)(\.\./|\.\/|%2e%2e%2f|%2e%2f)"#).unwrap(),
            severity: Severity::High,
            enabled: true,
            action: WafAction::Block,
        });
        
        // File inclusion prevention rules
        self.add_rule("file_inclusion".to_string(), WafRule {
            id: "FI-001".to_string(),
            name: "File Inclusion Detect".to_string(),
            pattern: Regex::new(r#"(?i)(file|php|zlib|data|glob|phar|ssh2|rar|ogg|expect)://"#).unwrap(),
            severity: Severity::Critical,
            enabled: true,
            action: WafAction::Block,
        });
        
        // CSRF prevention rules
        self.add_rule("csrf".to_string(), WafRule {
            id: "CSRF-001".to_string(),
            name: "CSRF Token Missing".to_string(),
            pattern: Regex::new(r#"csrf_token=.*"#).unwrap(), // Look for CSRF token in request
            severity: Severity::Medium,
            enabled: true,
            action: WafAction::Block,
        });
    }
    
    /// Add a WAF rule
    pub fn add_rule(&mut self, category: String, rule: WafRule) {
        self.rules.entry(category).or_insert_with(Vec::new).push(rule);
    }
    
    /// Remove a WAF rule by ID
    pub fn remove_rule(&mut self, rule_id: &str) {
        for (_, rules) in &mut self.rules {
            rules.retain(|rule| rule.id != rule_id);
        }
    }
    
    /// Enable a WAF rule
    pub fn enable_rule(&mut self, rule_id: &str) {
        for (_, rules) in &mut self.rules {
            for rule in rules {
                if rule.id == rule_id {
                    rule.enabled = true;
                    return;
                }
            }
        }
    }
    
    /// Disable a WAF rule
    pub fn disable_rule(&mut self, rule_id: &str) {
        for (_, rules) in &mut self.rules {
            for rule in rules {
                if rule.id == rule_id {
                    rule.enabled = false;
                    return;
                }
            }
        }
    }
    
    /// Check if a request should be blocked
    pub fn check_request(&mut self, request: &HttpRequest) -> WafResult {
        // Check all rules
        for (category, rules) in &self.rules {
            for rule in rules {
                if rule.enabled && rule.pattern.is_match(&request.raw_data) {
                    // Record rule hit
                    *self.rule_hits.entry(rule.id.clone()).or_insert(0) += 1;
                    
                    // Take action based on rule
                    match rule.action {
                        WafAction::Block => {
                            // Record block event
                            *self.block_events.entry(category.clone()).or_insert(0) += 1;
                            
                            return WafResult::Blocked {
                                rule_id: rule.id.clone(),
                                rule_name: rule.name.clone(),
                                severity: rule.severity.clone(),
                                category: category.clone(),
                            };
                        },
                        WafAction::Log => {
                            // Just log the event, don't block
                            continue;
                        },
                        WafAction::Redirect => {
                            // Record block event
                            *self.block_events.entry(category.clone()).or_insert(0) += 1;
                            
                            return WafResult::Redirect {
                                rule_id: rule.id.clone(),
                                rule_name: rule.name.clone(),
                                severity: rule.severity.clone(),
                                category: category.clone(),
                            };
                        }
                    }
                }
            }
        }
        
        WafResult::Allowed
    }
    
    /// Get block events count by category
    pub fn get_block_events(&self, category: &str) -> u64 {
        *self.block_events.get(category).unwrap_or(&0)
    }
    
    /// Get all block events
    pub fn get_all_block_events(&self) -> &HashMap<String, u64> {
        &self.block_events
    }
    
    /// Get rule hit count by rule ID
    pub fn get_rule_hit_count(&self, rule_id: &str) -> u64 {
        *self.rule_hits.get(rule_id).unwrap_or(&0)
    }
    
    /// Get all rule hit counts
    pub fn get_all_rule_hits(&self) -> &HashMap<String, u64> {
        &self.rule_hits
    }
    
    /// Get rule hit rate (hits per category)
    pub fn get_rule_hit_rate(&self) -> HashMap<String, f64> {
        let mut rates = HashMap::new();
        let total_hits: u64 = self.rule_hits.values().sum();
        
        if total_hits == 0 {
            return rates;
        }
        
        // Group hits by category
        let mut category_hits = HashMap::new();
        for (rule_id, hits) in &self.rule_hits {
            // Find which category this rule belongs to
            for (category, rules) in &self.rules {
                if rules.iter().any(|rule| rule.id == *rule_id) {
                    *category_hits.entry(category.clone()).or_insert(0) += hits;
                    break;
                }
            }
        }
        
        // Calculate rate for each category
        for (category, hits) in category_hits {
            rates.insert(category, hits as f64 / total_hits as f64);
        }
        
        rates
    }
}

impl Default for WebApplicationFirewall {
    fn default() -> Self {
        Self::new()
    }
}

/// WAF rule definition
#[derive(Clone, Debug)]
pub struct WafRule {
    /// Unique rule identifier
    pub id: String,
    /// Human-readable rule name
    pub name: String,
    /// Regex pattern to match
    pub pattern: Regex,
    /// Severity level
    pub severity: Severity,
    /// Whether the rule is enabled
    pub enabled: bool,
    /// Action to take when rule matches
    pub action: WafAction,
}

/// WAF action types
#[derive(Clone, Debug)]
pub enum WafAction {
    /// Block the request
    Block,
    /// Log the event but allow the request
    Log,
    /// Redirect the request
    Redirect,
}

/// HTTP request representation for WAF checking
#[derive(Clone, Debug)]
pub struct HttpRequest {
    /// Raw request data (for pattern matching)
    pub raw_data: String,
    /// HTTP method
    pub method: String,
    /// Request path
    pub path: String,
    /// Request headers
    pub headers: HashMap<String, String>,
    /// Request body
    pub body: String,
    /// Query parameters
    pub query_params: HashMap<String, String>,
    /// Client IP address
    pub client_ip: String,
}

/// WAF result
#[derive(Clone, Debug)]
pub enum WafResult {
    /// Request is allowed
    Allowed,
    /// Request is blocked
    Blocked {
        /// Rule ID that triggered the block
        rule_id: String,
        /// Rule name that triggered the block
        rule_name: String,
        /// Severity of the rule
        severity: Severity,
        /// Category of the rule
        category: String,
    },
    /// Request should be redirected
    Redirect {
        /// Rule ID that triggered the redirect
        rule_id: String,
        /// Rule name that triggered the redirect
        rule_name: String,
        /// Severity of the rule
        severity: Severity,
        /// Category of the rule
        category: String,
    },
}

/// Runtime Application Self-Protection (RASP) for in-app security
pub struct RuntimeSelfProtection {
    /// RASP hooks
    hooks: HashMap<String, Box<dyn RaspHook>>,
    /// Protection events
    protection_events: HashMap<String, u64>,
}

impl RuntimeSelfProtection {
    /// Create a new RuntimeSelfProtection instance
    pub fn new() -> Self {
        Self {
            hooks: HashMap::new(),
            protection_events: HashMap::new(),
        }
    }
    
    /// Add a RASP hook
    pub fn add_hook(&mut self, name: String, hook: Box<dyn RaspHook>) {
        self.hooks.insert(name, hook);
    }
    
    /// Check SQL injection at runtime
    pub fn check_sql_injection(&mut self, query: &str) -> RaspResult {
        // Check for common SQL injection patterns
        let patterns = vec![
            r"(?i)union\s+select",
            r"(?i)'\s*(or|and)\s*'?",
            r"(?i)'\s*(union|select|insert|update|delete|drop|create|alter)",
            r"(?i)--\s*$",
            r"(?i)/\*.*?\*/",
        ];
        
        for pattern in patterns {
            let regex = Regex::new(pattern).unwrap();
            if regex.is_match(query) {
                *self.protection_events.entry("sql_injection".to_string()).or_insert(0) += 1;
                return RaspResult::Blocked {
                    protection_type: "sql_injection".to_string(),
                    details: format!("Suspicious SQL pattern detected: {}", pattern),
                };
            }
        }
        
        RaspResult::Allowed
    }
    
    /// Check file access at runtime
    pub fn check_file_access(&mut self, filepath: &str) -> RaspResult {
        // Check for path traversal attempts
        if filepath.contains("../") || filepath.contains("..\\") {
            *self.protection_events.entry("path_traversal".to_string()).or_insert(0) += 1;
            return RaspResult::Blocked {
                protection_type: "path_traversal".to_string(),
                details: "Path traversal attempt detected".to_string(),
            };
        }
        
        RaspResult::Allowed
    }
    
    /// Check command execution at runtime
    pub fn check_command_execution(&mut self, command: &str) -> RaspResult {
        // Check for dangerous commands
        let dangerous_commands = vec![
            "rm", "del", "format", "shutdown", "reboot", "kill",
            "wget", "curl", "nc", "netcat", "telnet", "ssh",
        ];
        
        for dangerous in dangerous_commands {
            if command.contains(dangerous) {
                *self.protection_events.entry("command_execution".to_string()).or_insert(0) += 1;
                return RaspResult::Blocked {
                    protection_type: "command_execution".to_string(),
                    details: format!("Dangerous command detected: {}", dangerous),
                };
            }
        }
        
        RaspResult::Allowed
    }
    
    /// Get protection events count by type
    pub fn get_protection_events(&self, protection_type: &str) -> u64 {
        *self.protection_events.get(protection_type).unwrap_or(&0)
    }
    
    /// Get all protection events
    pub fn get_all_protection_events(&self) -> &HashMap<String, u64> {
        &self.protection_events
    }
}

impl Default for RuntimeSelfProtection {
    fn default() -> Self {
        Self::new()
    }
}

/// RASP hook trait for custom protection logic
pub trait RaspHook: Send + Sync {
    /// Check if an operation should be allowed
    fn check_operation(&self, operation: &str, data: &str) -> RaspResult;
}

/// RASP result
#[derive(Clone, Debug)]
pub enum RaspResult {
    /// Operation is allowed
    Allowed,
    /// Operation is blocked
    Blocked {
        /// Type of protection that triggered the block
        protection_type: String,
        /// Details about why the operation was blocked
        details: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validation() {
        let validator = InputProtection::new();
        
        // Valid emails
        assert!(validator.validate_input("email", "test@example.com").is_ok());
        assert!(validator.validate_input("email", "user.name+tag@domain.co.uk").is_ok());
        
        // Invalid emails
        assert!(validator.validate_input("email", "invalid.email").is_err());
        assert!(validator.validate_input("email", "@example.com").is_err());
        assert!(validator.validate_input("email", "test@").is_err());
    }
    
    #[test]
    fn test_username_validation() {
        let validator = InputProtection::new();
        
        // Valid usernames
        assert!(validator.validate_input("username", "testuser").is_ok());
        assert!(validator.validate_input("username", "user_123").is_ok());
        
        // Invalid usernames
        assert!(validator.validate_input("username", "ab").is_err()); // Too short
        assert!(validator.validate_input("username", "this_username_is_way_too_long").is_err()); // Too long
        assert!(validator.validate_input("username", "user-name").is_err()); // Invalid character
    }
    
    #[test]
    fn test_length_validation() {
        let validator = InputProtection::new();
        
        // Valid length
        assert!(validator.validate_input("username", "validuser").is_ok());
        
        // Exceeds length limit
        assert!(validator.validate_input("username", "this_username_is_way_too_long_and_should_fail").is_err());
    }
    
    #[test]
    fn test_sanitize_input() {
        let validator = InputProtection::new();
        
        // Test null byte removal
        let sanitized = validator.sanitize_input("test\0user");
        assert_eq!(sanitized, "testuser");
        
        // Test normal input
        let sanitized = validator.sanitize_input("normaluser");
        assert_eq!(sanitized, "normaluser");
    }
    
    #[test]
    fn test_validate_and_sanitize() {
        let validator = InputProtection::new();
        
        // Valid input should be validated and sanitized
        let result = validator.validate_and_sanitize("email", "test@example.com");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test@example.com");
        
        // Invalid input should fail validation
        let result = validator.validate_and_sanitize("email", "invalid.email");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_rejection_stats() {
        let mut stats = RejectionStats::new();
        
        // Record some rejections
        stats.record_rejection("email_validation");
        stats.record_rejection("email_validation");
        stats.record_rejection("username_validation");
        
        // Check counts
        assert_eq!(stats.get_rejection_count("email_validation"), 2);
        assert_eq!(stats.get_rejection_count("username_validation"), 1);
        assert_eq!(stats.get_rejection_count("nonexistent_rule"), 0);
        
        // Check all counts
        let all_counts = stats.get_all_rejection_counts();
        assert_eq!(all_counts.len(), 2);
    }
    
    #[test]
    fn test_custom_validation_pattern() {
        let mut validator = InputProtection::new();
        
        // Add a custom pattern for phone numbers
        assert!(validator.add_validation_pattern("custom_phone", r"^\d{3}-\d{3}-\d{4}$").is_ok());
        
        // Test the custom pattern
        assert!(validator.validate_input("custom_phone", "123-456-7890").is_ok());
        assert!(validator.validate_input("custom_phone", "123-45-67890").is_err());
    }
    
    #[test]
    fn test_html_encoding() {
        let protector = OutputProtection::new();
        
        // Test HTML encoding
        assert_eq!(protector.html_encode("<script>alert('XSS')</script>"), 
                   "&lt;script&gt;alert(&#x27;XSS&#x27;)&lt;/script&gt;");
        assert_eq!(protector.html_encode("Hello & welcome"), 
                   "Hello &amp; welcome");
        assert_eq!(protector.html_encode("\"quotes\""), 
                   "&quot;quotes&quot;");
    }
    
    #[test]
    fn test_json_encoding() {
        let protector = OutputProtection::new();
        
        // Test JSON encoding
        assert_eq!(protector.json_encode("Hello\nWorld"), 
                   "\"Hello\\nWorld\"");
        assert_eq!(protector.json_encode("Quote \"test\""), 
                   "\"Quote \\\"test\\\"\"");
        assert_eq!(protector.json_encode("Backslash \\ test"), 
                   "\"Backslash \\\\ test\"");
    }
    
    #[test]
    fn test_header_encoding() {
        let protector = OutputProtection::new();
        
        // Test header encoding
        assert_eq!(protector.header_encode("Normal Header Value"), 
                   "Normal Header Value");
        assert_eq!(protector.header_encode("Header with \n newline"), 
                   "Header with  newline");
        assert_eq!(protector.header_encode("Header with \r return"), 
                   "Header with  return");
    }
    
    #[test]
    fn test_context_encoding() {
        let protector = OutputProtection::new();
        
        // Test context-specific encoding
        assert_eq!(protector.encode_for_context("html", "<div>content</div>"), 
                   "&lt;div&gt;content&lt;/div&gt;");
        assert_eq!(protector.encode_for_context("json", "test\"value"), 
                   "\"test\\\"value\"");
        assert_eq!(protector.encode_for_context("header", "value\ntest"), 
                   "valuetest");
        assert_eq!(protector.encode_for_context("unknown", "test"), 
                   "test"); // No encoder for unknown context
    }
    
    #[test]
    fn test_custom_encoder() {
        let mut protector = OutputProtection::new();
        
        // Add a custom encoder
        struct CustomEncoder;
        impl Encoder for CustomEncoder {
            fn encode(&self, content: &str) -> String {
                format!("CUSTOM:{}", content)
            }
        }
        
        protector.add_encoder("custom", Box::new(CustomEncoder));
        assert_eq!(protector.encode_for_context("custom", "test"), 
                   "CUSTOM:test");
    }
    
    #[test]
    fn test_otp_retry_limits() {
        let mut controls = BusinessLogicControls::new();
        controls.configure_otp_retry_limits(3, 60); // 3 attempts per minute
        
        let user_id = "test_user";
        
        // First 3 attempts should be allowed
        assert!(controls.check_otp_retry_allowed(user_id).is_ok());
        assert!(controls.check_otp_retry_allowed(user_id).is_ok());
        assert!(controls.check_otp_retry_allowed(user_id).is_ok());
        
        // 4th attempt should be blocked
        assert!(controls.check_otp_retry_allowed(user_id).is_err());
        
        // Check throttle hit count
        assert_eq!(controls.get_throttle_hit_count(user_id, "otp_retry"), 1);
    }
    
    #[test]
    fn test_withdrawal_limits() {
        let mut controls = BusinessLogicControls::new();
        controls.configure_withdrawal_limits(1000.0, 3600); // $1000 per hour
        
        let user_id = "test_user";
        
        // First withdrawal of $500 should be allowed
        assert!(controls.check_withdrawal_allowed(user_id, 500.0).is_ok());
        
        // Second withdrawal of $400 should be allowed (total $900)
        assert!(controls.check_withdrawal_allowed(user_id, 400.0).is_ok());
        
        // Third withdrawal of $200 should be blocked (would exceed $1000)
        assert!(controls.check_withdrawal_allowed(user_id, 200.0).is_err());
        
        // Check throttle hit count
        assert_eq!(controls.get_throttle_hit_count(user_id, "withdrawal"), 1);
    }
    
    #[test]
    fn test_bruteforce_protection() {
        let mut controls = BusinessLogicControls::new();
        controls.configure_bruteforce_protection(3, 60, 300); // 3 attempts per minute, 5 min lockout
        
        let user_id = "test_user";
        
        // First 3 login attempts should be allowed
        assert!(controls.check_login_allowed(user_id).is_ok());
        assert!(controls.check_login_allowed(user_id).is_ok());
        assert!(controls.check_login_allowed(user_id).is_ok());
        
        // 4th attempt should be blocked and user locked out
        assert!(controls.check_login_allowed(user_id).is_err());
        assert!(controls.is_user_locked_out(user_id));
    }
    
    #[test]
    fn test_spam_throttles() {
        let mut controls = BusinessLogicControls::new();
        controls.configure_spam_throttles(5, 60); // 5 requests per minute
        
        let user_id = "test_user";
        
        // First 5 requests should be allowed
        for _ in 0..5 {
            assert!(controls.check_request_allowed(user_id).is_ok());
        }
        
        // 6th request should be blocked
        assert!(controls.check_request_allowed(user_id).is_err());
        
        // Check throttle hit count
        assert_eq!(controls.get_throttle_hit_count(user_id, "spam"), 1);
    }
    
    #[test]
    fn test_user_counter_reset() {
        let mut controls = BusinessLogicControls::new();
        controls.configure_otp_retry_limits(3, 60);
        controls.configure_bruteforce_protection(3, 60, 300);
        controls.configure_spam_throttles(5, 60);
        
        let user_id = "test_user";
        
        // Trigger some limits
        assert!(controls.check_otp_retry_allowed(user_id).is_ok());
        assert!(controls.check_otp_retry_allowed(user_id).is_ok());
        assert!(controls.check_otp_retry_allowed(user_id).is_ok());
        assert!(controls.check_otp_retry_allowed(user_id).is_err()); // Blocked
        
        assert!(controls.check_request_allowed(user_id).is_ok());
        assert!(controls.check_request_allowed(user_id).is_ok());
        assert!(controls.check_request_allowed(user_id).is_ok());
        assert!(controls.check_request_allowed(user_id).is_ok());
        assert!(controls.check_request_allowed(user_id).is_ok());
        assert!(controls.check_request_allowed(user_id).is_err()); // Blocked
        
        // Check throttle hit counts
        assert_eq!(controls.get_throttle_hit_count(user_id, "otp_retry"), 1);
        assert_eq!(controls.get_throttle_hit_count(user_id, "spam"), 1);
        
        // Reset counters
        controls.reset_user_counters(user_id);
        
        // After reset, operations should be allowed again
        assert!(controls.check_otp_retry_allowed(user_id).is_ok());
        assert!(controls.check_request_allowed(user_id).is_ok());
        
        // Throttle hit counts should be reset
        assert_eq!(controls.get_throttle_hit_count(user_id, "otp_retry"), 0);
        assert_eq!(controls.get_throttle_hit_count(user_id, "spam"), 0);
    }
    
    #[test]
    fn test_get_user_throttle_hits() {
        let mut controls = BusinessLogicControls::new();
        controls.configure_otp_retry_limits(3, 60);
        controls.configure_spam_throttles(5, 60);
        
        let user_id = "test_user";
        
        // Trigger some limits
        assert!(controls.check_otp_retry_allowed(user_id).is_ok());
        assert!(controls.check_otp_retry_allowed(user_id).is_ok());
        assert!(controls.check_otp_retry_allowed(user_id).is_ok());
        assert!(controls.check_otp_retry_allowed(user_id).is_err()); // Blocked
        
        assert!(controls.check_request_allowed(user_id).is_ok());
        assert!(controls.check_request_allowed(user_id).is_ok());
        assert!(controls.check_request_allowed(user_id).is_ok());
        assert!(controls.check_request_allowed(user_id).is_ok());
        assert!(controls.check_request_allowed(user_id).is_ok());
        assert!(controls.check_request_allowed(user_id).is_err()); // Blocked
        
        // Get all throttle hits for user
        let throttle_hits = controls.get_user_throttle_hits(user_id);
        // Print the throttle hits for debugging
        println!("Throttle hits: {:?}", throttle_hits);
        assert_eq!(throttle_hits.len(), 2);
        assert_eq!(throttle_hits.get("otp_retry"), Some(&1));
        assert_eq!(throttle_hits.get("spam"), Some(&1));
    }
    
    #[test]
    fn test_dependency_safety_vulnerability_scanning() {
        let mut safety = DependencySafety::new();
        
        // Add a vulnerability to the database
        let vuln = Vulnerability {
            id: "CVE-2023-12345".to_string(),
            package_name: "vulnerable-package".to_string(),
            affected_versions: vec!["1.0.0".to_string(), "1.0.1".to_string()],
            severity: Severity::Critical,
            description: "A critical security vulnerability".to_string(),
            published_date: std::time::SystemTime::now(),
        };
        safety.add_vulnerability(vuln);
        
        // Create dependencies to scan
        let dependencies = vec![
            Dependency {
                name: "vulnerable-package".to_string(),
                version: "1.0.1".to_string(),
                license: Some("MIT".to_string()),
            },
            Dependency {
                name: "safe-package".to_string(),
                version: "2.0.0".to_string(),
                license: Some("Apache-2.0".to_string()),
            }
        ];
        
        // Scan dependencies
        let findings = safety.scan_dependencies(&dependencies);
        
        // Should find one vulnerability
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].dependency.name, "vulnerable-package");
        assert_eq!(findings[0].vulnerability.id, "CVE-2023-12345");
        assert_eq!(findings[0].severity, Severity::Critical);
    }
    
    #[test]
    fn test_dependency_safety_license_scanning() {
        let mut safety = DependencySafety::new();
        
        // Add a license policy
        let policy = LicensePolicy {
            license_id: "GPL-3.0".to_string(),
            allowed: false,
            description: "GPL license not allowed due to licensing restrictions".to_string(),
        };
        safety.add_license_policy(policy);
        
        // Create dependencies to scan
        let dependencies = vec![
            Dependency {
                name: "allowed-package".to_string(),
                version: "1.0.0".to_string(),
                license: Some("MIT".to_string()),
            },
            Dependency {
                name: "restricted-package".to_string(),
                version: "2.0.0".to_string(),
                license: Some("GPL-3.0".to_string()),
            }
        ];
        
        // Scan licenses
        let findings = safety.scan_licenses(&dependencies);
        
        // Should find one license violation
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].dependency.name, "restricted-package");
        assert_eq!(findings[0].license_id, "GPL-3.0");
        assert!(findings[0].violation);
    }
    
    #[test]
    fn test_dependency_safety_static_analysis() {
        let safety = DependencySafety::new();
        
        // Perform static analysis
        let issues = safety.perform_static_analysis("src/main.rs");
        
        // Should find some simulated issues
        assert!(!issues.is_empty());
        assert_eq!(issues.len(), 2);
        assert_eq!(issues[0].id, "SAST-001");
        assert_eq!(issues[1].id, "SAST-002");
    }
    
    #[test]
    fn test_dependency_safety_sbom_generation() {
        let mut safety = DependencySafety::new();
        
        // Add components to SBOM
        let component = Component {
            id: "component-1".to_string(),
            name: "example-component".to_string(),
            version: "1.0.0".to_string(),
            license: Some("MIT".to_string()),
            supplier: Some("Example Corp".to_string()),
        };
        safety.add_component(component);
        
        // Generate SBOM
        let sbom = safety.generate_sbom("test-project");
        
        assert_eq!(sbom.project_name, "test-project");
        assert_eq!(sbom.components.len(), 1);
        assert_eq!(sbom.components[0].name, "example-component");
    }
    
    #[test]
    fn test_dependency_safety_vuln_statistics() {
        let safety = DependencySafety::new();
        
        // Create some vulnerability findings
        let findings = vec![
            VulnerabilityFinding {
                dependency: Dependency {
                    name: "critical-package".to_string(),
                    version: "1.0.0".to_string(),
                    license: Some("MIT".to_string()),
                },
                vulnerability: Vulnerability {
                    id: "CVE-2023-00001".to_string(),
                    package_name: "critical-package".to_string(),
                    affected_versions: vec!["1.0.0".to_string()],
                    severity: Severity::Critical,
                    description: "Critical vulnerability".to_string(),
                    published_date: std::time::SystemTime::now(),
                },
                severity: Severity::Critical,
            },
            VulnerabilityFinding {
                dependency: Dependency {
                    name: "high-package".to_string(),
                    version: "2.0.0".to_string(),
                    license: Some("Apache-2.0".to_string()),
                },
                vulnerability: Vulnerability {
                    id: "CVE-2023-00002".to_string(),
                    package_name: "high-package".to_string(),
                    affected_versions: vec!["2.0.0".to_string()],
                    severity: Severity::High,
                    description: "High vulnerability".to_string(),
                    published_date: std::time::SystemTime::now(),
                },
                severity: Severity::High,
            }
        ];
        
        // Test critical vulnerability count
        let critical_count = safety.get_critical_vuln_count(&findings);
        assert_eq!(critical_count, 1);
        
        // Test unresolved vulnerability age
        let ages = safety.get_unresolved_vuln_age(&findings);
        assert_eq!(ages.len(), 2);
    }
    
    #[test]
    fn test_waf_sql_injection_detection() {
        let mut waf = WebApplicationFirewall::new();
        
        // Create a request with SQL injection
        let request = HttpRequest {
            raw_data: "SELECT * FROM users WHERE id = '1'; DROP TABLE users;".to_string(),
            method: "POST".to_string(),
            path: "/api/users".to_string(),
            headers: HashMap::new(),
            body: "".to_string(),
            query_params: HashMap::new(),
            client_ip: "127.0.0.1".to_string(),
        };
        
        // Should be blocked by SQL injection rule
        let result = waf.check_request(&request);
        match result {
            WafResult::Blocked { rule_id, category, .. } => {
                assert_eq!(rule_id, "INJ-001");
                assert_eq!(category, "injection");
            },
            _ => panic!("Expected request to be blocked"),
        }
        
        // Check that block event was recorded
        assert_eq!(waf.get_block_events("injection"), 1);
        assert_eq!(waf.get_rule_hit_count("INJ-001"), 1);
    }
    
    #[test]
    fn test_waf_xss_detection() {
        let mut waf = WebApplicationFirewall::new();
        
        // Create a request with XSS
        let request = HttpRequest {
            raw_data: "<script>alert('XSS')</script>".to_string(),
            method: "GET".to_string(),
            path: "/search".to_string(),
            headers: HashMap::new(),
            body: "".to_string(),
            query_params: HashMap::new(),
            client_ip: "127.0.0.1".to_string(),
        };
        
        // Should be blocked by XSS rule
        let result = waf.check_request(&request);
        match result {
            WafResult::Blocked { rule_id, category, .. } => {
                assert_eq!(rule_id, "XSS-001");
                assert_eq!(category, "xss");
            },
            _ => panic!("Expected request to be blocked"),
        }
        
        // Check that block event was recorded
        assert_eq!(waf.get_block_events("xss"), 1);
        assert_eq!(waf.get_rule_hit_count("XSS-001"), 1);
    }
    
    #[test]
    fn test_waf_allowed_request() {
        let mut waf = WebApplicationFirewall::new();
        
        // Create a legitimate request
        let request = HttpRequest {
            raw_data: "SELECT name, email FROM users WHERE active = 1".to_string(),
            method: "GET".to_string(),
            path: "/api/users".to_string(),
            headers: HashMap::new(),
            body: "".to_string(),
            query_params: HashMap::new(),
            client_ip: "127.0.0.1".to_string(),
        };
        
        // Should be allowed
        let result = waf.check_request(&request);
        match result {
            WafResult::Allowed => {},
            _ => panic!("Expected request to be allowed"),
        }
    }
    
    #[test]
    fn test_waf_rule_hit_rate() {
        let mut waf = WebApplicationFirewall::new();
        
        // Create requests that will trigger different rules
        let sql_request = HttpRequest {
            raw_data: "SELECT * FROM users WHERE id = '1';".to_string(),
            method: "POST".to_string(),
            path: "/api/users".to_string(),
            headers: HashMap::new(),
            body: "".to_string(),
            query_params: HashMap::new(),
            client_ip: "127.0.0.1".to_string(),
        };
        
        let xss_request = HttpRequest {
            raw_data: "<script>alert('XSS')</script>".to_string(),
            method: "GET".to_string(),
            path: "/search".to_string(),
            headers: HashMap::new(),
            body: "".to_string(),
            query_params: HashMap::new(),
            client_ip: "127.0.0.1".to_string(),
        };
        
        // Trigger rules
        waf.check_request(&sql_request);
        waf.check_request(&xss_request);
        waf.check_request(&xss_request); // Trigger XSS rule again
        
        // Check rule hit rates
        let rates = waf.get_rule_hit_rate();
        assert_eq!(rates.len(), 2);
        assert!(rates.contains_key("injection"));
        assert!(rates.contains_key("xss"));
        
        // XSS should have higher rate since it was triggered twice
        assert!(rates.get("xss").unwrap() > rates.get("injection").unwrap());
    }
    
    #[test]
    fn test_rasp_sql_injection() {
        let mut rasp = RuntimeSelfProtection::new();
        
        // Test suspicious SQL query
        let result = rasp.check_sql_injection("SELECT * FROM users WHERE id = '1' OR '1'='1';");
        match result {
            RaspResult::Blocked { protection_type, .. } => {
                assert_eq!(protection_type, "sql_injection");
            },
            _ => panic!("Expected SQL injection to be blocked"),
        }
        
        // Check that protection event was recorded
        assert_eq!(rasp.get_protection_events("sql_injection"), 1);
    }
    
    #[test]
    fn test_rasp_path_traversal() {
        let mut rasp = RuntimeSelfProtection::new();
        
        // Test path traversal attempt
        let result = rasp.check_file_access("../../etc/passwd");
        match result {
            RaspResult::Blocked { protection_type, .. } => {
                assert_eq!(protection_type, "path_traversal");
            },
            _ => panic!("Expected path traversal to be blocked"),
        }
        
        // Check that protection event was recorded
        assert_eq!(rasp.get_protection_events("path_traversal"), 1);
    }
    
    #[test]
    fn test_rasp_command_execution() {
        let mut rasp = RuntimeSelfProtection::new();
        
        // Test dangerous command
        let result = rasp.check_command_execution("rm -rf /");
        match result {
            RaspResult::Blocked { protection_type, .. } => {
                assert_eq!(protection_type, "command_execution");
            },
            _ => panic!("Expected command execution to be blocked"),
        }
        
        // Check that protection event was recorded
        assert_eq!(rasp.get_protection_events("command_execution"), 1);
    }
    
    #[test]
    fn test_rasp_allowed_operations() {
        let mut rasp = RuntimeSelfProtection::new();
        
        // Test legitimate SQL query
        let result = rasp.check_sql_injection("SELECT name, email FROM users WHERE active = 1");
        match result {
            RaspResult::Allowed => {},
            _ => panic!("Expected legitimate query to be allowed"),
        }
        
        // Test legitimate file access
        let result = rasp.check_file_access("/home/user/documents/file.txt");
        match result {
            RaspResult::Allowed => {},
            _ => panic!("Expected legitimate file access to be allowed"),
        }
        
        // Test legitimate command
        let result = rasp.check_command_execution("ls -la");
        match result {
            RaspResult::Allowed => {},
            _ => panic!("Expected legitimate command to be allowed"),
        }
    }
}
