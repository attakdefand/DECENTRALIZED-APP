// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./SafeToken.sol";

/**
 * @title Orderbook Matching Engine
 * @notice Implements a centralized limit order book with price-time priority and partial fills
 * @dev Supports IOC/FOK orders and ensures fairness through deterministic matching
 */
contract Orderbook {
    // Order types
    enum OrderType { LIMIT, IOC, FOK }
    enum Side { BUY, SELL }
    
    // Order structure
    struct Order {
        uint256 id;
        address trader;
        address tokenIn;
        address tokenOut;
        uint256 amountIn;
        uint256 amountOut;
        uint256 price; // price as amountOut/amountIn * 1e18
        uint256 timestamp;
        OrderType orderType;
        Side side;
        bool filled;
        uint256 filledAmountIn;
        uint256 filledAmountOut;
    }
    
    // Events
    event OrderPlaced(
        uint256 indexed orderId,
        address indexed trader,
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 amountOut,
        uint256 price,
        OrderType orderType,
        Side side
    );
    
    event OrderCancelled(uint256 indexed orderId, address indexed trader);
    event OrderFilled(uint256 indexed orderId, uint256 filledAmountIn, uint256 filledAmountOut);
    event PartialFill(uint256 indexed orderId, uint256 filledAmountIn, uint256 filledAmountOut);
    event TradeExecuted(
        uint256 buyOrderId,
        uint256 sellOrderId,
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 amountOut,
        uint256 price
    );
    
    // State variables
    uint256 public nextOrderId;
    mapping(uint256 => Order) public orders;
    
    // Buy orders sorted by price (highest first) then timestamp (earliest first)
    uint256[] public buyOrderQueue;
    // Sell orders sorted by price (lowest first) then timestamp (earliest first)
    uint256[] public sellOrderQueue;
    
    // Token contracts
    mapping(address => SafeToken) public tokenContracts;
    
    // Owner
    address public owner;
    
    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner");
        _;
    }
    
    constructor() {
        owner = msg.sender;
        nextOrderId = 1;
    }
    
    /**
     * @notice Place a new order in the orderbook
     * @param tokenIn Token to spend
     * @param tokenOut Token to receive
     * @param amountIn Amount of tokenIn to spend
     * @param amountOut Minimum amount of tokenOut to receive
     * @param orderType Type of order (LIMIT, IOC, FOK)
     * @param side Side of the order (BUY, SELL)
     * @return orderId Unique identifier for the placed order
     */
    function placeOrder(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 amountOut,
        OrderType orderType,
        Side side
    ) external returns (uint256 orderId) {
        require(amountIn > 0, "Invalid amountIn");
        require(amountOut > 0, "Invalid amountOut");
        require(tokenIn != tokenOut, "Same token");
        require(tokenIn != address(0) && tokenOut != address(0), "Invalid token address");
        
        // Calculate price as amountOut/amountIn * 1e18
        uint256 price = (amountOut * 1e18) / amountIn;
        
        // Create order
        orderId = nextOrderId++;
        Order storage order = orders[orderId];
        order.id = orderId;
        order.trader = msg.sender;
        order.tokenIn = tokenIn;
        order.tokenOut = tokenOut;
        order.amountIn = amountIn;
        order.amountOut = amountOut;
        order.price = price;
        order.timestamp = block.timestamp;
        order.orderType = orderType;
        order.side = side;
        order.filled = false;
        order.filledAmountIn = 0;
        order.filledAmountOut = 0;
        
        // Transfer tokens from trader to contract
        SafeToken(tokenIn).transferFrom(msg.sender, address(this), amountIn);
        
        emit OrderPlaced(orderId, msg.sender, tokenIn, tokenOut, amountIn, amountOut, price, orderType, side);
        
        // Attempt to match order immediately
        _matchOrder(orderId);
        
        // If order is not fully filled and it's a limit order, add to queue
        if (!orders[orderId].filled && orderType == OrderType.LIMIT) {
            if (side == Side.BUY) {
                _insertBuyOrder(orderId);
            } else {
                _insertSellOrder(orderId);
            }
        } else if (!orders[orderId].filled && (orderType == OrderType.IOC || orderType == OrderType.FOK)) {
            // Cancel unfilled IOC/FOK orders
            _cancelOrder(orderId);
        }
        
        return orderId;
    }
    
    /**
     * @notice Cancel an existing order
     * @param orderId ID of the order to cancel
     */
    function cancelOrder(uint256 orderId) external {
        Order storage order = orders[orderId];
        require(order.trader == msg.sender, "Not owner of order");
        require(!order.filled, "Order already filled");
        
        _cancelOrder(orderId);
    }
    
    /**
     * @notice Get all buy orders sorted by price-time priority
     * @return Array of buy order IDs
     */
    function getBuyOrders() external view returns (uint256[] memory) {
        return buyOrderQueue;
    }
    
    /**
     * @notice Get all sell orders sorted by price-time priority
     * @return Array of sell order IDs
     */
    function getSellOrders() external view returns (uint256[] memory) {
        return sellOrderQueue;
    }
    
    /**
     * @notice Get order details
     * @param orderId ID of the order
     * @return Order details
     */
    function getOrder(uint256 orderId) external view returns (Order memory) {
        return orders[orderId];
    }
    
    /**
     * @notice Match orders in the orderbook
     * @dev Internal function that implements price-time priority matching
     */
    function _matchOrder(uint256 orderId) internal {
        Order storage takerOrder = orders[orderId];
        
        if (takerOrder.filled) return;
        
        if (takerOrder.side == Side.BUY) {
            // Match against sell orders
            _matchAgainstSellOrders(orderId);
        } else {
            // Match against buy orders
            _matchAgainstBuyOrders(orderId);
        }
    }
    
    /**
     * @notice Match a buy order against sell orders
     * @param buyOrderId ID of the buy order
     */
    function _matchAgainstSellOrders(uint256 buyOrderId) internal {
        Order storage buyOrder = orders[buyOrderId];
        if (buyOrder.filled) return;
        
        uint256 remainingAmountIn = buyOrder.amountIn - buyOrder.filledAmountIn;
        uint256 remainingAmountOut = buyOrder.amountOut - buyOrder.filledAmountOut;
        
        // Process sell orders in price-time priority order
        for (uint256 i = 0; i < sellOrderQueue.length && remainingAmountIn > 0; ) {
            uint256 sellOrderId = sellOrderQueue[i];
            Order storage sellOrder = orders[sellOrderId];
            
            // Skip if order is already filled or cancelled
            if (sellOrder.filled || sellOrder.id == 0) {
                unchecked {
                    i++;
                }
                continue;
            }
            
            // Check if prices cross (buy price >= sell price)
            // buyOrder.price = amountOut/amountIn * 1e18 for buy order
            // sellOrder.price = amountOut/amountIn * 1e18 for sell order
            // For crossing: buyOrder.price >= sellOrder.price
            if (buyOrder.price >= sellOrder.price) {
                // Calculate trade amounts
                uint256 tradeAmountIn;
                uint256 tradeAmountOut;
                
                // Determine trade amounts based on available quantities
                uint256 sellRemainingIn = sellOrder.amountIn - sellOrder.filledAmountIn;
                uint256 sellRemainingOut = sellOrder.amountOut - sellOrder.filledAmountOut;
                
                // Use the price of the maker order (sell order) for execution
                // tradeAmountOut = tradeAmountIn * sellOrder.price / 1e18
                uint256 maxPossibleIn = (sellRemainingOut * 1e18) / sellOrder.price;
                uint256 maxAffordableOut = (remainingAmountIn * buyOrder.price) / 1e18;
                
                if (maxPossibleIn <= remainingAmountIn && sellRemainingOut <= maxAffordableOut) {
                    // Sell order limits the trade
                    tradeAmountIn = maxPossibleIn;
                    tradeAmountOut = sellRemainingOut;
                } else if (maxAffordableOut <= sellRemainingOut && remainingAmountIn <= maxPossibleIn) {
                    // Buy order limits the trade
                    tradeAmountIn = remainingAmountIn;
                    tradeAmountOut = maxAffordableOut;
                } else {
                    // Partial fill based on available amounts
                    if (maxPossibleIn <= remainingAmountIn) {
                        tradeAmountIn = maxPossibleIn;
                        tradeAmountOut = sellRemainingOut;
                    } else {
                        tradeAmountIn = remainingAmountIn;
                        tradeAmountOut = maxAffordableOut;
                    }
                }
                
                if (tradeAmountIn > 0 && tradeAmountOut > 0) {
                    // Execute trade
                    _executeTrade(buyOrderId, sellOrderId, tradeAmountIn, tradeAmountOut);
                    
                    // Update remaining amounts
                    remainingAmountIn -= tradeAmountIn;
                    remainingAmountOut -= tradeAmountOut;
                }
            }
            
            unchecked {
                i++;
            }
        }
        
        // Update buy order filled status
        if (buyOrder.amountIn - buyOrder.filledAmountIn == 0) {
            buyOrder.filled = true;
            emit OrderFilled(buyOrderId, buyOrder.filledAmountIn, buyOrder.filledAmountOut);
        } else if (buyOrder.filledAmountIn > 0) {
            emit PartialFill(buyOrderId, buyOrder.filledAmountIn, buyOrder.filledAmountOut);
        }
    }
    
    /**
     * @notice Match a sell order against buy orders
     * @param sellOrderId ID of the sell order
     */
    function _matchAgainstBuyOrders(uint256 sellOrderId) internal {
        Order storage sellOrder = orders[sellOrderId];
        if (sellOrder.filled) return;
        
        uint256 remainingAmountIn = sellOrder.amountIn - sellOrder.filledAmountIn;
        uint256 remainingAmountOut = sellOrder.amountOut - sellOrder.filledAmountOut;
        
        // Process buy orders in price-time priority order
        for (uint256 i = 0; i < buyOrderQueue.length && remainingAmountIn > 0; ) {
            uint256 buyOrderId = buyOrderQueue[i];
            Order storage buyOrder = orders[buyOrderId];
            
            // Skip if order is already filled or cancelled
            if (buyOrder.filled || buyOrder.id == 0) {
                unchecked {
                    i++;
                }
                continue;
            }
            
            // Check if prices cross (buy price >= sell price)
            if (buyOrder.price >= sellOrder.price) {
                // Calculate trade amounts
                uint256 tradeAmountIn;
                uint256 tradeAmountOut;
                
                // Determine trade amounts based on available quantities
                uint256 buyRemainingIn = buyOrder.amountIn - buyOrder.filledAmountIn;
                uint256 buyRemainingOut = buyOrder.amountOut - buyOrder.filledAmountOut;
                
                // Use the price of the maker order (buy order) for execution
                // tradeAmountOut = tradeAmountIn * buyOrder.price / 1e18
                uint256 maxPossibleIn = (buyRemainingOut * 1e18) / buyOrder.price;
                uint256 maxSellableOut = (remainingAmountIn * sellOrder.price) / 1e18;
                
                if (maxPossibleIn <= remainingAmountIn && buyRemainingOut <= maxSellableOut) {
                    // Buy order limits the trade
                    tradeAmountIn = maxPossibleIn;
                    tradeAmountOut = buyRemainingOut;
                } else if (maxSellableOut <= buyRemainingOut && remainingAmountIn <= maxPossibleIn) {
                    // Sell order limits the trade
                    tradeAmountIn = remainingAmountIn;
                    tradeAmountOut = maxSellableOut;
                } else {
                    // Partial fill based on available amounts
                    if (maxPossibleIn <= remainingAmountIn) {
                        tradeAmountIn = maxPossibleIn;
                        tradeAmountOut = buyRemainingOut;
                    } else {
                        tradeAmountIn = remainingAmountIn;
                        tradeAmountOut = maxSellableOut;
                    }
                }
                
                if (tradeAmountIn > 0 && tradeAmountOut > 0) {
                    // Execute trade
                    _executeTrade(buyOrderId, sellOrderId, tradeAmountIn, tradeAmountOut);
                    
                    // Update remaining amounts
                    remainingAmountIn -= tradeAmountIn;
                    remainingAmountOut -= tradeAmountOut;
                }
            }
            
            unchecked {
                i++;
            }
        }
        
        // Update sell order filled status
        if (sellOrder.amountIn - sellOrder.filledAmountIn == 0) {
            sellOrder.filled = true;
            emit OrderFilled(sellOrderId, sellOrder.filledAmountIn, sellOrder.filledAmountOut);
        } else if (sellOrder.filledAmountIn > 0) {
            emit PartialFill(sellOrderId, sellOrder.filledAmountIn, sellOrder.filledAmountOut);
        }
    }
    
    /**
     * @notice Execute a trade between two orders
     * @param buyOrderId ID of the buy order
     * @param sellOrderId ID of the sell order
     * @param amountIn Amount of tokenIn to transfer
     * @param amountOut Amount of tokenOut to transfer
     */
    function _executeTrade(
        uint256 buyOrderId,
        uint256 sellOrderId,
        uint256 amountIn,
        uint256 amountOut
    ) internal {
        Order storage buyOrder = orders[buyOrderId];
        Order storage sellOrder = orders[sellOrderId];
        
        // Transfer tokens
        // From seller to buyer: tokenIn (what seller is selling)
        SafeToken(sellOrder.tokenIn).transferFrom(sellOrder.trader, buyOrder.trader, amountIn);
        // From buyer to seller: tokenOut (what buyer is selling)
        SafeToken(buyOrder.tokenOut).transferFrom(buyOrder.trader, sellOrder.trader, amountOut);
        
        // Update order states
        buyOrder.filledAmountIn += amountIn;
        buyOrder.filledAmountOut += amountOut;
        
        sellOrder.filledAmountIn += amountIn;
        sellOrder.filledAmountOut += amountOut;
        
        // Calculate execution price
        uint256 price = (amountOut * 1e18) / amountIn;
        
        emit TradeExecuted(buyOrderId, sellOrderId, sellOrder.tokenIn, sellOrder.tokenOut, amountIn, amountOut, price);
        
        // Check if orders are fully filled
        if (buyOrder.filledAmountIn >= buyOrder.amountIn) {
            buyOrder.filled = true;
            emit OrderFilled(buyOrderId, buyOrder.filledAmountIn, buyOrder.filledAmountOut);
            _removeBuyOrder(buyOrderId);
        }
        
        if (sellOrder.filledAmountIn >= sellOrder.amountIn) {
            sellOrder.filled = true;
            emit OrderFilled(sellOrderId, sellOrder.filledAmountIn, sellOrder.filledAmountOut);
            _removeSellOrder(sellOrderId);
        }
    }
    
    /**
     * @notice Insert a buy order into the buy queue maintaining price-time priority
     * @param orderId ID of the order to insert
     */
    function _insertBuyOrder(uint256 orderId) internal {
        Order storage newOrder = orders[orderId];
        uint256 price = newOrder.price;
        uint256 timestamp = newOrder.timestamp;
        
        // Find insertion point based on price (descending) then timestamp (ascending)
        uint256 insertIndex = buyOrderQueue.length;
        for (uint256 i = 0; i < buyOrderQueue.length; i++) {
            Order storage existingOrder = orders[buyOrderQueue[i]];
            if (price > existingOrder.price || 
                (price == existingOrder.price && timestamp < existingOrder.timestamp)) {
                insertIndex = i;
                break;
            }
        }
        
        // Insert at the correct position
        buyOrderQueue.push(0); // Extend array
        for (uint256 i = buyOrderQueue.length - 1; i > insertIndex; i--) {
            buyOrderQueue[i] = buyOrderQueue[i - 1];
        }
        buyOrderQueue[insertIndex] = orderId;
    }
    
    /**
     * @notice Insert a sell order into the sell queue maintaining price-time priority
     * @param orderId ID of the order to insert
     */
    function _insertSellOrder(uint256 orderId) internal {
        Order storage newOrder = orders[orderId];
        uint256 price = newOrder.price;
        uint256 timestamp = newOrder.timestamp;
        
        // Find insertion point based on price (ascending) then timestamp (ascending)
        uint256 insertIndex = sellOrderQueue.length;
        for (uint256 i = 0; i < sellOrderQueue.length; i++) {
            Order storage existingOrder = orders[sellOrderQueue[i]];
            if (price < existingOrder.price || 
                (price == existingOrder.price && timestamp < existingOrder.timestamp)) {
                insertIndex = i;
                break;
            }
        }
        
        // Insert at the correct position
        sellOrderQueue.push(0); // Extend array
        for (uint256 i = sellOrderQueue.length - 1; i > insertIndex; i--) {
            sellOrderQueue[i] = sellOrderQueue[i - 1];
        }
        sellOrderQueue[insertIndex] = orderId;
    }
    
    /**
     * @notice Remove a buy order from the buy queue
     * @param orderId ID of the order to remove
     */
    function _removeBuyOrder(uint256 orderId) internal {
        for (uint256 i = 0; i < buyOrderQueue.length; i++) {
            if (buyOrderQueue[i] == orderId) {
                // Shift elements left
                for (uint256 j = i; j < buyOrderQueue.length - 1; j++) {
                    buyOrderQueue[j] = buyOrderQueue[j + 1];
                }
                buyOrderQueue.pop();
                break;
            }
        }
    }
    
    /**
     * @notice Remove a sell order from the sell queue
     * @param orderId ID of the order to remove
     */
    function _removeSellOrder(uint256 orderId) internal {
        for (uint256 i = 0; i < sellOrderQueue.length; i++) {
            if (sellOrderQueue[i] == orderId) {
                // Shift elements left
                for (uint256 j = i; j < sellOrderQueue.length - 1; j++) {
                    sellOrderQueue[j] = sellOrderQueue[j + 1];
                }
                sellOrderQueue.pop();
                break;
            }
        }
    }
    
    /**
     * @notice Cancel an order and refund tokens
     * @param orderId ID of the order to cancel
     */
    function _cancelOrder(uint256 orderId) internal {
        Order storage order = orders[orderId];
        require(order.id != 0, "Order does not exist");
        require(!order.filled, "Order already filled");
        
        // Refund unused tokens
        uint256 refundAmount = order.amountIn - order.filledAmountIn;
        if (refundAmount > 0) {
            SafeToken(order.tokenIn).transfer(order.trader, refundAmount);
        }
        
        // Remove from queue if it's a limit order
        if (order.orderType == OrderType.LIMIT) {
            if (order.side == Side.BUY) {
                _removeBuyOrder(orderId);
            } else {
                _removeSellOrder(orderId);
            }
        }
        
        // Mark as cancelled by setting id to 0
        order.id = 0;
        
        emit OrderCancelled(orderId, order.trader);
    }
}