//! Oracle implementations for price feeds
//!
//! This module provides various oracle implementations:
//! - Medianizer for aggregating multiple feeds
//! - TWAP (Time Weighted Average Price) calculator

use core::Result;
use core::Error;
use core::types::{Address, TokenAmount};

/// Price feed structure
#[derive(Debug, Clone)]
pub struct PriceFeed {
    pub source: String,
    pub base_token: Address,
    pub quote_token: Address,
    pub price: f64,
    pub timestamp: u64,
    pub confidence: f64,
}

/// Medianizer oracle
///
/// Aggregates multiple price feeds and returns the median price
pub struct Medianizer {
    pub base_token: Address,
    pub quote_token: Address,
    pub feeds: Vec<PriceFeed>,
}

impl Medianizer {
    /// Create a new medianizer
    pub fn new(base_token: Address, quote_token: Address) -> Self {
        Self {
            base_token,
            quote_token,
            feeds: Vec::new(),
        }
    }
    
    /// Add a price feed
    pub fn add_feed(&mut self, feed: PriceFeed) {
        self.feeds.push(feed);
    }
    
    /// Get the median price from all feeds
    pub fn median_price(&self) -> Result<f64> {
        if self.feeds.is_empty() {
            return Err(Error::Custom("No price feeds available".to_string()));
        }
        
        let mut prices: Vec<f64> = self.feeds.iter().map(|feed| feed.price).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let len = prices.len();
        let median = if len % 2 == 0 {
            (prices[len / 2 - 1] + prices[len / 2]) / 2.0
        } else {
            prices[len / 2]
        };
        
        Ok(median)
    }
}

/// TWAP (Time Weighted Average Price) calculator
pub struct TwapCalculator {
    pub prices: Vec<(f64, u64)>, // (price, timestamp)
    pub window: u64, // in seconds
}

impl TwapCalculator {
    /// Create a new TWAP calculator
    pub fn new(window: u64) -> Self {
        Self {
            prices: Vec::new(),
            window,
        }
    }
    
    /// Add a price observation
    pub fn add_price(&mut self, price: f64, timestamp: u64) {
        self.prices.push((price, timestamp));
        
        // Remove old observations outside the window
        self.prices.retain(|(_, ts)| timestamp - ts <= self.window);
    }
    
    /// Calculate the TWAP
    pub fn calculate_twap(&self) -> Result<f64> {
        if self.prices.is_empty() {
            return Err(Error::Custom("No price observations available".to_string()));
        }
        
        // Sort prices by timestamp
        let mut sorted_prices = self.prices.clone();
        sorted_prices.sort_by(|a, b| a.1.cmp(&b.1));
        
        let mut weighted_sum = 0.0;
        let mut total_time = 0.0;
        
        for i in 1..sorted_prices.len() {
            let (price, timestamp) = sorted_prices[i];
            let (prev_price, prev_timestamp) = sorted_prices[i - 1];
            
            let time_diff = (timestamp - prev_timestamp) as f64;
            // Use the previous price for the time interval
            weighted_sum += prev_price * time_diff;
            total_time += time_diff;
        }
        
        if total_time == 0.0 {
            Ok(sorted_prices.last().unwrap().0)
        } else {
            Ok(weighted_sum / total_time)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_medianizer() {
        let mut medianizer = Medianizer::new(
            Address("ETH".to_string()),
            Address("USD".to_string()),
        );
        
        medianizer.add_feed(PriceFeed {
            source: "source1".to_string(),
            base_token: Address("ETH".to_string()),
            quote_token: Address("USD".to_string()),
            price: 3000.0,
            timestamp: 1234567890,
            confidence: 0.9,
        });
        
        medianizer.add_feed(PriceFeed {
            source: "source2".to_string(),
            base_token: Address("ETH".to_string()),
            quote_token: Address("USD".to_string()),
            price: 3100.0,
            timestamp: 1234567891,
            confidence: 0.8,
        });
        
        medianizer.add_feed(PriceFeed {
            source: "source3".to_string(),
            base_token: Address("ETH".to_string()),
            quote_token: Address("USD".to_string()),
            price: 2900.0,
            timestamp: 1234567892,
            confidence: 0.7,
        });
        
        let median_price = medianizer.median_price().unwrap();
        assert_eq!(median_price, 3000.0);
    }
    
    #[test]
    fn test_twap() {
        let mut twap = TwapCalculator::new(3600); // 1 hour window
        
        let now = 1234567890;
        twap.add_price(100.0, now);
        twap.add_price(110.0, now + 1800); // 30 minutes later
        twap.add_price(120.0, now + 3600); // 1 hour later
        
        let twap_price = twap.calculate_twap().unwrap();
        // TWAP should be weighted average:
        // First 30 min: 100.0
        // Next 30 min: 110.0
        // TWAP = (100.0 * 30 + 110.0 * 30) / 60 = 105.0
        assert_eq!(twap_price, 105.0);
    }
}