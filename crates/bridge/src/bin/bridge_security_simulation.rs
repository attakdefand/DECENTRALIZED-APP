//! Binary test runner for bridge security simulation

use bridge::{
    BridgeConfig, BridgeTransfer, LightClientBridge, OptimisticBridge, ZkBridge,
    WatcherConfig, WatcherAlert, ChallengerConfig, Challenge
};
use core::types::{Address, TokenAmount};

fn main() {
    println!("Starting Bridge Security Simulation Tests");
    println!("=====================================\n");
    
    test_complete_bridge_security_workflow();
    test_bridge_security_under_stress();
    test_bridge_security_edge_cases();
    
    println!("All Bridge Security Simulation Tests Passed!");
}

/// Test complete bridge security workflow
fn test_complete_bridge_security_workflow() {
    println!("1. Testing Complete Bridge Security Workflow");
    println!("-------------------------------------------");
    
    // 1. Create bridge configurations
    let config = BridgeConfig {
        source_chain_id: 1,
        destination_chain_id: 2,
        bridge_contract_address: Address("bridge_contract".to_string()),
        confirmation_blocks: 12,
        challenge_period: 3600,
        min_stake: 1000,
    };
    
    // 2. Create all bridge types
    let mut light_client_bridge = LightClientBridge::new(config.clone());
    let mut optimistic_bridge = OptimisticBridge::new(config.clone());
    let mut zk_bridge = ZkBridge::new(config);
    
    println!("   ✓ All bridge types created");
    
    // 3. Configure watchers and challengers
    let watcher = WatcherConfig {
        id: "main-watcher".to_string(),
        chain_id: 1,
        enabled: true,
        alert_threshold: 5,
    };
    
    let challenger = ChallengerConfig {
        id: "main-challenger".to_string(),
        address: Address("main_challenger_address".to_string()),
        stake: 2000,
        active: true,
    };
    
    assert!(light_client_bridge.add_watcher(watcher.clone()).is_ok());
    assert!(light_client_bridge.add_challenger(challenger.clone()).is_ok());
    assert!(optimistic_bridge.add_watcher(watcher.clone()).is_ok());
    assert!(optimistic_bridge.add_challenger(challenger.clone()).is_ok());
    assert!(zk_bridge.add_watcher(watcher).is_ok());
    assert!(zk_bridge.add_challenger(challenger).is_ok());
    
    println!("   ✓ Watchers and challengers configured");
    
    // 4. Create and submit transfers
    let transfer = BridgeTransfer {
        id: "test-transfer-001".to_string(),
        sender: Address("sender_address".to_string()),
        recipient: Address("recipient_address".to_string()),
        token: Address("token_address".to_string()),
        amount: TokenAmount {
            value: 1000000000000000000,
            decimals: 18,
        },
        source_chain_id: 1,
        destination_chain_id: 2,
        timestamp: 1234567890,
    };
    
    let lc_transfer_id = light_client_bridge.submit_transfer(transfer.clone()).unwrap();
    let opt_transfer_id = optimistic_bridge.submit_transfer(transfer.clone()).unwrap();
    let zk_transfer_id = zk_bridge.submit_transfer_with_proof(transfer, vec![1, 2, 3, 4, 5]).unwrap();
    
    assert_eq!(lc_transfer_id, "test-transfer-001");
    assert_eq!(opt_transfer_id, "test-transfer-001");
    assert_eq!(zk_transfer_id, "test-transfer-001");
    
    println!("   ✓ Transfers submitted to all bridge types");
    
    // 5. Generate alerts
    let alert = WatcherAlert {
        id: "test-alert-001".to_string(),
        watcher_id: "main-watcher".to_string(),
        transfer_id: "test-transfer-001".to_string(),
        alert_type: "suspicious_activity".to_string(),
        description: "Suspicious bridge activity detected".to_string(),
        timestamp: 1234567895,
        resolved: false,
    };
    
    assert!(light_client_bridge.generate_alert(alert.clone()).is_ok());
    assert!(optimistic_bridge.generate_alert(alert.clone()).is_ok());
    assert!(zk_bridge.generate_alert(alert).is_ok());
    
    println!("   ✓ Alerts generated for all bridges");
    
    // 6. Relay transfers and verify proofs
    assert!(light_client_bridge.relay_transfer("test-transfer-001", vec![1, 2, 3]).is_ok());
    // Note: OptimisticBridge doesn't have relay_transfer method, using submit_transfer instead
    assert!(light_client_bridge.verify_proof(&vec![1, 2, 3]).unwrap());
    assert!(optimistic_bridge.verify_proof(&vec![4, 5, 6]).unwrap());
    
    println!("   ✓ Transfers relayed and proofs verified");
    
    // 7. Submit and resolve challenges
    let challenge = Challenge {
        id: "test-challenge-001".to_string(),
        transfer_id: "test-transfer-001".to_string(),
        challenger: Address("main_challenger_address".to_string()),
        reason: "Testing challenge mechanism".to_string(),
        proof: vec![7, 8, 9],
        timestamp: 1234567900,
        resolved: false,
        successful: false,
    };
    
    assert!(light_client_bridge.submit_challenge(challenge.clone()).is_ok());
    assert!(optimistic_bridge.challenge_transfer(challenge).is_ok());
    assert!(light_client_bridge.resolve_challenge("test-challenge-001", true).is_ok());
    assert!(optimistic_bridge.resolve_challenge("test-challenge-001", false).is_ok());
    
    println!("   ✓ Challenges submitted and resolved");
    
    // 8. Verify final states
    assert_eq!(light_client_bridge.relays.len(), 1);
    assert_eq!(light_client_bridge.alerts.len(), 1);
    assert_eq!(light_client_bridge.challenges.len(), 1);
    
    assert_eq!(optimistic_bridge.transfers.len(), 1);
    assert_eq!(optimistic_bridge.alerts.len(), 1);
    assert_eq!(optimistic_bridge.challenges.len(), 1);
    
    assert_eq!(zk_bridge.transfers.len(), 1);
    assert_eq!(zk_bridge.alerts.len(), 1);
    
    println!("   ✓ Final bridge states verified\n");
}

/// Test bridge security under stress conditions
fn test_bridge_security_under_stress() {
    println!("2. Testing Bridge Security Under Stress");
    println!("--------------------------------------");
    
    // 1. Create bridge configuration
    let config = BridgeConfig {
        source_chain_id: 1,
        destination_chain_id: 2,
        bridge_contract_address: Address("bridge_contract".to_string()),
        confirmation_blocks: 12,
        challenge_period: 3600,
        min_stake: 1000,
    };
    
    // 2. Create optimistic bridge for stress testing
    let mut bridge = OptimisticBridge::new(config);
    
    // 3. Add multiple watchers and challengers
    for i in 0..3 {
        let watcher = WatcherConfig {
            id: format!("stress-watcher-{}", i),
            chain_id: 1,
            enabled: true,
            alert_threshold: 10,
        };
        assert!(bridge.add_watcher(watcher).is_ok());
        
        let challenger = ChallengerConfig {
            id: format!("stress-challenger-{}", i),
            address: Address(format!("stress_challenger_{}", i)),
            stake: 1500 + (i as u128) * 500,
            active: true,
        };
        assert!(bridge.add_challenger(challenger).is_ok());
    }
    
    println!("   ✓ Stress environment configured");
    
    // 4. Submit many transfers
    for i in 0..50 {
        let transfer = BridgeTransfer {
            id: format!("stress-transfer-{}", i),
            sender: Address(format!("stress_sender_{}", i)),
            recipient: Address(format!("stress_recipient_{}", i)),
            token: Address("stress_token".to_string()),
            amount: TokenAmount {
                value: 1000000000000000000 + (i as u128) * 10000000000000000,
                decimals: 18,
            },
            source_chain_id: 1,
            destination_chain_id: 2,
            timestamp: 2000000 + (i as u64) * 5,
        };
        
        assert!(bridge.submit_transfer(transfer).is_ok());
    }
    
    println!("   ✓ 50 transfers submitted under stress");
    
    // 5. Generate alerts and challenges
    for i in (0..50).step_by(10) {
        let alert = WatcherAlert {
            id: format!("stress-alert-{}", i / 10),
            watcher_id: "stress-watcher-0".to_string(),
            transfer_id: format!("stress-transfer-{}", i),
            alert_type: "high_frequency".to_string(),
            description: "High frequency transfer pattern".to_string(),
            timestamp: 2000500 + (i as u64),
            resolved: false,
        };
        assert!(bridge.generate_alert(alert).is_ok());
        
        let challenge = Challenge {
            id: format!("stress-challenge-{}", i / 10),
            transfer_id: format!("stress-transfer-{}", i),
            challenger: Address("stress_challenger_0".to_string()),
            reason: "Suspicious pattern detected".to_string(),
            proof: vec![1, 2, 3],
            timestamp: 2000600 + (i as u64),
            resolved: false,
            successful: false,
        };
        assert!(bridge.challenge_transfer(challenge).is_ok());
    }
    
    println!("   ✓ Alerts and challenges generated");
    
    // 6. Resolve challenges
    for i in 0..5 {
        assert!(bridge.resolve_challenge(&format!("stress-challenge-{}", i), i % 2 == 0).is_ok());
    }
    
    println!("   ✓ Challenges resolved");
    
    // 7. Verify results
    assert_eq!(bridge.transfers.len(), 50);
    assert_eq!(bridge.watchers.len(), 3);
    assert_eq!(bridge.challengers.len(), 3);
    assert_eq!(bridge.alerts.len(), 5);
    assert_eq!(bridge.challenges.len(), 5);
    
    println!("   ✓ Stress test results verified\n");
}

/// Test bridge security edge cases
fn test_bridge_security_edge_cases() {
    println!("3. Testing Bridge Security Edge Cases");
    println!("------------------------------------");
    
    // 1. Create bridge configuration
    let config = BridgeConfig {
        source_chain_id: 1,
        destination_chain_id: 2,
        bridge_contract_address: Address("bridge_contract".to_string()),
        confirmation_blocks: 12,
        challenge_period: 1800, // 30 minutes
        min_stake: 1000,
    };
    
    // 2. Create bridges
    let mut lc_bridge = LightClientBridge::new(config.clone());
    let mut opt_bridge = OptimisticBridge::new(config.clone());
    
    // 3. Test edge case: challenge with minimum stake
    let min_stake_challenger = ChallengerConfig {
        id: "min-stake-challenger".to_string(),
        address: Address("min_stake_challenger_address".to_string()),
        stake: 1000, // Exactly minimum stake
        active: true,
    };
    
    assert!(opt_bridge.add_challenger(min_stake_challenger).is_ok());
    
    let transfer = BridgeTransfer {
        id: "edge-transfer-001".to_string(),
        sender: Address("sender_address".to_string()),
        recipient: Address("recipient_address".to_string()),
        token: Address("token_address".to_string()),
        amount: TokenAmount {
            value: 1000000000000000000,
            decimals: 18,
        },
        source_chain_id: 1,
        destination_chain_id: 2,
        timestamp: 3000000,
    };
    
    assert!(opt_bridge.submit_transfer(transfer).is_ok());
    
    let min_stake_challenge = Challenge {
        id: "min-stake-challenge".to_string(),
        transfer_id: "edge-transfer-001".to_string(),
        challenger: Address("min_stake_challenger_address".to_string()),
        reason: "Testing minimum stake".to_string(),
        proof: vec![1, 2, 3],
        timestamp: 3000050,
        resolved: false,
        successful: false,
    };
    
    // Should succeed with minimum stake
    assert!(opt_bridge.challenge_transfer(min_stake_challenge).is_ok());
    assert!(opt_bridge.resolve_challenge("min-stake-challenge", true).is_ok());
    
    println!("   ✓ Minimum stake challenge handled correctly");
    
    // 4. Test edge case: challenge at exact challenge period end
    let edge_transfer = BridgeTransfer {
        id: "edge-transfer-002".to_string(),
        sender: Address("sender_address".to_string()),
        recipient: Address("recipient_address".to_string()),
        token: Address("token_address".to_string()),
        amount: TokenAmount {
            value: 2000000000000000000,
            decimals: 18,
        },
        source_chain_id: 1,
        destination_chain_id: 2,
        timestamp: 4000000,
    };
    
    assert!(opt_bridge.submit_transfer(edge_transfer).is_ok());
    
    let edge_challenge = Challenge {
        id: "edge-challenge".to_string(),
        transfer_id: "edge-transfer-002".to_string(),
        challenger: Address("min_stake_challenger_address".to_string()),
        reason: "Testing challenge period edge".to_string(),
        proof: vec![4, 5, 6],
        timestamp: 4000000 + 1800, // Exactly at challenge period end
        resolved: false,
        successful: false,
    };
    
    // Should succeed at exact challenge period end
    assert!(opt_bridge.challenge_transfer(edge_challenge).is_ok());
    
    println!("   ✓ Challenge period edge case handled correctly");
    
    // 5. Test edge case: multiple challenges on same transfer
    let multi_challenge_1 = Challenge {
        id: "multi-challenge-1".to_string(),
        transfer_id: "edge-transfer-002".to_string(),
        challenger: Address("min_stake_challenger_address".to_string()),
        reason: "First challenge".to_string(),
        proof: vec![7, 8, 9],
        timestamp: 4000000 + 1805,
        resolved: false,
        successful: false,
    };
    
    let multi_challenge_2 = Challenge {
        id: "multi-challenge-2".to_string(),
        transfer_id: "edge-transfer-002".to_string(),
        challenger: Address("min_stake_challenger_address".to_string()),
        reason: "Second challenge".to_string(),
        proof: vec![10, 11, 12],
        timestamp: 4000000 + 1810,
        resolved: false,
        successful: false,
    };
    
    // Both should succeed
    assert!(opt_bridge.challenge_transfer(multi_challenge_1).is_ok());
    assert!(opt_bridge.challenge_transfer(multi_challenge_2).is_ok());
    
    // Resolve all challenges
    assert!(opt_bridge.resolve_challenge("edge-challenge", false).is_ok());
    assert!(opt_bridge.resolve_challenge("multi-challenge-1", true).is_ok());
    assert!(opt_bridge.resolve_challenge("multi-challenge-2", false).is_ok());
    
    println!("   ✓ Multiple challenges on same transfer handled correctly");
    
    // 6. Verify final states
    assert_eq!(opt_bridge.transfers.len(), 2);
    assert_eq!(opt_bridge.challenges.len(), 3);
    
    println!("   ✓ Edge case test results verified\n");
}