//! Risk management module for economic security
//!
//! This module implements risk parameters, fee routing, and insurance fund management
//! for the DECENTRALIZED-APP protocol.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Collateral factor for different asset types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollateralFactor {
    /// Asset type identifier
    pub asset: String,
    /// Collateral factor as a percentage (0-100)
    pub factor: u32,
    /// Maximum exposure limit
    pub max_exposure: u128,
}

impl CollateralFactor {
    pub fn new(asset: String, factor: u32, max_exposure: u128) -> Self {
        Self {
            asset,
            factor,
            max_exposure,
        }
    }
    
    /// Calculate maximum borrowable amount
    pub fn max_borrow(&self, collateral_value: u128) -> u128 {
        (collateral_value * self.factor as u128) / 100
    }
}

/// Liquidation threshold parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidationThreshold {
    /// Asset type identifier
    pub asset: String,
    /// Liquidation threshold as a percentage (0-100)
    pub threshold: u32,
    /// Liquidation penalty percentage
    pub penalty: u32,
    /// Minimum liquidation amount
    pub min_liquidation: u128,
}

impl LiquidationThreshold {
    pub fn new(asset: String, threshold: u32, penalty: u32, min_liquidation: u128) -> Self {
        Self {
            asset,
            threshold,
            penalty,
            min_liquidation,
        }
    }
    
    /// Check if position is liquidatable
    pub fn is_liquidatable(&self, collateral_value: u128, debt_value: u128) -> bool {
        if collateral_value == 0 {
            return false;
        }
        
        let ratio = (debt_value * 100) / collateral_value;
        ratio >= self.threshold as u128
    }
    
    /// Calculate liquidation penalty
    pub fn liquidation_penalty(&self, debt_value: u128) -> u128 {
        (debt_value * self.penalty as u128) / 100
    }
}

/// Liquidation ratio parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidationRatio {
    /// Asset pair identifier
    pub pair: String,
    /// Target liquidation ratio
    pub target_ratio: f64,
    /// Maximum allowed ratio
    pub max_ratio: f64,
    /// Recovery rate target
    pub recovery_rate: f64,
}

impl LiquidationRatio {
    pub fn new(pair: String, target_ratio: f64, max_ratio: f64, recovery_rate: f64) -> Self {
        Self {
            pair,
            target_ratio,
            max_ratio,
            recovery_rate,
        }
    }
    
    /// Check if liquidation should occur
    pub fn should_liquidate(&self, current_ratio: f64) -> bool {
        current_ratio >= self.target_ratio
    }
    
    /// Calculate required collateral to maintain ratio
    pub fn required_collateral(&self, debt: f64) -> f64 {
        debt / self.target_ratio
    }
}

/// Fee distribution configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeRouter {
    /// Insurance fund percentage
    pub insurance: f64,
    /// Treasury percentage
    pub treasury: f64,
    /// Staker rewards percentage
    pub stakers: f64,
    /// Development fund percentage
    pub development: f64,
}

impl FeeRouter {
    pub fn new(insurance: f64, treasury: f64, stakers: f64, development: f64) -> Result<Self, &'static str> {
        let total = insurance + treasury + stakers + development;
        if (total - 1.0).abs() > 0.001 {
            return Err("Fee percentages must sum to 100%");
        }
        
        Ok(Self {
            insurance,
            treasury,
            stakers,
            development,
        })
    }
    
    /// Distribute fees according to configuration
    pub fn distribute_fees(&self, total_fees: u128) -> FeeDistribution {
        FeeDistribution {
            insurance: (total_fees as f64 * self.insurance) as u128,
            treasury: (total_fees as f64 * self.treasury) as u128,
            stakers: (total_fees as f64 * self.stakers) as u128,
            development: (total_fees as f64 * self.development) as u128,
        }
    }
}

/// Fee distribution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeDistribution {
    pub insurance: u128,
    pub treasury: u128,
    pub stakers: u128,
    pub development: u128,
}

/// Insurance fund configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsuranceFund {
    /// Minimum fund size
    pub min_size: u128,
    /// Maximum payout per incident
    pub max_payout: u128,
    /// Coverage percentage
    pub coverage: f64,
    /// Claim processing time (in blocks)
    pub claim_time: u64,
}

impl InsuranceFund {
    pub fn new(min_size: u128, max_payout: u128, coverage: f64, claim_time: u64) -> Self {
        Self {
            min_size,
            max_payout,
            coverage,
            claim_time,
        }
    }
    
    /// Calculate coverage amount
    pub fn coverage_amount(&self, loss: u128) -> u128 {
        let covered = (loss as f64 * self.coverage) as u128;
        covered.min(self.max_payout)
    }
    
    /// Check if fund is adequately capitalized
    pub fn is_adequately_capitalized(&self, current_size: u128) -> bool {
        current_size >= self.min_size
    }
}

/// Insurance claim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsuranceClaim {
    pub id: String,
    pub user: String,
    pub loss_amount: u128,
    pub claim_amount: u128,
    pub timestamp: u64,
    pub status: ClaimStatus,
}

/// Claim status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClaimStatus {
    Pending,
    Approved,
    Rejected,
    Paid,
}

/// Economic scenario for simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicScenario {
    /// Scenario name
    pub name: String,
    /// Price volatility factor
    pub volatility: f64,
    /// Liquidity stress factor
    pub liquidity_stress: f64,
    /// Duration in blocks
    pub duration: u64,
    /// Expected impact level
    pub impact: ScenarioImpact,
}

/// Scenario impact level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScenarioImpact {
    Low,
    Medium,
    High,
    Critical,
}

impl EconomicScenario {
    pub fn new(name: String, volatility: f64, liquidity_stress: f64, duration: u64, impact: ScenarioImpact) -> Self {
        Self {
            name,
            volatility,
            liquidity_stress,
            duration,
            impact,
        }
    }
}

/// Risk monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskMonitor {
    /// Update frequency in blocks
    pub frequency: u64,
    /// Alert threshold for collateral ratios
    pub collateral_alert: f64,
    /// Alert threshold for liquidity ratios
    pub liquidity_alert: f64,
    /// Alert threshold for price volatility
    pub volatility_alert: f64,
}

impl RiskMonitor {
    pub fn new(frequency: u64, collateral_alert: f64, liquidity_alert: f64, volatility_alert: f64) -> Self {
        Self {
            frequency,
            collateral_alert,
            liquidity_alert,
            volatility_alert,
        }
    }
    
    /// Check if collateral ratio is concerning
    pub fn check_collateral_ratio(&self, ratio: f64) -> bool {
        ratio >= self.collateral_alert
    }
    
    /// Check if liquidity is concerning
    pub fn check_liquidity(&self, ratio: f64) -> bool {
        ratio <= self.liquidity_alert
    }
    
    /// Check if volatility is concerning
    pub fn check_volatility(&self, vol: f64) -> bool {
        vol >= self.volatility_alert
    }
}

/// Emergency procedures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyProcedures {
    /// Circuit breaker threshold
    pub circuit_breaker: f64,
    /// Pause duration in blocks
    pub pause_duration: u64,
    /// Recovery threshold
    pub recovery_threshold: f64,
}

impl EmergencyProcedures {
    pub fn new(circuit_breaker: f64, pause_duration: u64, recovery_threshold: f64) -> Self {
        Self {
            circuit_breaker,
            pause_duration,
            recovery_threshold,
        }
    }
    
    /// Check if circuit breaker should trigger
    pub fn should_trigger_circuit_breaker(&self, current_loss: f64) -> bool {
        current_loss >= self.circuit_breaker
    }
    
    /// Check if system can recover
    pub fn can_recover(&self, recovery_metric: f64) -> bool {
        recovery_metric >= self.recovery_threshold
    }
}

/// Risk management service
pub struct RiskManager {
    /// Collateral factors by asset
    pub collateral_factors: HashMap<String, CollateralFactor>,
    /// Liquidation thresholds by asset
    pub liquidation_thresholds: HashMap<String, LiquidationThreshold>,
    /// Liquidation ratios by pair
    pub liquidation_ratios: HashMap<String, LiquidationRatio>,
    /// Fee router configuration
    pub fee_router: FeeRouter,
    /// Insurance fund configuration
    pub insurance_fund: InsuranceFund,
    /// Risk monitoring configuration
    pub risk_monitor: RiskMonitor,
    /// Emergency procedures
    pub emergency_procedures: EmergencyProcedures,
    /// Economic scenarios for testing
    pub scenarios: Vec<EconomicScenario>,
}

impl RiskManager {
    /// Create a new risk manager with default configurations
    pub fn new() -> Result<Self, &'static str> {
        let fee_router = FeeRouter::new(0.30, 0.40, 0.20, 0.10)?;
        
        let mut collateral_factors = HashMap::new();
        collateral_factors.insert("ETH".to_string(), CollateralFactor::new("ETH".to_string(), 80, 1000000000000000000000000));
        collateral_factors.insert("BTC".to_string(), CollateralFactor::new("BTC".to_string(), 70, 500000000000000000000000));
        collateral_factors.insert("USDC".to_string(), CollateralFactor::new("USDC".to_string(), 90, 1000000000000000000000000000));
        
        let mut liquidation_thresholds = HashMap::new();
        liquidation_thresholds.insert("ETH".to_string(), LiquidationThreshold::new("ETH".to_string(), 85, 10, 1000000000000000000));
        liquidation_thresholds.insert("BTC".to_string(), LiquidationThreshold::new("BTC".to_string(), 80, 12, 100000000));
        liquidation_thresholds.insert("USDC".to_string(), LiquidationThreshold::new("USDC".to_string(), 95, 5, 1000000000));
        
        let mut liquidation_ratios = HashMap::new();
        liquidation_ratios.insert("ETH/USD".to_string(), LiquidationRatio::new("ETH/USD".to_string(), 0.85, 0.90, 0.95));
        liquidation_ratios.insert("BTC/USD".to_string(), LiquidationRatio::new("BTC/USD".to_string(), 0.80, 0.85, 0.90));
        liquidation_ratios.insert("USDC/USD".to_string(), LiquidationRatio::new("USDC/USD".to_string(), 0.95, 0.98, 0.99));
        
        let scenarios = vec![
            EconomicScenario::new("Bull Market".to_string(), 0.02, 0.8, 28800, ScenarioImpact::Low),
            EconomicScenario::new("Bear Market".to_string(), 0.05, 0.6, 28800, ScenarioImpact::Medium),
            EconomicScenario::new("Flash Crash".to_string(), 0.15, 0.3, 1440, ScenarioImpact::High),
            EconomicScenario::new("Black Swan".to_string(), 0.30, 0.1, 720, ScenarioImpact::Critical),
        ];
        
        Ok(Self {
            collateral_factors,
            liquidation_thresholds,
            liquidation_ratios,
            fee_router,
            insurance_fund: InsuranceFund::new(1000000000000000000000000, 100000000000000000000000, 0.80, 7200),
            risk_monitor: RiskMonitor::new(100, 0.80, 0.20, 0.10),
            emergency_procedures: EmergencyProcedures::new(0.05, 28800, 0.95),
            scenarios,
        })
    }
    
    /// Get collateral factor for an asset
    pub fn get_collateral_factor(&self, asset: &str) -> Option<&CollateralFactor> {
        self.collateral_factors.get(asset)
    }
    
    /// Get liquidation threshold for an asset
    pub fn get_liquidation_threshold(&self, asset: &str) -> Option<&LiquidationThreshold> {
        self.liquidation_thresholds.get(asset)
    }
    
    /// Get liquidation ratio for a pair
    pub fn get_liquidation_ratio(&self, pair: &str) -> Option<&LiquidationRatio> {
        self.liquidation_ratios.get(pair)
    }
    
    /// Distribute fees according to router configuration
    pub fn distribute_fees(&self, total_fees: u128) -> FeeDistribution {
        self.fee_router.distribute_fees(total_fees)
    }
    
    /// Calculate insurance coverage for a loss
    pub fn calculate_coverage(&self, loss: u128) -> u128 {
        self.insurance_fund.coverage_amount(loss)
    }
    
    /// Check if a position is liquidatable
    pub fn is_liquidatable(&self, asset: &str, collateral_value: u128, debt_value: u128) -> bool {
        if let Some(threshold) = self.get_liquidation_threshold(asset) {
            threshold.is_liquidatable(collateral_value, debt_value)
        } else {
            false
        }
    }
    
    /// Check if risk monitors should trigger alerts
    pub fn check_risk_alerts(&self, collateral_ratio: f64, liquidity_ratio: f64, volatility: f64) -> Vec<String> {
        let mut alerts = Vec::new();
        
        if self.risk_monitor.check_collateral_ratio(collateral_ratio) {
            alerts.push("High collateral ratio detected".to_string());
        }
        
        if self.risk_monitor.check_liquidity(liquidity_ratio) {
            alerts.push("Low liquidity detected".to_string());
        }
        
        if self.risk_monitor.check_volatility(volatility) {
            alerts.push("High volatility detected".to_string());
        }
        
        alerts
    }
    
    /// Check if emergency procedures should be triggered
    pub fn check_emergency(&self, current_loss: f64, recovery_metric: f64) -> Option<String> {
        if self.emergency_procedures.should_trigger_circuit_breaker(current_loss) {
            return Some("Circuit breaker triggered due to excessive losses".to_string());
        }
        
        if self.emergency_procedures.can_recover(recovery_metric) {
            return Some("System can recover from current state".to_string());
        }
        
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collateral_factor_calculation() {
        let cf = CollateralFactor::new("ETH".to_string(), 80, 1000000000000000000000000);
        let borrowable = cf.max_borrow(100000000000000000000); // 100 ETH
        assert_eq!(borrowable, 80000000000000000000); // 80 ETH
    }

    #[test]
    fn test_liquidation_threshold_check() {
        let lt = LiquidationThreshold::new("ETH".to_string(), 85, 10, 1000000000000000000);
        assert!(lt.is_liquidatable(100000000000000000000, 85000000000000000000)); // 100 ETH collateral, 85 ETH debt
        assert!(!lt.is_liquidatable(100000000000000000000, 84000000000000000000)); // 100 ETH collateral, 84 ETH debt
    }

    #[test]
    fn test_fee_router_distribution() {
        let router = FeeRouter::new(0.30, 0.40, 0.20, 0.10).unwrap();
        let distribution = router.distribute_fees(100000000000000000000); // 100 tokens
        
        assert_eq!(distribution.insurance, 30000000000000000000); // 30 tokens
        assert_eq!(distribution.treasury, 40000000000000000000); // 40 tokens
        assert_eq!(distribution.stakers, 20000000000000000000); // 20 tokens
        assert_eq!(distribution.development, 10000000000000000000); // 10 tokens
    }

    #[test]
    fn test_insurance_coverage() {
        let fund = InsuranceFund::new(1000000000000000000000000, 100000000000000000000000, 0.80, 7200);
        let coverage = fund.coverage_amount(100000000000000000000); // 100 token loss
        
        assert_eq!(coverage, 80000000000000000000); // 80 tokens covered
    }

    #[test]
    fn test_risk_manager_creation() {
        let risk_manager = RiskManager::new().unwrap();
        
        assert!(risk_manager.get_collateral_factor("ETH").is_some());
        assert!(risk_manager.get_liquidation_threshold("BTC").is_some());
        assert!(risk_manager.get_liquidation_ratio("ETH/USD").is_some());
    }

    #[test]
    fn test_risk_alerts() {
        let risk_manager = RiskManager::new().unwrap();
        
        let alerts = risk_manager.check_risk_alerts(0.85, 0.15, 0.15);
        assert_eq!(alerts.len(), 3); // All three alerts should trigger
    }
}