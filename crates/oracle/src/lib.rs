//! Oracle service for DECENTRALIZED-APP
//!
//! This crate provides oracle adapters, price aggregation, and integrity monitoring.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread::sleep;
use std::time::Duration;

/// Oracle adapter trait
pub trait OracleAdapter: Send + Sync {
    /// Get the latest price for a pair
    fn get_price(&self, pair: &str) -> Result<PriceData, OracleError>;

    /// Get historical prices
    fn get_historical_prices(
        &self,
        pair: &str,
        count: usize,
    ) -> Result<Vec<PriceData>, OracleError>;

    /// Check oracle health
    fn is_healthy(&self) -> bool;
    
    /// Get the adapter identifier
    fn get_id(&self) -> String;
}

/// Price data structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PriceData {
    pub pair: String,
    pub price: u128,
    pub timestamp: u64,
    pub confidence: u64,
    pub oracle_provider: String,
}

/// Oracle error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OracleError {
    NetworkError(String),
    DataError(String),
    TimeoutError,
    ValidationError(String),
}

impl std::fmt::Display for OracleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OracleError::NetworkError(e) => write!(f, "Network error: {}", e),
            OracleError::DataError(e) => write!(f, "Data error: {}", e),
            OracleError::TimeoutError => write!(f, "Timeout error"),
            OracleError::ValidationError(e) => write!(f, "Validation error: {}", e),
        }
    }
}

impl std::error::Error for OracleError {}

/// Publisher key structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublisherKey {
    pub id: String,
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
    pub last_used: u64,
    pub is_active: bool,
}

/// Publisher key manager
pub struct PublisherKeyManager {
    /// Active publisher keys
    pub active_keys: HashMap<String, PublisherKey>,
    /// Emergency key for backup
    pub emergency_key: PublisherKey,
    /// Key rotation interval (in seconds)
    pub key_rotation_interval: u64,
}

impl PublisherKeyManager {
    /// Create a new publisher key manager
    pub fn new() -> Self {
        Self {
            active_keys: HashMap::new(),
            emergency_key: PublisherKey {
                id: "emergency".to_string(),
                public_key: vec![0; 32],
                private_key: vec![0; 32],
                last_used: 0,
                is_active: false,
            },
            key_rotation_interval: 86400, // 24 hours
        }
    }

    /// Add a publisher key
    pub fn add_key(&mut self, key: PublisherKey) -> Result<(), OracleError> {
        if key.public_key.len() != 32 || key.private_key.len() != 32 {
            return Err(OracleError::DataError(
                "Invalid key length".to_string(),
            ));
        }
        self.active_keys.insert(key.id.clone(), key);
        Ok(())
    }

    /// Validate a price feed signature
    pub fn validate_signature(&self, data: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
        // Simple validation - in a real implementation, we would use cryptographic verification
        let len = data.len();
        
        // Check if data length is even (simplified validation)
        if len.is_multiple_of(2) {
            signature.len() == 64 && public_key.len() == 32
        } else {
            false
        }
    }
}

// Add Default implementation
impl Default for PublisherKeyManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Connector configuration for external oracle services
#[derive(Debug, Clone)]
pub struct ConnectorConfig {
    /// Connector identifier
    pub id: String,
    /// Base URL for the oracle service
    pub base_url: String,
    /// API key for authentication
    pub api_key: String,
    /// Timeout in seconds
    pub timeout: u64,
    /// Retry configuration
    pub retry_config: RetryConfig,
    /// Allowlist of approved connectors
    pub is_allowed: bool,
}

/// Retry configuration for oracle connectors
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Initial backoff time in milliseconds
    pub initial_backoff_ms: u64,
    /// Maximum backoff time in milliseconds
    pub max_backoff_ms: u64,
    /// Backoff multiplier
    pub backoff_multiplier: f64,
}

/// Oracle connector with retry/backoff mechanisms
pub struct OracleConnector {
    /// Connector configuration
    config: ConnectorConfig,
    /// Failure count for backoff calculation
    failure_count: u32,
}

impl OracleConnector {
    /// Create a new oracle connector
    pub fn new(config: ConnectorConfig) -> Self {
        Self {
            config,
            failure_count: 0,
        }
    }
    
    /// Execute a request with retry/backoff logic
    pub fn execute_with_retry<T, F>(&mut self, mut operation: F) -> Result<T, OracleError>
    where
        F: FnMut() -> Result<T, OracleError>,
    {
        let mut attempts = 0;
        let mut last_error = None;
        
        while attempts < self.config.retry_config.max_attempts {
            match operation() {
                Ok(result) => {
                    // Reset failure count on success
                    self.failure_count = 0;
                    return Ok(result);
                }
                Err(e) => {
                    last_error = Some(e);
                    attempts += 1;
                    self.failure_count += 1;
                    
                    if attempts < self.config.retry_config.max_attempts {
                        // Calculate backoff time
                        let backoff_time = self.calculate_backoff();
                        sleep(Duration::from_millis(backoff_time));
                    }
                }
            }
        }
        
        // If we get here, all attempts failed
        Err(last_error.unwrap_or(OracleError::DataError("Unknown error".to_string())))
    }
    
    /// Calculate backoff time based on failure count
    fn calculate_backoff(&self) -> u64 {
        let backoff = (self.config.retry_config.initial_backoff_ms as f64
            * self.config.retry_config.backoff_multiplier.powi(self.failure_count as i32))
            as u64;
            
        // Cap at maximum backoff
        backoff.min(self.config.retry_config.max_backoff_ms)
    }
    
    /// Check if this connector is allowed
    pub fn is_allowed(&self) -> bool {
        self.config.is_allowed
    }
    
    /// Get connector ID
    pub fn get_id(&self) -> &str {
        &self.config.id
    }
}

/// Price aggregator for TWAP and median calculations with enhanced deviation checking
pub struct PriceAggregator {
    /// Time window for TWAP calculation (in seconds)
    pub twap_window: u64,
    /// Number of oracles required for median
    pub min_oracles: usize,
    /// Maximum price deviation allowed
    pub max_deviation: f64,
    /// Oracle adapters
    pub oracles: Vec<Box<dyn OracleAdapter>>,
    /// Connector allowlist
    pub connector_allowlist: Vec<String>,
    /// Enhanced deviation checking parameters
    pub deviation_config: DeviationConfig,
}

/// Configuration for enhanced deviation checking
#[derive(Debug, Clone)]
pub struct DeviationConfig {
    /// Standard deviation multiplier for outlier detection
    pub std_dev_multiplier: f64,
    /// Maximum allowed percentage change between consecutive readings
    pub max_percentage_change: f64,
    /// Minimum number of data points required for statistical analysis
    pub min_data_points: usize,
}

impl PriceAggregator {
    pub fn new(twap_window: u64, min_oracles: usize, max_deviation: f64) -> Self {
        Self {
            twap_window,
            min_oracles,
            max_deviation,
            oracles: Vec::new(),
            connector_allowlist: Vec::new(),
            deviation_config: DeviationConfig {
                std_dev_multiplier: 2.0,
                max_percentage_change: 5.0, // 5% maximum change
                min_data_points: 3,
            },
        }
    }

    /// Add an oracle adapter
    pub fn add_oracle(&mut self, oracle: Box<dyn OracleAdapter>) {
        self.oracles.push(oracle);
    }

    /// Add a connector to the allowlist
    pub fn add_to_allowlist(&mut self, connector_id: String) {
        if !self.connector_allowlist.contains(&connector_id) {
            self.connector_allowlist.push(connector_id);
        }
    }

    /// Check if a connector is in the allowlist
    pub fn is_connector_allowed(&self, connector_id: &str) -> bool {
        self.connector_allowlist.contains(&connector_id.to_string())
    }

    /// Calculate TWAP for a pair
    pub fn calculate_twap(&self, pair: &str) -> Result<PriceData, OracleError> {
        let mut prices = Vec::new();
        let mut timestamps = Vec::new();

        // Collect prices from all healthy and allowed oracles
        for oracle in &self.oracles {
            let oracle_id = oracle.get_id();
            if !self.is_connector_allowed(&oracle_id) {
                continue; // Skip non-allowed connectors
            }
            
            if oracle.is_healthy() {
                match oracle.get_historical_prices(pair, (self.twap_window / 60) as usize) {
                    Ok(historical_prices) => {
                        for price_data in historical_prices {
                            prices.push(price_data.price as f64);
                            timestamps.push(price_data.timestamp as f64);
                        }
                    }
                    Err(_) => continue, // Skip unhealthy oracle
                }
            }
        }

        if prices.is_empty() {
            return Err(OracleError::DataError(
                "No valid prices available".to_string(),
            ));
        }

        // Calculate TWAP
        let twap = self.calculate_time_weighted_average(&prices, &timestamps);

        Ok(PriceData {
            pair: pair.to_string(),
            price: twap as u128,
            timestamp: current_timestamp(),
            confidence: self.calculate_confidence(&prices),
            oracle_provider: "TWAP".to_string(),
        })
    }

    /// Calculate median price from multiple oracles
    pub fn calculate_median(&self, pair: &str) -> Result<PriceData, OracleError> {
        let mut prices = Vec::new();

        // Collect latest prices from all healthy and allowed oracles
        for oracle in &self.oracles {
            let oracle_id = oracle.get_id();
            if !self.is_connector_allowed(&oracle_id) {
                continue; // Skip non-allowed connectors
            }
            
            if oracle.is_healthy() {
                match oracle.get_price(pair) {
                    Ok(price_data) => {
                        prices.push(price_data.price as f64);
                    }
                    Err(_) => continue, // Skip unhealthy oracle
                }
            }
        }

        if prices.len() < self.min_oracles {
            return Err(OracleError::DataError(
                "Insufficient oracle responses".to_string(),
            ));
        }

        // Check for outliers using enhanced deviation checking
        if self.detect_outliers_enhanced(&prices) {
            return Err(OracleError::ValidationError(
                "Outliers detected".to_string(),
            ));
        }

        Ok(PriceData {
            pair: pair.to_string(),
            price: self.calculate_median_value(&mut prices.clone()) as u128,
            timestamp: current_timestamp(),
            confidence: self.calculate_confidence(&prices),
            oracle_provider: "Median".to_string(),
        })
    }

    /// Calculate time-weighted average
    fn calculate_time_weighted_average(&self, prices: &[f64], timestamps: &[f64]) -> f64 {
        if prices.is_empty() {
            return 0.0;
        }

        let mut weighted_sum = 0.0;
        let mut time_sum = 0.0;

        for i in 0..prices.len() - 1 {
            let time_diff = timestamps[i + 1] - timestamps[i];
            weighted_sum += prices[i] * time_diff;
            time_sum += time_diff;
        }

        if time_sum == 0.0 {
            prices[0]
        } else {
            weighted_sum / time_sum
        }
    }

    /// Calculate median value
    fn calculate_median_value(&self, prices: &mut [f64]) -> f64 {
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = prices.len();

        if len.is_multiple_of(2) {
            (prices[len / 2 - 1] + prices[len / 2]) / 2.0
        } else {
            prices[len / 2]
        }
    }

    /// Enhanced outlier detection using multiple methods
    fn detect_outliers_enhanced(&self, prices: &[f64]) -> bool {
        if prices.len() < self.deviation_config.min_data_points {
            return false; // Not enough data points for statistical analysis
        }

        // Method 1: Standard deviation based detection
        let mean: f64 = prices.iter().sum::<f64>() / prices.len() as f64;
        let variance: f64 =
            prices.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / prices.len() as f64;
        let std_dev = variance.sqrt();

        // Check if any price is beyond std_dev_multiplier standard deviations
        let std_dev_outlier = prices
            .iter()
            .any(|&price| (price - mean).abs() > std_dev * self.deviation_config.std_dev_multiplier);

        if std_dev_outlier {
            return true;
        }

        // Method 2: Percentage change detection between consecutive prices
        for i in 1..prices.len() {
            let percentage_change = ((prices[i] - prices[i - 1]) / prices[i - 1]).abs() * 100.0;
            if percentage_change > self.deviation_config.max_percentage_change {
                return true;
            }
        }

        false
    }

    /// Calculate confidence score
    fn calculate_confidence(&self, prices: &[f64]) -> u64 {
        if prices.is_empty() {
            return 0;
        }

        let mean: f64 = prices.iter().sum::<f64>() / prices.len() as f64;
        let variance: f64 =
            prices.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / prices.len() as f64;
        let std_dev = variance.sqrt();

        // Convert to confidence score (0-100)
        let cv = std_dev / mean; // Coefficient of variation
        let confidence = (1.0 - cv.min(1.0)) * 100.0;
        confidence as u64
    }
}

/// Oracle integrity tests
pub struct OracleIntegrityTests {
    /// Maximum allowed price change per minute
    pub max_price_change: f64,
    /// Maximum allowed staleness (in seconds)
    pub max_staleness: u64,
    /// Minimum confidence threshold
    pub min_confidence: u64,
}

impl OracleIntegrityTests {
    pub fn new(max_price_change: f64, max_staleness: u64, min_confidence: u64) -> Self {
        Self {
            max_price_change,
            max_staleness,
            min_confidence,
        }
    }

    /// Test for price manipulation
    pub fn test_price_manipulation(
        &self,
        current_price: &PriceData,
        previous_price: &PriceData,
    ) -> bool {
        if current_price.timestamp <= previous_price.timestamp {
            return false; // Invalid timestamp
        }

        let time_diff = current_price.timestamp - previous_price.timestamp;
        if time_diff == 0 {
            return false; // Same timestamp
        }

        let price_change = ((current_price.price as f64 - previous_price.price as f64)
            / previous_price.price as f64)
            .abs();
        let change_per_minute = price_change / (time_diff as f64 / 60.0);

        change_per_minute <= self.max_price_change
    }

    /// Test for data staleness
    pub fn test_data_staleness(&self, price_data: &PriceData) -> bool {
        let current_time = current_timestamp();
        let staleness = current_time - price_data.timestamp;
        staleness <= self.max_staleness
    }

    /// Test for confidence level
    pub fn test_confidence(&self, price_data: &PriceData) -> bool {
        price_data.confidence >= self.min_confidence
    }

    /// Run all integrity tests
    pub fn run_integrity_tests(
        &self,
        current_price: &PriceData,
        previous_price: Option<&PriceData>,
    ) -> Vec<String> {
        let mut failures = Vec::new();

        // Test staleness
        if !self.test_data_staleness(current_price) {
            failures.push("Data is stale".to_string());
        }

        // Test confidence
        if !self.test_confidence(current_price) {
            failures.push("Confidence level too low".to_string());
        }

        // Test manipulation if previous price is available
        if let Some(prev_price) = previous_price {
            if !self.test_price_manipulation(current_price, prev_price) {
                failures.push("Price manipulation detected".to_string());
            }
        }

        failures
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
    fn test_publisher_key_manager() {
        let mut key_manager = PublisherKeyManager::new();

        let key = PublisherKey {
            id: "test_key".to_string(),
            public_key: vec![1; 32],
            private_key: vec![2; 32],
            last_used: 1000,
            is_active: true,
        };

        assert!(key_manager.add_key(key).is_ok());
        assert!(key_manager.validate_signature(&[1, 2, 3, 4], &[1; 64], &[1; 32]));
    }

    #[test]
    fn test_price_aggregator() {
        let mut aggregator = PriceAggregator::new(3600, 3, 2.0); // 1 hour window, 3 oracles min, 2 std dev max
        
        // Add a connector to allowlist
        aggregator.add_to_allowlist("test_connector".to_string());
        assert!(aggregator.is_connector_allowed("test_connector"));
        assert!(!aggregator.is_connector_allowed("unknown_connector"));
    }

    #[test]
    fn test_integrity_tests() {
        let tests = OracleIntegrityTests::new(0.05, 300, 80); // 5% max change, 5min max staleness, 80 min confidence

        let current = PriceData {
            pair: "ETH/USD".to_string(),
            price: 3000000000000000000000, // $3000
            timestamp: 1000,
            confidence: 95,
            oracle_provider: "Test".to_string(),
        };

        let previous = PriceData {
            pair: "ETH/USD".to_string(),
            price: 2000000000000000000000, // $2000
            timestamp: 900,
            confidence: 95,
            oracle_provider: "Test".to_string(),
        };

        // This should detect manipulation (50% change in 100 seconds)
        assert!(!tests.test_price_manipulation(&current, &previous));
    }
    
    #[test]
    fn test_connector_config() {
        let retry_config = RetryConfig {
            max_attempts: 3,
            initial_backoff_ms: 100,
            max_backoff_ms: 1000,
            backoff_multiplier: 2.0,
        };
        
        let config = ConnectorConfig {
            id: "test_connector".to_string(),
            base_url: "https://api.example.com".to_string(),
            api_key: "test_key".to_string(),
            timeout: 30,
            retry_config,
            is_allowed: true,
        };
        
        let mut connector = OracleConnector::new(config);
        assert_eq!(connector.get_id(), "test_connector");
        assert!(connector.is_allowed());
    }
    
    #[test]
    fn test_enhanced_deviation_detection() {
        let mut aggregator = PriceAggregator::new(3600, 3, 2.0);
        // Set min_data_points to 2 for this test
        aggregator.deviation_config.min_data_points = 2;
        
        // Test with normal prices
        let normal_prices = vec![100.0, 101.0, 99.5, 100.5, 100.2];
        assert!(!aggregator.detect_outliers_enhanced(&normal_prices));
        
        // Test with outlier prices
        let outlier_prices = vec![100.0, 101.0, 99.5, 100.5, 150.0]; // 150 is an outlier
        assert!(aggregator.detect_outliers_enhanced(&outlier_prices));
        
        // Test with large percentage change - this should be detected by percentage change detection
        let large_change_prices = vec![100.0, 300.0]; // 200% change, which exceeds the 5% threshold
        assert!(aggregator.detect_outliers_enhanced(&large_change_prices));
    }
}