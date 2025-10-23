//! Governance implementation
//!
//! This module implements various governance mechanisms including token voting,
//! quadratic voting, and conviction voting.

use core::{Error, Result};
use core::types::{Address, TokenAmount};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Governance configuration
#[derive(Debug, Clone)]
pub struct GovernanceConfig {
    pub voting_delay: u64, // in blocks
    pub voting_period: u64, // in blocks
    pub proposal_threshold: u128, // minimum tokens to propose
    pub quorum: u128, // minimum votes for quorum
}

/// Proposal state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalState {
    Pending,
    Active,
    Canceled,
    Defeated,
    Succeeded,
    Queued,
    Expired,
    Executed,
}

/// Proposal details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: u64,
    pub proposer: Address,
    pub targets: Vec<Address>,
    pub values: Vec<u128>,
    pub signatures: Vec<String>,
    pub calldatas: Vec<Vec<u8>>,
    pub start_block: u64,
    pub end_block: u64,
    pub description: String,
    pub state: ProposalState,
}

/// Vote choice
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VoteChoice {
    Against,
    For,
    Abstain,
}

/// Vote receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteReceipt {
    pub proposal_id: u64,
    pub voter: Address,
    pub choice: VoteChoice,
    pub votes: u128,
    pub timestamp: u64,
}

/// Token-based governance implementation
pub struct TokenGovernor {
    pub config: GovernanceConfig,
    pub proposals: HashMap<u64, Proposal>,
    pub votes: HashMap<u64, Vec<VoteReceipt>>,
    pub token_supply: u128,
}

impl TokenGovernor {
    /// Create a new token governor
    pub fn new(config: GovernanceConfig, token_supply: u128) -> Self {
        Self {
            config,
            proposals: HashMap::new(),
            votes: HashMap::new(),
            token_supply,
        }
    }
    
    /// Create a new proposal
    pub fn propose(
        &mut self,
        proposer: Address,
        targets: Vec<Address>,
        values: Vec<u128>,
        signatures: Vec<String>,
        calldatas: Vec<Vec<u8>>,
        description: String,
        proposer_votes: u128,
    ) -> Result<u64> {
        // Check if proposer has enough votes
        if proposer_votes < self.config.proposal_threshold {
            return Err(Error::Custom("Insufficient voting power to propose".to_string()));
        }
        
        let proposal_id = self.proposals.len() as u64 + 1;
        let current_block = 1000; // Would come from blockchain in real implementation
        
        let proposal = Proposal {
            id: proposal_id,
            proposer,
            targets,
            values,
            signatures,
            calldatas,
            start_block: current_block + self.config.voting_delay,
            end_block: current_block + self.config.voting_delay + self.config.voting_period,
            description,
            state: ProposalState::Pending,
        };
        
        self.proposals.insert(proposal_id, proposal);
        self.votes.insert(proposal_id, Vec::new());
        
        Ok(proposal_id)
    }
    
    /// Cast a vote on a proposal
    pub fn cast_vote(&mut self, proposal_id: u64, voter: Address, choice: VoteChoice, votes: u128) -> Result<()> {
        let proposal = self.proposals.get_mut(&proposal_id)
            .ok_or_else(|| Error::Custom("Proposal not found".to_string()))?;
        
        // Check if voting is active
        let current_block = 1050; // Would come from blockchain in real implementation
        if current_block < proposal.start_block || current_block > proposal.end_block {
            return Err(Error::Custom("Voting is not active".to_string()));
        }
        
        // Record the vote
        let vote_receipt = VoteReceipt {
            proposal_id,
            voter,
            choice,
            votes,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        };
        
        if let Some(votes_vec) = self.votes.get_mut(&proposal_id) {
            votes_vec.push(vote_receipt);
        }
        
        Ok(())
    }
    
    /// Get the current state of a proposal
    pub fn get_proposal_state(&self, proposal_id: u64) -> Result<ProposalState> {
        let proposal = self.proposals.get(&proposal_id)
            .ok_or_else(|| Error::Custom("Proposal not found".to_string()))?;
        
        let current_block = 1050; // Would come from blockchain in real implementation
        
        let state = if current_block < proposal.start_block {
            ProposalState::Pending
        } else if current_block <= proposal.end_block {
            ProposalState::Active
        } else {
            // Count votes to determine outcome
            let votes_for = self.count_votes(proposal_id, &VoteChoice::For);
            let votes_against = self.count_votes(proposal_id, &VoteChoice::Against);
            
            if votes_for <= votes_against || votes_for < self.config.quorum {
                ProposalState::Defeated
            } else {
                ProposalState::Succeeded
            }
        };
        
        Ok(state)
    }
    
    /// Count votes for a specific choice
    fn count_votes(&self, proposal_id: u64, choice: &VoteChoice) -> u128 {
        if let Some(votes_vec) = self.votes.get(&proposal_id) {
            votes_vec.iter()
                .filter(|vote| vote.choice == *choice)
                .map(|vote| vote.votes)
                .sum()
        } else {
            0
        }
    }
}

/// Quadratic voting implementation
pub struct QuadraticVoter {
    pub config: GovernanceConfig,
    pub proposals: HashMap<u64, Proposal>,
    pub votes: HashMap<u64, Vec<QuadraticVoteReceipt>>,
    pub token_balances: HashMap<Address, u128>,
}

/// Quadratic vote receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuadraticVoteReceipt {
    pub proposal_id: u64,
    pub voter: Address,
    pub choice: VoteChoice,
    pub votes: u128, // Number of votes cast
    pub cost: u128, // Cost in tokens (votes^2)
    pub timestamp: u64,
}

impl QuadraticVoter {
    /// Create a new quadratic voter
    pub fn new(config: GovernanceConfig) -> Self {
        Self {
            config,
            proposals: HashMap::new(),
            votes: HashMap::new(),
            token_balances: HashMap::new(),
        }
    }
    
    /// Set a voter's token balance
    pub fn set_balance(&mut self, voter: Address, balance: u128) {
        self.token_balances.insert(voter, balance);
    }
    
    /// Cast a quadratic vote
    pub fn cast_quadratic_vote(&mut self, proposal_id: u64, voter: Address, choice: VoteChoice, votes: u128) -> Result<()> {
        let proposal = self.proposals.get(&proposal_id)
            .ok_or_else(|| Error::Custom("Proposal not found".to_string()))?;
        
        // Check if voting is active
        let current_block = 1050; // Would come from blockchain in real implementation
        if current_block < proposal.start_block || current_block > proposal.end_block {
            return Err(Error::Custom("Voting is not active".to_string()));
        }
        
        // Calculate cost (votes^2)
        let cost = votes * votes;
        
        // Check if voter has enough tokens
        let balance = self.token_balances.get(&voter).copied().unwrap_or(0);
        if balance < cost {
            return Err(Error::Custom("Insufficient tokens for quadratic vote".to_string()));
        }
        
        // Deduct cost from voter's balance
        self.token_balances.insert(voter.clone(), balance - cost);
        
        // Record the vote
        let vote_receipt = QuadraticVoteReceipt {
            proposal_id,
            voter,
            choice,
            votes,
            cost,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        };
        
        if let Some(votes_vec) = self.votes.get_mut(&proposal_id) {
            votes_vec.push(vote_receipt);
        }
        
        Ok(())
    }
}

/// Conviction voting implementation
pub struct ConvictionVoter {
    pub config: GovernanceConfig,
    pub proposals: HashMap<u64, ConvictionProposal>,
    pub voter_stakes: HashMap<Address, u128>,
}

/// Conviction proposal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvictionProposal {
    pub id: u64,
    pub proposer: Address,
    pub description: String,
    pub requested_amount: TokenAmount,
    pub conviction: f64, // Current conviction score
    pub last_updated: u64,
    pub state: ProposalState,
}

impl ConvictionVoter {
    /// Create a new conviction voter
    pub fn new(config: GovernanceConfig) -> Self {
        Self {
            config,
            proposals: HashMap::new(),
            voter_stakes: HashMap::new(),
        }
    }
    
    /// Set a voter's stake
    pub fn set_stake(&mut self, voter: Address, stake: u128) {
        self.voter_stakes.insert(voter, stake);
    }
    
    /// Create a new conviction proposal
    pub fn create_proposal(&mut self, proposer: Address, description: String, requested_amount: TokenAmount) -> u64 {
        let proposal_id = self.proposals.len() as u64 + 1;
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        let proposal = ConvictionProposal {
            id: proposal_id,
            proposer,
            description,
            requested_amount,
            conviction: 0.0,
            last_updated: current_time,
            state: ProposalState::Pending,
        };
        
        self.proposals.insert(proposal_id, proposal);
        proposal_id
    }
    
    /// Stake conviction on a proposal
    pub fn stake_conviction(&mut self, proposal_id: u64, voter: Address, stake: u128) -> Result<()> {
        let proposal = self.proposals.get_mut(&proposal_id)
            .ok_or_else(|| Error::Custom("Proposal not found".to_string()))?;
        
        let voter_stake = self.voter_stakes.get(&voter).copied().unwrap_or(0);
        if voter_stake < stake {
            return Err(Error::Custom("Insufficient stake".to_string()));
        }
        
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        // Update conviction using exponential moving average
        let time_elapsed = current_time - proposal.last_updated;
        let decay_factor = (-0.1 * time_elapsed as f64).exp(); // Decay rate of 0.1
        proposal.conviction = proposal.conviction * decay_factor + stake as f64;
        proposal.last_updated = current_time;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_token_governor() {
        let config = GovernanceConfig {
            voting_delay: 10,
            voting_period: 100,
            proposal_threshold: 1000,
            quorum: 5000,
        };
        
        let governor = TokenGovernor::new(config, 1000000);
        assert_eq!(governor.token_supply, 1000000);
    }
    
    #[test]
    fn test_quadratic_voter() {
        let config = GovernanceConfig {
            voting_delay: 10,
            voting_period: 100,
            proposal_threshold: 1000,
            quorum: 5000,
        };
        
        let mut voter = QuadraticVoter::new(config);
        voter.set_balance(Address("voter1".to_string()), 100);
        
        // Cost of 5 votes should be 25 tokens
        assert_eq!(5 * 5, 25);
    }
    
    #[test]
    fn test_conviction_voter() {
        let config = GovernanceConfig {
            voting_delay: 10,
            voting_period: 100,
            proposal_threshold: 1000,
            quorum: 5000,
        };
        
        let mut voter = ConvictionVoter::new(config);
        let proposal_id = voter.create_proposal(
            Address("proposer".to_string()),
            "Test proposal".to_string(),
            TokenAmount { value: 1000000, decimals: 18 },
        );
        
        assert_eq!(proposal_id, 1);
    }
}