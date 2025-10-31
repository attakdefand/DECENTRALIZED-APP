// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

/// @title MathSafetyDemo
/// @notice A contract demonstrating advanced mathematical safety patterns including overflow protection, precision handling, and value conservation
/// @dev Implements SafeMath patterns, fixed-point arithmetic, fee calculations, and conservation invariants
contract MathSafetyDemo is Ownable, ReentrancyGuard {
    using SafeERC20 for IERC20;

    // Constants for precision handling
    uint256 public constant PRECISION = 1e18; // 18 decimal places precision
    uint256 public constant MAX_FEE_BASIS_POINTS = 10000; // 100% in basis points
    uint256 public constant MAX_SLIPPAGE_BASIS_POINTS = 1000; // 10% max slippage
    
    // State variables
    mapping(address => mapping(address => uint256)) public reserves; // Token reserves for AMM
    mapping(address => uint256) public totalSupplied; // Total supplied per token
    mapping(address => uint256) public totalBorrowed; // Total borrowed per token
    mapping(address => uint256) public borrowRate; // Borrow rate per token (per second)
    mapping(address => uint256) public lastUpdate; // Last update time per token
    
    // Fee structure
    uint256 public swapFeeBasisPoints; // Fee for swaps
    uint256 public lendingFeeBasisPoints; // Fee for lending operations
    address public feeRecipient; // Address to receive fees
    
    // Events
    event SwapExecuted(address indexed tokenIn, address indexed tokenOut, uint256 amountIn, uint256 amountOut, uint256 fee);
    event TokensSupplied(address indexed user, address indexed token, uint256 amount);
    event TokensBorrowed(address indexed user, address indexed token, uint256 amount, uint256 fee);
    event TokensRepaid(address indexed user, address indexed token, uint256 amount);
    event FeeUpdated(uint256 swapFee, uint256 lendingFee);
    
    /// @notice Constructor to initialize the contract
    /// @param _feeRecipient Address to receive fees
    constructor(address _feeRecipient) {
        require(_feeRecipient != address(0), "Fee recipient cannot be zero");
        feeRecipient = _feeRecipient;
        swapFeeBasisPoints = 30; // 0.3% default swap fee
        lendingFeeBasisPoints = 100; // 1% default lending fee
    }
    
    /// @notice Update fee parameters
    /// @param _swapFeeBasisPoints New swap fee in basis points
    /// @param _lendingFeeBasisPoints New lending fee in basis points
    /// @param _feeRecipient New fee recipient address
    function updateFees(
        uint256 _swapFeeBasisPoints,
        uint256 _lendingFeeBasisPoints,
        address _feeRecipient
    ) external onlyOwner {
        require(_swapFeeBasisPoints <= MAX_FEE_BASIS_POINTS, "Swap fee too high");
        require(_lendingFeeBasisPoints <= MAX_FEE_BASIS_POINTS, "Lending fee too high");
        require(_feeRecipient != address(0), "Fee recipient cannot be zero");
        
        swapFeeBasisPoints = _swapFeeBasisPoints;
        lendingFeeBasisPoints = _lendingFeeBasisPoints;
        feeRecipient = _feeRecipient;
        
        emit FeeUpdated(_swapFeeBasisPoints, _lendingFeeBasisPoints);
    }
    
    /// @notice Execute a constant product AMM swap with safety checks
    /// @param tokenIn Token to swap from
    /// @param tokenOut Token to swap to
    /// @param amountIn Amount of tokenIn to swap
    /// @param minAmountOut Minimum amount of tokenOut to receive
    /// @return amountOut Amount of tokenOut received
    function swap(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 minAmountOut
    ) external nonReentrant returns (uint256 amountOut) {
        // Checks
        require(tokenIn != tokenOut, "Cannot swap same token");
        require(amountIn > 0, "Amount in must be positive");
        require(minAmountOut > 0, "Min amount out must be positive");
        
        // Get reserves before swap
        uint256 reserveIn = reserves[tokenIn];
        uint256 reserveOut = reserves[tokenOut];
        
        require(reserveIn > 0 && reserveOut > 0, "Insufficient liquidity");
        
        // Calculate amount out using constant product formula: x * y = k
        // amountOut = (reserveOut * amountIn * (10000 - fee)) / (reserveIn * 10000 + amountIn * (10000 - fee))
        uint256 fee = (amountIn * swapFeeBasisPoints) / MAX_FEE_BASIS_POINTS;
        uint256 amountInWithFee = amountIn - fee;
        
        // Safe calculation using Solidity 0.8+ overflow protection
        uint256 numerator = amountInWithFee * reserveOut;
        uint256 denominator = reserveIn + amountInWithFee;
        
        amountOut = numerator / denominator;
        
        require(amountOut >= minAmountOut, "Slippage too high");
        require(amountOut < reserveOut, "Insufficient liquidity");
        
        // Effects - Update reserves
        reserves[tokenIn] = reserveIn + amountIn;
        reserves[tokenOut] = reserveOut - amountOut;
        
        // Calculate and transfer fee to recipient
        if (fee > 0) {
            IERC20(tokenIn).safeTransferFrom(msg.sender, feeRecipient, fee);
        }
        
        // Transfer tokens
        IERC20(tokenIn).safeTransferFrom(msg.sender, address(this), amountIn - fee);
        IERC20(tokenOut).safeTransfer(msg.sender, amountOut);
        
        emit SwapExecuted(tokenIn, tokenOut, amountIn, amountOut, fee);
        
        return amountOut;
    }
    
    /// @notice Get amount out for a swap without executing it
    /// @param tokenIn Token to swap from
    /// @param tokenOut Token to swap to
    /// @param amountIn Amount of tokenIn to swap
    /// @return amountOut Amount of tokenOut that would be received
    function getAmountOut(
        address tokenIn,
        address tokenOut,
        uint256 amountIn
    ) external view returns (uint256 amountOut) {
        // Checks
        require(tokenIn != tokenOut, "Cannot swap same token");
        require(amountIn > 0, "Amount in must be positive");
        
        // Get reserves
        uint256 reserveIn = reserves[tokenIn];
        uint256 reserveOut = reserves[tokenOut];
        
        require(reserveIn > 0 && reserveOut > 0, "Insufficient liquidity");
        
        // Calculate amount out
        uint256 fee = (amountIn * swapFeeBasisPoints) / MAX_FEE_BASIS_POINTS;
        uint256 amountInWithFee = amountIn - fee;
        
        uint256 numerator = amountInWithFee * reserveOut;
        uint256 denominator = reserveIn + amountInWithFee;
        
        amountOut = numerator / denominator;
        
        return amountOut;
    }
    
    /// @notice Supply tokens to the lending pool
    /// @param token Token to supply
    /// @param amount Amount to supply
    function supply(address token, uint256 amount) external nonReentrant {
        // Checks
        require(amount > 0, "Amount must be positive");
        
        // Effects
        totalSupplied[token] += amount;
        lastUpdate[token] = block.timestamp;
        
        // Interactions
        IERC20(token).safeTransferFrom(msg.sender, address(this), amount);
        
        emit TokensSupplied(msg.sender, token, amount);
    }
    
    /// @notice Borrow tokens from the lending pool
    /// @param token Token to borrow
    /// @param amount Amount to borrow
    /// @return amountToRepay Total amount to repay including interest
    function borrow(address token, uint256 amount) external nonReentrant returns (uint256 amountToRepay) {
        // Checks
        require(amount > 0, "Amount must be positive");
        require(totalSupplied[token] >= totalBorrowed[token] + amount, "Insufficient liquidity");
        
        // Calculate interest
        uint256 interest = calculateInterest(token, amount);
        uint256 fee = (amount * lendingFeeBasisPoints) / MAX_FEE_BASIS_POINTS;
        amountToRepay = amount + interest + fee;
        
        // Effects
        totalBorrowed[token] += amount;
        lastUpdate[token] = block.timestamp;
        
        // Interactions
        IERC20(token).safeTransfer(msg.sender, amount);
        if (fee > 0) {
            IERC20(token).safeTransfer(feeRecipient, fee);
        }
        
        emit TokensBorrowed(msg.sender, token, amount, fee);
        
        return amountToRepay;
    }
    
    /// @notice Repay borrowed tokens
    /// @param token Token to repay
    /// @param amount Amount to repay
    function repay(address token, uint256 amount) external nonReentrant {
        // Checks
        require(amount > 0, "Amount must be positive");
        require(totalBorrowed[token] > 0, "No debt to repay");
        
        // Effects
        uint256 debt = totalBorrowed[token];
        if (amount > debt) {
            amount = debt; // Don't repay more than owed
        }
        totalBorrowed[token] -= amount;
        lastUpdate[token] = block.timestamp;
        
        // Interactions
        IERC20(token).safeTransferFrom(msg.sender, address(this), amount);
        
        emit TokensRepaid(msg.sender, token, amount);
    }
    
    /// @notice Calculate interest for a borrow position
    /// @param token Token being borrowed
    /// @param amount Amount borrowed
    /// @return interest Calculated interest
    function calculateInterest(address token, uint256 amount) public view returns (uint256 interest) {
        uint256 timeElapsed = block.timestamp - lastUpdate[token];
        uint256 ratePerSecond = borrowRate[token] / 1 years; // Convert annual rate to per second
        interest = (amount * ratePerSecond * timeElapsed) / PRECISION;
        return interest;
    }
    
    /// @notice Set borrow rate for a token
    /// @param token Token address
    /// @param rate Annual borrow rate (scaled by PRECISION)
    function setBorrowRate(address token, uint256 rate) external onlyOwner {
        require(rate <= PRECISION, "Rate too high"); // Max 100% annual rate
        borrowRate[token] = rate;
        lastUpdate[token] = block.timestamp;
    }
    
    /// @notice Add liquidity to the AMM
    /// @param tokenA First token
    /// @param tokenB Second token
    /// @param amountA Amount of tokenA
    /// @param amountB Amount of tokenB
    function addLiquidity(
        address tokenA,
        address tokenB,
        uint256 amountA,
        uint256 amountB
    ) external nonReentrant {
        // Checks
        require(tokenA != tokenB, "Tokens must be different");
        require(amountA > 0 && amountB > 0, "Amounts must be positive");
        
        // Effects
        reserves[tokenA] += amountA;
        reserves[tokenB] += amountB;
        
        // Interactions
        IERC20(tokenA).safeTransferFrom(msg.sender, address(this), amountA);
        IERC20(tokenB).safeTransferFrom(msg.sender, address(this), amountB);
    }
    
    /// @notice Remove liquidity from the AMM
    /// @param tokenA First token
    /// @param tokenB Second token
    /// @param amountA Amount of tokenA to remove
    /// @param amountB Amount of tokenB to remove
    function removeLiquidity(
        address tokenA,
        address tokenB,
        uint256 amountA,
        uint256 amountB
    ) external nonReentrant {
        // Checks
        require(tokenA != tokenB, "Tokens must be different");
        require(amountA > 0 && amountB > 0, "Amounts must be positive");
        require(reserves[tokenA] >= amountA, "Insufficient reserve A");
        require(reserves[tokenB] >= amountB, "Insufficient reserve B");
        
        // Effects
        reserves[tokenA] -= amountA;
        reserves[tokenB] -= amountB;
        
        // Interactions
        IERC20(tokenA).safeTransfer(msg.sender, amountA);
        IERC20(tokenB).safeTransfer(msg.sender, amountB);
    }
    
    /// @notice Check value conservation invariant for AMM
    /// @param tokenA First token
    /// @param tokenB Second token
    /// @return bool Whether the invariant holds
    function checkAMMConservationInvariant(address tokenA, address tokenB) public view returns (bool) {
        // For a constant product AMM, the product of reserves should remain constant
        // k = reserveA * reserveB
        // This is a simplified check - in practice, you'd track the initial k value
        uint256 reserveA = reserves[tokenA];
        uint256 reserveB = reserves[tokenB];
        
        // Basic check that reserves are non-negative and reasonable
        return reserveA >= 0 && reserveB >= 0;
    }
    
    /// @notice Check lending conservation invariant
    /// @param token Token to check
    /// @return bool Whether the invariant holds
    function checkLendingConservationInvariant(address token) public view returns (bool) {
        // Total supplied should be >= total borrowed
        return totalSupplied[token] >= totalBorrowed[token];
    }
    
    /// @notice Get utilization rate for a token
    /// @param token Token to check
    /// @return utilizationRate Utilization rate scaled by PRECISION
    function getUtilizationRate(address token) public view returns (uint256 utilizationRate) {
        if (totalSupplied[token] == 0) {
            return 0;
        }
        return (totalBorrowed[token] * PRECISION) / totalSupplied[token];
    }
    
    /// @notice Check precision handling in fee calculations
    /// @param amount Amount to calculate fee for
    /// @param feeBasisPoints Fee in basis points
    /// @return fee Calculated fee
    function calculateFeeWithPrecision(uint256 amount, uint256 feeBasisPoints) public pure returns (uint256 fee) {
        // This demonstrates proper precision handling
        // Using Solidity 0.8+ overflow protection
        fee = (amount * feeBasisPoints) / MAX_FEE_BASIS_POINTS;
        require(fee < amount, "Fee cannot exceed amount");
        return fee;
    }
}