//! WebSocket handler for real-time updates
//!
//! Provides secure WebSocket connections for market data, order updates, and pool changes

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tracing::{error, info, warn};

/// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    /// Market price update
    #[serde(rename = "market_update")]
    MarketUpdate {
        pair: String,
        price: f64,
        change_24h: f64,
        volume_24h: f64,
        timestamp: u64,
    },
    /// Order status update
    #[serde(rename = "order_update")]
    OrderUpdate {
        id: String,
        status: String,
        filled: String,
        timestamp: u64,
    },
    /// Pool liquidity update
    #[serde(rename = "pool_update")]
    PoolUpdate {
        id: String,
        liquidity: String,
        volume_24h: String,
        apr: String,
        timestamp: u64,
    },
    /// Heartbeat ping
    #[serde(rename = "ping")]
    Ping { timestamp: u64 },
    /// Heartbeat pong
    #[serde(rename = "pong")]
    Pong { timestamp: u64 },
    /// Subscription request
    #[serde(rename = "subscribe")]
    Subscribe { channels: Vec<String> },
    /// Unsubscribe request
    #[serde(rename = "unsubscribe")]
    Unsubscribe { channels: Vec<String> },
    /// Error message
    #[serde(rename = "error")]
    Error { code: u16, message: String },
}

/// WebSocket state
#[derive(Clone)]
pub struct WsState {
    /// Broadcast channel for market updates
    pub market_tx: broadcast::Sender<WsMessage>,
    /// Broadcast channel for order updates
    pub order_tx: broadcast::Sender<WsMessage>,
    /// Broadcast channel for pool updates
    pub pool_tx: broadcast::Sender<WsMessage>,
    /// Connected clients count
    pub clients: Arc<RwLock<usize>>,
}

impl WsState {
    pub fn new() -> Self {
        let (market_tx, _) = broadcast::channel(1000);
        let (order_tx, _) = broadcast::channel(1000);
        let (pool_tx, _) = broadcast::channel(1000);
        
        Self {
            market_tx,
            order_tx,
            pool_tx,
            clients: Arc::new(RwLock::new(0)),
        }
    }
    
    /// Get current client count
    pub async fn client_count(&self) -> usize {
        *self.clients.read().await
    }
    
    /// Increment client count
    async fn increment_clients(&self) {
        let mut count = self.clients.write().await;
        *count += 1;
        info!("WebSocket client connected. Total clients: {}", *count);
    }
    
    /// Decrement client count
    async fn decrement_clients(&self) {
        let mut count = self.clients.write().await;
        *count = count.saturating_sub(1);
        info!("WebSocket client disconnected. Total clients: {}", *count);
    }
}

/// WebSocket upgrade handler
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<WsState>,
) -> Response {
    // Security: Validate maximum connections
    if state.client_count().await >= 10000 {
        warn!("WebSocket connection rejected: max connections reached");
        return ws.on_failed_upgrade(|_| {
            error!("WebSocket upgrade failed: max connections");
        });
    }
    
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// Handle individual WebSocket connection
async fn handle_socket(socket: WebSocket, state: WsState) {
    state.increment_clients().await;
    
    let (mut sender, mut receiver) = socket.split();
    
    // Subscribe to all channels initially
    let mut market_rx = state.market_tx.subscribe();
    let mut order_rx = state.order_tx.subscribe();
    let mut pool_rx = state.pool_tx.subscribe();
    
    // Track subscribed channels
    let mut subscribed_channels: Vec<String> = vec![
        "markets".to_string(),
        "orders".to_string(),
        "pools".to_string(),
    ];
    
    // Heartbeat task
    let heartbeat_state = state.clone();
    let heartbeat_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
        loop {
            interval.tick().await;
            let ping = WsMessage::Ping {
                timestamp: chrono::Utc::now().timestamp() as u64,
            };
            if let Ok(json) = serde_json::to_string(&ping) {
                if sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
    });
    
    // Message receive task
    let receive_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            if let Ok(msg) = msg {
                match msg {
                    Message::Text(text) => {
                        // Security: Parse and validate message
                        if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                            match ws_msg {
                                WsMessage::Subscribe { channels } => {
                                    // Security: Validate channel names
                                    for channel in channels {
                                        if is_valid_channel(&channel) && !subscribed_channels.contains(&channel) {
                                            subscribed_channels.push(channel.clone());
                                            info!("Client subscribed to: {}", channel);
                                        }
                                    }
                                }
                                WsMessage::Unsubscribe { channels } => {
                                    for channel in channels {
                                        subscribed_channels.retain(|c| c != &channel);
                                        info!("Client unsubscribed from: {}", channel);
                                    }
                                }
                                WsMessage::Pong { .. } => {
                                    // Heartbeat received
                                }
                                _ => {
                                    warn!("Unexpected message type from client");
                                }
                            }
                        }
                    }
                    Message::Close(_) => {
                        info!("Client requested close");
                        break;
                    }
                    _ => {}
                }
            }
        }
    });
    
    // Broadcast task
    let broadcast_task = tokio::spawn(async move {
        loop {
            tokio::select! {
                Ok(msg) = market_rx.recv() => {
                    if subscribed_channels.contains(&"markets".to_string()) {
                        if let Ok(json) = serde_json::to_string(&msg) {
                            if sender.send(Message::Text(json)).await.is_err() {
                                break;
                            }
                        }
                    }
                }
                Ok(msg) = order_rx.recv() => {
                    if subscribed_channels.contains(&"orders".to_string()) {
                        if let Ok(json) = serde_json::to_string(&msg) {
                            if sender.send(Message::Text(json)).await.is_err() {
                                break;
                            }
                        }
                    }
                }
                Ok(msg) = pool_rx.recv() => {
                    if subscribed_channels.contains(&"pools".to_string()) {
                        if let Ok(json) = serde_json::to_string(&msg) {
                            if sender.send(Message::Text(json)).await.is_err() {
                                break;
                            }
                        }
                    }
                }
            }
        }
    });
    
    // Wait for any task to complete
    tokio::select! {
        _ = heartbeat_task => {},
        _ = receive_task => {},
        _ = broadcast_task => {},
    }
    
    state.decrement_clients().await;
}

/// Security: Validate channel name
fn is_valid_channel(channel: &str) -> bool {
    matches!(channel, "markets" | "orders" | "pools")
}

/// Broadcast market update
pub async fn broadcast_market_update(
    state: &WsState,
    pair: String,
    price: f64,
    change_24h: f64,
    volume_24h: f64,
) {
    let msg = WsMessage::MarketUpdate {
        pair,
        price,
        change_24h,
        volume_24h,
        timestamp: chrono::Utc::now().timestamp() as u64,
    };
    
    // Ignore send errors (no receivers)
    let _ = state.market_tx.send(msg);
}

/// Broadcast order update
pub async fn broadcast_order_update(
    state: &WsState,
    id: String,
    status: String,
    filled: String,
) {
    let msg = WsMessage::OrderUpdate {
        id,
        status,
        filled,
        timestamp: chrono::Utc::now().timestamp() as u64,
    };
    
    let _ = state.order_tx.send(msg);
}

/// Broadcast pool update
pub async fn broadcast_pool_update(
    state: &WsState,
    id: String,
    liquidity: String,
    volume_24h: String,
    apr: String,
) {
    let msg = WsMessage::PoolUpdate {
        id,
        liquidity,
        volume_24h,
        apr,
        timestamp: chrono::Utc::now().timestamp() as u64,
    };
    
    let _ = state.pool_tx.send(msg);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_channel_names() {
        assert!(is_valid_channel("markets"));
        assert!(is_valid_channel("orders"));
        assert!(is_valid_channel("pools"));
        assert!(!is_valid_channel("invalid"));
        assert!(!is_valid_channel(""));
    }
    
    #[test]
    fn test_ws_message_serialization() {
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
    }
    
    #[tokio::test]
    async fn test_ws_state_client_count() {
        let state = WsState::new();
        assert_eq!(state.client_count().await, 0);
        
        state.increment_clients().await;
        assert_eq!(state.client_count().await, 1);
        
        state.decrement_clients().await;
        assert_eq!(state.client_count().await, 0);
    }
}
