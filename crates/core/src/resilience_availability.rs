//! Resilience and Availability Module
//!
//! This module implements comprehensive resilience and availability measures including:
//! - High Availability (HA) and Failover mechanisms
//! - Traffic protection (Circuit Breakers, Bulkheads, Rate Shaping)
//! - Graceful degradation (Feature Flags, Read-only Mode)
//! - Disaster Recovery (DR Playbook, Chaos Testing)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a service instance for HA/failover
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    /// Service name
    pub name: String,
    /// Service endpoint
    pub endpoint: String,
    /// Availability zone
    pub zone: String,
    /// Whether the service is active
    pub active: bool,
    /// Last health check timestamp
    pub last_check: u64,
    /// Service health status
    pub health_status: ServiceHealth,
    /// Service priority (lower is higher priority)
    pub priority: u32,
}

impl ServiceInstance {
    /// Create a new service instance
    pub fn new(name: String, endpoint: String, zone: String, priority: u32) -> Self {
        Self {
            name,
            endpoint,
            zone,
            active: true,
            last_check: 0,
            health_status: ServiceHealth::Healthy,
            priority,
        }
    }
}

/// Represents service health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServiceHealth {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Represents HA/failover configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HaFailoverConfig {
    /// Enable multi-AZ deployment
    pub multi_az_enabled: bool,
    /// Load balancer health check interval in seconds
    pub health_check_interval: u64,
    /// Number of replicas per service
    pub replicas_per_service: u32,
    /// Failover timeout in milliseconds
    pub failover_timeout_ms: u64,
}

/// Represents traffic protection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficProtectionConfig {
    /// Circuit breaker failure threshold
    pub circuit_breaker_threshold: u32,
    /// Circuit breaker timeout in milliseconds
    pub circuit_breaker_timeout_ms: u64,
    /// Bulkhead concurrency limit
    pub bulkhead_concurrency_limit: u32,
    /// Rate shaping configuration
    pub rate_shaping: RateShapingConfig,
}

/// Represents rate shaping configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateShapingConfig {
    /// Maximum requests per second
    pub max_rps: u32,
    /// Burst size
    pub burst_size: u32,
    /// Shed load percentage when under stress
    pub shed_percentage: u32,
}

/// Represents graceful degradation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GracefulDegradationConfig {
    /// Enable feature flags
    pub feature_flags_enabled: bool,
    /// Enable read-only mode
    pub read_only_mode_enabled: bool,
    /// Cache TTL for degraded mode in seconds
    pub cache_ttl_seconds: u64,
    /// Fallback data sources
    pub fallback_data_sources: Vec<String>,
}

/// Represents disaster recovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisasterRecoveryConfig {
    /// Enable DR playbook
    pub dr_playbook_enabled: bool,
    /// Enable chaos testing
    pub chaos_testing_enabled: bool,
    /// RPO (Recovery Point Objective) in seconds
    pub rpo_seconds: u64,
    /// RTO (Recovery Time Objective) in seconds
    pub rto_seconds: u64,
    /// Backup retention period in days
    pub backup_retention_days: u32,
}

/// Represents circuit breaker state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CircuitBreakerState {
    Closed,
    Open,
    HalfOpen,
}

/// Represents a circuit breaker for service protection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreaker {
    /// Circuit breaker state
    pub state: CircuitBreakerState,
    /// Failure count
    pub failure_count: u32,
    /// Last failure timestamp
    pub last_failure: u64,
    /// Timeout for open state in milliseconds
    pub timeout_ms: u64,
    /// Threshold for opening circuit breaker
    pub failure_threshold: u32,
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    pub fn new(failure_threshold: u32, timeout_ms: u64) -> Self {
        Self {
            state: CircuitBreakerState::Closed,
            failure_count: 0,
            last_failure: 0,
            timeout_ms,
            failure_threshold,
        }
    }

    /// Record a failure and return old and new states for telemetry
    pub fn record_failure(&mut self, timestamp: u64) -> (CircuitBreakerState, CircuitBreakerState) {
        let old_state = self.state.clone();
        self.failure_count += 1;
        self.last_failure = timestamp;

        if self.failure_count >= self.failure_threshold {
            self.state = CircuitBreakerState::Open;
        }
        
        (old_state, self.state.clone())
    }

    /// Reset the circuit breaker and return old and new states for telemetry
    pub fn reset(&mut self) -> (CircuitBreakerState, CircuitBreakerState) {
        let old_state = self.state.clone();
        self.state = CircuitBreakerState::Closed;
        self.failure_count = 0;
        self.last_failure = 0;
        (old_state, self.state.clone())
    }

    /// Check if the circuit breaker allows requests and return state information for telemetry
    pub fn can_execute(&mut self, timestamp: u64) -> (bool, CircuitBreakerState, CircuitBreakerState) {
        let old_state = self.state.clone();
        let result = match self.state {
            CircuitBreakerState::Closed => true,
            CircuitBreakerState::Open => {
                if timestamp - self.last_failure > self.timeout_ms {
                    self.state = CircuitBreakerState::HalfOpen;
                    true
                } else {
                    false
                }
            }
            CircuitBreakerState::HalfOpen => true,
        };
        let new_state = self.state.clone();
        (result, old_state, new_state)
    }
    
    /// Get current state
    pub fn get_state(&self) -> CircuitBreakerState {
        self.state.clone()
    }
    
    /// Get failure count
    pub fn get_failure_count(&self) -> u32 {
        self.failure_count
    }
}

/// Represents a bulkhead for resource isolation
#[derive(Debug, Clone)]
pub struct Bulkhead {
    /// Current concurrent requests
    current_concurrent: Arc<AtomicU64>,
    /// Maximum concurrent requests allowed
    max_concurrent: u64,
}

impl Bulkhead {
    /// Create a new bulkhead
    pub fn new(max_concurrent: u64) -> Self {
        Self {
            current_concurrent: Arc::new(AtomicU64::new(0)),
            max_concurrent,
        }
    }

    /// Try to acquire a slot in the bulkhead
    pub fn try_acquire(&self) -> bool {
        let current = self.current_concurrent.load(Ordering::Relaxed);
        if current < self.max_concurrent {
            self.current_concurrent.fetch_add(1, Ordering::Relaxed);
            true
        } else {
            false
        }
    }

    /// Release a slot in the bulkhead
    pub fn release(&self) {
        self.current_concurrent.fetch_sub(1, Ordering::Relaxed);
    }
}

/// Custom error type for resilience and availability operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResilienceAvailabilityError {
    /// Service not found
    ServiceNotFound(String),
    /// Service unhealthy
    ServiceUnhealthy(String),
    /// Circuit breaker open
    CircuitBreakerOpen(String),
    /// Bulkhead saturated
    BulkheadSaturated(String),
    /// Feature disabled
    FeatureDisabled(String),
    /// Generic error
    GenericError(String),
}

impl std::fmt::Display for ResilienceAvailabilityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResilienceAvailabilityError::ServiceNotFound(name) => write!(f, "Service not found: {}", name),
            ResilienceAvailabilityError::ServiceUnhealthy(name) => {
                write!(f, "Service unhealthy: {}", name)
            }
            ResilienceAvailabilityError::CircuitBreakerOpen(name) => {
                write!(f, "Circuit breaker open: {}", name)
            }
            ResilienceAvailabilityError::BulkheadSaturated(name) => {
                write!(f, "Bulkhead saturated: {}", name)
            }
            ResilienceAvailabilityError::FeatureDisabled(name) => {
                write!(f, "Feature disabled: {}", name)
            }
            ResilienceAvailabilityError::GenericError(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for ResilienceAvailabilityError {}

/// Represents telemetry data for circuit breaker events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerEvent {
    /// Service name
    pub service_name: String,
    /// Timestamp of the event
    pub timestamp: u64,
    /// Previous state
    pub from_state: CircuitBreakerState,
    /// New state
    pub to_state: CircuitBreakerState,
    /// Failure count at time of event
    pub failure_count: u32,
}

/// Represents telemetry data for bulkhead saturation events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkheadSaturationEvent {
    /// Service name
    pub service_name: String,
    /// Timestamp of the event
    pub timestamp: u64,
    /// Current concurrent requests
    pub current_concurrent: u64,
    /// Maximum concurrent requests allowed
    pub max_concurrent: u64,
}

/// Represents telemetry data for rate shaping events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateShapingEvent {
    /// Service name
    pub service_name: String,
    /// Timestamp of the event
    pub timestamp: u64,
    /// Shed percentage
    pub shed_percentage: u32,
    /// Current RPS
    pub current_rps: u32,
    /// Max RPS
    pub max_rps: u32,
}

/// Represents telemetry data for graceful degradation events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GracefulDegradationEvent {
    /// Service name
    pub service_name: String,
    /// Timestamp of the event
    pub timestamp: u64,
    /// Type of degradation event
    pub event_type: GracefulDegradationEventType,
    /// Duration in seconds
    pub duration_seconds: u64,
    /// Additional context
    pub context: String,
}

/// Represents types of graceful degradation events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GracefulDegradationEventType {
    /// System entered degraded mode
    EnteredDegradedMode,
    /// System exited degraded mode
    ExitedDegradedMode,
    /// Feature flag toggled
    FeatureFlagToggled,
    /// Read-only mode activated
    ReadOnlyModeActivated,
    /// Read-only mode deactivated
    ReadOnlyModeDeactivated,
    /// Cache fallback used
    CacheFallbackUsed,
    /// Withdraw disabled mode
    WithdrawDisabledMode,
}

/// Represents cache data for graceful degradation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheData {
    /// Cache key
    pub key: String,
    /// Cached value
    pub value: String,
    /// Expiration timestamp
    pub expires_at: u64,
    /// Last accessed timestamp
    pub last_accessed: u64,
}

/// Manages resilience and availability including HA/failover, traffic protection, and graceful degradation
pub struct ResilienceAvailabilityManager {
    /// Service instances for HA/failover
    pub service_instances: HashMap<String, ServiceInstance>,
    /// Circuit breakers for each service
    pub circuit_breakers: HashMap<String, CircuitBreaker>,
    /// Bulkheads for resource isolation
    pub bulkheads: HashMap<String, Bulkhead>,
    /// HA/failover configuration
    pub ha_config: HaFailoverConfig,
    /// Traffic protection configuration
    pub traffic_config: TrafficProtectionConfig,
    /// Graceful degradation configuration
    pub degradation_config: GracefulDegradationConfig,
    /// Disaster recovery configuration
    pub dr_config: DisasterRecoveryConfig,
    /// Feature flags
    pub feature_flags: HashMap<String, AtomicBool>,
    /// Read-only mode flag
    pub read_only_mode: AtomicBool,
    /// Withdraw disabled mode flag
    pub withdraw_disabled_mode: AtomicBool,
    /// Uptime tracking
    pub uptime_start: u64,
    /// Failover event count
    pub failover_events: AtomicU64,
    /// Circuit breaker events for telemetry
    pub circuit_breaker_events: Arc<std::sync::Mutex<Vec<CircuitBreakerEvent>>>,
    /// Bulkhead saturation events for telemetry
    pub bulkhead_saturation_events: Arc<std::sync::Mutex<Vec<BulkheadSaturationEvent>>>,
    /// Rate shaping events for telemetry
    pub rate_shaping_events: Arc<std::sync::Mutex<Vec<RateShapingEvent>>>,
    /// Graceful degradation events for telemetry
    pub degradation_events: Arc<std::sync::Mutex<Vec<GracefulDegradationEvent>>>,
    /// Request counter for rate shaping
    pub request_counter: Arc<AtomicU64>,
    /// Last reset time for rate shaping
    pub last_reset_time: Arc<AtomicU64>,
    /// Degraded mode start time
    pub degraded_mode_start: Arc<AtomicU64>,
    /// Cache data for fallback
    pub cache_data: Arc<std::sync::Mutex<HashMap<String, CacheData>>>,
    /// Feature flag usage tracking
    pub feature_flag_usage: Arc<std::sync::Mutex<HashMap<String, u64>>>,
}

impl ResilienceAvailabilityManager {
    /// Create a new resilience and availability manager
    pub fn new(
        ha_config: HaFailoverConfig,
        traffic_config: TrafficProtectionConfig,
        degradation_config: GracefulDegradationConfig,
        dr_config: DisasterRecoveryConfig,
    ) -> Self {
        let uptime_start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            service_instances: HashMap::new(),
            circuit_breakers: HashMap::new(),
            bulkheads: HashMap::new(),
            ha_config,
            traffic_config,
            degradation_config,
            dr_config,
            feature_flags: HashMap::new(),
            read_only_mode: AtomicBool::new(false),
            withdraw_disabled_mode: AtomicBool::new(false),
            uptime_start,
            failover_events: AtomicU64::new(0),
            circuit_breaker_events: Arc::new(std::sync::Mutex::new(Vec::new())),
            bulkhead_saturation_events: Arc::new(std::sync::Mutex::new(Vec::new())),
            rate_shaping_events: Arc::new(std::sync::Mutex::new(Vec::new())),
            degradation_events: Arc::new(std::sync::Mutex::new(Vec::new())),
            request_counter: Arc::new(AtomicU64::new(0)),
            last_reset_time: Arc::new(AtomicU64::new(uptime_start)),
            degraded_mode_start: Arc::new(AtomicU64::new(0)),
            cache_data: Arc::new(std::sync::Mutex::new(HashMap::new())),
            feature_flag_usage: Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }

    /// Add a service instance
    pub fn add_service_instance(&mut self, service: ServiceInstance) {
        let service_name = service.name.clone();
        self.service_instances.insert(service_name.clone(), service);
        self.circuit_breakers.insert(
            service_name.clone(),
            CircuitBreaker::new(
                self.traffic_config.circuit_breaker_threshold,
                self.traffic_config.circuit_breaker_timeout_ms,
            ),
        );
        self.bulkheads.insert(
            service_name,
            Bulkhead::new(self.traffic_config.bulkhead_concurrency_limit as u64),
        );
    }

    /// Remove a service instance
    pub fn remove_service_instance(&mut self, name: &str) -> Result<(), ResilienceAvailabilityError> {
        if self.service_instances.remove(name).is_some() {
            self.circuit_breakers.remove(name);
            self.bulkheads.remove(name);
            Ok(())
        } else {
            Err(ResilienceAvailabilityError::ServiceNotFound(name.to_string()))
        }
    }

    /// Update service health status
    pub fn update_service_health(
        &mut self,
        name: &str,
        health: ServiceHealth,
    ) -> Result<(), ResilienceAvailabilityError> {
        let timestamp = self.current_timestamp();
        if let Some(service) = self.service_instances.get_mut(name) {
            service.health_status = health;
            service.last_check = timestamp;
            Ok(())
        } else {
            Err(ResilienceAvailabilityError::ServiceNotFound(name.to_string()))
        }
    }

    /// Get current timestamp in seconds
    fn current_timestamp(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    /// Get healthy service instances sorted by priority
    pub fn get_healthy_services(&self) -> Vec<ServiceInstance> {
        let mut healthy_services: Vec<ServiceInstance> = self
            .service_instances
            .iter()
            .filter(|(_, service)| {
                service.active && service.health_status == ServiceHealth::Healthy
            })
            .map(|(_, service)| service.clone())
            .collect();

        // Sort by priority (lower is higher priority)
        healthy_services.sort_by_key(|s| s.priority);
        healthy_services
    }

    /// Check if a service is healthy based on circuit breaker status
    fn is_service_healthy(
        &mut self,
        name: &str,
        timestamp: u64,
    ) -> Result<bool, ResilienceAvailabilityError> {
        if !self.circuit_breakers.contains_key(name) {
            return Err(ResilienceAvailabilityError::ServiceNotFound(name.to_string()));
        }
        
        if let Some(cb) = self.circuit_breakers.get_mut(name) {
            let failure_count = cb.get_failure_count();
            let (result, old_state, new_state) = cb.can_execute(timestamp);
            // Record telemetry if state changed
            if old_state != new_state {
                self.record_circuit_breaker_event(
                    name.to_string(),
                    old_state,
                    new_state,
                    failure_count,
                );
            }
            Ok(result)
        } else {
            Err(ResilienceAvailabilityError::ServiceNotFound(name.to_string()))
        }
    }

    /// Record a successful service call
    pub fn record_success(&mut self, service_name: &str) {
        if !self.circuit_breakers.contains_key(service_name) {
            return;
        }
        
        if let Some(cb) = self.circuit_breakers.get_mut(service_name) {
            let failure_count = cb.get_failure_count();
            let (old_state, new_state) = cb.reset();
            // Record telemetry for state change
            if old_state != new_state {
                self.record_circuit_breaker_event(
                    service_name.to_string(),
                    old_state,
                    new_state,
                    failure_count,
                );
            }
        }
    }

    /// Record a failed service call
    pub fn record_failure(&mut self, service_name: &str) {
        if !self.circuit_breakers.contains_key(service_name) {
            return;
        }
        
        let timestamp = self.current_timestamp();
        if let Some(cb) = self.circuit_breakers.get_mut(service_name) {
            // Get the failure count before the increment
            let failure_count_before = cb.get_failure_count();
            let (old_state, new_state) = cb.record_failure(timestamp);
            // Record telemetry for state change
            // Use the failure count after the increment for the event
            if old_state != new_state {
                self.record_circuit_breaker_event(
                    service_name.to_string(),
                    old_state,
                    new_state,
                    failure_count_before + 1,
                );
            }
        }
    }

    /// Record a failover event
    pub fn record_failover_event(&self) {
        self.failover_events.fetch_add(1, Ordering::Relaxed);
    }

    /// Get uptime percentage
    pub fn get_uptime_percentage(&self) -> f64 {
        let current_time = self.current_timestamp();
        let uptime_seconds = current_time - self.uptime_start;
        
        // In a real implementation, we would track downtime events
        // For now, we'll assume 100% uptime unless there are failover events
        let failover_count = self.failover_events.load(Ordering::Relaxed);
        
        if uptime_seconds > 0 && failover_count > 0 {
            // Simple calculation: assume each failover event causes 1 second of downtime
            let downtime_seconds = failover_count;
            let uptime_percentage = ((uptime_seconds - downtime_seconds) as f64 / uptime_seconds as f64) * 100.0;
            uptime_percentage.max(0.0) // Ensure we don't go below 0%
        } else {
            100.0
        }
    }

    /// Get failover event logs
    pub fn get_failover_event_logs(&self) -> Vec<String> {
        let failover_count = self.failover_events.load(Ordering::Relaxed);
        vec![format!("Total failover events: {}", failover_count)]
    }

    /// Enable read-only mode
    pub fn enable_read_only_mode(&self) {
        let was_enabled = self.read_only_mode.swap(true, Ordering::Relaxed);
        if !was_enabled {
            self.record_degradation_event(
                "system".to_string(),
                GracefulDegradationEventType::ReadOnlyModeActivated,
                0,
                "Read-only mode enabled".to_string(),
            );
            self.degraded_mode_start.store(self.current_timestamp(), Ordering::Relaxed);
        }
    }

    /// Disable read-only mode
    pub fn disable_read_only_mode(&self) {
        let was_enabled = self.read_only_mode.swap(false, Ordering::Relaxed);
        if was_enabled {
            let start_time = self.degraded_mode_start.load(Ordering::Relaxed);
            let duration = if start_time > 0 {
                self.current_timestamp() - start_time
            } else {
                0
            };
            self.record_degradation_event(
                "system".to_string(),
                GracefulDegradationEventType::ReadOnlyModeDeactivated,
                duration,
                "Read-only mode disabled".to_string(),
            );
        }
    }

    /// Check if read-only mode is enabled
    pub fn is_read_only_mode(&self) -> bool {
        self.read_only_mode.load(Ordering::Relaxed)
    }

    /// Enable withdraw disabled mode
    pub fn enable_withdraw_disabled_mode(&self) {
        let was_enabled = self.withdraw_disabled_mode.swap(true, Ordering::Relaxed);
        if !was_enabled {
            self.record_degradation_event(
                "system".to_string(),
                GracefulDegradationEventType::WithdrawDisabledMode,
                0,
                "Withdraw disabled mode enabled".to_string(),
            );
            self.degraded_mode_start.store(self.current_timestamp(), Ordering::Relaxed);
        }
    }

    /// Disable withdraw disabled mode
    pub fn disable_withdraw_disabled_mode(&self) {
        let was_enabled = self.withdraw_disabled_mode.swap(false, Ordering::Relaxed);
        if was_enabled {
            let start_time = self.degraded_mode_start.load(Ordering::Relaxed);
            let duration = if start_time > 0 {
                self.current_timestamp() - start_time
            } else {
                0
            };
            self.record_degradation_event(
                "system".to_string(),
                GracefulDegradationEventType::ExitedDegradedMode,
                duration,
                "Withdraw disabled mode disabled".to_string(),
            );
        }
    }

    /// Check if withdraw disabled mode is enabled
    pub fn is_withdraw_disabled_mode(&self) -> bool {
        self.withdraw_disabled_mode.load(Ordering::Relaxed)
    }

    /// Set a feature flag
    pub fn set_feature_flag(&mut self, name: String, enabled: bool) {
        self.feature_flags
            .entry(name.clone())
            .or_insert_with(|| AtomicBool::new(enabled))
            .store(enabled, Ordering::Relaxed);
        
        // Track feature flag usage
        if let Ok(mut usage) = self.feature_flag_usage.lock() {
            *usage.entry(name.clone()).or_insert(0) += 1;
        }
        
        self.record_degradation_event(
            name.clone(),
            GracefulDegradationEventType::FeatureFlagToggled,
            0,
            format!("Feature flag '{}' set to {}", name, enabled),
        );
    }

    /// Check if a feature flag is enabled
    pub fn is_feature_enabled(&self, name: &str) -> bool {
        // Track feature flag usage
        if let Ok(mut usage) = self.feature_flag_usage.lock() {
            *usage.entry(name.to_string()).or_insert(0) += 1;
        }
        
        self.feature_flags
            .get(name)
            .map(|flag| flag.load(Ordering::Relaxed))
            .unwrap_or(false)
    }

    /// Get feature flag usage statistics
    pub fn get_feature_flag_usage(&self) -> HashMap<String, u64> {
        if let Ok(usage) = self.feature_flag_usage.lock() {
            usage.clone()
        } else {
            HashMap::new()
        }
    }

    /// Record a circuit breaker state change for telemetry
    fn record_circuit_breaker_event(&self, service_name: String, from_state: CircuitBreakerState, to_state: CircuitBreakerState, failure_count: u32) {
        let timestamp = self.current_timestamp();
        let event = CircuitBreakerEvent {
            service_name,
            timestamp,
            from_state,
            to_state,
            failure_count,
        };
        
        if let Ok(mut events) = self.circuit_breaker_events.lock() {
            events.push(event);
        }
    }

    /// Get circuit breaker events
    pub fn get_circuit_breaker_events(&self) -> Vec<CircuitBreakerEvent> {
        if let Ok(events) = self.circuit_breaker_events.lock() {
            events.clone()
        } else {
            Vec::new()
        }
    }

    /// Record a bulkhead saturation event for telemetry
    fn record_bulkhead_saturation_event(&self, service_name: String, current_concurrent: u64, max_concurrent: u64) {
        let timestamp = self.current_timestamp();
        let event = BulkheadSaturationEvent {
            service_name,
            timestamp,
            current_concurrent,
            max_concurrent,
        };
        
        if let Ok(mut events) = self.bulkhead_saturation_events.lock() {
            events.push(event);
        }
    }

    /// Get bulkhead saturation events
    pub fn get_bulkhead_saturation_events(&self) -> Vec<BulkheadSaturationEvent> {
        if let Ok(events) = self.bulkhead_saturation_events.lock() {
            events.clone()
        } else {
            Vec::new()
        }
    }

    /// Record a rate shaping event for telemetry
    fn record_rate_shaping_event(&self, service_name: String, shed_percentage: u32, current_rps: u32, max_rps: u32) {
        let timestamp = self.current_timestamp();
        let event = RateShapingEvent {
            service_name,
            timestamp,
            shed_percentage,
            current_rps,
            max_rps,
        };
        
        if let Ok(mut events) = self.rate_shaping_events.lock() {
            events.push(event);
        }
    }

    /// Get rate shaping events
    pub fn get_rate_shaping_events(&self) -> Vec<RateShapingEvent> {
        if let Ok(events) = self.rate_shaping_events.lock() {
            events.clone()
        } else {
            Vec::new()
        }
    }

    /// Record a graceful degradation event for telemetry
    fn record_degradation_event(&self, service_name: String, event_type: GracefulDegradationEventType, duration_seconds: u64, context: String) {
        let timestamp = self.current_timestamp();
        let event = GracefulDegradationEvent {
            service_name,
            timestamp,
            event_type,
            duration_seconds,
            context,
        };
        
        if let Ok(mut events) = self.degradation_events.lock() {
            events.push(event);
        }
    }

    /// Get graceful degradation events
    pub fn get_degradation_events(&self) -> Vec<GracefulDegradationEvent> {
        if let Ok(events) = self.degradation_events.lock() {
            events.clone()
        } else {
            Vec::new()
        }
    }

    /// Get time spent in degraded mode
    pub fn get_time_spent_in_degraded_mode(&self) -> u64 {
        let start_time = self.degraded_mode_start.load(Ordering::Relaxed);
        if start_time > 0 {
            self.current_timestamp() - start_time
        } else {
            0
        }
    }

    /// Store data in cache for fallback
    pub fn cache_data(&self, key: String, value: String, ttl_seconds: u64) {
        let timestamp = self.current_timestamp();
        let cache_data = CacheData {
            key: key.clone(),
            value,
            expires_at: timestamp + ttl_seconds,
            last_accessed: timestamp,
        };
        
        if let Ok(mut cache) = self.cache_data.lock() {
            cache.insert(key, cache_data);
        }
    }

    /// Get cached data if available and not expired
    pub fn get_cached_data(&self, key: &str) -> Option<String> {
        let timestamp = self.current_timestamp();
        
        if let Ok(mut cache) = self.cache_data.lock() {
            if let Some(cache_data) = cache.get_mut(key) {
                // Check if data is expired
                if cache_data.expires_at > timestamp {
                    cache_data.last_accessed = timestamp;
                    self.record_degradation_event(
                        key.to_string(),
                        GracefulDegradationEventType::CacheFallbackUsed,
                        0,
                        "Cache fallback used".to_string(),
                    );
                    return Some(cache_data.value.clone());
                } else {
                    // Remove expired data
                    cache.remove(key);
                }
            }
        }
        
        None
    }

    /// Clear expired cache data
    pub fn clear_expired_cache(&self) {
        let timestamp = self.current_timestamp();
        
        if let Ok(mut cache) = self.cache_data.lock() {
            cache.retain(|_, cache_data| cache_data.expires_at > timestamp);
        }
    }

    /// Calculate current requests per second for rate shaping
    fn calculate_current_rps(&self) -> u32 {
        let current_time = self.current_timestamp();
        let last_reset = self.last_reset_time.load(Ordering::Relaxed);
        
        // Reset counter every second
        if current_time > last_reset {
            self.request_counter.store(0, Ordering::Relaxed);
            self.last_reset_time.store(current_time, Ordering::Relaxed);
        }
        
        self.request_counter.load(Ordering::Relaxed) as u32
    }

    /// Try to acquire a bulkhead slot with telemetry
    pub fn try_acquire_bulkhead(&self, service_name: &str) -> Result<bool, ResilienceAvailabilityError> {
        if let Some(bulkhead) = self.bulkheads.get(service_name) {
            let result = bulkhead.try_acquire();
            
            // Record telemetry if bulkhead is saturated
            if !result {
                if let Ok(current) = bulkhead.current_concurrent.load(Ordering::Relaxed).try_into() {
                    self.record_bulkhead_saturation_event(
                        service_name.to_string(),
                        current,
                        bulkhead.max_concurrent,
                    );
                }
            }
            
            Ok(result)
        } else {
            Err(ResilienceAvailabilityError::ServiceNotFound(service_name.to_string()))
        }
    }

    /// Release a bulkhead slot
    pub fn release_bulkhead(&self, service_name: &str) -> Result<(), ResilienceAvailabilityError> {
        if let Some(bulkhead) = self.bulkheads.get(service_name) {
            bulkhead.release();
            Ok(())
        } else {
            Err(ResilienceAvailabilityError::ServiceNotFound(service_name.to_string()))
        }
    }

    /// Execute a service call with resilience measures
    pub async fn execute_service_call<T, F, Fut>(
        &mut self,
        service_name: &str,
        mut operation: F,
    ) -> Result<T, ResilienceAvailabilityError>
    where
        F: FnMut(&ServiceInstance) -> Fut,
        Fut: std::future::Future<Output = Result<T, Box<dyn std::error::Error + Send + Sync>>>,
    {
        let timestamp = self.current_timestamp();
        
        // Increment request counter for rate shaping
        self.request_counter.fetch_add(1, Ordering::Relaxed);
        
        // Check if we need to shed load based on rate shaping configuration
        let current_rps = self.calculate_current_rps();
        if current_rps > self.traffic_config.rate_shaping.max_rps {
            // Apply load shedding based on configured percentage
            let shed_chance = self.traffic_config.rate_shaping.shed_percentage as f64 / 100.0;
            if rand::random::<f64>() < shed_chance {
                self.record_rate_shaping_event(
                    service_name.to_string(),
                    self.traffic_config.rate_shaping.shed_percentage,
                    current_rps,
                    self.traffic_config.rate_shaping.max_rps,
                );
                return Err(ResilienceAvailabilityError::GenericError(
                    "Load shed due to rate shaping".to_string(),
                ));
            }
        }

        // Check if service exists
        if !self.service_instances.contains_key(service_name) {
            return Err(ResilienceAvailabilityError::ServiceNotFound(service_name.to_string()));
        }

        // Check circuit breaker
        if !self.is_service_healthy(service_name, timestamp)? {
            return Err(ResilienceAvailabilityError::CircuitBreakerOpen(service_name.to_string()));
        }

        // Try to acquire bulkhead slot
        if !self.try_acquire_bulkhead(service_name)? {
            return Err(ResilienceAvailabilityError::BulkheadSaturated(service_name.to_string()));
        }

        // Get healthy service instances
        let healthy_services = self.get_healthy_services();
        if healthy_services.is_empty() {
            self.release_bulkhead(service_name)?;
            return Err(ResilienceAvailabilityError::GenericError(
                "No healthy services available".to_string(),
            ));
        }

        let mut last_error = None;

        // Try each service according to failover policy
        for (_attempt, service) in healthy_services.iter().enumerate() {
            // Execute with timeout
            let timeout_duration = std::time::Duration::from_millis(self.ha_config.failover_timeout_ms);
            let result = tokio::time::timeout(timeout_duration, operation(service)).await;

            match result {
                Ok(Ok(response)) => {
                    self.record_success(&service.name);
                    self.release_bulkhead(service_name)?;
                    return Ok(response);
                }
                Ok(Err(_e)) => {
                    self.record_failure(&service.name);
                    last_error = Some(ResilienceAvailabilityError::GenericError(
                        "Service call failed".to_string(),
                    ));
                }
                Err(_) => {
                    self.record_failure(&service.name);
                    last_error = Some(ResilienceAvailabilityError::GenericError(
                        format!("Service call timeout for {}", service.name),
                    ));
                }
            }

            // Record failover event
            self.record_failover_event();
        }

        self.release_bulkhead(service_name)?;
        Err(last_error.unwrap_or(ResilienceAvailabilityError::GenericError(
            "All services failed".to_string(),
        )))
    }
}
