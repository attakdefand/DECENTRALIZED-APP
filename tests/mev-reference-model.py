#!/usr/bin/env python3
"""
Reference Model for MEV & Fairness Order Protection
This model implements the same logic as the Solidity MEV contracts
for differential testing purposes.
"""

from decimal import Decimal
from typing import List, Dict, Optional, Tuple
from dataclasses import dataclass
from collections import deque
import time


@dataclass
class Commitment:
    commitment_hash: str
    timestamp: int
    block_number: int
    user: str
    revealed: bool
    data: bytes
    gas_limit: int
    max_fee_per_gas: int


@dataclass
class Batch:
    batch_id: int
    start_time: int
    end_time: int
    orders: List[dict]
    executed: bool
    clearing_price: Decimal


class MEVReferenceModel:
    """
    Reference model for MEV & Fairness Order Protection
    Implements commit-reveal scheme, batch auctions, and anti-sandwich protection
    """

    def __init__(self, commit_phase_duration: int = 5, reveal_phase_duration: int = 5, 
                 batch_interval: int = 12, anti_sandwich_window: int = 2):
        """
        Initialize MEV reference model
        
        Args:
            commit_phase_duration: Duration of commit phase in seconds
            reveal_phase_duration: Duration of reveal phase in seconds
            batch_interval: Interval between batches in seconds
            anti_sandwich_window: Anti-sandwich protection window in seconds
        """
        self.commitments: Dict[str, Commitment] = {}
        self.user_stakes: Dict[str, int] = {}
        self.last_reveal_time: Dict[str, int] = {}
        self.batches: Dict[int, Batch] = {}
        
        self.commit_phase_duration = commit_phase_duration
        self.reveal_phase_duration = reveal_phase_duration
        self.batch_interval = batch_interval
        self.anti_sandwich_window = anti_sandwich_window
        self.min_gas_limit = 50000
        self.max_gas_limit = 10000000
        self.last_batch_end_time = int(time.time())
        
        self.current_timestamp = int(time.time())
        self.current_block_number = 1000000

    def add_stake(self, user: str, amount: int):
        """Add stake for a user"""
        if user not in self.user_stakes:
            self.user_stakes[user] = 0
        self.user_stakes[user] += amount

    def remove_stake(self, user: str, amount: int):
        """Remove stake for a user"""
        if user not in self.user_stakes or self.user_stakes[user] < amount:
            raise ValueError("Insufficient stake")
        self.user_stakes[user] -= amount

    def commit(self, user: str, commitment_hash: str, gas_limit: int, max_fee_per_gas: int):
        """Commit to a transaction"""
        # Check batch interval
        if self.current_timestamp < self.last_batch_end_time + self.batch_interval:
            raise RuntimeError("Batch interval not respected")
        
        # Check gas limits
        if gas_limit < self.min_gas_limit or gas_limit > self.max_gas_limit:
            raise ValueError("Gas limit out of bounds")
        
        if max_fee_per_gas <= 0:
            raise ValueError("Max fee per gas must be positive")
        
        # Check user stake
        if user not in self.user_stakes or self.user_stakes[user] < 1000000000000000000:  # 1 ETH
            raise PermissionError("Insufficient stake")
        
        commit_id = f"{commitment_hash}_{user}_{self.current_timestamp}_{self.current_block_number}"
        
        if commit_id in self.commitments:
            raise ValueError("Commitment already exists")
        
        self.commitments[commit_id] = Commitment(
            commitment_hash=commitment_hash,
            timestamp=self.current_timestamp,
            block_number=self.current_block_number,
            user=user,
            revealed=False,
            data=b"",
            gas_limit=gas_limit,
            max_fee_per_gas=max_fee_per_gas
        )
        
        return commit_id

    def reveal(self, user: str, data: bytes, salt: str):
        """Reveal a committed transaction"""
        # Check anti-sandwich window
        if user in self.last_reveal_time:
            if self.current_timestamp < self.last_reveal_time[user] + self.anti_sandwich_window:
                raise RuntimeError("Anti-sandwich window active")
        
        # Create commitment hash and ID
        commitment_hash = self._hash_data_and_salt(data, salt)
        commit_id = f"{commitment_hash}_{user}_{self.commitments.get(commitment_hash, Commitment('', 0, 0, '', False, b'', 0, 0)).timestamp}_{self.commitments.get(commitment_hash, Commitment('', 0, 0, '', False, b'', 0, 0)).block_number}"
        
        # Find the actual commitment
        actual_commit_id = None
        for cid, commitment in self.commitments.items():
            if (commitment.commitment_hash == commitment_hash and 
                commitment.user == user and 
                not commitment.revealed):
                actual_commit_id = cid
                break
        
        if not actual_commit_id:
            raise ValueError("No commitment found")
        
        commitment = self.commitments[actual_commit_id]
        
        # Check timing constraints
        if self.current_timestamp < commitment.timestamp + self.commit_phase_duration:
            raise RuntimeError("Commit phase not ended")
        
        if self.current_timestamp > commitment.timestamp + self.commit_phase_duration + self.reveal_phase_duration:
            raise RuntimeError("Reveal phase ended")
        
        if commitment_hash != commitment.commitment_hash:
            raise ValueError("Invalid commitment")
        
        # Update commitment
        commitment.revealed = True
        commitment.data = data
        self.commitments[actual_commit_id] = commitment
        
        # Update last reveal time
        self.last_reveal_time[user] = self.current_timestamp
        
        # Execute transaction
        self._execute_transaction(data)
        
        return actual_commit_id

    def execute_batch(self):
        """Execute a batch of transactions"""
        if self.current_timestamp < self.last_batch_end_time + self.batch_interval:
            raise RuntimeError("Batch interval not reached")
        
        batch_id = self.current_timestamp // self.batch_interval
        clearing_price = Decimal('1000000000000000000')  # 1e18 placeholder
        
        self.batches[batch_id] = Batch(
            batch_id=batch_id,
            start_time=self.last_batch_end_time,
            end_time=self.current_timestamp,
            orders=[],
            executed=True,
            clearing_price=clearing_price
        )
        
        self.last_batch_end_time = self.current_timestamp
        
        return batch_id, clearing_price

    def _execute_transaction(self, data: bytes):
        """Execute a transaction (placeholder)"""
        # In a real implementation, this would execute the actual transaction
        pass

    def _hash_data_and_salt(self, data: bytes, salt: str) -> str:
        """Hash data and salt (simplified)"""
        return f"hash_{data.hex()}_{salt}"

    def get_commitment(self, commit_id: str) -> Optional[Commitment]:
        """Get commitment details"""
        return self.commitments.get(commit_id)

    def update_parameters(self, commit_phase_duration: Optional[int] = None,
                         reveal_phase_duration: Optional[int] = None,
                         batch_interval: Optional[int] = None,
                         anti_sandwich_window: Optional[int] = None):
        """Update configuration parameters"""
        if commit_phase_duration is not None:
            self.commit_phase_duration = commit_phase_duration
        if reveal_phase_duration is not None:
            self.reveal_phase_duration = reveal_phase_duration
        if batch_interval is not None:
            self.batch_interval = batch_interval
        if anti_sandwich_window is not None:
            self.anti_sandwich_window = anti_sandwich_window

    def advance_time(self, seconds: int):
        """Advance the current timestamp for testing"""
        self.current_timestamp += seconds

    def mine_blocks(self, blocks: int):
        """Mine blocks for testing"""
        self.current_block_number += blocks


# Example usage and testing
if __name__ == "__main__":
    # Create MEV reference model
    mev = MEVReferenceModel()
    
    # Add stake for user
    user = "user1"
    mev.add_stake(user, 2000000000000000000)  # 2 ETH
    
    # Commit a transaction
    data = b"transfer(address,uint256)"
    salt = "salt123"
    commitment_hash = mev._hash_data_and_salt(data, salt)
    
    try:
        commit_id = mev.commit(user, commitment_hash, 100000, 20000000000)  # 20 gwei
        print(f"Committed transaction with ID: {commit_id}")
    except Exception as e:
        print(f"Commit failed: {e}")
        commit_id = None
    
    if commit_id:
        # Advance time to end commit phase
        mev.advance_time(6)
        
        # Reveal the transaction
        try:
            revealed_id = mev.reveal(user, data, salt)
            print(f"Revealed transaction with ID: {revealed_id}")
        except Exception as e:
            print(f"Reveal failed: {e}")
            revealed_id = None
        
        if revealed_id:
            # Check commitment status
            commitment = mev.get_commitment(revealed_id)
            if commitment:
                print(f"Commitment revealed: {commitment.revealed}")
    
    # Execute batch
    try:
        batch_id, clearing_price = mev.execute_batch()
        print(f"Executed batch {batch_id} with clearing price {clearing_price}")
    except Exception as e:
        print(f"Batch execution failed: {e}")