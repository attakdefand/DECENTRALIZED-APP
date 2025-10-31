// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../../src/core/Orderbook.sol";
import "../../src/core/SafeToken.sol";

/**
 * @title Orderbook Test Suite
 * @notice Comprehensive tests for the Orderbook matching engine
 */
contract OrderbookTest is Test {
    Orderbook public orderbook;
    SafeToken public tokenA;
    SafeToken public tokenB;
    address public user1 = address(0x1001);
    address public user2 = address(0x1002);
    address public user3 = address(0x1003);
    
    event OrderPlaced(
        uint256 indexed orderId,
        address indexed trader,
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 amountOut,
        uint256 price,
        Orderbook.OrderType orderType,
        Orderbook.Side side
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
    
    function setUp() public {
        // Deploy contracts
        orderbook = new Orderbook();
        tokenA = new SafeToken("TokenA", "TKA", 18);
        tokenB = new SafeToken("TokenB", "TKB", 18);
        
        // Mint tokens to users
        tokenA.mint(user1, 1000000 * 1e18);
        tokenA.mint(user2, 1000000 * 1e18);
        tokenA.mint(user3, 1000000 * 1e18);
        tokenB.mint(user1, 1000000 * 1e18);
        tokenB.mint(user2, 1000000 * 1e18);
        tokenB.mint(user3, 1000000 * 1e18);
        
        // Approve orderbook to spend tokens
        vm.prank(user1);
        tokenA.approve(address(orderbook), type(uint256).max);
        vm.prank(user1);
        tokenB.approve(address(orderbook), type(uint256).max);
        
        vm.prank(user2);
        tokenA.approve(address(orderbook), type(uint256).max);
        vm.prank(user2);
        tokenB.approve(address(orderbook), type(uint256).max);
        
        vm.prank(user3);
        tokenA.approve(address(orderbook), type(uint256).max);
        vm.prank(user3);
        tokenB.approve(address(orderbook), type(uint256).max);
    }
    
    /**
     * @notice Test basic order placement
     */
    function testPlaceOrder() public {
        vm.prank(user1);
        uint256 orderId = orderbook.placeOrder(
            address(tokenA),
            address(tokenB),
            100 * 1e18,
            200 * 1e18,
            Orderbook.OrderType.LIMIT,
            Orderbook.Side.BUY
        );
        
        assertTrue(orderId > 0);
        Orderbook.Order memory order = orderbook.getOrder(orderId);
        assertEq(order.trader, user1);
        assertEq(order.tokenIn, address(tokenA));
        assertEq(order.tokenOut, address(tokenB));
        assertEq(order.amountIn, 100 * 1e18);
        assertEq(order.amountOut, 200 * 1e18);
        assertEq(order.price, 2 * 1e18); // 200/100 * 1e18
        assertEq(uint8(order.orderType), uint8(Orderbook.OrderType.LIMIT));
        assertEq(uint8(order.side), uint8(Orderbook.Side.BUY));
        assertFalse(order.filled);
    }
    
    /**
     * @notice Test price-time priority for buy orders
     */
    function testBuyOrderPriority() public {
        // Place buy orders with different prices and timestamps
        vm.prank(user1);
        uint256 orderId1 = orderbook.placeOrder(
            address(tokenA),
            address(tokenB),
            100 * 1e18,
            200 * 1e18, // Price: 2
            Orderbook.OrderType.LIMIT,
            Orderbook.Side.BUY
        );
        
        // Advance time to ensure different timestamps
        vm.warp(block.timestamp + 1);
        
        vm.prank(user2);
        uint256 orderId2 = orderbook.placeOrder(
            address(tokenA),
            address(tokenB),
            100 * 1e18,
            250 * 1e18, // Price: 2.5 (higher price, should be first)
            Orderbook.OrderType.LIMIT,
            Orderbook.Side.BUY
        );
        
        vm.warp(block.timestamp + 1);
        
        vm.prank(user3);
        uint256 orderId3 = orderbook.placeOrder(
            address(tokenA),
            address(tokenB),
            100 * 1e18,
            200 * 1e18, // Price: 2 (same price as order1, but later timestamp)
            Orderbook.OrderType.LIMIT,
            Orderbook.Side.BUY
        );
        
        // Check order queue - should be ordered by price (desc) then timestamp (asc)
        uint256[] memory buyOrders = orderbook.getBuyOrders();
        assertEq(buyOrders.length, 3);
        assertEq(buyOrders[0], orderId2); // Highest price
        assertEq(buyOrders[1], orderId1); // Same price as 3, but earlier timestamp
        assertEq(buyOrders[2], orderId3); // Same price as 1, but later timestamp
    }
    
    /**
     * @notice Test price-time priority for sell orders
     */
    function testSellOrderPriority() public {
        // Place sell orders with different prices and timestamps
        vm.prank(user1);
        uint256 orderId1 = orderbook.placeOrder(
            address(tokenB),
            address(tokenA),
            100 * 1e18,
            200 * 1e18, // Price: 2
            Orderbook.OrderType.LIMIT,
            Orderbook.Side.SELL
        );
        
        // Advance time to ensure different timestamps
        vm.warp(block.timestamp + 1);
        
        vm.prank(user2);
        uint256 orderId2 = orderbook.placeOrder(
            address(tokenB),
            address(tokenA),
            100 * 1e18,
            150 * 1e18, // Price: 1.5 (lower price, should be first)
            Orderbook.OrderType.LIMIT,
            Orderbook.Side.SELL
        );
        
        vm.warp(block.timestamp + 1);
        
        vm.prank(user3);
        uint256 orderId3 = orderbook.placeOrder(
            address(tokenB),
            address(tokenA),
            100 * 1e18,
            200 * 1e18, // Price: 2 (same price as order1, but later timestamp)
            Orderbook.OrderType.LIMIT,
            Orderbook.Side.SELL
        );
        
        // Check order queue - should be ordered by price (asc) then timestamp (asc)
        uint256[] memory sellOrders = orderbook.getSellOrders();
        assertEq(sellOrders.length, 3);
        assertEq(sellOrders[0], orderId2); // Lowest price
        assertEq(sellOrders[1], orderId1); // Same price as 3, but earlier timestamp
        assertEq(sellOrders[2], orderId3); // Same price as 1, but later timestamp
    }
    
    /**
     * @notice Test basic matching of orders
     */
    function testBasicMatching() public {
        // Place sell order first
        vm.prank(user1);
        uint256 sellOrderId = orderbook.placeOrder(
            address(tokenB),
            address(tokenA),
            100 * 1e18,
            200 * 1e18, // Price: 2
            Orderbook.OrderType.LIMIT,
            Orderbook.Side.SELL
        );
        
        // Place matching buy order
        vm.prank(user2);
        uint256 buyOrderId = orderbook.placeOrder(
            address(tokenA),
            address(tokenB),
            200 * 1e18,
            100 * 1e18, // Price: 0.5 (reciprocal)
            Orderbook.OrderType.LIMIT,
            Orderbook.Side.BUY
        );
        
        // Check that trade was executed
        Orderbook.Order memory buyOrder = orderbook.getOrder(buyOrderId);
        Orderbook.Order memory sellOrder = orderbook.getOrder(sellOrderId);
        
        assertTrue(buyOrder.filled);
        assertTrue(sellOrder.filled);
        assertEq(buyOrder.filledAmountIn, 200 * 1e18);
        assertEq(buyOrder.filledAmountOut, 100 * 1e18);
        assertEq(sellOrder.filledAmountIn, 100 * 1e18);
        assertEq(sellOrder.filledAmountOut, 200 * 1e18);
        
        // Check token balances
        assertEq(tokenA.balanceOf(user1), 1000000 * 1e18 + 200 * 1e18); // User1 receives tokenA
        assertEq(tokenB.balanceOf(user1), 1000000 * 1e18 - 100 * 1e18); // User1 sends tokenB
        assertEq(tokenA.balanceOf(user2), 1000000 * 1e18 - 200 * 1e18); // User2 sends tokenA
        assertEq(tokenB.balanceOf(user2), 1000000 * 1e18 + 100 * 1e18); // User2 receives tokenB
    }
    
    /**
     * @notice Test partial fill matching
     */
    function testPartialFill() public {
        // Place large sell order
        vm.prank(user1);
        uint256 sellOrderId = orderbook.placeOrder(
            address(tokenB),
            address(tokenA),
            200 * 1e18,
            400 * 1e18, // Price: 2
            Orderbook.OrderType.LIMIT,
            Orderbook.Side.SELL
        );
        
        // Place smaller buy order that partially matches
        vm.prank(user2);
        uint256 buyOrderId = orderbook.placeOrder(
            address(tokenA),
            address(tokenB),
            100 * 1e18,
            50 * 1e18, // Price: 0.5
            Orderbook.OrderType.LIMIT,
            Orderbook.Side.BUY
        );
        
        // Check partial fill
        Orderbook.Order memory buyOrder = orderbook.getOrder(buyOrderId);
        Orderbook.Order memory sellOrder = orderbook.getOrder(sellOrderId);
        
        assertTrue(buyOrder.filled); // Buy order fully filled
        assertFalse(sellOrder.filled); // Sell order partially filled
        assertEq(sellOrder.filledAmountIn, 50 * 1e18); // 50 tokenB filled
        assertEq(sellOrder.filledAmountOut, 100 * 1e18); // 100 tokenA filled
        assertEq(sellOrder.amountIn - sellOrder.filledAmountIn, 150 * 1e18); // 150 tokenB remaining
    }
    
    /**
     * @notice Test IOC (Immediate or Cancel) order behavior
     */
    function testIOCOrder() public {
        // Place IOC buy order that doesn't match
        vm.prank(user1);
        uint256 iocOrderId = orderbook.placeOrder(
            address(tokenA),
            address(tokenB),
            100 * 1e18,
            200 * 1e18, // Price: 2
            Orderbook.OrderType.IOC,
            Orderbook.Side.BUY
        );
        
        // Check that IOC order was cancelled (no match)
        Orderbook.Order memory iocOrder = orderbook.getOrder(iocOrderId);
        assertEq(iocOrder.id, 0); // Cancelled orders have id = 0
        
        // Place matching sell order
        vm.prank(user2);
        uint256 sellOrderId = orderbook.placeOrder(
            address(tokenB),
            address(tokenA),
            100 * 1e18,
            50 * 1e18, // Price: 0.5
            Orderbook.OrderType.LIMIT,
            Orderbook.Side.SELL
        );
        
        // Place IOC buy order that matches
        vm.prank(user3);
        uint256 iocOrderId2 = orderbook.placeOrder(
            address(tokenA),
            address(tokenB),
            50 * 1e18,
            100 * 1e18, // Price: 2
            Orderbook.OrderType.IOC,
            Orderbook.Side.BUY
        );
        
        // Check that IOC order was filled
        Orderbook.Order memory iocOrder2 = orderbook.getOrder(iocOrderId2);
        assertTrue(iocOrder2.filled);
    }
    
    /**
     * @notice Test FOK (Fill or Kill) order behavior
     */
    function testFOKOrder() public {
        // Place FOK buy order that doesn't fully match
        vm.prank(user1);
        uint256 fokOrderId = orderbook.placeOrder(
            address(tokenA),
            address(tokenB),
            100 * 1e18,
            200 * 1e18, // Price: 2
            Orderbook.OrderType.FOK,
            Orderbook.Side.BUY
        );
        
        // Check that FOK order was cancelled (no match)
        Orderbook.Order memory fokOrder = orderbook.getOrder(fokOrderId);
        assertEq(fokOrder.id, 0); // Cancelled orders have id = 0
        
        // Place matching sell order
        vm.prank(user2);
        uint256 sellOrderId = orderbook.placeOrder(
            address(tokenB),
            address(tokenA),
            100 * 1e18,
            50 * 1e18, // Price: 0.5
            Orderbook.OrderType.LIMIT,
            Orderbook.Side.SELL
        );
        
        // Place FOK buy order that fully matches
        vm.prank(user3);
        uint256 fokOrderId2 = orderbook.placeOrder(
            address(tokenA),
            address(tokenB),
            50 * 1e18,
            100 * 1e18, // Price: 2
            Orderbook.OrderType.FOK,
            Orderbook.Side.BUY
        );
        
        // Check that FOK order was filled
        Orderbook.Order memory fokOrder2 = orderbook.getOrder(fokOrderId2);
        assertTrue(fokOrder2.filled);
    }
    
    /**
     * @notice Test order cancellation
     */
    function testCancelOrder() public {
        // Place order
        vm.prank(user1);
        uint256 orderId = orderbook.placeOrder(
            address(tokenA),
            address(tokenB),
            100 * 1e18,
            200 * 1e18,
            Orderbook.OrderType.LIMIT,
            Orderbook.Side.BUY
        );
        
        // Check initial state
        uint256[] memory buyOrdersBefore = orderbook.getBuyOrders();
        assertEq(buyOrdersBefore.length, 1);
        
        // Cancel order
        vm.prank(user1);
        orderbook.cancelOrder(orderId);
        
        // Check that order was cancelled
        Orderbook.Order memory order = orderbook.getOrder(orderId);
        assertEq(order.id, 0); // Cancelled orders have id = 0
        
        // Check that order was removed from queue
        uint256[] memory buyOrdersAfter = orderbook.getBuyOrders();
        assertEq(buyOrdersAfter.length, 0);
    }
    
    /**
     * @notice Test that only order owner can cancel
     */
    function testCancelOrderUnauthorized() public {
        // Place order
        vm.prank(user1);
        uint256 orderId = orderbook.placeOrder(
            address(tokenA),
            address(tokenB),
            100 * 1e18,
            200 * 1e18,
            Orderbook.OrderType.LIMIT,
            Orderbook.Side.BUY
        );
        
        // Try to cancel from different user (should fail)
        vm.prank(user2);
        vm.expectRevert("Not owner of order");
        orderbook.cancelOrder(orderId);
    }
    
    /**
     * @notice Test fairness with multiple matching orders
     */
    function testFairness() public {
        // Place multiple sell orders at same price
        vm.prank(user1);
        uint256 sellOrderId1 = orderbook.placeOrder(
            address(tokenB),
            address(tokenA),
            100 * 1e18,
            200 * 1e18, // Price: 2
            Orderbook.OrderType.LIMIT,
            Orderbook.Side.SELL
        );
        
        vm.warp(block.timestamp + 1);
        
        vm.prank(user2);
        uint256 sellOrderId2 = orderbook.placeOrder(
            address(tokenB),
            address(tokenA),
            100 * 1e18,
            200 * 1e18, // Price: 2
            Orderbook.OrderType.LIMIT,
            Orderbook.Side.SELL
        );
        
        // Place large buy order that matches both
        vm.prank(user3);
        uint256 buyOrderId = orderbook.placeOrder(
            address(tokenA),
            address(tokenB),
            400 * 1e18,
            200 * 1e18, // Price: 0.5
            Orderbook.OrderType.LIMIT,
            Orderbook.Side.BUY
        );
        
        // First sell order (earlier timestamp) should be filled first
        Orderbook.Order memory sellOrder1 = orderbook.getOrder(sellOrderId1);
        Orderbook.Order memory sellOrder2 = orderbook.getOrder(sellOrderId2);
        
        assertTrue(sellOrder1.filled);
        assertTrue(sellOrder2.filled);
    }
    
    /**
     * @notice Fuzz test for order placement
     */
    function testFuzzPlaceOrder(uint256 amountIn, uint256 amountOut) public {
        // Constrain inputs to reasonable values
        vm.assume(amountIn > 0 && amountIn < 1000000 * 1e18);
        vm.assume(amountOut > 0 && amountOut < 1000000 * 1e18);
        vm.assume(amountIn > amountOut / 1e18); // Avoid division by zero issues
        
        vm.prank(user1);
        uint256 orderId = orderbook.placeOrder(
            address(tokenA),
            address(tokenB),
            amountIn,
            amountOut,
            Orderbook.OrderType.LIMIT,
            Orderbook.Side.BUY
        );
        
        assertTrue(orderId > 0);
        Orderbook.Order memory order = orderbook.getOrder(orderId);
        assertEq(order.amountIn, amountIn);
        assertEq(order.amountOut, amountOut);
    }
    
    /**
     * @notice Test edge case: zero amounts
     */
    function testZeroAmounts() public {
        vm.prank(user1);
        vm.expectRevert("Invalid amountIn");
        orderbook.placeOrder(
            address(tokenA),
            address(tokenB),
            0,
            100 * 1e18,
            Orderbook.OrderType.LIMIT,
            Orderbook.Side.BUY
        );
        
        vm.prank(user1);
        vm.expectRevert("Invalid amountOut");
        orderbook.placeOrder(
            address(tokenA),
            address(tokenB),
            100 * 1e18,
            0,
            Orderbook.OrderType.LIMIT,
            Orderbook.Side.BUY
        );
    }
    
    /**
     * @notice Test edge case: same token addresses
     */
    function testSameTokenAddresses() public {
        vm.prank(user1);
        vm.expectRevert("Same token");
        orderbook.placeOrder(
            address(tokenA),
            address(tokenA),
            100 * 1e18,
            100 * 1e18,
            Orderbook.OrderType.LIMIT,
            Orderbook.Side.BUY
        );
    }
}