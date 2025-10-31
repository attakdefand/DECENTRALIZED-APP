// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../../src/core/LendingPool.sol";
import "../../src/core/SafeToken.sol";

/**
 * @title LendingPoolTest
 * @notice Test suite for the LendingPool contract
 */
contract LendingPoolTest is Test {
    LendingPool public lendingPool;
    SafeToken public tokenA;
    SafeToken public tokenB;
    address public user1 = address(0x1001);
    address public user2 = address(0x1002);
    address public liquidator = address(0x1003);

    event Supply(address indexed user, address indexed token, uint256 amount);
    event Withdraw(address indexed user, address indexed token, uint256 amount);
    event Borrow(address indexed user, address indexed token, uint256 amount);
    event Repay(address indexed user, address indexed token, uint256 amount);
    event Liquidate(
        address indexed liquidator,
        address indexed user,
        address indexed collateralToken,
        address debtToken,
        uint256 collateralAmount,
        uint256 debtAmount
    );
    event InterestAccrued(address indexed token, uint256 interest);
    event RiskParamsUpdated(address indexed token, LendingPool.RiskParams riskParams);

    function setUp() public {
        // Deploy contracts
        lendingPool = new LendingPool();
        tokenA = new SafeToken("TokenA", "TKA", 18);
        tokenB = new SafeToken("TokenB", "TKB", 18);

        // Mint tokens to users
        tokenA.mint(user1, 1000000 * 1e18);
        tokenA.mint(user2, 1000000 * 1e18);
        tokenA.mint(liquidator, 1000000 * 1e18);
        tokenB.mint(user1, 1000000 * 1e18);
        tokenB.mint(user2, 1000000 * 1e18);
        tokenB.mint(liquidator, 1000000 * 1e18);

        // Approve lending pool to spend tokens
        vm.prank(user1);
        tokenA.approve(address(lendingPool), type(uint256).max);
        vm.prank(user1);
        tokenB.approve(address(lendingPool), type(uint256).max);

        vm.prank(user2);
        tokenA.approve(address(lendingPool), type(uint256).max);
        vm.prank(user2);
        tokenB.approve(address(lendingPool), type(uint256).max);

        vm.prank(liquidator);
        tokenA.approve(address(lendingPool), type(uint256).max);
        vm.prank(liquidator);
        tokenB.approve(address(lendingPool), type(uint256).max);

        // Set risk parameters
        LendingPool.RiskParams memory riskParams = LendingPool.RiskParams({
            collateralFactor: 0.8 * 1e18, // 80%
            liquidationThreshold: 0.85 * 1e18, // 85%
            liquidationPenalty: 0.1 * 1e18, // 10%
            minHealthFactor: 1 * 1e18 // 1.0
        });

        vm.prank(address(lendingPool));
        lendingPool.setRiskParams(address(tokenA), riskParams);
        lendingPool.setRiskParams(address(tokenB), riskParams);

        // Set interest rate model
        vm.prank(address(lendingPool));
        lendingPool.setInterestRateModel(
            address(tokenA),
            0.02 * 1e18, // 2% base rate
            0.1 * 1e18, // 10% slope1
            0.5 * 1e18, // 50% slope2
            0.8 * 1e18, // 80% kink
            0.1 * 1e18 // 10% reserve factor
        );

        vm.prank(address(lendingPool));
        lendingPool.setInterestRateModel(
            address(tokenB),
            0.03 * 1e18, // 3% base rate
            0.15 * 1e18, // 15% slope1
            0.6 * 1e18, // 60% slope2
            0.75 * 1e18, // 75% kink
            0.15 * 1e18 // 15% reserve factor
        );

        // Set tokens as collateral
        vm.prank(address(lendingPool));
        lendingPool.setCollateralToken(address(tokenA), true);
        lendingPool.setCollateralToken(address(tokenB), true);
    }

    /**
     * @notice Test supply functionality
     */
    function testSupply() public {
        uint256 amount = 1000 * 1e18;

        vm.prank(user1);
        lendingPool.supply(address(tokenA), amount);

        // Check user position
        LendingPool.UserPosition memory position = lendingPool.getUserPosition(address(tokenA), user1);
        assertEq(position.supplied, amount);

        // Check reserve data
        LendingPool.ReserveData memory reserve = lendingPool.getReserveData(address(tokenA));
        assertEq(reserve.totalSupplied, amount);
    }

    /**
     * @notice Test withdraw functionality
     */
    function testWithdraw() public {
        uint256 supplyAmount = 1000 * 1e18;
        uint256 withdrawAmount = 500 * 1e18;

        // Supply tokens first
        vm.prank(user1);
        lendingPool.supply(address(tokenA), supplyAmount);

        // Withdraw tokens
        vm.prank(user1);
        lendingPool.withdraw(address(tokenA), withdrawAmount);

        // Check user position
        LendingPool.UserPosition memory position = lendingPool.getUserPosition(address(tokenA), user1);
        assertEq(position.supplied, supplyAmount - withdrawAmount);

        // Check reserve data
        LendingPool.ReserveData memory reserve = lendingPool.getReserveData(address(tokenA));
        assertEq(reserve.totalSupplied, supplyAmount - withdrawAmount);
    }

    /**
     * @notice Test borrow functionality
     */
    function testBorrow() public {
        uint256 supplyAmount = 10000 * 1e18;
        uint256 borrowAmount = 1000 * 1e18;

        // Supply tokens first to create liquidity
        vm.prank(user1);
        lendingPool.supply(address(tokenA), supplyAmount);

        // Borrow tokens
        vm.prank(user1);
        lendingPool.borrow(address(tokenA), borrowAmount);

        // Check user position
        LendingPool.UserPosition memory position = lendingPool.getUserPosition(address(tokenA), user1);
        assertEq(position.borrowed, borrowAmount);

        // Check reserve data
        LendingPool.ReserveData memory reserve = lendingPool.getReserveData(address(tokenA));
        assertEq(reserve.totalBorrowed, borrowAmount);
    }

    /**
     * @notice Test repay functionality
     */
    function testRepay() public {
        uint256 supplyAmount = 10000 * 1e18;
        uint256 borrowAmount = 1000 * 1e18;
        uint256 repayAmount = 500 * 1e18;

        // Supply tokens first to create liquidity
        vm.prank(user1);
        lendingPool.supply(address(tokenA), supplyAmount);

        // Borrow tokens
        vm.prank(user1);
        lendingPool.borrow(address(tokenA), borrowAmount);

        // Repay tokens
        vm.prank(user1);
        lendingPool.repay(address(tokenA), repayAmount);

        // Check user position
        LendingPool.UserPosition memory position = lendingPool.getUserPosition(address(tokenA), user1);
        assertEq(position.borrowed, borrowAmount - repayAmount);

        // Check reserve data
        LendingPool.ReserveData memory reserve = lendingPool.getReserveData(address(tokenA));
        assertEq(reserve.totalBorrowed, borrowAmount - repayAmount);
    }

    /**
     * @notice Test add collateral functionality
     */
    function testAddCollateral() public {
        uint256 collateralAmount = 1000 * 1e18;

        vm.prank(user1);
        lendingPool.addCollateral(address(tokenA), collateralAmount);

        // Check user position
        LendingPool.UserPosition memory position = lendingPool.getUserPosition(address(tokenA), user1);
        assertEq(position.collateral, collateralAmount);
    }

    /**
     * @notice Test remove collateral functionality
     */
    function testRemoveCollateral() public {
        uint256 addAmount = 1000 * 1e18;
        uint256 removeAmount = 500 * 1e18;

        // Add collateral first
        vm.prank(user1);
        lendingPool.addCollateral(address(tokenA), addAmount);

        // Remove collateral
        vm.prank(user1);
        lendingPool.removeCollateral(address(tokenA), removeAmount);

        // Check user position
        LendingPool.UserPosition memory position = lendingPool.getUserPosition(address(tokenA), user1);
        assertEq(position.collateral, addAmount - removeAmount);
    }

    /**
     * @notice Test interest accrual
     */
    function testInterestAccrual() public {
        uint256 supplyAmount = 10000 * 1e18;
        uint256 borrowAmount = 5000 * 1e18;

        // Supply tokens
        vm.prank(user1);
        lendingPool.supply(address(tokenA), supplyAmount);

        // Borrow tokens
        vm.prank(user1);
        lendingPool.borrow(address(tokenA), borrowAmount);

        // Advance time to accrue interest
        vm.warp(block.timestamp + 30 days);

        // Trigger interest accrual by doing another operation
        vm.prank(user1);
        lendingPool.supply(address(tokenA), 100 * 1e18);

        // Check that interest was accrued
        LendingPool.ReserveData memory reserve = lendingPool.getReserveData(address(tokenA));
        assertTrue(reserve.totalBorrowed > borrowAmount);
        assertTrue(reserve.totalReserves > 0);
    }

    /**
     * @notice Test risk parameters setting
     */
    function testSetRiskParams() public {
        LendingPool.RiskParams memory newRiskParams = LendingPool.RiskParams({
            collateralFactor: 0.7 * 1e18, // 70%
            liquidationThreshold: 0.75 * 1e18, // 75%
            liquidationPenalty: 0.15 * 1e18, // 15%
            minHealthFactor: 1.1 * 1e18 // 1.1
        });

        vm.prank(address(lendingPool));
        lendingPool.setRiskParams(address(tokenA), newRiskParams);

        // Check that risk parameters were set
        // This would require a getter function in the actual implementation
    }

    /**
     * @notice Test interest rate model setting
     */
    function testSetInterestRateModel() public {
        vm.prank(address(lendingPool));
        lendingPool.setInterestRateModel(
            address(tokenA),
            0.05 * 1e18, // 5% base rate
            0.2 * 1e18, // 20% slope1
            0.7 * 1e18, // 70% slope2
            0.7 * 1e18, // 70% kink
            0.2 * 1e18 // 20% reserve factor
        );

        // Check that interest rate model was set
        // This would require a getter function in the actual implementation
    }

    /**
     * @notice Fuzz test for supply
     */
    function testFuzzSupply(uint256 amount) public {
        // Constrain amount to reasonable values
        vm.assume(amount > 0 && amount < 100000 * 1e18);

        vm.prank(user1);
        lendingPool.supply(address(tokenA), amount);

        // Check user position
        LendingPool.UserPosition memory position = lendingPool.getUserPosition(address(tokenA), user1);
        assertEq(position.supplied, amount);
    }

    /**
     * @notice Fuzz test for borrow
     */
    function testFuzzBorrow(uint256 supplyAmount, uint256 borrowAmount) public {
        // Constrain amounts to reasonable values
        vm.assume(supplyAmount > borrowAmount && borrowAmount > 0 && supplyAmount < 100000 * 1e18);

        // Supply tokens first to create liquidity
        vm.prank(user1);
        lendingPool.supply(address(tokenA), supplyAmount);

        // Borrow tokens
        vm.prank(user1);
        lendingPool.borrow(address(tokenA), borrowAmount);

        // Check user position
        LendingPool.UserPosition memory position = lendingPool.getUserPosition(address(tokenA), user1);
        assertEq(position.borrowed, borrowAmount);
    }

    /**
     * @notice Invariant test for supply and withdraw
     */
    function invariantSupplyWithdraw() public {
        // This would test that total supplied always equals sum of all user supplies
        // Plus any accrued interest
        assertTrue(true); // Placeholder
    }

    /**
     * @notice Invariant test for borrow and repay
     */
    function invariantBorrowRepay() public {
        // This would test that total borrowed always equals sum of all user debts
        // Plus any accrued interest
        assertTrue(true); // Placeholder
    }
}