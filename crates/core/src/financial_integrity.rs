//! Financial Integrity Module
//!
//! This module implements financial integrity controls including double-entry ledger,
//! idempotency, nonce management, atomic commits, reconciliation jobs, and invariant tests
//! as specified in the @RULES.md requirements.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Represents a financial transaction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    /// Unique transaction identifier
    pub id: String,
    /// Transaction type
    pub tx_type: TransactionType,
    /// Transaction amount
    pub amount: u128,
    /// Currency code
    pub currency: String,
    /// Timestamp of the transaction
    pub timestamp: u64,
    /// Source account
    pub from_account: String,
    /// Destination account
    pub to_account: String,
    /// Transaction status
    pub status: TransactionStatus,
    /// Nonce for ordering
    pub nonce: u64,
    /// Idempotency key to prevent duplicate processing
    pub idempotency_key: String,
    /// Metadata for the transaction
    pub metadata: HashMap<String, String>,
}

/// Transaction types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionType {
    /// Deposit transaction
    Deposit,
    /// Withdrawal transaction
    Withdrawal,
    /// Transfer transaction
    Transfer,
    /// Fee transaction
    Fee,
    /// Interest transaction
    Interest,
    /// Adjustment transaction
    Adjustment,
}

/// Transaction status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus {
    /// Pending processing
    Pending,
    /// Successfully processed
    Success,
    /// Failed processing
    Failed,
    /// Reversed
    Reversed,
}

/// Represents a ledger entry in the double-entry bookkeeping system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LedgerEntry {
    /// Unique entry identifier
    pub id: String,
    /// Transaction ID this entry belongs to
    pub transaction_id: String,
    /// Account affected by this entry
    pub account: String,
    /// Debit amount (if any)
    pub debit: u128,
    /// Credit amount (if any)
    pub credit: u128,
    /// Currency code
    pub currency: String,
    /// Timestamp of the entry
    pub timestamp: u64,
    /// Entry description
    pub description: String,
    /// Balance after this entry
    pub balance: u128,
}

/// Represents an account in the ledger
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Account {
    /// Account identifier
    pub id: String,
    /// Account name
    pub name: String,
    /// Account type
    pub account_type: AccountType,
    /// Current balance
    pub balance: u128,
    /// Currency code
    pub currency: String,
    /// Timestamp of last update
    pub last_updated: u64,
    /// Account status
    pub status: AccountStatus,
}

/// Account types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountType {
    /// Asset account (e.g., cash, inventory)
    Asset,
    /// Liability account (e.g., loans, accounts payable)
    Liability,
    /// Equity account (e.g., owner's equity)
    Equity,
    /// Revenue account (e.g., sales, service revenue)
    Revenue,
    /// Expense account (e.g., rent, utilities)
    Expense,
}

/// Account status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountStatus {
    /// Active account
    Active,
    /// Frozen account
    Frozen,
    /// Closed account
    Closed,
}

/// Nonce manager for ensuring transaction ordering
#[derive(Debug)]
pub struct NonceManager {
    /// Nonces by account
    nonces: HashMap<String, u64>,
}

impl NonceManager {
    /// Create a new nonce manager
    pub fn new() -> Self {
        Self {
            nonces: HashMap::new(),
        }
    }

    /// Get the next nonce for an account
    pub fn get_next_nonce(&mut self, account: &str) -> u64 {
        let nonce = self.nonces.entry(account.to_string()).or_insert(0);
        *nonce += 1;
        *nonce
    }

    /// Validate a nonce for an account
    pub fn validate_nonce(&self, account: &str, nonce: u64) -> bool {
        if let Some(current_nonce) = self.nonces.get(account) {
            nonce == *current_nonce + 1
        } else {
            nonce == 1
        }
    }

    /// Reset nonce for an account
    pub fn reset_nonce(&mut self, account: &str) {
        self.nonces.remove(account);
    }
}

/// Idempotency manager for preventing duplicate processing
#[derive(Debug)]
pub struct IdempotencyManager {
    /// Processed idempotency keys
    processed_keys: HashSet<String>,
}

impl IdempotencyManager {
    /// Create a new idempotency manager
    pub fn new() -> Self {
        Self {
            processed_keys: HashSet::new(),
        }
    }

    /// Check if a key has been processed
    pub fn is_processed(&self, key: &str) -> bool {
        self.processed_keys.contains(key)
    }

    /// Mark a key as processed
    pub fn mark_processed(&mut self, key: &str) {
        self.processed_keys.insert(key.to_string());
    }

    /// Remove a key from processed set (for testing)
    pub fn remove_processed(&mut self, key: &str) {
        self.processed_keys.remove(key);
    }
}

/// Ledger manager for double-entry bookkeeping
#[derive(Debug)]
pub struct LedgerManager {
    /// All accounts
    accounts: HashMap<String, Account>,
    /// All ledger entries
    entries: Vec<LedgerEntry>,
    /// Nonce manager
    nonce_manager: NonceManager,
    /// Idempotency manager
    idempotency_manager: IdempotencyManager,
    /// Transaction log
    transactions: Vec<Transaction>,
}

impl LedgerManager {
    /// Create a new ledger manager
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            entries: Vec::new(),
            nonce_manager: NonceManager::new(),
            idempotency_manager: IdempotencyManager::new(),
            transactions: Vec::new(),
        }
    }

    /// Create a new account
    pub fn create_account(
        &mut self,
        id: String,
        name: String,
        account_type: AccountType,
        currency: String,
    ) -> Result<(), String> {
        if self.accounts.contains_key(&id) {
            return Err("Account already exists".to_string());
        }

        let account = Account {
            id: id.clone(),
            name,
            account_type,
            balance: 0,
            currency,
            last_updated: self.current_timestamp(),
            status: AccountStatus::Active,
        };

        self.accounts.insert(id, account);
        Ok(())
    }

    /// Process a transaction with double-entry bookkeeping
    pub fn process_transaction(&mut self, mut transaction: Transaction) -> Result<(), String> {
        // Check if transaction has already been processed
        if self
            .idempotency_manager
            .is_processed(&transaction.idempotency_key)
        {
            return Err("Transaction already processed".to_string());
        }

        // For deposit transactions, validate nonce for the to_account (destination)
        // For all other transactions, validate nonce for the from_account (source)
        let nonce_account = if transaction.tx_type == TransactionType::Deposit {
            transaction.to_account.clone()
        } else {
            transaction.from_account.clone()
        };
        
        let nonce_valid = self
            .nonce_manager
            .validate_nonce(&nonce_account, transaction.nonce);

        if !nonce_valid {
            return Err("Invalid nonce".to_string());
        }

        // For deposit transactions, the from_account doesn't need to exist
        // as it represents funds coming from outside the system
        if transaction.tx_type != TransactionType::Deposit {
            // Validate accounts exist (for non-deposit transactions)
            if !self.accounts.contains_key(&transaction.from_account) {
                return Err("Source account not found".to_string());
            }

            if !self.accounts.contains_key(&transaction.to_account) {
                return Err("Destination account not found".to_string());
            }

            // Validate account status
            let from_account = self.accounts.get(&transaction.from_account).unwrap();
            if from_account.status != AccountStatus::Active {
                return Err("Source account is not active".to_string());
            }

            let to_account = self.accounts.get(&transaction.to_account).unwrap();
            if to_account.status != AccountStatus::Active {
                return Err("Destination account is not active".to_string());
            }

            // Validate currency match
            if from_account.currency != transaction.currency
                || to_account.currency != transaction.currency
            {
                return Err("Currency mismatch".to_string());
            }
        } else {
            // For deposit transactions, only validate that the to_account exists
            if !self.accounts.contains_key(&transaction.to_account) {
                return Err("Destination account not found".to_string());
            }

            let to_account = self.accounts.get(&transaction.to_account).unwrap();
            if to_account.status != AccountStatus::Active {
                return Err("Destination account is not active".to_string());
            }

            // Validate currency match
            if to_account.currency != transaction.currency {
                return Err("Currency mismatch".to_string());
            }
        }

        // Process the transaction based on type
        match transaction.tx_type {
            TransactionType::Transfer => {
                self.process_transfer(&mut transaction)?;
            }
            TransactionType::Deposit => {
                self.process_deposit(&mut transaction)?;
            }
            TransactionType::Withdrawal => {
                self.process_withdrawal(&mut transaction)?;
            }
            TransactionType::Fee => {
                self.process_fee(&mut transaction)?;
            }
            TransactionType::Interest => {
                self.process_interest(&mut transaction)?;
            }
            TransactionType::Adjustment => {
                self.process_adjustment(&mut transaction)?;
            }
        }

        // Mark transaction as processed
        self.idempotency_manager
            .mark_processed(&transaction.idempotency_key);
        
        // Update nonce for the appropriate account
        self.nonce_manager
            .nonces
            .insert(nonce_account, transaction.nonce);

        // Add transaction to log
        self.transactions.push(transaction.clone());

        Ok(())
    }

    /// Process a transfer transaction
    fn process_transfer(&mut self, transaction: &mut Transaction) -> Result<(), String> {
        // Validate sufficient funds
        let from_account_id = transaction.from_account.clone();
        let from_account_balance = self.accounts.get(&from_account_id).unwrap().balance;
        
        if from_account_balance < transaction.amount {
            transaction.status = TransactionStatus::Failed;
            return Err("Insufficient funds".to_string());
        }

        // Get all the data we need before borrowing mutably
        let timestamp = self.current_timestamp();
        let to_account_id = transaction.to_account.clone();
        let to_account_balance = self.accounts.get(&to_account_id).unwrap().balance;

        let debit_balance = from_account_balance - transaction.amount;
        let credit_balance = to_account_balance + transaction.amount;

        // Update account balances
        {
            let from_account = self.accounts.get_mut(&from_account_id).unwrap();
            from_account.balance = debit_balance;
            from_account.last_updated = timestamp;
        }

        {
            let to_account = self.accounts.get_mut(&to_account_id).unwrap();
            to_account.balance = credit_balance;
            to_account.last_updated = timestamp;
        }

        // Create ledger entries
        let debit_entry = LedgerEntry {
            id: format!("{}-debit", transaction.id),
            transaction_id: transaction.id.clone(),
            account: from_account_id.clone(),
            debit: transaction.amount,
            credit: 0,
            currency: transaction.currency.clone(),
            timestamp,
            description: format!("Transfer to {}", to_account_id),
            balance: debit_balance,
        };

        let credit_entry = LedgerEntry {
            id: format!("{}-credit", transaction.id),
            transaction_id: transaction.id.clone(),
            account: to_account_id.clone(),
            debit: 0,
            credit: transaction.amount,
            currency: transaction.currency.clone(),
            timestamp,
            description: format!("Transfer from {}", from_account_id.clone()),
            balance: credit_balance,
        };

        // Add entries to ledger
        self.entries.push(debit_entry);
        self.entries.push(credit_entry);

        transaction.status = TransactionStatus::Success;
        Ok(())
    }

    /// Process a deposit transaction
    fn process_deposit(&mut self, transaction: &mut Transaction) -> Result<(), String> {
        // For deposits, we credit the account and debit a special equity account
        let account_id = transaction.to_account.clone();
        let current_balance = self.accounts.get(&account_id).unwrap().balance;
        let new_balance = current_balance + transaction.amount;
        let timestamp = self.current_timestamp();
        
        // Update account
        let account = self.accounts.get_mut(&account_id).unwrap();
        account.balance = new_balance;
        account.last_updated = timestamp;

        // Create credit entry for the receiving account
        let credit_entry = LedgerEntry {
            id: format!("{}-credit", transaction.id),
            transaction_id: transaction.id.clone(),
            account: account_id.clone(),
            debit: 0,
            credit: transaction.amount,
            currency: transaction.currency.clone(),
            timestamp,
            description: "Deposit".to_string(),
            balance: new_balance,
        };

        // Create debit entry for the equity account
        let debit_entry = LedgerEntry {
            id: format!("{}-debit", transaction.id),
            transaction_id: transaction.id.clone(),
            account: "__equity_deposit__".to_string(), // Special internal account for equity
            debit: transaction.amount,
            credit: 0,
            currency: transaction.currency.clone(),
            timestamp,
            description: format!("Equity increase from deposit to {}", account_id),
            balance: transaction.amount,
        };

        self.entries.push(credit_entry);
        self.entries.push(debit_entry);
        transaction.status = TransactionStatus::Success;
        Ok(())
    }

    /// Process a withdrawal transaction
    fn process_withdrawal(&mut self, transaction: &mut Transaction) -> Result<(), String> {
        // Validate sufficient funds
        let account_id = transaction.from_account.clone();
        let current_balance = self.accounts.get(&account_id).unwrap().balance;
        
        if current_balance < transaction.amount {
            transaction.status = TransactionStatus::Failed;
            return Err("Insufficient funds".to_string());
        }

        // Debit the account
        let new_balance = current_balance - transaction.amount;
        let timestamp = self.current_timestamp();
        
        // Update account
        let account = self.accounts.get_mut(&account_id).unwrap();
        account.balance = new_balance;
        account.last_updated = timestamp;

        // Create debit entry
        let debit_entry = LedgerEntry {
            id: format!("{}-debit", transaction.id),
            transaction_id: transaction.id.clone(),
            account: account_id,
            debit: transaction.amount,
            credit: 0,
            currency: transaction.currency.clone(),
            timestamp,
            description: "Withdrawal".to_string(),
            balance: new_balance,
        };

        self.entries.push(debit_entry);
        transaction.status = TransactionStatus::Success;
        Ok(())
    }

    /// Process a fee transaction
    fn process_fee(&mut self, transaction: &mut Transaction) -> Result<(), String> {
        // Validate sufficient funds
        let account_id = transaction.from_account.clone();
        let current_balance = self.accounts.get(&account_id).unwrap().balance;
        
        if current_balance < transaction.amount {
            transaction.status = TransactionStatus::Failed;
            return Err("Insufficient funds".to_string());
        }

        // Debit the account
        let new_balance = current_balance - transaction.amount;
        let timestamp = self.current_timestamp();
        
        // Update account
        let account = self.accounts.get_mut(&account_id).unwrap();
        account.balance = new_balance;
        account.last_updated = timestamp;

        // Create debit entry
        let debit_entry = LedgerEntry {
            id: format!("{}-debit", transaction.id),
            transaction_id: transaction.id.clone(),
            account: account_id,
            debit: transaction.amount,
            credit: 0,
            currency: transaction.currency.clone(),
            timestamp,
            description: "Fee".to_string(),
            balance: new_balance,
        };

        self.entries.push(debit_entry);
        transaction.status = TransactionStatus::Success;
        Ok(())
    }

    /// Process an interest transaction
    fn process_interest(&mut self, transaction: &mut Transaction) -> Result<(), String> {
        // Credit the account
        let account_id = transaction.to_account.clone();
        let current_balance = self.accounts.get(&account_id).unwrap().balance;
        let new_balance = current_balance + transaction.amount;
        let timestamp = self.current_timestamp();
        
        // Update account
        let account = self.accounts.get_mut(&account_id).unwrap();
        account.balance = new_balance;
        account.last_updated = timestamp;

        // Create credit entry
        let credit_entry = LedgerEntry {
            id: format!("{}-credit", transaction.id),
            transaction_id: transaction.id.clone(),
            account: account_id,
            debit: 0,
            credit: transaction.amount,
            currency: transaction.currency.clone(),
            timestamp,
            description: "Interest".to_string(),
            balance: new_balance,
        };

        self.entries.push(credit_entry);
        transaction.status = TransactionStatus::Success;
        Ok(())
    }

    /// Process an adjustment transaction
    fn process_adjustment(&mut self, transaction: &mut Transaction) -> Result<(), String> {
        let new_balance = transaction.amount;
        let account_id = transaction.to_account.clone();
        let timestamp = self.current_timestamp();
        
        // Update account
        let account = self.accounts.get_mut(&account_id).unwrap();
        account.balance = new_balance;
        account.last_updated = timestamp;

        // Create adjustment entry
        let adjustment_entry = LedgerEntry {
            id: format!("{}-adjustment", transaction.id),
            transaction_id: transaction.id.clone(),
            account: account_id,
            debit: 0,
            credit: transaction.amount,
            currency: transaction.currency.clone(),
            timestamp,
            description: "Account adjustment".to_string(),
            balance: new_balance,
        };

        self.entries.push(adjustment_entry);
        transaction.status = TransactionStatus::Success;
        Ok(())
    }

    /// Get account balance
    pub fn get_account_balance(&self, account_id: &str) -> Option<u128> {
        self.accounts.get(account_id).map(|a| a.balance)
    }

    /// Get account details
    pub fn get_account(&self, account_id: &str) -> Option<&Account> {
        self.accounts.get(account_id)
    }

    /// Get all accounts
    pub fn get_all_accounts(&self) -> Vec<&Account> {
        self.accounts.values().collect()
    }

    /// Get ledger entries for an account
    pub fn get_account_entries(&self, account_id: &str) -> Vec<&LedgerEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.account == account_id)
            .collect()
    }

    /// Get all ledger entries
    pub fn get_all_entries(&self) -> &Vec<LedgerEntry> {
        &self.entries
    }

    /// Get transaction by ID
    pub fn get_transaction(&self, transaction_id: &str) -> Option<&Transaction> {
        self.transactions
            .iter()
            .find(|tx| tx.id == transaction_id)
    }

    /// Get all transactions
    pub fn get_all_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    /// Run reconciliation job to verify ledger integrity
    pub fn run_reconciliation(&self) -> Result<ReconciliationReport, String> {
        let mut report = ReconciliationReport::new();

        // Check that all accounts have non-negative balances (for asset accounts)
        for account in self.accounts.values() {
            if matches!(account.account_type, AccountType::Asset) && account.balance > 0 {
                report.asset_accounts_positive += 1;
            } else if matches!(account.account_type, AccountType::Liability) && account.balance > 0 {
                report.liability_accounts_positive += 1;
            }
        }

        // Check that total debits equal total credits
        let total_debits: u128 = self.entries.iter().map(|entry| entry.debit).sum();
        let total_credits: u128 = self.entries.iter().map(|entry| entry.credit).sum();

        report.total_debits = total_debits;
        report.total_credits = total_credits;

        if total_debits == total_credits {
            report.balanced = true;
        } else {
            report.balanced = false;
            report.errors
                .push("Total debits do not equal total credits".to_string());
        }

        // Check for duplicate transactions
        let mut transaction_ids = HashSet::new();
        for transaction in &self.transactions {
            if !transaction_ids.insert(&transaction.id) {
                report.errors.push(format!(
                    "Duplicate transaction ID found: {}",
                    transaction.id
                ));
            }
        }

        Ok(report)
    }

    /// Run invariant tests to verify system integrity
    pub fn run_invariant_tests(&self) -> Result<InvariantTestReport, String> {
        let mut report = InvariantTestReport::new();

        // Test 1: Account balances should match sum of ledger entries
        for account in self.accounts.values() {
            let account_entries: Vec<&LedgerEntry> = self
                .entries
                .iter()
                .filter(|entry| entry.account == account.id)
                .collect();

            // Calculate balance by summing credits and subtracting debits
            let mut calculated_balance = 0u128;
            for entry in &account_entries {
                calculated_balance = calculated_balance + entry.credit - entry.debit;
            }

            if calculated_balance != account.balance {
                report.errors.push(format!(
                    "Account {} balance mismatch: stored={}, calculated={}",
                    account.id, account.balance, calculated_balance
                ));
            }
        }

        // Test 2: Every transaction should have corresponding ledger entries
        for transaction in &self.transactions {
            let transaction_entries: Vec<&LedgerEntry> = self
                .entries
                .iter()
                .filter(|entry| entry.transaction_id == transaction.id)
                .collect();

            match transaction.tx_type {
                TransactionType::Transfer => {
                    if transaction_entries.len() != 2 {
                        report.errors.push(format!(
                            "Transfer transaction {} should have 2 entries, found {}",
                            transaction.id,
                            transaction_entries.len()
                        ));
                    }
                }
                TransactionType::Deposit | TransactionType::Interest => {
                    if transaction_entries.len() != 2 {
                        report.errors.push(format!(
                            "Credit transaction {} should have 2 entries, found {}",
                            transaction.id,
                            transaction_entries.len()
                        ));
                    }
                }
                TransactionType::Withdrawal | TransactionType::Fee => {
                    if transaction_entries.len() != 1 {
                        report.errors.push(format!(
                            "Debit transaction {} should have 1 entry, found {}",
                            transaction.id,
                            transaction_entries.len()
                        ));
                    }
                }
                TransactionType::Adjustment => {
                    if transaction_entries.len() != 1 {
                        report.errors.push(format!(
                            "Adjustment transaction {} should have 1 entry, found {}",
                            transaction.id,
                            transaction_entries.len()
                        ));
                    }
                }
            }
        }

        // Test 3: Nonces should be sequential
        let mut account_nonces: HashMap<String, u64> = HashMap::new();
        for transaction in &self.transactions {
            let current_nonce = account_nonces
                .entry(transaction.from_account.clone())
                .or_insert(0);
            if transaction.nonce <= *current_nonce && transaction.nonce > 0 {
                report.errors.push(format!(
                    "Non-sequential nonce for account {}: expected >{}, got {}",
                    transaction.from_account, current_nonce, transaction.nonce
                ));
            }
            *current_nonce = transaction.nonce;
        }

        report.passed = report.errors.is_empty();
        Ok(report)
    }

    /// Get current timestamp
    fn current_timestamp(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

/// Reconciliation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconciliationReport {
    /// Whether the ledger is balanced
    pub balanced: bool,
    /// Total debit entries
    pub total_debits: u128,
    /// Total credit entries
    pub total_credits: u128,
    /// Number of asset accounts with positive balances
    pub asset_accounts_positive: u32,
    /// Number of liability accounts with positive balances
    pub liability_accounts_positive: u32,
    /// Any errors found during reconciliation
    pub errors: Vec<String>,
}

impl ReconciliationReport {
    /// Create a new reconciliation report
    pub fn new() -> Self {
        Self {
            balanced: false,
            total_debits: 0,
            total_credits: 0,
            asset_accounts_positive: 0,
            liability_accounts_positive: 0,
            errors: Vec::new(),
        }
    }
}

/// Invariant test report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvariantTestReport {
    /// Whether all tests passed
    pub passed: bool,
    /// Any errors found during testing
    pub errors: Vec<String>,
}

impl InvariantTestReport {
    /// Create a new invariant test report
    pub fn new() -> Self {
        Self {
            passed: false,
            errors: Vec::new(),
        }
    }
}

/// Risk limit for economic controls
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskLimit {
    /// User identifier
    pub user_id: String,
    /// Maximum position size
    pub max_position_size: u128,
    /// Maximum daily loss
    pub max_daily_loss: u128,
    /// Maximum leverage ratio
    pub max_leverage: f64,
    /// Slippage tolerance
    pub slippage_tolerance: f64,
}

/// Risk manager for economic controls
#[derive(Debug)]
pub struct RiskManager {
    /// Risk limits by user
    risk_limits: HashMap<String, RiskLimit>,
    /// User positions
    user_positions: HashMap<String, u128>,
    /// User daily losses
    user_daily_losses: HashMap<String, u128>,
}

impl RiskManager {
    /// Create a new risk manager
    pub fn new() -> Self {
        Self {
            risk_limits: HashMap::new(),
            user_positions: HashMap::new(),
            user_daily_losses: HashMap::new(),
        }
    }

    /// Set risk limits for a user
    pub fn set_risk_limits(&mut self, limits: RiskLimit) {
        self.risk_limits.insert(limits.user_id.clone(), limits);
    }

    /// Check if a transaction complies with risk limits
    pub fn check_risk_limits(
        &self,
        user_id: &str,
        transaction_amount: u128,
        transaction_type: &TransactionType,
    ) -> Result<(), String> {
        if let Some(limits) = self.risk_limits.get(user_id) {
            // Check position size limit
            let current_position = self.user_positions.get(user_id).unwrap_or(&0);
            let new_position = match transaction_type {
                TransactionType::Transfer | TransactionType::Deposit => {
                    current_position + transaction_amount
                }
                TransactionType::Withdrawal => current_position.saturating_sub(transaction_amount),
                _ => *current_position,
            };

            if new_position > limits.max_position_size {
                return Err("Transaction exceeds maximum position size".to_string());
            }

            // Check daily loss limit
            let daily_loss = self.user_daily_losses.get(user_id).unwrap_or(&0);
            if *daily_loss > limits.max_daily_loss {
                return Err("Transaction exceeds maximum daily loss limit".to_string());
            }
        }

        Ok(())
    }

    /// Update user position after successful transaction
    pub fn update_position(&mut self, user_id: &str, amount: u128, tx_type: &TransactionType) {
        let position = self.user_positions.entry(user_id.to_string()).or_insert(0);
        match tx_type {
            TransactionType::Transfer | TransactionType::Deposit => *position += amount,
            TransactionType::Withdrawal => *position = position.saturating_sub(amount),
            _ => {}
        }
    }

    /// Record a loss for a user
    pub fn record_loss(&mut self, user_id: &str, amount: u128) {
        let loss = self.user_daily_losses.entry(user_id.to_string()).or_insert(0);
        *loss += amount;
    }
}

/// Economic scenario for game theory simulations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicScenario {
    /// Scenario name
    pub name: String,
    /// Description of the scenario
    pub description: String,
    /// Probability of occurrence (0.0 - 1.0)
    pub probability: f64,
    /// Expected impact on positions
    pub impact: f64,
    /// Cost to execute attack
    pub attack_cost: u128,
}

/// Game theory simulator
#[derive(Debug)]
pub struct GameTheorySimulator {
    /// Economic scenarios to simulate
    scenarios: Vec<EconomicScenario>,
}

impl GameTheorySimulator {
    /// Create a new game theory simulator
    pub fn new() -> Self {
        Self {
            scenarios: Vec::new(),
        }
    }

    /// Add an economic scenario
    pub fn add_scenario(&mut self, scenario: EconomicScenario) {
        self.scenarios.push(scenario);
    }

    /// Run simulation for a user's position
    pub fn simulate_attack(&self, user_id: &str, position: u128) -> SimulationResult {
        let mut results = Vec::new();

        for scenario in &self.scenarios {
            let expected_loss = (position as f64 * scenario.impact) as u128;
            let expected_gain = expected_loss; // Simplified model
            let net_result = expected_gain as i128 - scenario.attack_cost as i128;

            results.push(ScenarioResult {
                scenario_name: scenario.name.clone(),
                probability: scenario.probability,
                expected_loss,
                attack_cost: scenario.attack_cost,
                net_result,
            });
        }

        SimulationResult {
            user_id: user_id.to_string(),
            position,
            scenario_results: results,
        }
    }
}

/// Result of a single scenario simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioResult {
    /// Scenario name
    pub scenario_name: String,
    /// Probability of occurrence
    pub probability: f64,
    /// Expected loss from scenario
    pub expected_loss: u128,
    /// Cost to execute attack
    pub attack_cost: u128,
    /// Net result (positive = profitable, negative = unprofitable)
    pub net_result: i128,
}

/// Result of a complete simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResult {
    /// User identifier
    pub user_id: String,
    /// Current position size
    pub position: u128,
    /// Results for each scenario
    pub scenario_results: Vec<ScenarioResult>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nonce_manager() {
        let mut nonce_manager = NonceManager::new();

        // First nonce should be 1
        assert_eq!(nonce_manager.get_next_nonce("account1"), 1);

        // Second nonce should be 2
        assert_eq!(nonce_manager.get_next_nonce("account1"), 2);

        // First nonce for a new account should be 1
        assert_eq!(nonce_manager.get_next_nonce("account2"), 1);

        // Validate nonces
        assert!(nonce_manager.validate_nonce("account1", 3));
        assert!(!nonce_manager.validate_nonce("account1", 2)); // Already used
        assert!(!nonce_manager.validate_nonce("account1", 1)); // Already used
    }

    #[test]
    fn test_idempotency_manager() {
        let mut idempotency_manager = IdempotencyManager::new();

        // Key should not be processed initially
        assert!(!idempotency_manager.is_processed("key1"));

        // Mark key as processed
        idempotency_manager.mark_processed("key1");

        // Key should now be processed
        assert!(idempotency_manager.is_processed("key1"));

        // Different key should not be processed
        assert!(!idempotency_manager.is_processed("key2"));
    }

    #[test]
    fn test_ledger_manager_account_creation() {
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
            metadata: HashMap::new(),
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
            metadata: HashMap::new(),
        };

        let result = ledger_manager.process_transaction(transfer);
        println!("Transfer result: {:?}", result);
        if let Err(e) = &result {
            println!("Transfer error: {}", e);
            // Let's also print the nonce manager state
            println!("Nonce manager state: {:?}", ledger_manager.nonce_manager.nonces);
        }
        assert!(result.is_ok());

        // Check balances
        assert_eq!(ledger_manager.get_account_balance("account1"), Some(500));
        assert_eq!(ledger_manager.get_account_balance("account2"), Some(500));

        // Check ledger entries
        let account1_entries = ledger_manager.get_account_entries("account1");
        assert_eq!(account1_entries.len(), 2); // 1 deposit credit, 1 transfer debit

        let account2_entries = ledger_manager.get_account_entries("account2");
        assert_eq!(account2_entries.len(), 1); // 1 transfer credit
    }

    #[test]
    fn test_idempotency_prevention() {
        let mut ledger_manager = LedgerManager::new();

        // Create accounts
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
            metadata: HashMap::new(),
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
            metadata: HashMap::new(),
        };

        assert!(ledger_manager.process_transaction(deposit_tx).is_ok());

        // Try to process a transaction with an old nonce (should fail)
        let transfer_tx = Transaction {
            id: "tx2".to_string(),
            tx_type: TransactionType::Transfer,
            amount: 500,
            currency: "USD".to_string(),
            timestamp: 1234567891,
            from_account: "account1".to_string(),
            to_account: "account2".to_string(),
            status: TransactionStatus::Pending,
            nonce: 1, // Same as previous transaction from "system" account, but should be 1 for account1
            idempotency_key: "transfer1".to_string(),
            metadata: HashMap::new(),
        };

        assert!(ledger_manager.process_transaction(transfer_tx).is_err());

        // Process transaction with correct nonce
        let transfer_tx2 = Transaction {
            id: "tx3".to_string(),
            tx_type: TransactionType::Transfer,
            amount: 500,
            currency: "USD".to_string(),
            timestamp: 1234567892,
            from_account: "account1".to_string(),
            to_account: "account2".to_string(),
            status: TransactionStatus::Pending,
            nonce: 2, // Second transaction from account1
            idempotency_key: "transfer2".to_string(),
            metadata: HashMap::new(),
        };

        assert!(ledger_manager.process_transaction(transfer_tx2).is_ok());

        // Check balances
        assert_eq!(ledger_manager.get_account_balance("account1"), Some(500));
        assert_eq!(ledger_manager.get_account_balance("account2"), Some(500));
    }

    #[test]
    fn test_reconciliation() {
        let mut ledger_manager = LedgerManager::new();

        // Create accounts
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
            metadata: HashMap::new(),
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
            metadata: HashMap::new(),
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
            nonce: 2, // Second transaction from asset1
            idempotency_key: "transfer1".to_string(),
            metadata: HashMap::new(),
        };

        assert!(ledger_manager.process_transaction(transfer).is_ok());

        // Run reconciliation
        let report = ledger_manager.run_reconciliation().unwrap();

        // Check that ledger is balanced
        assert!(report.balanced);
        assert_eq!(report.total_debits, report.total_credits);

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
            metadata: HashMap::new(),
        };

        let deposit_result = ledger_manager.process_transaction(deposit);
        println!("Deposit result: {:?}", deposit_result);
        if let Err(e) = &deposit_result {
            println!("Deposit error: {}", e);
        }
        assert!(deposit_result.is_ok());

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
            metadata: HashMap::new(),
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
}