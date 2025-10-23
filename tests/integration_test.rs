//! Integration tests for the decentralized application

use core::types::{Address, TokenAmount};

#[test]
fn test_cpmm_swap() {
    let pool = amm::cpmm::Pool::new(
        Address("token_a".to_string()),
        Address("token_b".to_string()),
        TokenAmount { value: 1000000, decimals: 18 },
        TokenAmount { value: 1000000, decimals: 18 },
        0.3, // 0.3% fee
    );
    
    let amount_in = TokenAmount { value: 1000, decimals: 18 };
    let amount_out = pool.get_amount_out(&amount_in, &Address("token_a".to_string())).unwrap();
    
    assert!(amount_out.value > 0);
    assert!(amount_out.value < amount_in.value);
}

#[test]
fn test_order_book() {
    let orderbook = orderbook::OrderBook::new(
        Address("ETH".to_string()),
        Address("USDC".to_string()),
    );
    
    assert_eq!(orderbook.best_bid(), None);
    assert_eq!(orderbook.best_ask(), None);
}

#[test]
fn test_lending_market() {
    let model = lending::InterestRateModel {
        base_rate: 0.02,
        slope_1: 0.1,
        slope_2: 0.5,
        kink: 0.8,
    };
    
    let market = lending::LendingMarket::new(
        Address("DAI".to_string()),
        model,
        0.1, // 10% reserve factor
    );
    
    assert_eq!(market.utilization_rate(), 0.0);
}

#[test]
fn test_medianizer() {
    let mut medianizer = oracle::Medianizer::new(
        Address("ETH".to_string()),
        Address("USD".to_string()),
    );
    
    medianizer.add_feed(oracle::PriceFeed {
        source: "source1".to_string(),
        base_token: Address("ETH".to_string()),
        quote_token: Address("USD".to_string()),
        price: 3000.0,
        timestamp: 1234567890,
        confidence: 0.9,
    });
    
    medianizer.add_feed(oracle::PriceFeed {
        source: "source2".to_string(),
        base_token: Address("ETH".to_string()),
        quote_token: Address("USD".to_string()),
        price: 3100.0,
        timestamp: 1234567891,
        confidence: 0.8,
    });
    
    medianizer.add_feed(oracle::PriceFeed {
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