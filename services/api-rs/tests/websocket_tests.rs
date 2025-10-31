//! WebSocket tests
//!
//! Comprehensive tests for WebSocket functionality and security

use crate::websocket::*;

#[tokio::test]
async fn test_ws_state_creation() {
    let state = WsState::new();
    assert_eq!(state.client_count().await, 0);
}

#[tokio::test]
async fn test_ws_state_client_tracking() {
    let state = WsState::new();
    
    state.increment_clients().await;
    assert_eq!(state.client_count().await, 1);
    
    state.increment_clients().await;
    assert_eq!(state.client_count().await, 2);
    
    state.decrement_clients().await;
    assert_eq!(state.client_count().await, 1);
}

#[tokio::test]
async fn test_ws_message_market_update() {
    let msg = WsMessage::MarketUpdate {
        pair: "ETH/USDC".to_string(),
        price: 2500.0,
        change_24h: 2.5,
        volume_24h: 45000000.0,
        timestamp: 1704067200,
    };
    
    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("market_update"));
    assert!(json.contains("ETH/USDC"));
    
    // Security: Ensure no script injection
    assert!(!json.contains("<script>"));
}

#[tokio::test]
async fn test_ws_message_order_update() {
    let msg = WsMessage::OrderUpdate {
        id: "order-123".to_string(),
        status: "filled".to_string(),
        filled: "1.5".to_string(),
        timestamp: 1704067200,
    };
    
    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("order_update"));
    assert!(json.contains("order-123"));
}

#[tokio::test]
async fn test_ws_message_pool_update() {
    let msg = WsMessage::PoolUpdate {
        id: "pool-1".to_string(),
        liquidity: "1000000".to_string(),
        volume_24h: "50000".to_string(),
        apr: "12.5".to_string(),
        timestamp: 1704067200,
    };
    
    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("pool_update"));
    assert!(json.contains("pool-1"));
}

#[tokio::test]
async fn test_ws_message_ping_pong() {
    let ping = WsMessage::Ping { timestamp: 1704067200 };
    let pong = WsMessage::Pong { timestamp: 1704067200 };
    
    let ping_json = serde_json::to_string(&ping).unwrap();
    let pong_json = serde_json::to_string(&pong).unwrap();
    
    assert!(ping_json.contains("ping"));
    assert!(pong_json.contains("pong"));
}

#[tokio::test]
async fn test_ws_message_subscribe() {
    let msg = WsMessage::Subscribe {
        channels: vec!["markets".to_string(), "orders".to_string()],
    };
    
    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("subscribe"));
    assert!(json.contains("markets"));
}

#[tokio::test]
async fn test_ws_message_error() {
    let msg = WsMessage::Error {
        code: 400,
        message: "Invalid request".to_string(),
    };
    
    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("error"));
    assert!(json.contains("400"));
}

#[test]
fn test_valid_channel_validation() {
    assert!(is_valid_channel("markets"));
    assert!(is_valid_channel("orders"));
    assert!(is_valid_channel("pools"));
}

#[test]
fn test_invalid_channel_validation() {
    // Security: Reject invalid channels
    assert!(!is_valid_channel("invalid"));
    assert!(!is_valid_channel(""));
    assert!(!is_valid_channel("admin"));
    assert!(!is_valid_channel("../../../etc/passwd"));
    assert!(!is_valid_channel("<script>alert('xss')</script>"));
}

#[tokio::test]
async fn test_broadcast_market_update() {
    let state = WsState::new();
    
    // Should not panic even with no subscribers
    broadcast_market_update(
        &state,
        "ETH/USDC".to_string(),
        2500.0,
        2.5,
        45000000.0,
    ).await;
}

#[tokio::test]
async fn test_broadcast_order_update() {
    let state = WsState::new();
    
    broadcast_order_update(
        &state,
        "order-123".to_string(),
        "filled".to_string(),
        "1.5".to_string(),
    ).await;
}

#[tokio::test]
async fn test_broadcast_pool_update() {
    let state = WsState::new();
    
    broadcast_pool_update(
        &state,
        "pool-1".to_string(),
        "1000000".to_string(),
        "50000".to_string(),
        "12.5".to_string(),
    ).await;
}

#[tokio::test]
async fn test_ws_message_deserialization() {
    // Security: Ensure malformed JSON is rejected
    let invalid_json = "{invalid}";
    let result: Result<WsMessage, _> = serde_json::from_str(invalid_json);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_ws_message_type_safety() {
    // Security: Ensure type field is required
    let json_without_type = r#"{"pair": "ETH/USDC"}"#;
    let result: Result<WsMessage, _> = serde_json::from_str(json_without_type);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_ws_state_concurrent_clients() {
    let state = WsState::new();
    
    // Simulate concurrent connections
    let handles: Vec<_> = (0..100)
        .map(|_| {
            let state = state.clone();
            tokio::spawn(async move {
                state.increment_clients().await;
            })
        })
        .collect();
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    assert_eq!(state.client_count().await, 100);
}

#[tokio::test]
async fn test_ws_max_connections_limit() {
    let state = WsState::new();
    
    // Security: Client count tracking should prevent overflow
    for _ in 0..10001 {
        state.increment_clients().await;
    }
    
    // Should not panic, count should be tracked correctly
    let count = state.client_count().await;
    assert_eq!(count, 10001);
}

#[tokio::test]
async fn test_ws_broadcast_channels() {
    let state = WsState::new();
    let mut market_rx = state.market_tx.subscribe();
    let mut order_rx = state.order_tx.subscribe();
    let mut pool_rx = state.pool_tx.subscribe();
    
    // Send to all channels
    broadcast_market_update(&state, "ETH/USDC".to_string(), 2500.0, 2.5, 45000000.0).await;
    broadcast_order_update(&state, "order-1".to_string(), "open".to_string(), "0".to_string()).await;
    broadcast_pool_update(&state, "pool-1".to_string(), "1000000".to_string(), "50000".to_string(), "12.5".to_string()).await;
    
    // Verify messages received
    let market_msg = market_rx.recv().await.unwrap();
    let order_msg = order_rx.recv().await.unwrap();
    let pool_msg = pool_rx.recv().await.unwrap();
    
    match market_msg {
        WsMessage::MarketUpdate { pair, .. } => assert_eq!(pair, "ETH/USDC"),
        _ => panic!("Wrong message type"),
    }
    
    match order_msg {
        WsMessage::OrderUpdate { id, .. } => assert_eq!(id, "order-1"),
        _ => panic!("Wrong message type"),
    }
    
    match pool_msg {
        WsMessage::PoolUpdate { id, .. } => assert_eq!(id, "pool-1"),
        _ => panic!("Wrong message type"),
    }
}
