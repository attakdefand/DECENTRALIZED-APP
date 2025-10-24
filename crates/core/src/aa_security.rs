//! Account Abstraction Security Module
//!
//! This module implements security measures for account abstraction including
//! session key management, UserOperation validation, and paymaster security.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a session key for account abstraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionKey {
    /// Unique identifier for the session
    pub id: String,
    /// Owner of the session key
    pub owner: String,
    /// Permissions encoded as bytes
    pub permissions: Vec<u8>,
    /// Timestamp when the key becomes valid
    pub valid_after: u64,
    /// Timestamp when the key expires
    pub valid_until: u64,
    /// Current usage count
    pub use_count: u32,
    /// Maximum number of times the key can be used
    pub max_uses: u32,
    /// Whether the key has been revoked
    pub revoked: bool,
}

/// Represents a UserOperation in account abstraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserOperation {
    /// Sender address
    pub sender: String,
    /// Nonce to prevent replay attacks
    pub nonce: u64,
    /// Initialization code for deploying account
    pub init_code: Vec<u8>,
    /// Call data for the operation
    pub call_data: Vec<u8>,
    /// Gas limit for the call
    pub call_gas_limit: u64,
    /// Gas limit for verification
    pub verification_gas_limit: u64,
    /// Pre-verification gas
    pub pre_verification_gas: u64,
    /// Maximum fee per gas
    pub max_fee_per_gas: u128,
    /// Maximum priority fee per gas
    pub max_priority_fee_per_gas: u128,
    /// Paymaster and data
    pub paymaster_and_data: Vec<u8>,
    /// Signature
    pub signature: Vec<u8>,
}

/// Represents a paymaster in account abstraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paymaster {
    /// Address of the paymaster
    pub address: String,
    /// Stake amount
    pub stake: u128,
    /// Unstake delay in seconds
    pub unstake_delay: u64,
    /// Current balance
    pub balance: u128,
    /// Daily budget limit
    pub daily_budget: u128,
    /// Used budget today
    pub used_today: u128,
    /// Last reset timestamp
    pub last_reset: u64,
}

/// Session Key Manager for account abstraction
#[derive(Debug, Clone)]
pub struct SessionKeyManager {
    /// Mapping from session ID to session key
    session_keys: HashMap<String, SessionKey>,
    /// Mapping from owner to their session key count
    owner_session_count: HashMap<String, u32>,
}

/// Paymaster Security Manager
#[derive(Debug, Clone)]
pub struct PaymasterSecurityManager {
    /// Mapping from address to paymaster
    paymasters: HashMap<String, Paymaster>,
    /// Daily budget limit for all paymasters
    global_daily_budget: u128,
    /// Used global budget today
    global_used_today: u128,
}

impl SessionKeyManager {
    /// Create a new session key manager
    pub fn new() -> Self {
        Self {
            session_keys: HashMap::new(),
            owner_session_count: HashMap::new(),
        }
    }
}

// Add Default implementation
impl Default for SessionKeyManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionKeyManager {
    /// Create a new session key
    pub fn create_session_key(
        &mut self,
        id: String,
        owner: String,
        permissions: Vec<u8>,
        valid_after: u64,
        valid_until: u64,
        max_uses: u32,
    ) -> Result<(), String> {
        // Validate parameters
        if valid_until <= valid_after {
            return Err("Invalid time bounds".to_string());
        }

        if max_uses == 0 {
            return Err("Max uses must be positive".to_string());
        }

        if self.session_keys.contains_key(&id) {
            return Err("Session ID already exists".to_string());
        }

        let session_key = SessionKey {
            id: id.clone(),
            owner: owner.clone(),
            permissions,
            valid_after,
            valid_until,
            use_count: 0,
            max_uses,
            revoked: false,
        };

        self.session_keys.insert(id.clone(), session_key);
        let count = self.owner_session_count.entry(owner.clone()).or_insert(0);
        *count += 1;

        Ok(())
    }

    /// Validate a session key
    pub fn validate_session_key(&self, id: &str, _signature: &[u8]) -> bool {
        let session_key = match self.session_keys.get(id) {
            Some(key) => key,
            None => return false,
        };

        if session_key.revoked {
            return false;
        }

        // Clone the necessary data to avoid borrowing issues
        let valid_after = session_key.valid_after;
        let valid_until = session_key.valid_until;
        let use_count = session_key.use_count;
        let max_uses = session_key.max_uses;

        let current_time = self.current_timestamp();

        current_time >= valid_after && current_time <= valid_until && use_count < max_uses
    }

    /// Use a session key (increment usage count)
    pub fn use_session_key(&mut self, id: &str) -> Result<(), String> {
        // First check if the session key exists and get its current use count
        let use_count = {
            let session_key = match self.session_keys.get(id) {
                Some(key) => key,
                None => return Err("Session key does not exist".to_string()),
            };

            if session_key.revoked {
                return Err("Session key revoked".to_string());
            }

            // Check if usage limit would be exceeded
            if session_key.use_count >= session_key.max_uses {
                return Err("Session usage limit exceeded".to_string());
            }

            session_key.use_count
        };

        // Now update the session key
        let session_key = self.session_keys.get_mut(id).unwrap();
        session_key.use_count = use_count + 1;
        Ok(())
    }

    /// Revoke a session key
    pub fn revoke_session_key(&mut self, id: &str, requester: &str) -> Result<(), String> {
        let session_key = match self.session_keys.get_mut(id) {
            Some(key) => key,
            None => return Err("Session key does not exist".to_string()),
        };

        if session_key.owner != requester {
            return Err("Not owner".to_string());
        }

        if session_key.revoked {
            return Err("Already revoked".to_string());
        }

        session_key.revoked = true;
        let count = self
            .owner_session_count
            .entry(session_key.owner.clone())
            .or_insert(0);
        if *count > 0 {
            *count -= 1;
        }

        Ok(())
    }

    /// Get session key details
    pub fn get_session_key(&self, id: &str) -> Option<&SessionKey> {
        self.session_keys.get(id)
    }

    /// Check if a session key is valid
    pub fn is_session_key_valid(&self, id: &str) -> bool {
        let session_key = match self.session_keys.get(id) {
            Some(key) => key,
            None => return false,
        };

        if session_key.revoked {
            return false;
        }

        let current_time = self.current_timestamp();

        current_time >= session_key.valid_after
            && current_time <= session_key.valid_until
            && session_key.use_count < session_key.max_uses
    }

    /// Get current timestamp
    fn current_timestamp(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

impl PaymasterSecurityManager {
    /// Create a new paymaster security manager
    pub fn new(global_daily_budget: u128) -> Self {
        Self {
            paymasters: HashMap::new(),
            global_daily_budget,
            global_used_today: 0,
        }
    }

    /// Add a new paymaster
    pub fn add_paymaster(
        &mut self,
        address: String,
        stake: u128,
        unstake_delay: u64,
        daily_budget: u128,
    ) -> Result<(), String> {
        if self.paymasters.contains_key(&address) {
            return Err("Paymaster already exists".to_string());
        }

        let paymaster = Paymaster {
            address: address.clone(),
            stake,
            unstake_delay,
            balance: 0,
            daily_budget,
            used_today: 0,
            last_reset: self.current_timestamp(),
        };

        self.paymasters.insert(address, paymaster);
        Ok(())
    }

    /// Validate paymaster can sponsor a UserOperation
    pub fn validate_paymaster_sponsorship(
        &mut self,
        paymaster_address: &str,
        max_cost: u128,
    ) -> Result<(), String> {
        // Reset daily budgets if needed
        self.reset_daily_budgets();

        let paymaster = match self.paymasters.get_mut(paymaster_address) {
            Some(p) => p,
            None => return Err("Paymaster not found".to_string()),
        };

        // Check global budget
        if self.global_used_today + max_cost > self.global_daily_budget {
            return Err("Global budget exceeded".to_string());
        }

        // Check paymaster balance
        if paymaster.balance < max_cost {
            return Err("Insufficient balance".to_string());
        }

        // Check paymaster daily budget
        if paymaster.used_today + max_cost > paymaster.daily_budget {
            return Err("Paymaster daily budget exceeded".to_string());
        }

        // Update usage
        paymaster.used_today += max_cost;
        self.global_used_today += max_cost;

        Ok(())
    }

    /// Handle post-operation processing for paymaster
    pub fn post_operation(
        &mut self,
        paymaster_address: &str,
        actual_gas_cost: u128,
    ) -> Result<(), String> {
        let paymaster = match self.paymasters.get_mut(paymaster_address) {
            Some(p) => p,
            None => return Err("Paymaster not found".to_string()),
        };

        // Deduct actual cost from balance
        if paymaster.balance < actual_gas_cost {
            return Err("Insufficient balance for actual cost".to_string());
        }

        paymaster.balance -= actual_gas_cost;
        Ok(())
    }

    /// Add funds to paymaster balance
    pub fn add_funds(&mut self, paymaster_address: &str, amount: u128) -> Result<(), String> {
        let paymaster = match self.paymasters.get_mut(paymaster_address) {
            Some(p) => p,
            None => return Err("Paymaster not found".to_string()),
        };

        paymaster.balance += amount;
        Ok(())
    }

    /// Reset daily budgets if a new day has started
    fn reset_daily_budgets(&mut self) {
        let current_time = self.current_timestamp();
        let current_day = current_time / 86400; // Seconds in a day

        for paymaster in self.paymasters.values_mut() {
            let last_reset_day = paymaster.last_reset / 86400;
            if current_day > last_reset_day {
                paymaster.used_today = 0;
                paymaster.last_reset = current_time;
            }
        }

        let global_last_reset_day = (self.global_used_today / 86400) as u64;
        if current_day > global_last_reset_day {
            self.global_used_today = 0;
        }
    }

    /// Get paymaster details
    pub fn get_paymaster(&self, address: &str) -> Option<&Paymaster> {
        self.paymasters.get(address)
    }

    /// Get current timestamp
    fn current_timestamp(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

/// Validate a UserOperation
pub fn validate_user_operation(user_op: &UserOperation) -> Result<(), String> {
    // Validate gas limits
    if user_op.call_gas_limit == 0 {
        return Err("Call gas limit must be positive".to_string());
    }

    if user_op.verification_gas_limit == 0 {
        return Err("Verification gas limit must be positive".to_string());
    }

    if user_op.pre_verification_gas == 0 {
        return Err("Pre-verification gas must be positive".to_string());
    }

    // Validate fees
    if user_op.max_fee_per_gas == 0 {
        return Err("Max fee per gas must be positive".to_string());
    }

    if user_op.max_priority_fee_per_gas > user_op.max_fee_per_gas {
        return Err("Max priority fee per gas cannot exceed max fee per gas".to_string());
    }

    // Validate signature is present
    if user_op.signature.is_empty() {
        return Err("Signature is required".to_string());
    }

    Ok(())
}

/// Detect suspicious UserOperations
pub fn detect_suspicious_userop(user_op: &UserOperation) -> Vec<String> {
    let mut issues = Vec::new();

    // Check for extremely high gas limits
    if user_op.call_gas_limit > 10_000_000 {
        issues.push("Extremely high call gas limit".to_string());
    }

    if user_op.verification_gas_limit > 5_000_000 {
        issues.push("Extremely high verification gas limit".to_string());
    }

    // Check for extremely high fees
    if user_op.max_fee_per_gas > 1_000_000_000_000_000_000 {
        // 1 ETH in wei
        issues.push("Extremely high max fee per gas".to_string());
    }

    // Check for suspicious call data patterns
    if user_op.call_data.len() > 10_000 {
        issues.push("Extremely large call data".to_string());
    }

    issues
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_key_manager() {
        let mut manager = SessionKeyManager::new();

        // Get current time for valid time bounds
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Create a session key with valid time bounds
        let result = manager.create_session_key(
            "test_key".to_string(),
            "0x1234".to_string(),
            vec![1, 2, 3, 4],
            current_time - 1000, // Valid from 1000 seconds ago
            current_time + 1000, // Valid until 1000 seconds from now
            5,
        );
        assert!(result.is_ok());

        // Validate session key
        assert!(manager.validate_session_key("test_key", &[1, 2, 3, 4]));

        // Use session key
        assert!(manager.use_session_key("test_key").is_ok());

        // Check usage count
        let session_key = manager.get_session_key("test_key").unwrap();
        assert_eq!(session_key.use_count, 1);

        // Revoke session key
        assert!(manager.revoke_session_key("test_key", "0x1234").is_ok());
        assert!(!manager.is_session_key_valid("test_key"));
    }

    #[test]
    fn test_paymaster_security_manager() {
        let mut manager = PaymasterSecurityManager::new(1000000);

        // Add a paymaster
        let result = manager.add_paymaster(
            "0x5678".to_string(),
            1000000000000000000, // 1 ETH stake
            604800,              // 7 days
            100000,              // Daily budget
        );
        assert!(result.is_ok());

        // Add funds
        assert!(manager.add_funds("0x5678", 50000).is_ok());

        // Validate sponsorship
        assert!(manager
            .validate_paymaster_sponsorship("0x5678", 10000)
            .is_ok());

        // Check updated usage
        let paymaster = manager.get_paymaster("0x5678").unwrap();
        assert_eq!(paymaster.used_today, 10000);
        assert_eq!(paymaster.balance, 50000); // Balance should still be 50000, only used_today is updated
    }

    #[test]
    fn test_user_operation_validation() {
        let user_op = UserOperation {
            sender: "0x1234".to_string(),
            nonce: 1,
            init_code: vec![],
            call_data: vec![1, 2, 3],
            call_gas_limit: 100000,
            verification_gas_limit: 50000,
            pre_verification_gas: 21000,
            max_fee_per_gas: 1000000000,
            max_priority_fee_per_gas: 500000000,
            paymaster_and_data: vec![],
            signature: vec![1, 2, 3, 4],
        };

        assert!(validate_user_operation(&user_op).is_ok());
    }

    #[test]
    fn test_suspicious_userop_detection() {
        let user_op = UserOperation {
            sender: "0x1234".to_string(),
            nonce: 1,
            init_code: vec![],
            call_data: vec![0; 15000],       // Large call data
            call_gas_limit: 15000000,        // Extremely high gas limit
            verification_gas_limit: 6000000, // Extremely high verification gas
            pre_verification_gas: 21000,
            max_fee_per_gas: 2000000000000000000, // 2 ETH in wei
            max_priority_fee_per_gas: 1000000000000000000, // 1 ETH in wei
            paymaster_and_data: vec![],
            signature: vec![1, 2, 3, 4],
        };

        let issues = detect_suspicious_userop(&user_op);
        assert_eq!(issues.len(), 4); // Should detect 4 issues
    }
}
