//! Tests for the financial integrity module

use core::financial_integrity::{
    Account, AccountType, LedgerManager, Transaction, TransactionType, TransactionStatus,
    RiskManager, RiskLimit, GameTheorySimulator, EconomicScenario,
};

#[test]
fn test_account_creation() {
    let mut ledger_manager = LedgerManager::new();
    
    // Create an account
    assert!(ledger_manager
        .create_account(
            "account1".to_string(),
            "Test Account".to_string(),
            AccountType::Asset,
            "USD".to_string()
        )
        .is_ok());

    // Try to create the same account again
    assert!(ledger_manager
        .create_account(
            "account1".to_string(),
            "Test Account 2".to_string(),
            AccountType::Liability,
            "USD".to_string()
        )
        .is_err());

    // Check account exists
    let account = ledger_manager.get_account("account1");
    assert!(account.is_some());
    let account = account.unwrap();
    assert_eq!(account.name, "Test Account");
    assert_eq!(account.account_type, AccountType::Asset);
    assert_eq!(account.currency, "USD");
    assert_eq!(account.balance, 0);
}

#[test]
fn test_transfer_transaction() {
    let mut ledger_manager = LedgerManager::new();

    // Create accounts
    ledger_manager
        .create_account(
            "system".to_string(),
            "System Account".to_string(),
            AccountType::Asset,
            "USD".to_string(),
        )
        .unwrap();

    ledger_manager
        .create_account(
            "account1".to_string(),
            "Source Account".to_string(),
            AccountType::Asset,
            "USD".to_string(),
        )
        .unwrap();

    ledger_manager
        .create_account(
            "account2".to_string(),
            "Destination Account".to_string(),
            AccountType::Asset,
            "USD".to_string(),
        )
        .unwrap();

    // Add funds to source account
    let deposit_tx = Transaction {
        id: "tx1".to_string(),
        tx_type: TransactionType::Deposit,
        amount: 1000,
        currency: "USD".to_string(),
        timestamp: 1234567890,
        from_account: "system".to_string(),
        to_account: "account1".to_string(),
        status: TransactionStatus::Pending,
        nonce: 1,
        idempotency_key: "deposit1".to_string(),
        metadata: std::collections::HashMap::new(),
    };

    assert!(ledger_manager.process_transaction(deposit_tx).is_ok());

    // Transfer funds
    let transfer = Transaction {
        id: "tx2".to_string(),
        tx_type: TransactionType::Transfer,
        amount: 500,
        currency: "USD".to_string(),
        timestamp: 1234567891,
        from_account: "account1".to_string(),
        to_account: "account2".to_string(),
        status: TransactionStatus::Pending,
        nonce: 2, // Second transaction from account1 (after deposit)
        idempotency_key: "transfer1".to_string(),
        metadata: std::collections::HashMap::new(),
    };

    let result = ledger_manager.process_transaction(transfer);
    println!("Transfer result: {:?}", result);
    if let Err(e) = &result {
        println!("Transfer error: {}", e);
    }
    assert!(result.is_ok());

    // Check balances
    assert_eq!(ledger_manager.get_account_balance("account1"), Some(500));
    assert_eq!(ledger_manager.get_account_balance("account2"), Some(500));

    // Check ledger entries
    let account1_entries = ledger_manager.get_account_entries("account1");
    assert_eq!(account1_entries.len(), 2); // 1 deposit, 1 transfer debit

    let account2_entries = ledger_manager.get_account_entries("account2");
    assert_eq!(account2_entries.len(), 1); // 1 transfer credit
}

#[test]
fn test_idempotency_prevention() {
    let mut ledger_manager = LedgerManager::new();

    // Create accounts
    ledger_manager
        .create_account(
            "system".to_string(),
            "System Account".to_string(),
            AccountType::Asset,
            "USD".to_string(),
        )
        .unwrap();

    ledger_manager
        .create_account(
            "account1".to_string(),
            "Source Account".to_string(),
            AccountType::Asset,
            "USD".to_string(),
        )
        .unwrap();

    ledger_manager
        .create_account(
            "account2".to_string(),
            "Destination Account".to_string(),
            AccountType::Asset,
            "USD".to_string(),
        )
        .unwrap();

    // Add funds to source account
    let deposit_tx = Transaction {
        id: "tx1".to_string(),
        tx_type: TransactionType::Deposit,
        amount: 1000,
        currency: "USD".to_string(),
        timestamp: 1234567890,
        from_account: "system".to_string(),
        to_account: "account1".to_string(),
        status: TransactionStatus::Pending,
        nonce: 1,
        idempotency_key: "deposit1".to_string(),
        metadata: std::collections::HashMap::new(),
    };

    assert!(ledger_manager.process_transaction(deposit_tx.clone()).is_ok());

    // Try to process the same transaction again (should fail)
    assert!(ledger_manager.process_transaction(deposit_tx).is_err());

    // Check that balance is correct (transaction only processed once)
    assert_eq!(ledger_manager.get_account_balance("account1"), Some(1000));
}

#[test]
fn test_nonce_validation() {
    let mut ledger_manager = LedgerManager::new();

    // Create accounts
    ledger_manager
        .create_account(
            "system".to_string(),
            "System Account".to_string(),
            AccountType::Asset,
            "USD".to_string(),
        )
        .unwrap();

    ledger_manager
        .create_account(
            "account1".to_string(),
            "Source Account".to_string(),
            AccountType::Asset,
            "USD".to_string(),
        )
        .unwrap();

    ledger_manager
        .create_account(
            "account2".to_string(),
            "Destination Account".to_string(),
            AccountType::Asset,
            "USD".to_string(),
        )
        .unwrap();

    // Add funds to source account
    let deposit_tx = Transaction {
        id: "tx1".to_string(),
        tx_type: TransactionType::Deposit,
        amount: 1000,
        currency: "USD".to_string(),
        timestamp: 1234567890,
        from_account: "system".to_string(),
        to_account: "account1".to_string(),
        status: TransactionStatus::Pending,
        nonce: 1,
        idempotency_key: "deposit1".to_string(),
        metadata: std::collections::HashMap::new(),
    };

    assert!(ledger_manager.process_transaction(deposit_tx).is_ok());

    // Transfer funds from account1 to account2
    let transfer_tx1 = Transaction {
        id: "tx2".to_string(),
        tx_type: TransactionType::Transfer,
        amount: 500,
        currency: "USD".to_string(),
        timestamp: 1234567891,
        from_account: "account1".to_string(),
        to_account: "account2".to_string(),
        status: TransactionStatus::Pending,
        nonce: 2, // Second transaction from account1 (after deposit)
        idempotency_key: "transfer1".to_string(),
        metadata: std::collections::HashMap::new(),
    };

    assert!(ledger_manager.process_transaction(transfer_tx1).is_ok());

    // Try to process another transaction with the same nonce from account1 (should fail)
    let transfer_tx2 = Transaction {
        id: "tx3".to_string(),
        tx_type: TransactionType::Transfer,
        amount: 200,
        currency: "USD".to_string(),
        timestamp: 1234567892,
        from_account: "account1".to_string(),
        to_account: "account2".to_string(),
        status: TransactionStatus::Pending,
        nonce: 1, // Same nonce as previous transaction from account1 - should fail
        idempotency_key: "transfer2".to_string(),
        metadata: std::collections::HashMap::new(),
    };

    // This should fail because nonce 1 is already used for account1
    let result = ledger_manager.process_transaction(transfer_tx2);
    assert!(result.is_err());
    println!("Nonce validation error: {:?}", result.err().unwrap());

    // Process transaction with correct nonce
    let transfer_tx3 = Transaction {
        id: "tx4".to_string(),
        tx_type: TransactionType::Transfer,
        amount: 200,
        currency: "USD".to_string(),
        timestamp: 1234567893,
        from_account: "account1".to_string(),
        to_account: "account2".to_string(),
        status: TransactionStatus::Pending,
        nonce: 3, // Next nonce for account1 - should succeed
        idempotency_key: "transfer3".to_string(),
        metadata: std::collections::HashMap::new(),
    };

    assert!(ledger_manager.process_transaction(transfer_tx3).is_ok());

    // Check balances
    assert_eq!(ledger_manager.get_account_balance("account1"), Some(300)); // 1000 - 500 - 200
    assert_eq!(ledger_manager.get_account_balance("account2"), Some(700)); // 500 + 200
}

#[test]
fn test_reconciliation() {
    let mut ledger_manager = LedgerManager::new();

    // Create accounts
    ledger_manager
        .create_account(
            "system".to_string(),
            "System Account".to_string(),
            AccountType::Asset,
            "USD".to_string(),
        )
        .unwrap();

    ledger_manager
        .create_account(
            "asset1".to_string(),
            "Asset Account 1".to_string(),
            AccountType::Asset,
            "USD".to_string(),
        )
        .unwrap();

    ledger_manager
        .create_account(
            "asset2".to_string(),
            "Asset Account 2".to_string(),
            AccountType::Asset,
            "USD".to_string(),
        )
        .unwrap();

    ledger_manager
        .create_account(
            "liability1".to_string(),
            "Liability Account 1".to_string(),
            AccountType::Liability,
            "USD".to_string(),
        )
        .unwrap();

    // Add funds
    let deposit1 = Transaction {
        id: "tx1".to_string(),
        tx_type: TransactionType::Deposit,
        amount: 1000,
        currency: "USD".to_string(),
        timestamp: 1234567890,
        from_account: "system".to_string(),
        to_account: "asset1".to_string(),
        status: TransactionStatus::Pending,
        nonce: 1,
        idempotency_key: "deposit1".to_string(),
        metadata: std::collections::HashMap::new(),
    };

    let deposit2 = Transaction {
        id: "tx2".to_string(),
        tx_type: TransactionType::Deposit,
        amount: 500,
        currency: "USD".to_string(),
        timestamp: 1234567891,
        from_account: "system".to_string(),
        to_account: "liability1".to_string(),
        status: TransactionStatus::Pending,
        nonce: 1, // Use nonce 1 for the first deposit to "liability1" account
        idempotency_key: "deposit2".to_string(),
        metadata: std::collections::HashMap::new(),
    };

    assert!(ledger_manager.process_transaction(deposit1).is_ok());
    assert!(ledger_manager.process_transaction(deposit2).is_ok());

    // Transfer between asset accounts
    let transfer = Transaction {
        id: "tx3".to_string(),
        tx_type: TransactionType::Transfer,
        amount: 300,
        currency: "USD".to_string(),
        timestamp: 1234567892,
        from_account: "asset1".to_string(),
        to_account: "asset2".to_string(),
        status: TransactionStatus::Pending,
        nonce: 2, // Second transaction from asset1 (after deposit)
        idempotency_key: "transfer1".to_string(),
        metadata: std::collections::HashMap::new(),
    };

    assert!(ledger_manager.process_transaction(transfer).is_ok());

    // Run reconciliation
    let report = ledger_manager.run_reconciliation().unwrap();
    println!("Reconciliation report: {:?}", report);

    // Check that ledger is balanced (with proper double-entry bookkeeping, deposits create both credit and debit entries)
    assert_eq!(report.total_debits, 1800);
    assert_eq!(report.total_credits, 1800);
    assert!(report.balanced); // Should be balanced with proper double-entry bookkeeping

    // Check account counts
    assert_eq!(report.asset_accounts_positive, 2); // asset1 has 700, asset2 has 300
    assert_eq!(report.liability_accounts_positive, 1); // liability1 has 500
}

#[test]
fn test_invariant_tests() {
    let mut ledger_manager = LedgerManager::new();

    // Create accounts
    ledger_manager
        .create_account(
            "system".to_string(),
            "System Account".to_string(),
            AccountType::Asset,
            "USD".to_string(),
        )
        .unwrap();

    ledger_manager
        .create_account(
            "account1".to_string(),
            "Test Account".to_string(),
            AccountType::Asset,
            "USD".to_string(),
        )
        .unwrap();

    ledger_manager
        .create_account(
            "account2".to_string(),
            "Test Account 2".to_string(),
            AccountType::Asset,
            "USD".to_string(),
        )
        .unwrap();

    // Add funds
    let deposit = Transaction {
        id: "tx1".to_string(),
        tx_type: TransactionType::Deposit,
        amount: 1000,
        currency: "USD".to_string(),
        timestamp: 1234567890,
        from_account: "system".to_string(),
        to_account: "account1".to_string(),
        status: TransactionStatus::Pending,
        nonce: 1,
        idempotency_key: "deposit1".to_string(),
        metadata: std::collections::HashMap::new(),
    };

    assert!(ledger_manager.process_transaction(deposit).is_ok());

    // Transfer funds
    let transfer = Transaction {
        id: "tx2".to_string(),
        tx_type: TransactionType::Transfer,
        amount: 500,
        currency: "USD".to_string(),
        timestamp: 1234567891,
        from_account: "account1".to_string(),
        to_account: "account2".to_string(),
        status: TransactionStatus::Pending,
        nonce: 2, // Second transaction from account1 (after deposit)
        idempotency_key: "transfer1".to_string(),
        metadata: std::collections::HashMap::new(),
    };

    assert!(ledger_manager.process_transaction(transfer).is_ok());

    // Run invariant tests
    let report = ledger_manager.run_invariant_tests().unwrap();

    // All tests should pass
    assert!(report.passed);
    assert_eq!(report.errors.len(), 0);

    // Check account balances match ledger entries
    assert_eq!(ledger_manager.get_account_balance("account1"), Some(500));
    assert_eq!(ledger_manager.get_account_balance("account2"), Some(500));
}

#[test]
fn test_risk_limits() {
    let mut risk_manager = RiskManager::new();

    // Set risk limits
    let limits = RiskLimit {
        user_id: "user1".to_string(),
        max_position_size: 10000,
        max_daily_loss: 1000,
        max_leverage: 5.0,
        slippage_tolerance: 0.01,
    };

    risk_manager.set_risk_limits(limits);

    // Check that transaction within limits passes
    assert!(risk_manager
        .check_risk_limits("user1", 5000, &TransactionType::Deposit)
        .is_ok());

    // Check that transaction exceeding limits fails
    assert!(risk_manager
        .check_risk_limits("user1", 15000, &TransactionType::Deposit)
        .is_err());

    // Update position
    risk_manager.update_position("user1", 5000, &TransactionType::Deposit);

    // Record a loss
    risk_manager.record_loss("user1", 500);
}

#[test]
fn test_game_theory_simulation() {
    let mut simulator = GameTheorySimulator::new();

    // Add scenarios
    let scenario1 = EconomicScenario {
        name: "Market Crash".to_string(),
        description: "Sudden market downturn".to_string(),
        probability: 0.1,
        impact: 0.3,
        attack_cost: 1000,
    };

    let scenario2 = EconomicScenario {
        name: "Flash Crash".to_string(),
        description: "Brief but severe price drop".to_string(),
        probability: 0.05,
        impact: 0.5,
        attack_cost: 2000,
    };

    simulator.add_scenario(scenario1);
    simulator.add_scenario(scenario2);

    // Run simulation
    let result = simulator.simulate_attack("user1", 10000);

    assert_eq!(result.user_id, "user1");
    assert_eq!(result.position, 10000);
    assert_eq!(result.scenario_results.len(), 2);

    // Check scenario results
    for scenario_result in result.scenario_results {
        // Net result should be calculated correctly
        let expected_net = scenario_result.expected_loss as i128 - scenario_result.attack_cost as i128;
        assert_eq!(scenario_result.net_result, expected_net);
    }
}