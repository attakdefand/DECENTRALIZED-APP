#!/usr/bin/env python3
"""
Python reference model for smart contract math safety testing.
This module provides reference implementations for AMM calculations, fee calculations, 
and other mathematical operations to validate against the Solidity implementation.
"""

import math
from decimal import Decimal, getcontext

# Set precision for decimal calculations
getcontext().prec = 50

class AMMReferenceModel:
    """Reference model for Automated Market Maker calculations"""
    
    def __init__(self, reserve_a=0, reserve_b=0, fee_basis_points=30):
        """
        Initialize AMM reference model
        
        Args:
            reserve_a: Reserve of token A
            reserve_b: Reserve of token B
            fee_basis_points: Fee in basis points (30 = 0.3%)
        """
        self.reserve_a = Decimal(str(reserve_a))
        self.reserve_b = Decimal(str(reserve_b))
        self.fee_basis_points = Decimal(str(fee_basis_points))
        self.max_fee_basis_points = Decimal('10000')  # 100%
    
    def get_amount_out(self, amount_in, reserve_in=None, reserve_out=None):
        """
        Calculate amount out for a swap using constant product formula
        
        Args:
            amount_in: Amount of input token
            reserve_in: Reserve of input token (optional, uses instance values if not provided)
            reserve_out: Reserve of output token (optional, uses instance values if not provided)
            
        Returns:
            amount_out: Amount of output token
        """
        if reserve_in is None:
            reserve_in = self.reserve_a
        if reserve_out is None:
            reserve_out = self.reserve_b
            
        amount_in = Decimal(str(amount_in))
        reserve_in = Decimal(str(reserve_in))
        reserve_out = Decimal(str(reserve_out))
        
        # Calculate fee
        fee = (amount_in * self.fee_basis_points) / self.max_fee_basis_points
        amount_in_with_fee = amount_in - fee
        
        # Constant product formula: (x * y) = k
        # amount_out = (reserve_out * amount_in_with_fee) / (reserve_in + amount_in_with_fee)
        numerator = reserve_out * amount_in_with_fee
        denominator = reserve_in + amount_in_with_fee
        
        if denominator == 0:
            return Decimal('0')
            
        amount_out = numerator / denominator
        return amount_out
    
    def update_reserves_after_swap(self, amount_in, amount_out, is_a_to_b=True):
        """
        Update reserves after a swap
        
        Args:
            amount_in: Amount of input token
            amount_out: Amount of output token
            is_a_to_b: True if swapping token A to B, False if B to A
        """
        amount_in = Decimal(str(amount_in))
        amount_out = Decimal(str(amount_out))
        
        if is_a_to_b:
            self.reserve_a += amount_in
            self.reserve_b -= amount_out
        else:
            self.reserve_b += amount_in
            self.reserve_a -= amount_out

class FeeCalculatorReferenceModel:
    """Reference model for fee calculations"""
    
    def __init__(self):
        self.max_fee_basis_points = Decimal('10000')  # 100%
    
    def calculate_fee(self, amount, fee_basis_points):
        """
        Calculate fee for an amount
        
        Args:
            amount: Amount to calculate fee for
            fee_basis_points: Fee in basis points
            
        Returns:
            fee: Calculated fee
        """
        amount = Decimal(str(amount))
        fee_basis_points = Decimal(str(fee_basis_points))
        
        fee = (amount * fee_basis_points) / self.max_fee_basis_points
        return fee
    
    def calculate_fee_with_precision_check(self, amount, fee_basis_points):
        """
        Calculate fee with precision checking
        
        Args:
            amount: Amount to calculate fee for
            fee_basis_points: Fee in basis points
            
        Returns:
            tuple: (fee, is_safe) where is_safe indicates if fee < amount
        """
        fee = self.calculate_fee(amount, fee_basis_points)
        amount = Decimal(str(amount))
        
        is_safe = fee < amount
        return fee, is_safe

class LendingReferenceModel:
    """Reference model for lending calculations"""
    
    def __init__(self, precision=Decimal('1e18')):
        self.precision = precision
        self.total_supplied = Decimal('0')
        self.total_borrowed = Decimal('0')
        self.borrow_rate = Decimal('0')  # Annual rate
        self.last_update = Decimal('0')
    
    def set_borrow_rate(self, rate):
        """
        Set annual borrow rate
        
        Args:
            rate: Annual borrow rate (scaled by precision)
        """
        self.borrow_rate = Decimal(str(rate))
        self.last_update = Decimal(str(self._current_time()))
    
    def calculate_interest(self, amount, time_elapsed):
        """
        Calculate interest for a borrow position
        
        Args:
            amount: Amount borrowed
            time_elapsed: Time elapsed in seconds
            
        Returns:
            interest: Calculated interest
        """
        amount = Decimal(str(amount))
        time_elapsed = Decimal(str(time_elapsed))
        
        # Convert annual rate to per second
        rate_per_second = self.borrow_rate / Decimal('31536000')  # 365 * 24 * 60 * 60
        interest = (amount * rate_per_second * time_elapsed) / self.precision
        return interest
    
    def check_conservation_invariant(self):
        """
        Check lending conservation invariant
        
        Returns:
            bool: True if total supplied >= total borrowed
        """
        return self.total_supplied >= self.total_borrowed
    
    def get_utilization_rate(self):
        """
        Get utilization rate
        
        Returns:
            utilization_rate: Utilization rate scaled by precision
        """
        if self.total_supplied == 0:
            return Decimal('0')
        return (self.total_borrowed * self.precision) / self.total_supplied
    
    def _current_time(self):
        """Mock current time for testing purposes"""
        return Decimal('1000000')  # Fixed time for reproducible tests

def test_amm_calculations():
    """Test AMM calculations against reference model"""
    print("Testing AMM Calculations...")
    
    # Test case 1: Basic swap
    amm = AMMReferenceModel(10000, 20000, 30)  # 10000 A, 20000 B, 0.3% fee
    amount_in = 1000
    expected_out = amm.get_amount_out(amount_in)
    print(f"Swap {amount_in} A for B: {amount_in} A -> {expected_out:.18f} B")
    
    # Test case 2: Large swap
    amount_in_large = 5000
    expected_out_large = amm.get_amount_out(amount_in_large)
    print(f"Swap {amount_in_large} A for B: {amount_in_large} A -> {expected_out_large:.18f} B")
    
    # Test case 3: Very small swap
    amount_in_small = 1
    expected_out_small = amm.get_amount_out(amount_in_small)
    print(f"Swap {amount_in_small} A for B: {amount_in_small} A -> {expected_out_small:.18f} B")

def test_fee_calculations():
    """Test fee calculations against reference model"""
    print("\nTesting Fee Calculations...")
    
    fee_calc = FeeCalculatorReferenceModel()
    
    # Test various fee calculations
    test_cases = [
        (1000, 30),    # 1000 tokens, 0.3% fee
        (10000, 100),  # 10000 tokens, 1% fee
        (100000, 50),  # 100000 tokens, 0.5% fee
        (1000000, 10), # 1000000 tokens, 0.1% fee
    ]
    
    for amount, fee_bp in test_cases:
        fee, is_safe = fee_calc.calculate_fee_with_precision_check(amount, fee_bp)
        print(f"Fee for {amount} tokens at {fee_bp/100:.2f}%: {fee:.18f} tokens (Safe: {is_safe})")

def test_lending_calculations():
    """Test lending calculations against reference model"""
    print("\nTesting Lending Calculations...")
    
    lending = LendingReferenceModel()
    lending.total_supplied = Decimal('100000')
    lending.total_borrowed = Decimal('50000')
    lending.set_borrow_rate(Decimal('0.05') * lending.precision)  # 5% annual rate
    
    # Test conservation invariant
    is_conserved = lending.check_conservation_invariant()
    print(f"Lending conservation: {is_conserved}")
    
    # Test utilization rate
    utilization = lending.get_utilization_rate()
    print(f"Utilization rate: {utilization / lending.precision * 100:.2f}%")
    
    # Test interest calculation
    interest = lending.calculate_interest(10000, 86400)  # 10000 tokens for 1 day
    print(f"Interest for 10000 tokens for 1 day: {interest:.18f} tokens")

if __name__ == "__main__":
    print("Smart Contract Math Safety Reference Models")
    print("=" * 50)
    
    test_amm_calculations()
    test_fee_calculations()
    test_lending_calculations()
    
    print("\n" + "=" * 50)
    print("Reference model testing completed.")