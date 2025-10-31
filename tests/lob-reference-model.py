#!/usr/bin/env python3
"""
Reference Model for Limit Order Book (LOB) Matching Engine
This model implements the same logic as the Solidity Orderbook contract
for differential testing purposes.
"""

from decimal import Decimal
from typing import List, Dict, Optional, Tuple
from enum import Enum
from dataclasses import dataclass
from sortedcontainers import SortedList


class OrderType(Enum):
    LIMIT = "LIMIT"
    IOC = "IOC"
    FOK = "FOK"


class Side(Enum):
    BUY = "BUY"
    SELL = "SELL"


@dataclass
class Order:
    id: int
    trader: str
    token_in: str
    token_out: str
    amount_in: Decimal
    amount_out: Decimal
    price: Decimal
    timestamp: int
    order_type: OrderType
    side: Side
    filled: bool = False
    filled_amount_in: Decimal = Decimal('0')
    filled_amount_out: Decimal = Decimal('0')


class LOBReferenceModel:
    """
    Reference model for Limit Order Book matching engine
    Implements price-time priority and partial fills
    """

    def __init__(self):
        self.next_order_id = 1
        self.orders: Dict[int, Order] = {}
        self.buy_orders = SortedList(key=lambda x: (-x.price, x.timestamp))
        self.sell_orders = SortedList(key=lambda x: (x.price, x.timestamp))

    def place_order(
        self,
        trader: str,
        token_in: str,
        token_out: str,
        amount_in: int,
        amount_out: int,
        order_type: OrderType,
        side: Side,
        timestamp: int
    ) -> int:
        """Place a new order in the orderbook"""
        if amount_in <= 0 or amount_out <= 0:
            raise ValueError("Invalid amounts")
        
        if token_in == token_out:
            raise ValueError("Same token")
        
        # Calculate price as amount_out/amount_in * 1e18
        price = Decimal(amount_out * 10**18) / Decimal(amount_in)
        
        # Create order
        order_id = self.next_order_id
        self.next_order_id += 1
        
        order = Order(
            id=order_id,
            trader=trader,
            token_in=token_in,
            token_out=token_out,
            amount_in=Decimal(amount_in),
            amount_out=Decimal(amount_out),
            price=price,
            timestamp=timestamp,
            order_type=order_type,
            side=side,
            filled=False,
            filled_amount_in=Decimal('0'),
            filled_amount_out=Decimal('0')
        )
        
        self.orders[order_id] = order
        
        # Attempt to match order immediately
        self._match_order(order_id)
        
        # If order is not fully filled and it's a limit order, add to queue
        if not self.orders[order_id].filled and order_type == OrderType.LIMIT:
            if side == Side.BUY:
                self.buy_orders.add(order)
            else:
                self.sell_orders.add(order)
        elif not self.orders[order_id].filled and (order_type == OrderType.IOC or order_type == OrderType.FOK):
            # Cancel unfilled IOC/FOK orders
            self._cancel_order(order_id)
        
        return order_id

    def cancel_order(self, order_id: int, requester: str) -> bool:
        """Cancel an existing order"""
        if order_id not in self.orders:
            return False
        
        order = self.orders[order_id]
        if order.trader != requester:
            raise PermissionError("Not owner of order")
        
        if order.filled:
            return False
        
        return self._cancel_order(order_id)

    def _cancel_order(self, order_id: int) -> bool:
        """Internal method to cancel an order"""
        if order_id not in self.orders:
            return False
        
        order = self.orders[order_id]
        if order.filled:
            return False
        
        # Remove from queue if it's a limit order
        if order.order_type == OrderType.LIMIT:
            if order.side == Side.BUY:
                try:
                    self.buy_orders.remove(order)
                except ValueError:
                    pass
            else:
                try:
                    self.sell_orders.remove(order)
                except ValueError:
                    pass
        
        # Mark as cancelled by removing from orders dict
        del self.orders[order_id]
        return True

    def _match_order(self, order_id: int):
        """Match orders in the orderbook"""
        if order_id not in self.orders:
            return
        
        taker_order = self.orders[order_id]
        if taker_order.filled:
            return
        
        if taker_order.side == Side.BUY:
            self._match_against_sell_orders(order_id)
        else:
            self._match_against_buy_orders(order_id)

    def _match_against_sell_orders(self, buy_order_id: int):
        """Match a buy order against sell orders"""
        if buy_order_id not in self.orders:
            return
        
        buy_order = self.orders[buy_order_id]
        if buy_order.filled:
            return
        
        remaining_amount_in = buy_order.amount_in - buy_order.filled_amount_in
        remaining_amount_out = buy_order.amount_out - buy_order.filled_amount_out
        
        # Process sell orders in price-time priority order
        i = 0
        while i < len(self.sell_orders) and remaining_amount_in > 0:
            sell_order = self.sell_orders[i]
            
            # Skip if order is already filled
            if sell_order.filled:
                i += 1
                continue
            
            # Check if prices cross (buy price >= sell price)
            if buy_order.price >= sell_order.price:
                # Calculate trade amounts
                trade_amount_in, trade_amount_out = self._calculate_trade_amounts(
                    buy_order, sell_order, remaining_amount_in, remaining_amount_out
                )
                
                if trade_amount_in > 0 and trade_amount_out > 0:
                    # Execute trade
                    self._execute_trade(buy_order_id, sell_order.id, trade_amount_in, trade_amount_out)
                    
                    # Update remaining amounts
                    remaining_amount_in -= trade_amount_in
                    remaining_amount_out -= trade_amount_out
                    i = 0  # Restart from beginning to maintain priority
                    continue
            
            i += 1
        
        # Update buy order filled status
        buy_order = self.orders[buy_order_id]
        if buy_order.amount_in - buy_order.filled_amount_in == 0:
            buy_order.filled = True
        elif buy_order.filled_amount_in > 0:
            pass  # Partially filled

    def _match_against_buy_orders(self, sell_order_id: int):
        """Match a sell order against buy orders"""
        if sell_order_id not in self.orders:
            return
        
        sell_order = self.orders[sell_order_id]
        if sell_order.filled:
            return
        
        remaining_amount_in = sell_order.amount_in - sell_order.filled_amount_in
        remaining_amount_out = sell_order.amount_out - sell_order.filled_amount_out
        
        # Process buy orders in price-time priority order
        i = 0
        while i < len(self.buy_orders) and remaining_amount_in > 0:
            buy_order = self.buy_orders[i]
            
            # Skip if order is already filled
            if buy_order.filled:
                i += 1
                continue
            
            # Check if prices cross (buy price >= sell price)
            if buy_order.price >= sell_order.price:
                # Calculate trade amounts
                trade_amount_in, trade_amount_out = self._calculate_trade_amounts(
                    buy_order, sell_order, remaining_amount_in, remaining_amount_out
                )
                
                if trade_amount_in > 0 and trade_amount_out > 0:
                    # Execute trade
                    self._execute_trade(buy_order.id, sell_order_id, trade_amount_in, trade_amount_out)
                    
                    # Update remaining amounts
                    remaining_amount_in -= trade_amount_in
                    remaining_amount_out -= trade_amount_out
                    i = 0  # Restart from beginning to maintain priority
                    continue
            
            i += 1
        
        # Update sell order filled status
        sell_order = self.orders[sell_order_id]
        if sell_order.amount_in - sell_order.filled_amount_in == 0:
            sell_order.filled = True
        elif sell_order.filled_amount_in > 0:
            pass  # Partially filled

    def _calculate_trade_amounts(
        self,
        buy_order: Order,
        sell_order: Order,
        remaining_buy_in: Decimal,
        remaining_buy_out: Decimal
    ) -> Tuple[Decimal, Decimal]:
        """Calculate trade amounts based on available quantities"""
        sell_remaining_in = sell_order.amount_in - sell_order.filled_amount_in
        sell_remaining_out = sell_order.amount_out - sell_order.filled_amount_out
        
        # Use the price of the maker order (sell order) for execution
        max_possible_in = (sell_remaining_out * Decimal(10**18)) / sell_order.price
        max_affordable_out = (remaining_buy_in * buy_order.price) / Decimal(10**18)
        
        if max_possible_in <= remaining_buy_in and sell_remaining_out <= max_affordable_out:
            # Sell order limits the trade
            trade_amount_in = max_possible_in
            trade_amount_out = sell_remaining_out
        elif max_affordable_out <= sell_remaining_out and remaining_buy_in <= max_possible_in:
            # Buy order limits the trade
            trade_amount_in = remaining_buy_in
            trade_amount_out = max_affordable_out
        else:
            # Partial fill based on available amounts
            if max_possible_in <= remaining_buy_in:
                trade_amount_in = max_possible_in
                trade_amount_out = sell_remaining_out
            else:
                trade_amount_in = remaining_buy_in
                trade_amount_out = max_affordable_out
        
        return trade_amount_in, trade_amount_out

    def _execute_trade(
        self,
        buy_order_id: int,
        sell_order_id: int,
        amount_in: Decimal,
        amount_out: Decimal
    ):
        """Execute a trade between two orders"""
        if buy_order_id not in self.orders or sell_order_id not in self.orders:
            return
        
        buy_order = self.orders[buy_order_id]
        sell_order = self.orders[sell_order_id]
        
        # Update order states
        buy_order.filled_amount_in += amount_in
        buy_order.filled_amount_out += amount_out
        
        sell_order.filled_amount_in += amount_in
        sell_order.filled_amount_out += amount_out
        
        # Check if orders are fully filled
        if buy_order.filled_amount_in >= buy_order.amount_in:
            buy_order.filled = True
            # Remove from buy queue
            try:
                self.buy_orders.remove(buy_order)
            except ValueError:
                pass
        
        if sell_order.filled_amount_in >= sell_order.amount_in:
            sell_order.filled = True
            # Remove from sell queue
            try:
                self.sell_orders.remove(sell_order)
            except ValueError:
                pass

    def get_order(self, order_id: int) -> Optional[Order]:
        """Get order details"""
        return self.orders.get(order_id)

    def get_buy_orders(self) -> List[Order]:
        """Get all buy orders sorted by price-time priority"""
        return list(self.buy_orders)

    def get_sell_orders(self) -> List[Order]:
        """Get all sell orders sorted by price-time priority"""
        return list(self.sell_orders)


# Example usage and testing
if __name__ == "__main__":
    # Create LOB reference model
    lob = LOBReferenceModel()
    
    # Place some orders
    buy_order_id = lob.place_order(
        trader="user1",
        token_in="TKA",
        token_out="TKB",
        amount_in=100 * 10**18,
        amount_out=200 * 10**18,
        order_type=OrderType.LIMIT,
        side=Side.BUY,
        timestamp=1000
    )
    
    sell_order_id = lob.place_order(
        trader="user2",
        token_in="TKB",
        token_out="TKA",
        amount_in=100 * 10**18,
        amount_out=200 * 10**18,
        order_type=OrderType.LIMIT,
        side=Side.SELL,
        timestamp=1001
    )
    
    # Check orders
    buy_order = lob.get_order(buy_order_id)
    sell_order = lob.get_order(sell_order_id)
    
    print(f"Buy order filled: {buy_order.filled}")
    print(f"Sell order filled: {sell_order.filled}")