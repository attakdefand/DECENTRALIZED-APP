#!/usr/bin/env python3
"""
Python reference model for AMM (Automated Market Maker) calculations.
This module provides reference implementations for CPMM calculations to validate against the Solidity implementation.
"""

import math
from decimal import Decimal, getcontext

# Set precision for decimal calculations
getcontext().prec = 50

class CPMMReferenceModel:
    """Reference model for Constant Product Market Maker calculations"""
    
    def __init__(self, reserve_a=0, reserve_b=0, fee_basis_points=30):
        """
        Initialize CPMM reference model
        
        Args:
            reserve_a: Reserve of token A
            reserve_b: Reserve of token B
            fee_basis_points: Fee in basis points (30 = 0.3%)
        """
        self.reserve_a = Decimal(str(reserve_a))
        self.reserve_b = Decimal(str(reserve_b))
        self.fee_basis_points = Decimal(str(fee_basis_points))
        self.max_fee_basis_points = Decimal('10000')  # 100%
        self.k_last = self.reserve_a * self.reserve_b  # Initial k value
    
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
    
    def get_amount_in(self, amount_out, reserve_in=None, reserve_out=None):
        """
        Calculate amount in needed for a desired amount out
        
        Args:
            amount_out: Desired amount of output token
            reserve_in: Reserve of input token (optional, uses instance values if not provided)
            reserve_out: Reserve of output token (optional, uses instance values if not provided)
            
        Returns:
            amount_in: Amount of input token needed
        """
        if reserve_in is None:
            reserve_in = self.reserve_a
        if reserve_out is None:
            reserve_out = self.reserve_b
            
        amount_out = Decimal(str(amount_out))
        reserve_in = Decimal(str(reserve_in))
        reserve_out = Decimal(str(reserve_out))
        
        # Check if sufficient liquidity
        if amount_out >= reserve_out:
            raise ValueError("Insufficient liquidity")
        
        # Calculate amount in using constant product formula
        # amount_in = (reserve_in * amount_out * MAX_FEE_BASIS_POINTS) / 
        #             ((reserve_out - amount_out) * (MAX_FEE_BASIS_POINTS - feeBasisPoints)) + 1
        numerator = reserve_in * amount_out * self.max_fee_basis_points
        denominator = (reserve_out - amount_out) * (self.max_fee_basis_points - self.fee_basis_points)
        
        if denominator == 0:
            return Decimal('0')
            
        amount_in = (numerator / denominator) + Decimal('1')  # Add 1 for rounding
        return amount_in
    
    def update_reserves_after_swap(self, amount_in, amount_out, is_a_to_b=True):
        """
        Update reserves after a swap and track k value
        
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
        
        # Update k value
        self.k_last = self.reserve_a * self.reserve_b
    
    def add_liquidity(self, amount_a, amount_b):
        """
        Add liquidity to the pool
        
        Args:
            amount_a: Amount of token A to add
            amount_b: Amount of token B to add
            
        Returns:
            tuple: (actual_a, actual_b, liquidity_minted)
        """
        amount_a = Decimal(str(amount_a))
        amount_b = Decimal(str(amount_b))
        
        # For first liquidity provider
        if self.reserve_a == 0 and self.reserve_b == 0:
            actual_a = amount_a
            actual_b = amount_b
            # Mint initial liquidity (subtract MIN_LIQUIDITY)
            liquidity_minted = (actual_a * actual_b).sqrt() - Decimal('1000')
            self.reserve_a = actual_a
            self.reserve_b = actual_b
            self.k_last = self.reserve_a * self.reserve_b
            return actual_a, actual_b, liquidity_minted
        
        # Calculate optimal deposit amounts
        amount_b_optimal = (amount_a * self.reserve_b) / self.reserve_a
        if amount_b_optimal <= amount_b:
            actual_a = amount_a
            actual_b = amount_b_optimal
        else:
            amount_a_optimal = (amount_b * self.reserve_a) / self.reserve_b
            actual_a = amount_a_optimal
            actual_b = amount_b
        
        # Calculate liquidity to mint
        liquidity_a = (actual_a * self.total_supply) / self.reserve_a if self.total_supply > 0 else Decimal('0')
        liquidity_b = (actual_b * self.total_supply) / self.reserve_b if self.total_supply > 0 else Decimal('0')
        liquidity_minted = min(liquidity_a, liquidity_b)
        
        # Update reserves
        self.reserve_a += actual_a
        self.reserve_b += actual_b
        self.total_supply += liquidity_minted
        self.k_last = self.reserve_a * self.reserve_b
        
        return actual_a, actual_b, liquidity_minted
    
    def check_constant_product_invariant(self):
        """
        Check constant product invariant (x * y = k)
        
        Returns:
            tuple: (invariant_holds, current_k, expected_k)
        """
        current_k = self.reserve_a * self.reserve_b
        expected_k = self.k_last
        
        # For first liquidity provider or empty pool, k_last might be 0
        if expected_k == 0:
            return (True, current_k, expected_k)
        
        # Allow for small differences due to fees
        # In a perfect CPMM, k should never decrease, but fees can cause it to increase
        invariant_holds = current_k >= expected_k
        return (invariant_holds, current_k, expected_k)
    
    def __init__(self, reserve_a=0, reserve_b=0, fee_basis_points=30):
        """
        Initialize CPMM reference model with additional state tracking
        
        Args:
            reserve_a: Reserve of token A
            reserve_b: Reserve of token B
            fee_basis_points: Fee in basis points (30 = 0.3%)
        """
        self.reserve_a = Decimal(str(reserve_a))
        self.reserve_b = Decimal(str(reserve_b))
        self.fee_basis_points = Decimal(str(fee_basis_points))
        self.max_fee_basis_points = Decimal('10000')  # 100%
        self.k_last = self.reserve_a * self.reserve_b if self.reserve_a > 0 and self.reserve_b > 0 else Decimal('0')
        self.total_supply = Decimal('0')
        self.MIN_LIQUIDITY = Decimal('1000')

def test_cpmm_calculations():
    """Test CPMM calculations against reference model"""
    print("Testing CPMM Calculations...")
    
    # Test case 1: Basic swap
    cpmm = CPMMReferenceModel(10000, 20000, 30)  # 10000 A, 20000 B, 0.3% fee
    amount_in = 1000
    expected_out = cpmm.get_amount_out(amount_in)
    print(f"Swap {amount_in} A for B: {amount_in} A -> {expected_out:.18f} B")
    
    # Test case 2: Large swap
    amount_in_large = 5000
    expected_out_large = cpmm.get_amount_out(amount_in_large)
    print(f"Swap {amount_in_large} A for B: {amount_in_large} A -> {expected_out_large:.18f} B")
    
    # Test case 3: Very small swap
    amount_in_small = 1
    expected_out_small = cpmm.get_amount_out(amount_in_small)
    print(f"Swap {amount_in_small} A for B: {amount_in_small} A -> {expected_out_small:.18f} B")
    
    # Test case 4: Get amount in
    amount_out = 1000
    expected_in = cpmm.get_amount_in(amount_out)
    print(f"Get amount in for {amount_out} B: {expected_in:.18f} A needed")
    
    # Test case 5: Invariant check
    invariant_result = cpmm.check_constant_product_invariant()
    print(f"Invariant check: {invariant_result}")

def test_edge_cases():
    """Test edge cases"""
    print("\nTesting Edge Cases...")
    
    # Test with zero reserves
    cpmm_zero = CPMMReferenceModel(0, 0, 30)
    try:
        amount_out = cpmm_zero.get_amount_out(1000)
        print(f"Zero reserves swap: 0 -> {amount_out}")
    except Exception as e:
        print(f"Zero reserves swap error: {e}")
    
    # Test with extreme ratios
    cpmm_extreme = CPMMReferenceModel(1, 1000000, 30)  # 1:1000000 ratio
    amount_out_extreme = cpmm_extreme.get_amount_out(1)
    print(f"Extreme ratio swap: 1 A -> {amount_out_extreme} B")
    
    # Test with maximum fee
    cpmm_max_fee = CPMMReferenceModel(10000, 10000, 1000)  # 10% fee
    amount_out_max_fee = cpmm_max_fee.get_amount_out(1000)
    print(f"Max fee swap: 1000 A -> {amount_out_max_fee} B")

if __name__ == "__main__":
    print("AMM CPMM Reference Models")
    print("=" * 50)
    
    test_cpmm_calculations()
    test_edge_cases()
    
    print("\n" + "=" * 50)
    print("Reference model testing completed.")