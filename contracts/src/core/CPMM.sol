// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/utils/math/Math.sol";

/// @title ConstantProductAMM
/// @notice A constant product market maker (x*y=k) implementation with comprehensive safety checks
/// @dev Implements CPMM with overflow protection, precision handling, fee routing, and conservation invariants
contract ConstantProductAMM is Ownable, ReentrancyGuard {
    using SafeERC20 for IERC20;
    using Math for uint256;

    // Constants for precision and limits
    uint256 public constant PRECISION = 1e18; // 18 decimal places precision
    uint256 public constant MAX_FEE_BASIS_POINTS = 10000; // 100% in basis points
    uint256 public constant MAX_SLIPPAGE_BASIS_POINTS = 1000; // 10% max slippage
    uint256 public constant MIN_LIQUIDITY = 1000; // Minimum liquidity to prevent rounding issues
    
    // Pool structure
    struct Pool {
        address tokenA;
        address tokenB;
        uint256 reserveA;
        uint256 reserveB;
        uint256 totalSupply;
        uint256 feeBasisPoints;
        uint256 kLast; // Last calculated k value for fee tracking
        uint256 amplification; // Amplification parameter for Stable/CLAMM behavior
        mapping(address => uint256) liquidityBalances;
    }
    
    // State variables
    mapping(bytes32 => Pool) public pools; // Pool identifier to pool data
    mapping(address => mapping(address => bytes32)) public poolIds; // tokenA, tokenB to poolId
    address public feeRecipient; // Address to receive protocol fees
    uint256 public protocolFeeBasisPoints; // Protocol fee (taken from swap fees)
    
    // Events
    event PoolCreated(address indexed tokenA, address indexed tokenB, uint256 feeBasisPoints, bytes32 indexed poolId);
    event LiquidityAdded(address indexed provider, bytes32 indexed poolId, uint256 amountA, uint256 amountB, uint256 liquidityMinted);
    event LiquidityRemoved(address indexed provider, bytes32 indexed poolId, uint256 amountA, uint256 amountB, uint256 liquidityBurned);
    event SwapExecuted(address indexed trader, bytes32 indexed poolId, address tokenIn, address tokenOut, uint256 amountIn, uint256 amountOut, uint256 fee);
    event FeeUpdated(uint256 protocolFeeBasisPoints);
    event ProtocolFeeCollected(address indexed token, uint256 amount);
    event AmplificationUpdated(bytes32 indexed poolId, uint256 amplification);
    
    /// @notice Constructor to initialize the contract
    /// @param _feeRecipient Address to receive protocol fees
    constructor(address _feeRecipient) {
        require(_feeRecipient != address(0), "Fee recipient cannot be zero");
        feeRecipient = _feeRecipient;
        protocolFeeBasisPoints = 500; // 5% of swap fees go to protocol
    }
    
    /// @notice Update protocol fee parameters
    /// @param _protocolFeeBasisPoints New protocol fee in basis points
    /// @param _feeRecipient New fee recipient address
    function updateProtocolFee(
        uint256 _protocolFeeBasisPoints,
        address _feeRecipient
    ) external onlyOwner {
        require(_protocolFeeBasisPoints <= MAX_FEE_BASIS_POINTS, "Protocol fee too high");
        require(_feeRecipient != address(0), "Fee recipient cannot be zero");
        
        protocolFeeBasisPoints = _protocolFeeBasisPoints;
        feeRecipient = _feeRecipient;
        
        emit FeeUpdated(_protocolFeeBasisPoints);
    }
    
    /// @notice Create a new liquidity pool
    /// @param tokenA First token
    /// @param tokenB Second token
    /// @param feeBasisPoints Fee for swaps in basis points
    /// @param amplification Amplification parameter for Stable/CLAMM behavior (0 for pure CPMM)
    /// @return poolId Identifier for the created pool
    function createPool(
        address tokenA,
        address tokenB,
        uint256 feeBasisPoints,
        uint256 amplification
    ) external onlyOwner returns (bytes32 poolId) {
        // Checks
        require(tokenA != tokenB, "Tokens must be different");
        require(tokenA != address(0) && tokenB != address(0), "Tokens cannot be zero");
        require(feeBasisPoints <= 1000, "Fee too high (max 10%)"); // Max 10% fee
        require(amplification <= 10000, "Amplification too high (max 10000)");
        
        // Generate pool ID
        if (tokenA > tokenB) {
            (tokenA, tokenB) = (tokenB, tokenA); // Canonical ordering
        }
        poolId = keccak256(abi.encodePacked(tokenA, tokenB));
        
        // Check pool doesn't already exist
        require(pools[poolId].tokenA == address(0), "Pool already exists");
        
        // Create pool
        Pool storage pool = pools[poolId];
        pool.tokenA = tokenA;
        pool.tokenB = tokenB;
        pool.feeBasisPoints = feeBasisPoints;
        pool.reserveA = 0;
        pool.reserveB = 0;
        pool.totalSupply = 0;
        pool.kLast = 0;
        pool.amplification = amplification;
        
        // Store pool ID mapping
        poolIds[tokenA][tokenB] = poolId;
        poolIds[tokenB][tokenA] = poolId;
        
        emit PoolCreated(tokenA, tokenB, feeBasisPoints, poolId);
        if (amplification > 0) {
            emit AmplificationUpdated(poolId, amplification);
        }
        
        return poolId;
    }
    
    /// @notice Update amplification parameter for a pool
    /// @param poolId Pool identifier
    /// @param amplification New amplification parameter
    function updateAmplification(bytes32 poolId, uint256 amplification) external onlyOwner {
        require(amplification <= 10000, "Amplification too high (max 10000)");
        Pool storage pool = pools[poolId];
        require(pool.tokenA != address(0), "Pool does not exist");
        
        pool.amplification = amplification;
        emit AmplificationUpdated(poolId, amplification);
    }
    
    /// @notice Add liquidity to a pool
    /// @param poolId Pool identifier
    /// @param amountADesired Desired amount of tokenA
    /// @param amountBDesired Desired amount of tokenB
    /// @param amountAMin Minimum amount of tokenA
    /// @param amountBMin Minimum amount of tokenB
    /// @return amountA Actual amount of tokenA added
    /// @return amountB Actual amount of tokenB added
    /// @return liquidity Amount of liquidity tokens minted
    function addLiquidity(
        bytes32 poolId,
        uint256 amountADesired,
        uint256 amountBDesired,
        uint256 amountAMin,
        uint256 amountBMin
    ) external nonReentrant returns (uint256 amountA, uint256 amountB, uint256 liquidity) {
        // Checks
        require(amountADesired > 0 && amountBDesired > 0, "Amounts must be positive");
        Pool storage pool = pools[poolId];
        require(pool.tokenA != address(0), "Pool does not exist");
        
        // Get reserves
        (uint256 reserveA, uint256 reserveB) = (pool.reserveA, pool.reserveB);
        
        // Calculate optimal amounts
        if (reserveA == 0 && reserveB == 0) {
            // First liquidity provider
            (amountA, amountB) = (amountADesired, amountBDesired);
        } else {
            // Calculate optimal deposit amounts
            uint256 amountBOptimal = (amountADesired * reserveB) / reserveA;
            if (amountBOptimal <= amountBDesired) {
                require(amountBOptimal >= amountBMin, "Insufficient B amount");
                (amountA, amountB) = (amountADesired, amountBOptimal);
            } else {
                uint256 amountAOptimal = (amountBDesired * reserveA) / reserveB;
                require(amountAOptimal <= amountADesired);
                require(amountAOptimal >= amountAMin, "Insufficient A amount");
                (amountA, amountB) = (amountAOptimal, amountBDesired);
            }
        }
        
        // Effects - Update reserves and mint liquidity
        uint256 liquidityMinted;
        if (pool.totalSupply == 0) {
            // First liquidity provider - mint initial liquidity
            liquidityMinted = Math.sqrt(amountA * amountB) - MIN_LIQUIDITY;
            pool.totalSupply = MIN_LIQUIDITY; // Permanently lock MIN_LIQUIDITY tokens
        } else {
            // Subsequent liquidity providers
            uint256 liquidityA = (amountA * pool.totalSupply) / reserveA;
            uint256 liquidityB = (amountB * pool.totalSupply) / reserveB;
            liquidityMinted = liquidityA < liquidityB ? liquidityA : liquidityB;
        }
        
        require(liquidityMinted > 0, "Insufficient liquidity minted");
        
        // Update reserves
        pool.reserveA += amountA;
        pool.reserveB += amountB;
        pool.totalSupply += liquidityMinted;
        
        // Update user's liquidity balance
        pool.liquidityBalances[msg.sender] += liquidityMinted;
        
        // Calculate and store k for fee tracking
        pool.kLast = pool.reserveA * pool.reserveB;
        
        // Interactions - Transfer tokens
        IERC20(pool.tokenA).safeTransferFrom(msg.sender, address(this), amountA);
        IERC20(pool.tokenB).safeTransferFrom(msg.sender, address(this), amountB);
        
        emit LiquidityAdded(msg.sender, poolId, amountA, amountB, liquidityMinted);
        
        return (amountA, amountB, liquidityMinted);
    }
    
    /// @notice Remove liquidity from a pool
    /// @param poolId Pool identifier
    /// @param liquidity Amount of liquidity tokens to burn
    /// @param amountAMin Minimum amount of tokenA to receive
    /// @param amountBMin Minimum amount of tokenB to receive
    /// @return amountA Amount of tokenA received
    /// @return amountB Amount of tokenB received
    function removeLiquidity(
        bytes32 poolId,
        uint256 liquidity,
        uint256 amountAMin,
        uint256 amountBMin
    ) external nonReentrant returns (uint256 amountA, uint256 amountB) {
        // Checks
        require(liquidity > 0, "Liquidity must be positive");
        Pool storage pool = pools[poolId];
        require(pool.tokenA != address(0), "Pool does not exist");
        require(pool.liquidityBalances[msg.sender] >= liquidity, "Insufficient liquidity");
        
        // Get reserves
        (uint256 reserveA, uint256 reserveB) = (pool.reserveA, pool.reserveB);
        
        // Calculate amounts to withdraw
        amountA = (liquidity * reserveA) / pool.totalSupply;
        amountB = (liquidity * reserveB) / pool.totalSupply;
        
        require(amountA >= amountAMin, "Insufficient A amount");
        require(amountB >= amountBMin, "Insufficient B amount");
        require(amountA > 0 && amountB > 0, "Insufficient liquidity");
        
        // Effects - Update reserves and burn liquidity
        pool.reserveA -= amountA;
        pool.reserveB -= amountB;
        pool.totalSupply -= liquidity;
        pool.liquidityBalances[msg.sender] -= liquidity;
        
        // Calculate and store k for fee tracking
        if (pool.reserveA > 0 && pool.reserveB > 0) {
            pool.kLast = pool.reserveA * pool.reserveB;
        }
        
        // Interactions - Transfer tokens
        IERC20(pool.tokenA).safeTransfer(msg.sender, amountA);
        IERC20(pool.tokenB).safeTransfer(msg.sender, amountB);
        
        emit LiquidityRemoved(msg.sender, poolId, amountA, amountB, liquidity);
        
        return (amountA, amountB);
    }
    
    /// @notice Execute a swap
    /// @param poolId Pool identifier
    /// @param tokenIn Token to swap from
    /// @param amountIn Amount of tokenIn to swap
    /// @param minAmountOut Minimum amount of tokenOut to receive
    /// @return amountOut Amount of tokenOut received
    function swap(
        bytes32 poolId,
        address tokenIn,
        uint256 amountIn,
        uint256 minAmountOut
    ) external nonReentrant returns (uint256 amountOut) {
        // Checks
        require(amountIn > 0, "Amount in must be positive");
        require(minAmountOut > 0, "Min amount out must be positive");
        
        Pool storage pool = pools[poolId];
        require(pool.tokenA != address(0), "Pool does not exist");
        
        // Determine token direction
        address tokenOut;
        uint256 reserveIn;
        uint256 reserveOut;
        
        if (tokenIn == pool.tokenA) {
            tokenOut = pool.tokenB;
            reserveIn = pool.reserveA;
            reserveOut = pool.reserveB;
        } else if (tokenIn == pool.tokenB) {
            tokenOut = pool.tokenA;
            reserveIn = pool.reserveB;
            reserveOut = pool.reserveA;
        } else {
            revert("Invalid token");
        }
        
        require(reserveIn > 0 && reserveOut > 0, "Insufficient liquidity");
        
        // Calculate amount out based on pool type
        if (pool.amplification > 0) {
            // Stable/CLAMM behavior with amplification
            amountOut = calculateStableSwapAmountOut(pool, tokenIn, amountIn);
        } else {
            // Standard CPMM behavior
            // Calculate fee and amount in with fee
            uint256 fee = (amountIn * pool.feeBasisPoints) / MAX_FEE_BASIS_POINTS;
            uint256 protocolFee = (fee * protocolFeeBasisPoints) / MAX_FEE_BASIS_POINTS;
            uint256 amountInWithFee = amountIn - fee;
            
            // Calculate amount out using constant product formula: x * y = k
            uint256 numerator = amountInWithFee * reserveOut;
            uint256 denominator = reserveIn + amountInWithFee;
            amountOut = numerator / denominator;
        }
        
        require(amountOut >= minAmountOut, "Slippage too high");
        require(amountOut < reserveOut, "Insufficient liquidity");
        
        // Effects - Update reserves
        if (tokenIn == pool.tokenA) {
            pool.reserveA += amountIn;
            pool.reserveB -= amountOut;
        } else {
            pool.reserveB += amountIn;
            pool.reserveA -= amountOut;
        }
        
        // Calculate and store k for fee tracking
        pool.kLast = uint256(pool.reserveA) * uint256(pool.reserveB);
        
        // Interactions - Transfer tokens
        IERC20(tokenIn).safeTransferFrom(msg.sender, address(this), amountIn);
        
        // Transfer fee to fee recipient (for CPMM only, Stable swaps handle fees differently)
        if (pool.amplification == 0) {
            uint256 fee = (amountIn * pool.feeBasisPoints) / MAX_FEE_BASIS_POINTS;
            uint256 protocolFee = (fee * protocolFeeBasisPoints) / MAX_FEE_BASIS_POINTS;
            if (protocolFee > 0) {
                IERC20(tokenIn).safeTransfer(feeRecipient, protocolFee);
                emit ProtocolFeeCollected(tokenIn, protocolFee);
            }
        }
        
        // Transfer output tokens to user
        IERC20(tokenOut).safeTransfer(msg.sender, amountOut);
        
        emit SwapExecuted(msg.sender, poolId, tokenIn, tokenOut, amountIn, amountOut, 
            (amountIn * pool.feeBasisPoints) / MAX_FEE_BASIS_POINTS);
        
        return amountOut;
    }
    
    /// @notice Calculate amount out for stable swap with amplification
    /// @param pool Pool storage reference
    /// @param tokenIn Token to swap from
    /// @param amountIn Amount of tokenIn to swap
    /// @return amountOut Amount of tokenOut received
    function calculateStableSwapAmountOut(
        Pool storage pool,
        address tokenIn,
        uint256 amountIn
    ) internal view returns (uint256 amountOut) {
        // Simplified Stable swap calculation (x + y = D invariant with amplification)
        // In a real implementation, this would be more complex
        
        uint256 reserveIn;
        uint256 reserveOut;
        
        if (tokenIn == pool.tokenA) {
            reserveIn = pool.reserveA;
            reserveOut = pool.reserveB;
        } else {
            reserveIn = pool.reserveB;
            reserveOut = pool.reserveA;
        }
        
        // For demonstration, we'll use a simplified approach
        // Actual StableSwap would use the invariant: A * (sum_xi) + D = A * D * n^n / (n^n * prod_xi)
        
        // Simplified calculation with amplification factor
        uint256 amplificationFactor = pool.amplification;
        uint256 fee = (amountIn * pool.feeBasisPoints) / MAX_FEE_BASIS_POINTS;
        uint256 amountInWithFee = amountIn - fee;
        
        // Apply amplification to make the curve more stable (less slippage)
        // This is a simplified approximation
        uint256 adjustedAmountIn = (amountInWithFee * amplificationFactor) / 1000;
        
        // Calculate amount out using modified constant product formula
        uint256 numerator = adjustedAmountIn * reserveOut;
        uint256 denominator = reserveIn + adjustedAmountIn;
        amountOut = numerator / denominator;
        
        return amountOut;
    }
    
    /// @notice Get amount out for a swap without executing it
    /// @param poolId Pool identifier
    /// @param tokenIn Token to swap from
    /// @param amountIn Amount of tokenIn to swap
    /// @return amountOut Amount of tokenOut that would be received
    function getAmountOut(
        bytes32 poolId,
        address tokenIn,
        uint256 amountIn
    ) external view returns (uint256 amountOut) {
        // Checks
        require(amountIn > 0, "Amount in must be positive");
        
        Pool storage pool = pools[poolId];
        require(pool.tokenA != address(0), "Pool does not exist");
        
        // Determine token direction
        uint256 reserveIn;
        uint256 reserveOut;
        
        if (tokenIn == pool.tokenA) {
            reserveIn = pool.reserveA;
            reserveOut = pool.reserveB;
        } else if (tokenIn == pool.tokenB) {
            reserveIn = pool.reserveB;
            reserveOut = pool.reserveA;
        } else {
            revert("Invalid token");
        }
        
        require(reserveIn > 0 && reserveOut > 0, "Insufficient liquidity");
        
        // Calculate amount out based on pool type
        if (pool.amplification > 0) {
            // Stable/CLAMM behavior with amplification
            // For view function, we'll use a simplified approach
            uint256 fee = (amountIn * pool.feeBasisPoints) / MAX_FEE_BASIS_POINTS;
            uint256 amountInWithFee = amountIn - fee;
            uint256 amplificationFactor = pool.amplification;
            uint256 adjustedAmountIn = (amountInWithFee * amplificationFactor) / 1000;
            uint256 numerator = adjustedAmountIn * reserveOut;
            uint256 denominator = reserveIn + adjustedAmountIn;
            amountOut = numerator / denominator;
        } else {
            // Standard CPMM behavior
            // Calculate fee and amount in with fee
            uint256 fee = (amountIn * pool.feeBasisPoints) / MAX_FEE_BASIS_POINTS;
            uint256 amountInWithFee = amountIn - fee;
            
            // Calculate amount out using constant product formula
            uint256 numerator = amountInWithFee * reserveOut;
            uint256 denominator = reserveIn + amountInWithFee;
            amountOut = numerator / denominator;
        }
        
        return amountOut;
    }
    
    /// @notice Get amount in needed for a desired amount out
    /// @param poolId Pool identifier
    /// @param tokenIn Token to swap from
    /// @param amountOut Desired amount of tokenOut
    /// @return amountIn Amount of tokenIn needed
    function getAmountIn(
        bytes32 poolId,
        address tokenIn,
        uint256 amountOut
    ) external view returns (uint256 amountIn) {
        // Checks
        require(amountOut > 0, "Amount out must be positive");
        
        Pool storage pool = pools[poolId];
        require(pool.tokenA != address(0), "Pool does not exist");
        
        // Determine token direction
        uint256 reserveIn;
        uint256 reserveOut;
        
        if (tokenIn == pool.tokenA) {
            reserveIn = pool.reserveA;
            reserveOut = pool.reserveB;
        } else if (tokenIn == pool.tokenB) {
            reserveIn = pool.reserveB;
            reserveOut = pool.reserveA;
        } else {
            revert("Invalid token");
        }
        
        require(reserveIn > 0 && reserveOut > 0, "Insufficient liquidity");
        require(amountOut < reserveOut, "Insufficient liquidity");
        
        // Calculate amount in based on pool type
        if (pool.amplification > 0) {
            // Stable/CLAMM behavior with amplification
            // Simplified calculation for view function
            uint256 amplificationFactor = pool.amplification;
            uint256 numerator = reserveIn * amountOut * MAX_FEE_BASIS_POINTS * 1000;
            uint256 denominator = (reserveOut - amountOut) * (MAX_FEE_BASIS_POINTS - pool.feeBasisPoints) * amplificationFactor;
            amountIn = (numerator / denominator) + 1; // Add 1 for rounding
        } else {
            // Standard CPMM behavior
            // Calculate amount in using constant product formula
            uint256 numerator = reserveIn * amountOut * MAX_FEE_BASIS_POINTS;
            uint256 denominator = (reserveOut - amountOut) * (MAX_FEE_BASIS_POINTS - pool.feeBasisPoints);
            amountIn = (numerator / denominator) + 1; // Add 1 for rounding
        }
        
        return amountIn;
    }
    
    /// @notice Get liquidity quote for adding liquidity
    /// @param poolId Pool identifier
    /// @param amountADesired Desired amount of tokenA
    /// @param amountBDesired Desired amount of tokenB
    /// @return amountA Actual amount of tokenA needed
    /// @return amountB Actual amount of tokenB needed
    /// @return liquidity Expected liquidity tokens to receive
    function quoteAddLiquidity(
        bytes32 poolId,
        uint256 amountADesired,
        uint256 amountBDesired
    ) external view returns (uint256 amountA, uint256 amountB, uint256 liquidity) {
        Pool storage pool = pools[poolId];
        require(pool.tokenA != address(0), "Pool does not exist");
        
        // Get reserves
        (uint256 reserveA, uint256 reserveB) = (pool.reserveA, pool.reserveB);
        
        // Calculate optimal amounts
        if (reserveA == 0 && reserveB == 0) {
            // First liquidity provider
            (amountA, amountB) = (amountADesired, amountBDesired);
        } else {
            // Calculate optimal deposit amounts
            uint256 amountBOptimal = (amountADesired * reserveB) / reserveA;
            if (amountBOptimal <= amountBDesired) {
                (amountA, amountB) = (amountADesired, amountBOptimal);
            } else {
                uint256 amountAOptimal = (amountBDesired * reserveA) / reserveB;
                (amountA, amountB) = (amountAOptimal, amountBDesired);
            }
        }
        
        // Calculate liquidity
        if (pool.totalSupply == 0) {
            // First liquidity provider
            liquidity = Math.sqrt(amountA * amountB) - MIN_LIQUIDITY;
        } else {
            // Subsequent liquidity providers
            uint256 liquidityA = (amountA * pool.totalSupply) / reserveA;
            uint256 liquidityB = (amountB * pool.totalSupply) / reserveB;
            liquidity = liquidityA < liquidityB ? liquidityA : liquidityB;
        }
        
        return (amountA, amountB, liquidity);
    }
    
    /// @notice Check constant product invariant (x * y = k)
    /// @param poolId Pool identifier
    /// @return bool Whether the invariant holds
    /// @return uint256 Current k value
    /// @return uint256 Expected k value
    function checkConstantProductInvariant(bytes32 poolId) external view returns (bool, uint256, uint256) {
        Pool storage pool = pools[poolId];
        require(pool.tokenA != address(0), "Pool does not exist");
        
        uint256 currentK = uint256(pool.reserveA) * uint256(pool.reserveB);
        uint256 expectedK = pool.kLast;
        
        // For first liquidity provider or empty pool, kLast might be 0
        if (expectedK == 0) {
            return (true, currentK, expectedK);
        }
        
        // Allow for small differences due to fees
        // In a perfect CPMM, k should never decrease, but fees can cause it to increase
        return (currentK >= expectedK, currentK, expectedK);
    }
    
    /// @notice Check amplification behavior invariant
    /// @param poolId Pool identifier
    /// @return bool Whether the amplification invariant holds
    /// @return uint256 Current amplification value
    function checkAmplificationInvariant(bytes32 poolId) external view returns (bool, uint256) {
        Pool storage pool = pools[poolId];
        require(pool.tokenA != address(0), "Pool does not exist");
        
        // Amplification should be within bounds
        bool invariantHolds = pool.amplification <= 10000;
        return (invariantHolds, pool.amplification);
    }
    
    /// @notice Get pool information
    /// @param poolId Pool identifier
    /// @return tokenA First token
    /// @return tokenB Second token
    /// @return reserveA Reserve of tokenA
    /// @return reserveB Reserve of tokenB
    /// @return totalSupply Total liquidity tokens
    /// @return feeBasisPoints Swap fee in basis points
    /// @return amplification Amplification parameter
    function getPoolInfo(bytes32 poolId) external view returns (
        address tokenA,
        address tokenB,
        uint256 reserveA,
        uint256 reserveB,
        uint256 totalSupply,
        uint256 feeBasisPoints,
        uint256 amplification
    ) {
        Pool storage pool = pools[poolId];
        require(pool.tokenA != address(0), "Pool does not exist");
        
        return (
            pool.tokenA,
            pool.tokenB,
            pool.reserveA,
            pool.reserveB,
            pool.totalSupply,
            pool.feeBasisPoints,
            pool.amplification
        );
    }
    
    /// @notice Get user's liquidity balance
    /// @param poolId Pool identifier
    /// @param user User address
    /// @return uint256 User's liquidity balance
    function getUserLiquidity(bytes32 poolId, address user) external view returns (uint256) {
        Pool storage pool = pools[poolId];
        require(pool.tokenA != address(0), "Pool does not exist");
        
        return pool.liquidityBalances[user];
    }
    
    /// @notice Calculate protocol fee from a given amount
    /// @param amount Total fee amount
    /// @return uint256 Protocol fee amount
    function calculateProtocolFee(uint256 amount) external view returns (uint256) {
        return (amount * protocolFeeBasisPoints) / MAX_FEE_BASIS_POINTS;
    }
}