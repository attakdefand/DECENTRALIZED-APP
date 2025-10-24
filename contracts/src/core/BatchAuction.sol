// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

/// @title BatchAuction
/// @notice Implements frequent batch auctions to mitigate MEV
/// @dev This contract collects orders over time and executes them at a uniform price
contract BatchAuction is Ownable, ReentrancyGuard {
    using SafeERC20 for IERC20;

    struct Order {
        address trader;
        address tokenIn;
        address tokenOut;
        uint256 amountIn;
        uint256 minAmountOut;
        uint256 timestamp;
    }

    struct Batch {
        uint256 batchId;
        uint256 startTime;
        uint256 endTime;
        Order[] orders;
        bool executed;
        uint256 clearingPrice;
    }

    // State variables
    uint256 public batchInterval; // Time between batch executions in seconds
    uint256 public lastBatchTime; // Timestamp of last batch execution
    uint256 public currentBatchId; // ID of current batch
    mapping(uint256 => Batch) public batches; // All batches
    mapping(address => mapping(address => uint256)) public tokenBalances; // Token balances in the contract
    
    // Configuration
    address public feeRecipient; // Address to receive fees
    uint256 public feeBasisPoints; // Fee in basis points (100 = 1%)
    
    // Events
    event OrderSubmitted(uint256 batchId, address indexed trader, address tokenIn, address tokenOut, uint256 amountIn, uint256 minAmountOut);
    event BatchExecuted(uint256 batchId, uint256 clearingPrice, uint256 totalTrades);
    event FeeUpdated(address feeRecipient, uint256 feeBasisPoints);
    event BatchIntervalUpdated(uint256 batchInterval);
    
    // Modifiers
    modifier onlyAfterBatchInterval() {
        require(block.timestamp >= lastBatchTime + batchInterval, "Batch interval not elapsed");
        _;
    }
    
    /// @notice Constructor to initialize the contract
    /// @param _batchInterval Time between batch executions in seconds
    /// @param _feeRecipient Address to receive fees
    /// @param _feeBasisPoints Fee in basis points
    constructor(
        uint256 _batchInterval,
        address _feeRecipient,
        uint256 _feeBasisPoints
    ) {
        require(_batchInterval > 0, "Batch interval must be positive");
        require(_feeRecipient != address(0), "Fee recipient cannot be zero address");
        require(_feeBasisPoints <= 10000, "Fee cannot exceed 100%");
        
        batchInterval = _batchInterval;
        lastBatchTime = block.timestamp;
        feeRecipient = _feeRecipient;
        feeBasisPoints = _feeBasisPoints;
        currentBatchId = 1;
        
        // Initialize the first batch
        batches[currentBatchId] = Batch({
            batchId: currentBatchId,
            startTime: block.timestamp,
            endTime: block.timestamp + _batchInterval,
            orders: new Order[](0),
            executed: false,
            clearingPrice: 0
        });
    }
    
    /// @notice Submit an order to the current batch
    /// @param tokenIn Token to sell
    /// @param tokenOut Token to buy
    /// @param amountIn Amount of tokenIn to sell
    /// @param minAmountOut Minimum amount of tokenOut to receive
    function submitOrder(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minAmountOut
    ) external nonReentrant {
        // Ensure tokens are different
        require(tokenIn != tokenOut, "Tokens must be different");
        require(amountIn > 0, "Amount must be positive");
        require(minAmountOut > 0, "Min amount must be positive");
        
        // Transfer tokens from user to contract
        IERC20(tokenIn).safeTransferFrom(msg.sender, address(this), amountIn);
        
        // Add to token balances
        tokenBalances[tokenIn][msg.sender] += amountIn;
        
        // Create order
        Order memory order = Order({
            trader: msg.sender,
            tokenIn: tokenIn,
            tokenOut: tokenOut,
            amountIn: amountIn,
            minAmountOut: minAmountOut,
            timestamp: block.timestamp
        });
        
        // Add order to current batch
        batches[currentBatchId].orders.push(order);
        
        emit OrderSubmitted(currentBatchId, msg.sender, tokenIn, tokenOut, amountIn, minAmountOut);
    }
    
    /// @notice Execute the current batch
    function executeBatch() external onlyAfterBatchInterval nonReentrant {
        Batch storage currentBatch = batches[currentBatchId];
        
        require(!currentBatch.executed, "Batch already executed");
        
        // Set batch end time
        currentBatch.endTime = block.timestamp;
        currentBatch.executed = true;
        
        // Calculate clearing price and execute trades
        if (currentBatch.orders.length > 0) {
            uint256 clearingPrice = calculateClearingPrice(currentBatch);
            currentBatch.clearingPrice = clearingPrice;
            
            executeTrades(currentBatch, clearingPrice);
            
            emit BatchExecuted(currentBatchId, clearingPrice, currentBatch.orders.length);
        }
        
        // Prepare for next batch
        lastBatchTime = block.timestamp;
        currentBatchId++;
        
        batches[currentBatchId] = Batch({
            batchId: currentBatchId,
            startTime: block.timestamp,
            endTime: block.timestamp + batchInterval,
            orders: new Order[](0),
            executed: false,
            clearingPrice: 0
        });
    }
    
    /// @notice Calculate the clearing price for a batch
    /// @param batch The batch to calculate price for
    /// @return clearingPrice The uniform clearing price
    function calculateClearingPrice(Batch storage batch) internal view returns (uint256) {
        // Simple implementation: average of all order ratios
        // In a real implementation, this would be more sophisticated
        if (batch.orders.length == 0) {
            return 0;
        }
        
        uint256 totalRatio = 0;
        for (uint256 i = 0; i < batch.orders.length; i++) {
            // Avoid division by zero
            if (batch.orders[i].amountIn > 0) {
                // Ratio = minAmountOut / amountIn
                totalRatio += (batch.orders[i].minAmountOut * 1e18) / batch.orders[i].amountIn;
            }
        }
        
        return totalRatio / batch.orders.length;
    }
    
    /// @notice Execute trades at the clearing price
    /// @param batch The batch to execute
    /// @param clearingPrice The uniform clearing price
    function executeTrades(Batch storage batch, uint256 clearingPrice) internal {
        for (uint256 i = 0; i < batch.orders.length; i++) {
            Order storage order = batch.orders[i];
            
            // Calculate amount out based on clearing price
            uint256 amountOut = (order.amountIn * clearingPrice) / 1e18;
            
            // Check if it meets minimum requirement
            if (amountOut >= order.minAmountOut) {
                // Calculate fee
                uint256 fee = (amountOut * feeBasisPoints) / 10000;
                uint256 amountOutMinusFee = amountOut - fee;
                
                // Update balances
                tokenBalances[order.tokenIn][order.trader] -= order.amountIn;
                tokenBalances[order.tokenOut][order.trader] += amountOutMinusFee;
                tokenBalances[order.tokenOut][feeRecipient] += fee;
                
                // Transfer tokens to trader
                IERC20(order.tokenIn).safeTransferFrom(address(this), order.trader, order.amountIn);
                IERC20(order.tokenOut).safeTransfer(order.trader, amountOutMinusFee);
            }
        }
    }
    
    /// @notice Get current batch information
    /// @return batch The current batch
    function getCurrentBatch() external view returns (Batch memory) {
        return batches[currentBatchId];
    }
    
    /// @notice Get order count in current batch
    /// @return orderCount Number of orders in current batch
    function getCurrentBatchOrderCount() external view returns (uint256) {
        return batches[currentBatchId].orders.length;
    }
    
    /// @notice Update fee parameters
    /// @param _feeRecipient New fee recipient
    /// @param _feeBasisPoints New fee in basis points
    function updateFee(address _feeRecipient, uint256 _feeBasisPoints) external onlyOwner {
        require(_feeRecipient != address(0), "Fee recipient cannot be zero address");
        require(_feeBasisPoints <= 10000, "Fee cannot exceed 100%");
        
        feeRecipient = _feeRecipient;
        feeBasisPoints = _feeBasisPoints;
        
        emit FeeUpdated(_feeRecipient, _feeBasisPoints);
    }
    
    /// @notice Update batch interval
    /// @param _batchInterval New batch interval in seconds
    function updateBatchInterval(uint256 _batchInterval) external onlyOwner {
        require(_batchInterval > 0, "Batch interval must be positive");
        batchInterval = _batchInterval;
        
        emit BatchIntervalUpdated(_batchInterval);
    }
    
    /// @notice Withdraw tokens from the contract
    /// @param token Token to withdraw
    /// @param amount Amount to withdraw
    function withdrawTokens(address token, uint256 amount) external onlyOwner {
        IERC20(token).safeTransfer(owner(), amount);
    }
    
    /// @notice Get token balance for a user
    /// @param token Token address
    /// @param user User address
    /// @return balance Token balance
    function getUserTokenBalance(address token, address user) external view returns (uint256) {
        return tokenBalances[token][user];
    }
}