//! Central Limit Order Book implementation
//!
//! This module provides a central limit order book with price-time priority.

use core::types::{Address, TokenAmount};
use core::Result;
use std::collections::BTreeMap;

/// Order side (buy or sell)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

/// Order structure
#[derive(Debug, Clone)]
pub struct Order {
    pub id: String,
    pub trader: Address,
    pub base_token: Address,
    pub quote_token: Address,
    pub side: Side,
    pub price: f64,
    pub amount: TokenAmount,
    pub timestamp: u64,
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.trader == other.trader
            && self.base_token == other.base_token
            && self.quote_token == other.quote_token
            && self.side == other.side
            && self.price == other.price
            && self.timestamp == other.timestamp
        // Note: We're not comparing amount because TokenAmount doesn't implement PartialEq
    }
}

/// Order book for a specific token pair
pub struct OrderBook {
    pub base_token: Address,
    pub quote_token: Address,
    pub bids: BTreeMap<u64, Vec<Order>>, // price -> orders (sorted by timestamp)
    pub asks: BTreeMap<u64, Vec<Order>>, // price -> orders (sorted by timestamp)
}

impl OrderBook {
    /// Create a new order book
    pub fn new(base_token: Address, quote_token: Address) -> Self {
        Self {
            base_token,
            quote_token,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    /// Add an order to the book
    pub fn add_order(&mut self, order: Order) -> Result<()> {
        let price_key = (order.price * 1_000_000.0) as u64; // Convert to integer for sorting

        match order.side {
            Side::Buy => {
                self.bids
                    .entry(price_key)
                    .or_insert_with(Vec::new)
                    .push(order);
            }
            Side::Sell => {
                self.asks
                    .entry(price_key)
                    .or_insert_with(Vec::new)
                    .push(order);
            }
        }

        Ok(())
    }

    /// Get the best bid (highest buy price)
    pub fn best_bid(&self) -> Option<&Order> {
        self.bids
            .iter()
            .rev() // Reverse to get highest price first
            .flat_map(|(_, orders)| orders.iter())
            .next()
    }

    /// Get the best ask (lowest sell price)
    pub fn best_ask(&self) -> Option<&Order> {
        self.asks
            .iter()
            .flat_map(|(_, orders)| orders.iter())
            .next()
    }

    /// Match orders and execute trades
    pub fn match_orders(&mut self) -> Result<Vec<Trade>> {
        let trades = Vec::new();

        // In a real implementation, this would:
        // 1. Match buy and sell orders based on price-time priority
        // 2. Execute trades
        // 3. Remove filled orders from the book
        // 4. Update trader balances

        Ok(trades)
    }
}

/// Trade execution
#[derive(Debug, Clone)]
pub struct Trade {
    pub id: String,
    pub base_token: Address,
    pub quote_token: Address,
    pub price: f64,
    pub amount: TokenAmount,
    pub buyer: Address,
    pub seller: Address,
    pub timestamp: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_book() {
        let orderbook = OrderBook::new(Address("ETH".to_string()), Address("USDC".to_string()));

        assert!(orderbook.best_bid().is_none());
        assert!(orderbook.best_ask().is_none());
    }
}
