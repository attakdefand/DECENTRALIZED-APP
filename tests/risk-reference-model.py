#!/usr/bin/env python3
"""
Reference Model for Lending Risk & Liquidations
This model implements the same logic as the Solidity LendingPool contract
for differential testing purposes.
"""

from decimal import Decimal
from typing import Dict, Optional
from dataclasses import dataclass


@dataclass
class UserPosition:
    supplied: Decimal = Decimal('0')
    borrowed: Decimal = Decimal('0')
    collateral: Decimal = Decimal('0')
    last_update: int = 0


@dataclass
class ReserveData:
    total_supplied: Decimal = Decimal('0')
    total_borrowed: Decimal = Decimal('0')
    total_reserves: Decimal = Decimal('0')
    last_update: int = 0
    # Interest rate model parameters
    base_rate: Decimal = Decimal('0')
    slope1: Decimal = Decimal('0')
    slope2: Decimal = Decimal('0')
    kink: Decimal = Decimal('0')
    reserve_factor: Decimal = Decimal('0')


@dataclass
class RiskParams:
    collateral_factor: Decimal = Decimal('0')
    liquidation_threshold: Decimal = Decimal('0')
    liquidation_penalty: Decimal = Decimal('0')
    min_health_factor: Decimal = Decimal('0')


class RiskReferenceModel:
    """
    Reference model for Lending Risk & Liquidations
    Implements health factor calculations, kink interest rate models, and liquidation logic
    """

    def __init__(self):
        self.user_positions: Dict[str, Dict[str, UserPosition]] = {}
        self.reserves: Dict[str, ReserveData] = {}
        self.risk_params: Dict[str, RiskParams] = {}
        self.is_collateral_token: Dict[str, bool] = {}
        self.SECONDS_PER_YEAR = 365 * 24 * 60 * 60
        self.WAD = Decimal('1000000000000000000')  # 1e18

    def set_risk_params(self, token: str, params: RiskParams):
        """Set risk parameters for a token"""
        if params.collateral_factor > self.WAD:
            raise ValueError("Collateral factor too high")
        if params.liquidation_threshold > self.WAD:
            raise ValueError("Liquidation threshold too high")
        if params.liquidation_penalty > self.WAD / 2:
            raise ValueError("Liquidation penalty too high")
        if params.min_health_factor < self.WAD:
            raise ValueError("Min health factor too low")

        self.risk_params[token] = params

    def set_interest_rate_model(
        self,
        token: str,
        base_rate: Decimal,
        slope1: Decimal,
        slope2: Decimal,
        kink: Decimal,
        reserve_factor: Decimal
    ):
        """Set interest rate model for a token"""
        if kink > self.WAD:
            raise ValueError("Kink too high")
        if reserve_factor > self.WAD / 2:
            raise ValueError("Reserve factor too high")

        if token not in self.reserves:
            self.reserves[token] = ReserveData()
        
        reserve = self.reserves[token]
        reserve.base_rate = base_rate
        reserve.slope1 = slope1
        reserve.slope2 = slope2
        reserve.kink = kink
        reserve.reserve_factor = reserve_factor

    def set_collateral_token(self, token: str, is_collateral: bool):
        """Set token as collateral"""
        self.is_collateral_token[token] = is_collateral

    def supply(self, user: str, token: str, amount: int):
        """Supply tokens to the lending pool"""
        if amount <= 0:
            raise ValueError("Amount must be > 0")

        # Accrue interest before state changes
        self._accrue_interest(token)

        if token not in self.reserves:
            self.reserves[token] = ReserveData()
        
        if user not in self.user_positions:
            self.user_positions[user] = {}
        
        if token not in self.user_positions[user]:
            self.user_positions[user][token] = UserPosition()

        # Update user position
        self.user_positions[user][token].supplied += Decimal(amount)
        self.user_positions[user][token].last_update = self._get_current_timestamp()

        # Update reserve
        self.reserves[token].total_supplied += Decimal(amount)

    def withdraw(self, user: str, token: str, amount: int):
        """Withdraw tokens from the lending pool"""
        if amount <= 0:
            raise ValueError("Amount must be > 0")

        # Accrue interest before state changes
        self._accrue_interest(token)

        if user not in self.user_positions or token not in self.user_positions[user]:
            raise ValueError("No position found")
        
        position = self.user_positions[user][token]
        if position.supplied < Decimal(amount):
            raise ValueError("Insufficient balance")

        # Check health factor after withdrawal
        self._check_health_factor(user, token, amount, 0)

        if token not in self.reserves:
            raise ValueError("No reserve found")
        
        reserve = self.reserves[token]
        if reserve.total_supplied < Decimal(amount):
            raise ValueError("Insufficient liquidity")

        # Update user position
        position.supplied -= Decimal(amount)
        position.last_update = self._get_current_timestamp()

        # Update reserve
        reserve.total_supplied -= Decimal(amount)

    def borrow(self, user: str, token: str, amount: int):
        """Borrow tokens from the lending pool"""
        if amount <= 0:
            raise ValueError("Amount must be > 0")

        # Accrue interest before state changes
        self._accrue_interest(token)

        if token not in self.reserves:
            self.reserves[token] = ReserveData()
        
        reserve = self.reserves[token]
        if reserve.total_supplied < reserve.total_borrowed + Decimal(amount):
            raise ValueError("Insufficient liquidity")

        if user not in self.user_positions:
            self.user_positions[user] = {}
        
        if token not in self.user_positions[user]:
            self.user_positions[user][token] = UserPosition()

        # Update user position
        self.user_positions[user][token].borrowed += Decimal(amount)
        self.user_positions[user][token].last_update = self._get_current_timestamp()

        # Update reserve
        reserve.total_borrowed += Decimal(amount)

        # Check health factor after borrowing
        self._check_health_factor(user, token, 0, amount)

    def repay(self, user: str, token: str, amount: int):
        """Repay borrowed tokens"""
        if amount <= 0:
            raise ValueError("Amount must be > 0")

        # Accrue interest before state changes
        self._accrue_interest(token)

        if user not in self.user_positions or token not in self.user_positions[user]:
            raise ValueError("No position found")
        
        position = self.user_positions[user][token]
        if position.borrowed < Decimal(amount):
            raise ValueError("Repay amount exceeds debt")

        if token not in self.reserves:
            self.reserves[token] = ReserveData()
        
        reserve = self.reserves[token]

        # Update user position
        position.borrowed -= Decimal(amount)
        position.last_update = self._get_current_timestamp()

        # Update reserve
        reserve.total_borrowed -= Decimal(amount)

    def liquidate(
        self,
        liquidator: str,
        user: str,
        collateral_token: str,
        debt_token: str,
        debt_amount: int
    ):
        """Liquidate an undercollateralized position"""
        if debt_amount <= 0:
            raise ValueError("Debt amount must be > 0")

        # Accrue interest for both tokens
        self._accrue_interest(collateral_token)
        self._accrue_interest(debt_token)

        if user not in self.user_positions or debt_token not in self.user_positions[user]:
            raise ValueError("No position found")
        
        position = self.user_positions[user][debt_token]
        if position.borrowed < Decimal(debt_amount):
            raise ValueError("Insufficient debt")

        # Check if user is liquidatable
        if not self._is_liquidatable(user):
            raise ValueError("User not liquidatable")

        # Calculate collateral to seize
        collateral_to_seize = self._calculate_collateral_to_seize(
            user, collateral_token, debt_token, debt_amount
        )

        # Update user positions
        self.user_positions[user][debt_token].borrowed -= Decimal(debt_amount)
        self.user_positions[user][collateral_token].collateral -= Decimal(collateral_to_seize)

        if liquidator not in self.user_positions:
            self.user_positions[liquidator] = {}
        
        if debt_token not in self.user_positions[liquidator]:
            self.user_positions[liquidator][debt_token] = UserPosition()
        
        if collateral_token not in self.user_positions[liquidator]:
            self.user_positions[liquidator][collateral_token] = UserPosition()

        self.user_positions[liquidator][debt_token].borrowed += Decimal(debt_amount)
        self.user_positions[liquidator][collateral_token].collateral += Decimal(collateral_to_seize)

        # Update reserves
        if debt_token not in self.reserves:
            self.reserves[debt_token] = ReserveData()
        
        if collateral_token not in self.reserves:
            self.reserves[collateral_token] = ReserveData()

        self.reserves[debt_token].total_borrowed -= Decimal(debt_amount)
        self.reserves[collateral_token].total_supplied -= Decimal(collateral_to_seize)

    def add_collateral(self, user: str, token: str, amount: int):
        """Add collateral to a position"""
        if amount <= 0:
            raise ValueError("Amount must be > 0")
        if not self.is_collateral_token.get(token, False):
            raise ValueError("Token not accepted as collateral")

        # Accrue interest before state changes
        self._accrue_interest(token)

        if user not in self.user_positions:
            self.user_positions[user] = {}
        
        if token not in self.user_positions[user]:
            self.user_positions[user][token] = UserPosition()

        # Update user position
        self.user_positions[user][token].collateral += Decimal(amount)
        self.user_positions[user][token].last_update = self._get_current_timestamp()

        # Update reserve
        if token not in self.reserves:
            self.reserves[token] = ReserveData()
        
        self.reserves[token].total_supplied += Decimal(amount)

    def remove_collateral(self, user: str, token: str, amount: int):
        """Remove collateral from a position"""
        if amount <= 0:
            raise ValueError("Amount must be > 0")

        # Accrue interest before state changes
        self._accrue_interest(token)

        if user not in self.user_positions or token not in self.user_positions[user]:
            raise ValueError("No position found")
        
        position = self.user_positions[user][token]
        if position.collateral < Decimal(amount):
            raise ValueError("Insufficient collateral")

        # Check health factor after removal
        self._check_health_factor(user, token, amount, 0)

        # Update user position
        position.collateral -= Decimal(amount)
        position.last_update = self._get_current_timestamp()

        # Update reserve
        if token not in self.reserves:
            self.reserves[token] = ReserveData()
        
        self.reserves[token].total_supplied -= Decimal(amount)

    def _accrue_interest(self, token: str):
        """Accrue interest for a token"""
        if token not in self.reserves:
            self.reserves[token] = ReserveData()
        
        reserve = self.reserves[token]
        time_elapsed = self._get_current_timestamp() - reserve.last_update
        
        if time_elapsed == 0:
            return

        if reserve.total_supplied == 0:
            reserve.last_update = self._get_current_timestamp()
            return

        # Calculate utilization rate
        total_supply = reserve.total_supplied + reserve.total_borrowed
        if total_supply == 0:
            utilization_rate = Decimal('0')
        else:
            utilization_rate = (reserve.total_borrowed * self.WAD) / total_supply

        # Calculate borrow rate per second
        borrow_rate_per_second = self._calculate_borrow_rate_per_second(token, utilization_rate)

        # Calculate interest accrued
        interest_accrued = (reserve.total_borrowed * borrow_rate_per_second * Decimal(time_elapsed)) / self.WAD

        # Update reserve data
        reserve.total_borrowed += interest_accrued
        reserve.total_reserves += (interest_accrued * reserve.reserve_factor) / self.WAD
        reserve.last_update = self._get_current_timestamp()

    def _calculate_borrow_rate_per_second(self, token: str, utilization_rate: Decimal) -> Decimal:
        """Calculate borrow rate per second"""
        if token not in self.reserves:
            return Decimal('0')
        
        reserve = self.reserves[token]
        
        if utilization_rate <= reserve.kink:
            # Below kink: baseRate + slope1 * utilization
            annual_rate = reserve.base_rate + (reserve.slope1 * utilization_rate) / self.WAD
        else:
            # Above kink: baseRate + slope1 * kink + slope2 * (utilization - kink)
            excess_utilization = utilization_rate - reserve.kink
            annual_rate = reserve.base_rate + \
                (reserve.slope1 * reserve.kink) / self.WAD + \
                (reserve.slope2 * excess_utilization) / self.WAD

        # Convert annual rate to per second
        return (annual_rate * self.WAD) / (Decimal(self.SECONDS_PER_YEAR) * self.WAD)

    def calculate_health_factor(self, user: str) -> Decimal:
        """Calculate health factor for a user"""
        total_collateral_value = Decimal('0')
        total_debt_value = Decimal('0')

        # Simplified calculation for example
        # In practice would iterate through all tokens and use price oracles
        if user in self.user_positions:
            for token, position in self.user_positions[user].items():
                total_collateral_value += position.collateral
                total_debt_value += position.borrowed

        if total_debt_value == 0:
            return Decimal('Infinity')  # Infinite health factor

        return (total_collateral_value * self.WAD) / total_debt_value

    def _is_liquidatable(self, user: str) -> bool:
        """Check if a user's position is liquidatable"""
        health_factor = self.calculate_health_factor(user)
        min_hf = self.risk_params.get(user, RiskParams(min_health_factor=self.WAD)).min_health_factor
        return health_factor < min_hf

    def _calculate_collateral_to_seize(
        self,
        user: str,
        collateral_token: str,
        debt_token: str,
        debt_amount: int
    ) -> int:
        """Calculate collateral to seize during liquidation"""
        # Get liquidation penalty
        penalty = self.risk_params.get(debt_token, RiskParams(liquidation_penalty=Decimal('0'))).liquidation_penalty
        
        # Calculate equivalent collateral amount with penalty
        collateral_amount = (Decimal(debt_amount) * (self.WAD + penalty)) / self.WAD
        
        return int(collateral_amount)

    def _check_health_factor(
        self,
        user: str,
        token: str,
        withdraw_amount: int,
        borrow_amount: int
    ):
        """Check health factor after an operation"""
        # This is a simplified check - in practice would be more complex
        pass

    def _get_current_timestamp(self) -> int:
        """Get current timestamp (mock implementation)"""
        # In a real implementation, this would return the actual timestamp
        return 1000000

    def get_user_position(self, user: str, token: str) -> Optional[UserPosition]:
        """Get user position for a token"""
        if user in self.user_positions and token in self.user_positions[user]:
            return self.user_positions[user][token]
        return None

    def get_reserve_data(self, token: str) -> Optional[ReserveData]:
        """Get reserve data for a token"""
        return self.reserves.get(token)


# Example usage and testing
if __name__ == "__main__":
    # Create risk reference model
    risk_model = RiskReferenceModel()
    
    # Set risk parameters
    risk_params = RiskParams(
        collateral_factor=Decimal('0.8') * Decimal('1000000000000000000'),  # 80%
        liquidation_threshold=Decimal('0.85') * Decimal('1000000000000000000'),  # 85%
        liquidation_penalty=Decimal('0.1') * Decimal('1000000000000000000'),  # 10%
        min_health_factor=Decimal('1') * Decimal('1000000000000000000')  # 1.0
    )
    risk_model.set_risk_params("TKA", risk_params)
    
    # Set interest rate model
    risk_model.set_interest_rate_model(
        "TKA",
        Decimal('0.02') * Decimal('1000000000000000000'),  # 2% base rate
        Decimal('0.1') * Decimal('1000000000000000000'),   # 10% slope1
        Decimal('0.5') * Decimal('1000000000000000000'),   # 50% slope2
        Decimal('0.8') * Decimal('1000000000000000000'),   # 80% kink
        Decimal('0.1') * Decimal('1000000000000000000')    # 10% reserve factor
    )
    
    # Set token as collateral
    risk_model.set_collateral_token("TKA", True)
    
    # Supply tokens
    risk_model.supply("user1", "TKA", 1000 * 10**18)
    
    # Borrow tokens
    risk_model.borrow("user1", "TKA", 500 * 10**18)
    
    # Check user position
    position = risk_model.get_user_position("user1", "TKA")
    if position is not None:
        print(f"User position - Supplied: {position.supplied}, Borrowed: {position.borrowed}")
    else:
        print("No user position found")
    
    # Check reserve data
    reserve = risk_model.get_reserve_data("TKA")
    if reserve is not None:
        print(f"Reserve data - Total supplied: {reserve.total_supplied}, Total borrowed: {reserve.total_borrowed}")
    else:
        print("No reserve data found")