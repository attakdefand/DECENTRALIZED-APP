#!/usr/bin/env python3
"""
Reference Model for Oracle Manipulation/Liveness
This model implements the same logic as the Solidity Oracle contract
for differential testing purposes.
"""

from decimal import Decimal
from typing import List, Dict, Optional, Tuple
from dataclasses import dataclass
from collections import deque
import time


@dataclass
class PriceFeed:
    price: Decimal
    timestamp: int
    is_valid: bool


@dataclass
class TWAPDataPoint:
    price: Decimal
    timestamp: int


class OracleReferenceModel:
    """
    Reference model for Oracle manipulation/liveness protection
    Implements medianization, TWAP, outlier detection, and staleness protection
    """

    def __init__(self, staleness_threshold: int = 600, outlier_threshold: Decimal = Decimal('0.05')):
        """
        Initialize Oracle reference model
        
        Args:
            staleness_threshold: Maximum age of price feeds in seconds
            outlier_threshold: Maximum deviation threshold (5% default)
        """
        self.price_feeds: Dict[str, Dict[str, List[PriceFeed]]] = {}
        self.is_valid_publisher: Dict[str, bool] = {}
        self.last_valid_price: Dict[str, Decimal] = {}
        self.twap_history: Dict[str, deque] = {}  # Using deque for efficient operations
        self.publishers: List[str] = []
        self.is_emergency_shutdown = False
        self.fallback_oracle = None
        self.staleness_threshold = staleness_threshold
        self.outlier_threshold = outlier_threshold
        self.current_timestamp = int(time.time())

    def add_publisher(self, publisher: str):
        """Add a valid price publisher"""
        if publisher in self.is_valid_publisher and self.is_valid_publisher[publisher]:
            raise ValueError("Already a publisher")
        
        self.is_valid_publisher[publisher] = True
        self.publishers.append(publisher)

    def remove_publisher(self, publisher: str):
        """Remove a price publisher"""
        if publisher not in self.is_valid_publisher or not self.is_valid_publisher[publisher]:
            raise ValueError("Not a publisher")
        
        self.is_valid_publisher[publisher] = False
        self.publishers.remove(publisher)

    def set_fallback_oracle(self, fallback_oracle):
        """Set fallback oracle"""
        self.fallback_oracle = fallback_oracle

    def set_staleness_threshold(self, threshold: int):
        """Set staleness threshold"""
        self.staleness_threshold = threshold

    def set_outlier_threshold(self, threshold: Decimal):
        """Set outlier threshold"""
        self.outlier_threshold = threshold

    def publish_price(self, token: str, publisher: str, price: int):
        """Publish a price feed"""
        if self.is_emergency_shutdown:
            raise RuntimeError("Oracle shutdown")
        
        if publisher not in self.is_valid_publisher or not self.is_valid_publisher[publisher]:
            raise PermissionError("Not a valid publisher")
        
        if price <= 0:
            raise ValueError("Price must be positive")
        
        # Initialize token structure if needed
        if token not in self.price_feeds:
            self.price_feeds[token] = {}
        
        if publisher not in self.price_feeds[token]:
            self.price_feeds[token][publisher] = []
        
        # Add new price feed
        feed = PriceFeed(
            price=Decimal(price),
            timestamp=self.current_timestamp,
            is_valid=True
        )
        self.price_feeds[token][publisher].append(feed)
        
        # Update last valid price
        self.last_valid_price[token] = Decimal(price)
        
        # Add to TWAP history
        if token not in self.twap_history:
            self.twap_history[token] = deque()
        
        twap_point = TWAPDataPoint(
            price=Decimal(price),
            timestamp=self.current_timestamp
        )
        self.twap_history[token].append(twap_point)
        
        # Clean up old TWAP data
        self._cleanup_twap_history(token)

    def get_price(self, token: str) -> Tuple[Decimal, int]:
        """Get the current price for a token"""
        if self.is_emergency_shutdown:
            raise RuntimeError("Oracle shutdown")
        
        # Try to get median price from active publishers
        prices = self._get_valid_prices(token)
        
        if not prices:
            # No valid prices, try fallback oracle
            if self.fallback_oracle is not None:
                try:
                    return self.fallback_oracle.get_price(token)
                except Exception:
                    raise RuntimeError("No valid price available")
            raise RuntimeError("No valid price available")
        
        # Calculate median price
        price = self._calculate_median(prices)
        timestamp = self.current_timestamp
        
        return (price, timestamp)

    def get_twap_price(self, token: str, window: int) -> Tuple[Decimal, int]:
        """Get the TWAP price for a token"""
        if self.is_emergency_shutdown:
            raise RuntimeError("Oracle shutdown")
        
        if token not in self.twap_history or not self.twap_history[token]:
            raise ValueError("No TWAP history")
        
        history = self.twap_history[token]
        end_time = self.current_timestamp
        start_time = end_time - window
        
        # Find the start index
        start_index = 0
        for i, point in enumerate(history):
            if point.timestamp >= start_time:
                start_index = i
                break
        
        # Calculate TWAP
        sum_price = Decimal('0')
        count = 0
        
        for i in range(start_index, len(history)):
            sum_price += history[i].price
            count += 1
        
        if count == 0:
            raise ValueError("No data points in window")
        
        price = sum_price / Decimal(count)
        timestamp = end_time
        
        return (price, timestamp)

    def emergency_shutdown(self):
        """Emergency shutdown of the oracle"""
        self.is_emergency_shutdown = True

    def resume_operations(self):
        """Resume oracle operations after emergency shutdown"""
        self.is_emergency_shutdown = False

    def _get_valid_prices(self, token: str) -> List[Decimal]:
        """Get valid prices from active publishers"""
        if token not in self.price_feeds:
            return []
        
        valid_prices = []
        
        for publisher in self.publishers:
            if not self.is_valid_publisher.get(publisher, False):
                continue
            
            if publisher not in self.price_feeds[token]:
                continue
            
            feeds = self.price_feeds[token][publisher]
            if not feeds:
                continue
            
            latest_feed = feeds[-1]
            if not latest_feed.is_valid:
                continue
            
            # Check for staleness
            if self.current_timestamp - latest_feed.timestamp > self.staleness_threshold:
                continue
            
            # Check for outliers
            if token in self.last_valid_price and self.last_valid_price[token] > 0:
                deviation = self._calculate_deviation(latest_feed.price, self.last_valid_price[token])
                if deviation > self.outlier_threshold:
                    continue
            
            valid_prices.append(latest_feed.price)
        
        return valid_prices

    def _calculate_median(self, array: List[Decimal]) -> Decimal:
        """Calculate median of an array"""
        if not array:
            raise ValueError("Empty array")
        
        # Sort array
        sorted_array = sorted(array)
        length = len(sorted_array)
        
        if length % 2 == 0:
            # Even number of elements
            median = (sorted_array[length // 2 - 1] + sorted_array[length // 2]) / 2
        else:
            # Odd number of elements
            median = sorted_array[length // 2]
        
        return median

    def _calculate_deviation(self, price1: Decimal, price2: Decimal) -> Decimal:
        """Calculate deviation between two prices"""
        if price2 == 0:
            return Decimal('0')
        
        if price1 > price2:
            deviation = ((price1 - price2) * Decimal('1000000000000000000')) / price2
        else:
            deviation = ((price2 - price1) * Decimal('1000000000000000000')) / price2
        
        return deviation

    def _cleanup_twap_history(self, token: str):
        """Clean up old TWAP history data"""
        if token not in self.twap_history:
            return
        
        history = self.twap_history[token]
        cutoff_time = self.current_timestamp - (self.staleness_threshold * 2)
        
        # Remove old data from the front
        while history and history[0].timestamp < cutoff_time:
            history.popleft()

    def get_publisher_count(self) -> int:
        """Get number of active publishers"""
        count = 0
        for publisher in self.publishers:
            if self.is_valid_publisher.get(publisher, False):
                count += 1
        return count

    def is_quorum_met(self, token: str) -> bool:
        """Check if quorum is met (at least 2 publishers)"""
        valid_count = 0
        
        for publisher in self.publishers:
            if not self.is_valid_publisher.get(publisher, False):
                continue
            
            if token not in self.price_feeds or publisher not in self.price_feeds[token]:
                continue
            
            feeds = self.price_feeds[token][publisher]
            if not feeds:
                continue
            
            latest_feed = feeds[-1]
            if not latest_feed.is_valid:
                continue
            
            # Check for staleness
            if self.current_timestamp - latest_feed.timestamp > self.staleness_threshold:
                continue
            
            valid_count += 1
        
        return valid_count >= 2

    def advance_time(self, seconds: int):
        """Advance the current timestamp for testing"""
        self.current_timestamp += seconds


# Example usage and testing
if __name__ == "__main__":
    # Create oracle reference model
    oracle = OracleReferenceModel(staleness_threshold=600, outlier_threshold=Decimal('0.05'))
    
    # Add publishers
    oracle.add_publisher("publisher1")
    oracle.add_publisher("publisher2")
    oracle.add_publisher("publisher3")
    
    # Publish prices
    oracle.publish_price("ETH", "publisher1", 1000 * 10**18)  # $1000
    oracle.publish_price("ETH", "publisher2", 1050 * 10**18)  # $1050
    oracle.publish_price("ETH", "publisher3", 950 * 10**18)   # $950
    
    # Get median price
    price, timestamp = oracle.get_price("ETH")
    print(f"Median price: {price}")
    
    # Advance time and publish more prices for TWAP
    oracle.advance_time(600)  # Advance 10 minutes
    oracle.publish_price("ETH", "publisher1", 1100 * 10**18)  # $1100
    
    oracle.advance_time(600)  # Advance 10 minutes
    oracle.publish_price("ETH", "publisher1", 900 * 10**18)   # $900
    
    # Get TWAP price
    twap_price, twap_timestamp = oracle.get_twap_price("ETH", 1800)  # 30-minute window
    print(f"TWAP price: {twap_price}")
    
    # Check quorum
    print(f"Quorum met: {oracle.is_quorum_met('ETH')}")