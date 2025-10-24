//! Integration tests for bridge security functionality

use bridge::{
    BridgeConfig, BridgeTransfer, Challenge, ChallengerConfig, LightClientBridge, OptimisticBridge,
    WatcherAlert, WatcherConfig, ZkBridge,
};
use core::types::{Address, TokenAmount};

/// Integration test for the complete light client bridge workflow
#[test]
fn test_complete_light_client_bridge_workflow() {
    println!("Starting complete light client bridge workflow test");

    // 1. Create bridge configuration
    let config = BridgeConfig {
        source_chain_id: 1,
        destination_chain_id: 2,
        bridge_contract_address: Address("bridge_contract".to_string()),
        confirmation_blocks: 12,
        challenge_period: 3600,
        min_stake: 1000,
    };

    // 2. Create light client bridge
    let mut bridge = LightClientBridge::new(config);
    println!("✓ Light client bridge created");

    // 3. Create a bridge transfer with current timestamp
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let transfer = BridgeTransfer {
        id: "transfer-001".to_string(),
        sender: Address("sender_address".to_string()),
        recipient: Address("recipient_address".to_string()),
        token: Address("token_address".to_string()),
        amount: TokenAmount {
            value: 1000000000000000000, // 1 token
            decimals: 18,
        },
        source_chain_id: 1,
        destination_chain_id: 2,
        timestamp: current_time,
    };
    println!("✓ Bridge transfer created");

    // 4. Submit transfer
    let transfer_id = bridge.submit_transfer(transfer).unwrap();
    assert_eq!(transfer_id, "transfer-001");
    println!("✓ Bridge transfer submitted");

    // 5. Add watchers
    let watcher = WatcherConfig {
        id: "watcher-001".to_string(),
        chain_id: 1,
        enabled: true,
        alert_threshold: 5,
    };

    assert!(bridge.add_watcher(watcher).is_ok());
    println!("✓ Watcher added");

    // 6. Add challengers
    let challenger = ChallengerConfig {
        id: "challenger-001".to_string(),
        address: Address("challenger_address".to_string()),
        stake: 2000,
        active: true,
    };

    assert!(bridge.add_challenger(challenger).is_ok());
    println!("✓ Challenger added");

    // 7. Generate alerts
    let alert = WatcherAlert {
        id: "alert-001".to_string(),
        watcher_id: "watcher-001".to_string(),
        transfer_id: "transfer-001".to_string(),
        alert_type: "suspicious_transfer".to_string(),
        description: "Suspicious transfer detected".to_string(),
        timestamp: 1234567895,
        resolved: false,
    };

    assert!(bridge.generate_alert(alert).is_ok());
    println!("✓ Alert generated");

    // 8. Submit challenge
    let challenge = Challenge {
        id: "challenge-001".to_string(),
        transfer_id: "transfer-001".to_string(),
        challenger: Address("challenger_address".to_string()),
        reason: "Invalid proof".to_string(),
        proof: vec![1, 2, 3, 4, 5],
        timestamp: 1234567900,
        resolved: false,
        successful: false,
    };

    assert!(bridge.submit_challenge(challenge).is_ok());
    println!("✓ Challenge submitted");

    // 9. Relay transfer
    assert!(bridge.relay_transfer("transfer-001", vec![1, 2, 3]).is_ok());
    println!("✓ Transfer relayed");

    // 10. Verify proof
    assert!(bridge.verify_proof(&vec![1, 2, 3]).unwrap());
    println!("✓ Proof verified");

    // 11. Resolve challenge
    assert!(bridge.resolve_challenge("challenge-001", true).is_ok());
    println!("✓ Challenge resolved");

    println!("Complete light client bridge workflow test passed!");
}

/// Integration test for the complete optimistic bridge workflow
#[test]
fn test_complete_optimistic_bridge_workflow() {
    println!("Starting complete optimistic bridge workflow test");

    // 1. Create bridge configuration
    let config = BridgeConfig {
        source_chain_id: 1,
        destination_chain_id: 2,
        bridge_contract_address: Address("bridge_contract".to_string()),
        confirmation_blocks: 12,
        challenge_period: 3600,
        min_stake: 1000,
    };

    // 2. Create optimistic bridge
    let mut bridge = OptimisticBridge::new(config);
    println!("✓ Optimistic bridge created");

    // 3. Create a bridge transfer with current timestamp
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let transfer = BridgeTransfer {
        id: "transfer-002".to_string(),
        sender: Address("sender_address".to_string()),
        recipient: Address("recipient_address".to_string()),
        token: Address("token_address".to_string()),
        amount: TokenAmount {
            value: 2000000000000000000, // 2 tokens
            decimals: 18,
        },
        source_chain_id: 1,
        destination_chain_id: 2,
        timestamp: current_time,
    };
    println!("✓ Bridge transfer created");

    // 4. Submit transfer
    let transfer_id = bridge.submit_transfer(transfer).unwrap();
    assert_eq!(transfer_id, "transfer-002");
    println!("✓ Bridge transfer submitted");

    // 5. Add watchers
    let watcher = WatcherConfig {
        id: "watcher-002".to_string(),
        chain_id: 1,
        enabled: true,
        alert_threshold: 5,
    };

    assert!(bridge.add_watcher(watcher).is_ok());
    println!("✓ Watcher added");

    // 6. Add challengers
    let challenger = ChallengerConfig {
        id: "challenger-002".to_string(),
        address: Address("challenger_address".to_string()),
        stake: 2000,
        active: true,
    };

    assert!(bridge.add_challenger(challenger).is_ok());
    println!("✓ Challenger added");

    // 7. Generate alerts
    let alert = WatcherAlert {
        id: "alert-002".to_string(),
        watcher_id: "watcher-002".to_string(),
        transfer_id: "transfer-002".to_string(),
        alert_type: "suspicious_transfer".to_string(),
        description: "Suspicious transfer detected".to_string(),
        timestamp: 1234567895,
        resolved: false,
    };

    assert!(bridge.generate_alert(alert).is_ok());
    println!("✓ Alert generated");

    // 8. Submit challenge
    let challenge = Challenge {
        id: "challenge-002".to_string(),
        transfer_id: "transfer-002".to_string(),
        challenger: Address("challenger_address".to_string()),
        reason: "Invalid transfer".to_string(),
        proof: vec![1, 2, 3, 4, 5],
        timestamp: 1234567900,
        resolved: false,
        successful: false,
    };

    assert!(bridge.challenge_transfer(challenge).is_ok());
    println!("✓ Challenge submitted");

    // 9. Resolve challenge
    assert!(bridge.resolve_challenge("challenge-002", true).is_ok());
    println!("✓ Challenge resolved");

    println!("Complete optimistic bridge workflow test passed!");
}

/// Integration test for the complete ZK bridge workflow
#[test]
fn test_complete_zk_bridge_workflow() {
    println!("Starting complete ZK bridge workflow test");

    // 1. Create bridge configuration
    let config = BridgeConfig {
        source_chain_id: 1,
        destination_chain_id: 2,
        bridge_contract_address: Address("bridge_contract".to_string()),
        confirmation_blocks: 12,
        challenge_period: 3600,
        min_stake: 1000,
    };

    // 2. Create ZK bridge
    let mut bridge = ZkBridge::new(config);
    println!("✓ ZK bridge created");

    // 3. Create a bridge transfer with current timestamp
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let transfer = BridgeTransfer {
        id: "transfer-003".to_string(),
        sender: Address("sender_address".to_string()),
        recipient: Address("recipient_address".to_string()),
        token: Address("token_address".to_string()),
        amount: TokenAmount {
            value: 3000000000000000000, // 3 tokens
            decimals: 18,
        },
        source_chain_id: 1,
        destination_chain_id: 2,
        timestamp: current_time,
    };
    println!("✓ Bridge transfer created");

    // 4. Submit transfer with proof
    let transfer_id = bridge
        .submit_transfer_with_proof(transfer, vec![1, 2, 3, 4, 5])
        .unwrap();
    assert_eq!(transfer_id, "transfer-003");
    println!("✓ Bridge transfer submitted with ZK proof");

    // 5. Add watchers
    let watcher = WatcherConfig {
        id: "watcher-003".to_string(),
        chain_id: 1,
        enabled: true,
        alert_threshold: 5,
    };

    assert!(bridge.add_watcher(watcher).is_ok());
    println!("✓ Watcher added");

    // 6. Add challengers
    let challenger = ChallengerConfig {
        id: "challenger-003".to_string(),
        address: Address("challenger_address".to_string()),
        stake: 2000,
        active: true,
    };

    assert!(bridge.add_challenger(challenger).is_ok());
    println!("✓ Challenger added");

    // 7. Generate alerts
    let alert = WatcherAlert {
        id: "alert-003".to_string(),
        watcher_id: "watcher-003".to_string(),
        transfer_id: "transfer-003".to_string(),
        alert_type: "suspicious_transfer".to_string(),
        description: "Suspicious transfer detected".to_string(),
        timestamp: 1234567895,
        resolved: false,
    };

    assert!(bridge.generate_alert(alert).is_ok());
    println!("✓ Alert generated");

    // 8. Submit challenge (rare for ZK bridges but possible)
    let challenge = Challenge {
        id: "challenge-003".to_string(),
        transfer_id: "transfer-003".to_string(),
        challenger: Address("challenger_address".to_string()),
        reason: "Invalid proof".to_string(),
        proof: vec![1, 2, 3, 4, 5],
        timestamp: 1234567900,
        resolved: false,
        successful: false,
    };

    assert!(bridge.submit_challenge(challenge).is_ok());
    println!("✓ Challenge submitted");

    // 9. Resolve challenge
    assert!(bridge.resolve_challenge("challenge-003", true).is_ok());
    println!("✓ Challenge resolved");

    println!("Complete ZK bridge workflow test passed!");
}

/// Test bridge security error handling
#[test]
fn test_bridge_security_error_handling() {
    println!("Starting bridge security error handling test");

    // 1. Create bridge configuration
    let config = BridgeConfig {
        source_chain_id: 1,
        destination_chain_id: 2,
        bridge_contract_address: Address("bridge_contract".to_string()),
        confirmation_blocks: 12,
        challenge_period: 3600,
        min_stake: 1000,
    };

    // 2. Create light client bridge
    let mut bridge = LightClientBridge::new(config);

    // 3. Test challenge with insufficient stake
    let low_stake_challenger = ChallengerConfig {
        id: "low-stake-challenger".to_string(),
        address: Address("low_stake_challenger_address".to_string()),
        stake: 500, // Below minimum required stake
        active: true,
    };

    assert!(bridge.add_challenger(low_stake_challenger).is_ok());

    let challenge_with_low_stake = Challenge {
        id: "challenge-low-stake".to_string(),
        transfer_id: "non-existent-transfer".to_string(),
        challenger: Address("low_stake_challenger_address".to_string()),
        reason: "Invalid proof".to_string(),
        proof: vec![1, 2, 3, 4, 5],
        timestamp: 1234567900,
        resolved: false,
        successful: false,
    };

    // Should fail due to insufficient stake
    assert!(bridge.submit_challenge(challenge_with_low_stake).is_err());
    println!("✓ Challenge with insufficient stake correctly rejected");

    // 4. Test challenge with non-existent challenger
    let challenge_with_nonexistent_challenger = Challenge {
        id: "challenge-nonexistent".to_string(),
        transfer_id: "non-existent-transfer".to_string(),
        challenger: Address("nonexistent_challenger_address".to_string()),
        reason: "Invalid proof".to_string(),
        proof: vec![1, 2, 3, 4, 5],
        timestamp: 1234567900,
        resolved: false,
        successful: false,
    };

    // Should fail due to non-existent challenger
    assert!(bridge
        .submit_challenge(challenge_with_nonexistent_challenger)
        .is_err());
    println!("✓ Challenge with non-existent challenger correctly rejected");

    println!("Bridge security error handling test passed!");
}
