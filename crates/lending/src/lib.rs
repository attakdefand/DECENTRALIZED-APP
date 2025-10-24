//! Lending protocol implementation
//!
//! This module implements lending and borrowing functionality with interest rate models.

use core::types::{Address, TokenAmount};
use core::{Error, Result};
use serde::{Deserialize, Serialize};

/// Interest rate model parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterestRateModel {
    /// Base interest rate (annual percentage)
    pub base_rate: f64,
    /// Slope of the interest rate curve before the kink
    pub slope_1: f64,
    /// Slope of the interest rate curve after the kink
    pub slope_2: f64,
    /// Utilization rate at which the kink occurs
    pub kink: f64,
}

impl InterestRateModel {
    /// Calculate the borrow interest rate based on utilization
    pub fn calculate_borrow_rate(&self, utilization: f64) -> f64 {
        if utilization <= self.kink {
            self.base_rate + self.slope_1 * utilization
        } else {
            self.base_rate + self.slope_1 * self.kink + self.slope_2 * (utilization - self.kink)
        }
    }

    /// Calculate the supply interest rate based on utilization and borrow rate
    pub fn calculate_supply_rate(
        &self,
        utilization: f64,
        borrow_rate: f64,
        reserve_factor: f64,
    ) -> f64 {
        borrow_rate * utilization * (1.0 - reserve_factor)
    }
}

/// Represents a lending market for a specific token
#[derive(Debug, Clone)]
pub struct LendingMarket {
    pub token: Address,
    pub total_cash: TokenAmount,
    pub total_borrows: TokenAmount,
    pub total_reserves: TokenAmount,
    pub interest_rate_model: InterestRateModel,
    pub reserve_factor: f64, // Percentage of interest that goes to reserves
    pub last_updated: u64,   // Timestamp of last interest accrual
}

impl LendingMarket {
    /// Create a new lending market
    pub fn new(
        token: Address,
        interest_rate_model: InterestRateModel,
        reserve_factor: f64,
    ) -> Self {
        Self {
            token,
            total_cash: TokenAmount {
                value: 0,
                decimals: 18,
            },
            total_borrows: TokenAmount {
                value: 0,
                decimals: 18,
            },
            total_reserves: TokenAmount {
                value: 0,
                decimals: 18,
            },
            interest_rate_model,
            reserve_factor,
            last_updated: 0,
        }
    }

    /// Get the current utilization rate
    pub fn utilization_rate(&self) -> f64 {
        if self.total_cash.value + self.total_borrows.value == 0 {
            0.0
        } else {
            self.total_borrows.value as f64
                / (self.total_cash.value + self.total_borrows.value) as f64
        }
    }

    /// Get the current borrow interest rate (per second)
    pub fn borrow_rate_per_second(&self) -> f64 {
        let annual_rate = self
            .interest_rate_model
            .calculate_borrow_rate(self.utilization_rate());
        annual_rate / (365.0 * 24.0 * 60.0 * 60.0) // Convert annual to per second
    }

    /// Get the current supply interest rate (per second)
    pub fn supply_rate_per_second(&self) -> f64 {
        let borrow_rate = self.borrow_rate_per_second() * (365.0 * 24.0 * 60.0 * 60.0); // Annual rate
        let supply_annual_rate = self.interest_rate_model.calculate_supply_rate(
            self.utilization_rate(),
            borrow_rate,
            self.reserve_factor,
        );
        supply_annual_rate / (365.0 * 24.0 * 60.0 * 60.0) // Convert annual to per second
    }

    /// Accrue interest since last update
    pub fn accrue_interest(&mut self, current_timestamp: u64) -> Result<()> {
        let time_elapsed = current_timestamp - self.last_updated;
        if time_elapsed == 0 {
            return Ok(());
        }

        let borrow_rate_per_second = self.borrow_rate_per_second();
        let interest_accumulated =
            (self.total_borrows.value as f64) * (borrow_rate_per_second * time_elapsed as f64);

        self.total_borrows.value += interest_accumulated as u128;
        self.total_reserves.value += (interest_accumulated * self.reserve_factor) as u128;

        self.last_updated = current_timestamp;

        Ok(())
    }

    /// Deposit tokens into the lending market
    pub fn deposit(&mut self, amount: &TokenAmount) -> Result<()> {
        self.total_cash.value += amount.value;
        Ok(())
    }

    /// Withdraw tokens from the lending market
    pub fn withdraw(&mut self, amount: &TokenAmount) -> Result<()> {
        if self.total_cash.value < amount.value {
            return Err(Error::Custom("Insufficient cash in market".to_string()));
        }
        self.total_cash.value -= amount.value;
        Ok(())
    }

    /// Borrow tokens from the lending market
    pub fn borrow(&mut self, amount: &TokenAmount) -> Result<()> {
        if self.total_cash.value < amount.value {
            return Err(Error::Custom(
                "Insufficient cash available for borrowing".to_string(),
            ));
        }
        self.total_cash.value -= amount.value;
        self.total_borrows.value += amount.value;
        Ok(())
    }

    /// Repay borrowed tokens
    pub fn repay(&mut self, amount: &TokenAmount) -> Result<()> {
        self.total_borrows.value = self.total_borrows.value.saturating_sub(amount.value);
        self.total_cash.value += amount.value;
        Ok(())
    }
}

/// Represents a user's position in a lending market
#[derive(Debug, Clone)]
pub struct UserPosition {
    pub user: Address,
    pub market: Address,
    pub supplied_amount: TokenAmount,
    pub borrowed_amount: TokenAmount,
    pub collateral_amount: TokenAmount,
}

impl UserPosition {
    /// Create a new user position
    pub fn new(user: Address, market: Address) -> Self {
        Self {
            user,
            market,
            supplied_amount: TokenAmount {
                value: 0,
                decimals: 18,
            },
            borrowed_amount: TokenAmount {
                value: 0,
                decimals: 18,
            },
            collateral_amount: TokenAmount {
                value: 0,
                decimals: 18,
            },
        }
    }

    /// Calculate the health factor of the user's position
    /// Health factor = (collateral_value * liquidation_threshold) / borrowed_value
    pub fn health_factor(
        &self,
        collateral_value: f64,
        borrowed_value: f64,
        liquidation_threshold: f64,
    ) -> f64 {
        if borrowed_value == 0.0 {
            return f64::INFINITY;
        }
        (collateral_value * liquidation_threshold) / borrowed_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interest_rate_model() {
        let model = InterestRateModel {
            base_rate: 0.02, // 2% base rate
            slope_1: 0.1,    // 10% slope before kink
            slope_2: 0.5,    // 50% slope after kink
            kink: 0.8,       // 80% utilization kink
        };

        // Below kink
        let rate_low = model.calculate_borrow_rate(0.5);
        assert_eq!(rate_low, 0.02 + 0.1 * 0.5);

        // Above kink
        let rate_high = model.calculate_borrow_rate(0.9);
        assert_eq!(rate_high, 0.02 + 0.1 * 0.8 + 0.5 * 0.1);
    }

    #[test]
    fn test_lending_market() {
        let model = InterestRateModel {
            base_rate: 0.02,
            slope_1: 0.1,
            slope_2: 0.5,
            kink: 0.8,
        };

        let mut market = LendingMarket::new(
            Address("DAI".to_string()),
            model,
            0.1, // 10% reserve factor
        );

        assert_eq!(market.utilization_rate(), 0.0);
    }
}
