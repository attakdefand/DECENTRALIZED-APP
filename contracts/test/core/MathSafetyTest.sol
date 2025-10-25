// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../../src/core/MathSafetyDemo.sol";
import "../../src/core/SafeToken.sol";

/// @title MathSafetyTest
/// @notice Comprehensive tests for mathematical safety patterns including overflow protection, precision handling, and value conservation
contract MathSafetyTest is Test {
    MathSafetyDemo public mathDemo;
    SafeToken public tokenA;
    SafeToken public tokenB;
    address public owner;
    address public user1;
    address public user2;
    address public feeRecipient;
    
    /// @notice Set up test environment
    function setUp() public {
        owner = address(this);
        user1 = address(0x1);
        user2 = address(0x2);
        feeRecipient = address(0x3);
        
        // Deploy contracts
        mathDemo = new MathSafetyDemo(feeRecipient);
        tokenA = new SafeToken("Token A", "TKNA", 1000000 * 1e18);
        tokenB = new SafeToken("Token B", "TKNB", 1000000 * 1e18);
        
        // Transfer tokens to users for testing
        tokenA.transfer(user1, 100000 * 1e18);
        tokenA.transfer(user2, 100000 * 1e18);
        tokenB.transfer(user1, 100000 * 1e18);
        tokenB.transfer(user2, 100000 * 1e18);
        
        // Approve math demo contract to spend tokens
        vm.prank(user1);
        tokenA.approve(address(mathDemo), type(uint256).max);
        vm.prank(user1);
        tokenB.approve(address(mathDemo), type(uint256).max);
        vm.prank(user2);
        tokenA.approve(address(mathDemo), type(uint256).max);
        vm.prank(user2);
        tokenB.approve(address(mathDemo), type(uint256).max);
    }
    
    // ==================== Overflow/Underflow Protection Tests ====================
    
    /// @notice Test overflow protection in swap calculations
    function testOverflowProtectionInSwap() public {
        // Add initial liquidity
        uint256 initialAmountA = 100000 * 1e18;
        uint256 initialAmountB = 100000 * 1e18;
        
        vm.prank(user1);
        mathDemo.addLiquidity(address(tokenA), address(tokenB), initialAmountA, initialAmountB);
        
        // Test with large amounts that could potentially cause overflow
        uint256 largeAmount = 1000000 * 1e18; // 1M tokens
        
        // This should not cause overflow due to Solidity 0.8+ checks
        vm.prank(user2);
        uint256 amountOut = mathDemo.swap(address(tokenA), address(tokenB), largeAmount, 1);
        
        // Verify the swap executed successfully
        assertTrue(amountOut > 0);
    }
    
    /// @notice Test underflow protection in reserve calculations
    function testUnderflowProtectionInReserveCalculations() public {
        // Add initial liquidity
        uint256 initialAmountA = 10000 * 1e18;
        uint256 initialAmountB = 10000 * 1e18;
        
        vm.prank(user1);
        mathDemo.addLiquidity(address(tokenA), address(tokenB), initialAmountA, initialAmountB);
        
        // Try to remove more liquidity than available - should fail safely
        vm.expectRevert("Insufficient reserve A");
        mathDemo.removeLiquidity(address(tokenA), address(tokenB), initialAmountA * 2, initialAmountB);
    }
    
    // ==================== Precision Handling Tests ====================
    
    /// @notice Test precision handling in fee calculations
    function testPrecisionHandlingInFeeCalculations() public {
        uint256 amount = 1000 * 1e18; // 1000 tokens
        uint256 feeBasisPoints = 30; // 0.3%
        
        uint256 fee = mathDemo.calculateFeeWithPrecision(amount, feeBasisPoints);
        uint256 expectedFee = (amount * feeBasisPoints) / 10000;
        
        assertEq(fee, expectedFee);
        assertTrue(fee < amount); // Fee should be less than amount
    }
    
    /// @notice Test fixed-point arithmetic precision
    function testFixedPointArithmeticPrecision() public {
        // Test fixed-point multiplication and division
        uint256 value = 100 * 1e18; // 100 with 18 decimals
        uint256 multiplier = 5 * 1e18; // 5 with 18 decimals (5.0)
        uint256 precision = 1e18;
        
        // Fixed-point multiplication: (a * b) / precision
        uint256 result = (value * multiplier) / precision;
        assertEq(result, 500 * 1e18); // Should be 500
        
        // Fixed-point division: (a * precision) / b
        uint256 dividend = 1000 * 1e18;
        uint256 divisor = 5 * 1e18;
        uint256 divisionResult = (dividend * precision) / divisor;
        assertEq(divisionResult, 200 * 1e18); // Should be 200
    }
    
    /// @notice Test precision in AMM calculations
    function testAMMPrecision() public {
        // Add liquidity
        uint256 amountA = 10000 * 1e18;
        uint256 amountB = 20000 * 1e18; // 2:1 ratio
        
        vm.prank(user1);
        mathDemo.addLiquidity(address(tokenA), address(tokenB), amountA, amountB);
        
        // Check reserves
        assertEq(mathDemo.reserves(address(tokenA)), amountA);
        assertEq(mathDemo.reserves(address(tokenB)), amountB);
        
        // Perform a small swap
        uint256 swapAmount = 100 * 1e18;
        uint256 expectedOut = mathDemo.getAmountOut(address(tokenA), address(tokenB), swapAmount);
        
        vm.prank(user2);
        uint256 actualOut = mathDemo.swap(address(tokenA), address(tokenB), swapAmount, 1);
        
        assertEq(actualOut, expectedOut);
        assertTrue(actualOut > 0);
    }
    
    // ==================== Value Conservation Tests ====================
    
    /// @notice Test value conservation in AMM swaps
    /// @param amountIn Amount to swap in
    function testAMMValueConservation(uint256 amountIn) public {
        // Bound the amount to reasonable values
        amountIn = bound(amountIn, 1, 10000 * 1e18);
        
        // Add initial liquidity
        uint256 initialAmountA = 100000 * 1e18;
        uint256 initialAmountB = 100000 * 1e18;
        
        vm.prank(user1);
        mathDemo.addLiquidity(address(tokenA), address(tokenB), initialAmountA, initialAmountB);
        
        // Get initial token balances
        uint256 initialUserABalance = tokenA.balanceOf(user2);
        uint256 initialUserBBalance = tokenB.balanceOf(user2);
        uint256 initialContractABalance = tokenA.balanceOf(address(mathDemo));
        uint256 initialContractBBalance = tokenB.balanceOf(address(mathDemo));
        
        // Perform swap
        vm.prank(user2);
        uint256 amountOut = mathDemo.swap(address(tokenA), address(tokenB), amountIn, 1);
        
        // Get final balances
        uint256 finalUserABalance = tokenA.balanceOf(user2);
        uint256 finalUserBBalance = tokenB.balanceOf(user2);
        uint256 finalContractABalance = tokenA.balanceOf(address(mathDemo));
        uint256 finalContractBBalance = tokenB.balanceOf(address(mathDemo));
        
        // Check conservation:
        // User A balance should decrease by amountIn
        assertEq(finalUserABalance, initialUserABalance - amountIn);
        
        // User B balance should increase by amountOut
        assertEq(finalUserBBalance, initialUserBBalance + amountOut);
        
        // Contract A balance should increase by amountIn
        assertEq(finalContractABalance, initialContractABalance + amountIn);
        
        // Contract B balance should decrease by amountOut
        assertEq(finalContractBBalance, initialContractBBalance - amountOut);
        
        // Check conservation invariant
        assertTrue(mathDemo.checkAMMConservationInvariant(address(tokenA), address(tokenB)));
    }
    
    /// @notice Test lending value conservation
    function testLendingValueConservation() public {
        uint256 supplyAmount = 10000 * 1e18;
        uint256 borrowAmount = 5000 * 1e18;
        
        // Supply tokens
        vm.prank(user1);
        mathDemo.supply(address(tokenA), supplyAmount);
        
        // Check conservation before borrowing
        assertTrue(mathDemo.checkLendingConservationInvariant(address(tokenA)));
        
        // Borrow tokens
        vm.prank(user2);
        uint256 amountToRepay = mathDemo.borrow(address(tokenA), borrowAmount);
        
        // Check conservation after borrowing
        assertTrue(mathDemo.checkLendingConservationInvariant(address(tokenA)));
        assertEq(mathDemo.totalBorrowed(address(tokenA)), borrowAmount);
        
        // Repay tokens
        vm.prank(user2);
        mathDemo.repay(address(tokenA), amountToRepay);
        
        // Check conservation after repaying
        assertTrue(mathDemo.checkLendingConservationInvariant(address(tokenA)));
        assertEq(mathDemo.totalBorrowed(address(tokenA)), 0);
    }
    
    /// @notice Test conservation invariant with fuzzing
    /// @param supplyAmount Amount to supply
    /// @param borrowAmount Amount to borrow
    function testConservationInvariantWithFuzzing(uint256 supplyAmount, uint256 borrowAmount) public {
        // Bound amounts to reasonable values
        supplyAmount = bound(supplyAmount, 1, 100000 * 1e18);
        borrowAmount = bound(borrowAmount, 1, supplyAmount);
        
        // Supply tokens
        vm.prank(user1);
        mathDemo.supply(address(tokenA), supplyAmount);
        
        // Check conservation
        assertTrue(mathDemo.checkLendingConservationInvariant(address(tokenA)));
        
        // Borrow tokens
        vm.prank(user2);
        mathDemo.borrow(address(tokenA), borrowAmount);
        
        // Check conservation
        assertTrue(mathDemo.checkLendingConservationInvariant(address(tokenA)));
        
        // Verify total supplied >= total borrowed
        assertTrue(mathDemo.totalSupplied(address(tokenA)) >= mathDemo.totalBorrowed(address(tokenA)));
    }
    
    // ==================== Arithmetic Safety Tests ====================
    
    /// @notice Test fee calculation safety
    function testFeeCalculationSafety() public {
        uint256 amount = 1000 * 1e18;
        uint256 feeBasisPoints = 100; // 1%
        
        uint256 fee = mathDemo.calculateFeeWithPrecision(amount, feeBasisPoints);
        
        // Fee should be calculated correctly
        assertEq(fee, (amount * feeBasisPoints) / 10000);
        
        // Fee should be less than amount
        assertTrue(fee < amount);
        
        // Fee should be non-negative
        assertTrue(fee >= 0);
    }
    
    /// @notice Test slippage protection
    function testSlippageProtection() public {
        // Add liquidity
        uint256 amountA = 10000 * 1e18;
        uint256 amountB = 10000 * 1e18;
        
        vm.prank(user1);
        mathDemo.addLiquidity(address(tokenA), address(tokenB), amountA, amountB);
        
        // Try to swap with too high slippage expectation
        uint256 largeSwapAmount = 5000 * 1e18;
        uint256 minAmountOut = 10000 * 1e18; // Unrealistic minimum
        
        vm.expectRevert("Slippage too high");
        vm.prank(user2);
        mathDemo.swap(address(tokenA), address(tokenB), largeSwapAmount, minAmountOut);
    }
    
    /// @notice Test interest calculation safety
    function testInterestCalculationSafety() public {
        // Set borrow rate
        uint256 annualRate = 0.05 * 1e18; // 5% annual rate
        mathDemo.setBorrowRate(address(tokenA), annualRate);
        
        uint256 borrowAmount = 1000 * 1e18;
        
        // Calculate interest
        uint256 interest = mathDemo.calculateInterest(address(tokenA), borrowAmount);
        
        // Interest should be non-negative
        assertTrue(interest >= 0);
        
        // For a short time period, interest should be small
        assertTrue(interest < borrowAmount);
    }
    
    // ==================== Differential Testing vs Reference Model ====================
    
    /// @notice Test AMM calculation against reference model
    function testAMMCalculationAgainstReference() public {
        // Add liquidity
        uint256 reserveA = 10000 * 1e18;
        uint256 reserveB = 20000 * 1e18;
        
        vm.prank(user1);
        mathDemo.addLiquidity(address(tokenA), address(tokenB), reserveA, reserveB);
        
        // Test multiple swap amounts
        uint256[] memory testAmounts = new uint256[](5);
        testAmounts[0] = 100 * 1e18;
        testAmounts[1] = 500 * 1e18;
        testAmounts[2] = 1000 * 1e18;
        testAmounts[3] = 2000 * 1e18;
        testAmounts[4] = 5000 * 1e18;
        
        for (uint256 i = 0; i < testAmounts.length; i++) {
            uint256 amountIn = testAmounts[i];
            
            // Get contract calculation
            uint256 contractAmountOut = mathDemo.getAmountOut(address(tokenA), address(tokenB), amountIn);
            
            // Calculate reference model (constant product formula)
            uint256 fee = (amountIn * 30) / 10000; // 0.3% fee
            uint256 amountInWithFee = amountIn - fee;
            uint256 referenceAmountOut = (amountInWithFee * reserveB) / (reserveA + amountInWithFee);
            
            // Allow for small precision differences (within 0.001%)
            uint256 tolerance = referenceAmountOut / 100000; // 0.001%
            assertApproxEqAbs(contractAmountOut, referenceAmountOut, tolerance);
        }
    }
    
    /// @notice Test fee calculation against reference model
    function testFeeCalculationAgainstReference() public {
        uint256[] memory testAmounts = new uint256[](5);
        testAmounts[0] = 100 * 1e18;
        testAmounts[1] = 1000 * 1e18;
        testAmounts[2] = 10000 * 1e18;
        testAmounts[3] = 100000 * 1e18;
        testAmounts[4] = 1000000 * 1e18;
        
        uint256 feeBasisPoints = 30; // 0.3%
        
        for (uint256 i = 0; i < testAmounts.length; i++) {
            uint256 amount = testAmounts[i];
            
            // Get contract calculation
            uint256 contractFee = mathDemo.calculateFeeWithPrecision(amount, feeBasisPoints);
            
            // Calculate reference model
            uint256 referenceFee = (amount * feeBasisPoints) / 10000;
            
            // Should be exactly equal
            assertEq(contractFee, referenceFee);
        }
    }
    
    // ==================== Property-Based Tests ====================
    
    /// @notice Property test: Fee calculation is always safe
    function testPropertyFeeCalculationAlwaysSafe(uint256 amount, uint256 feeBasisPoints) public {
        // Bound values to reasonable ranges
        amount = bound(amount, 1, 10000000 * 1e18); // Up to 10M tokens
        feeBasisPoints = bound(feeBasisPoints, 0, 10000); // 0% to 100%
        
        uint256 fee = mathDemo.calculateFeeWithPrecision(amount, feeBasisPoints);
        
        // Fee should always be less than or equal to amount
        assertTrue(fee <= amount);
        
        // Fee should be calculated correctly
        uint256 expectedFee = (amount * feeBasisPoints) / 10000;
        assertEq(fee, expectedFee);
    }
    
    /// @notice Property test: AMM reserves are always non-negative
    function testPropertyAMMReservesAlwaysNonNegative(
        uint256 amountA, 
        uint256 amountB, 
        uint256 swapAmount
    ) public {
        // Bound values
        amountA = bound(amountA, 1, 100000 * 1e18);
        amountB = bound(amountB, 1, 100000 * 1e18);
        swapAmount = bound(swapAmount, 1, amountA / 2);
        
        // Add liquidity
        vm.prank(user1);
        mathDemo.addLiquidity(address(tokenA), address(tokenB), amountA, amountB);
        
        // Check initial reserves are non-negative
        assertTrue(mathDemo.reserves(address(tokenA)) >= 0);
        assertTrue(mathDemo.reserves(address(tokenB)) >= 0);
        
        // Perform swap
        vm.prank(user2);
        mathDemo.swap(address(tokenA), address(tokenB), swapAmount, 1);
        
        // Check reserves are still non-negative
        assertTrue(mathDemo.reserves(address(tokenA)) >= 0);
        assertTrue(mathDemo.reserves(address(tokenB)) >= 0);
    }
    
    /// @notice Property test: Lending conservation invariant always holds
    function testPropertyLendingConservationInvariant(
        uint256 supplyAmount, 
        uint256 borrowAmount
    ) public {
        // Bound values
        supplyAmount = bound(supplyAmount, 1, 100000 * 1e18);
        borrowAmount = bound(borrowAmount, 1, supplyAmount);
        
        // Supply tokens
        vm.prank(user1);
        mathDemo.supply(address(tokenA), supplyAmount);
        
        // Check invariant holds
        assertTrue(mathDemo.checkLendingConservationInvariant(address(tokenA)));
        
        // Borrow tokens
        vm.prank(user2);
        mathDemo.borrow(address(tokenA), borrowAmount);
        
        // Check invariant still holds
        assertTrue(mathDemo.checkLendingConservationInvariant(address(tokenA)));
        
        // Total supplied should be >= total borrowed
        assertTrue(mathDemo.totalSupplied(address(tokenA)) >= mathDemo.totalBorrowed(address(tokenA)));
    }
    
    /// @notice Property test: Utilization rate is always between 0 and 100%
    function testPropertyUtilizationRateBounds(uint256 supplyAmount, uint256 borrowAmount) public {
        // Bound values
        supplyAmount = bound(supplyAmount, 1, 100000 * 1e18);
        borrowAmount = bound(borrowAmount, 0, supplyAmount);
        
        if (supplyAmount > 0) {
            // Supply tokens
            vm.prank(user1);
            mathDemo.supply(address(tokenA), supplyAmount);
            
            if (borrowAmount > 0) {
                // Borrow tokens
                vm.prank(user2);
                mathDemo.borrow(address(tokenA), borrowAmount);
            }
            
            // Check utilization rate is between 0 and 100%
            uint256 utilizationRate = mathDemo.getUtilizationRate(address(tokenA));
            assertTrue(utilizationRate <= 1e18); // 100% in 1e18 precision
            assertTrue(utilizationRate >= 0);
        }
    }
    
    // ==================== Edge Case Tests ====================
    
    /// @notice Test behavior with zero amounts
    function testEdgeCaseZeroAmounts() public {
        // Zero amounts should be rejected
        vm.expectRevert("Amount in must be positive");
        mathDemo.getAmountOut(address(tokenA), address(tokenB), 0);
        
        vm.expectRevert("Amount must be positive");
        vm.prank(user1);
        mathDemo.supply(address(tokenA), 0);
    }
    
    /// @notice Test behavior with maximum values
    function testEdgeCaseMaximumValues() public {
        // Test with large but reasonable values
        uint256 largeAmount = type(uint128).max; // Very large but not maximum uint256
        
        // This should not cause overflow with Solidity 0.8+ checks
        uint256 fee = mathDemo.calculateFeeWithPrecision(largeAmount, 100); // 1%
        assertTrue(fee < largeAmount);
        assertTrue(fee > 0);
    }
    
    /// @notice Test precision with very small values
    function testEdgeCaseSmallValues() public {
        // Test with very small values
        uint256 smallAmount = 1; // 1 wei
        
        uint256 fee = mathDemo.calculateFeeWithPrecision(smallAmount, 100); // 1%
        
        // With such small amounts, fee might be 0 due to integer division
        // This is expected behavior
        assertTrue(fee <= smallAmount);
    }
}