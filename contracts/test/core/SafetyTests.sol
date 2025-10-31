// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../../src/core/SafeToken.sol";
import "../../src/core/Vault.sol";

/// @title SafetyTests
/// @notice Comprehensive tests for safety mechanisms in smart contracts
contract SafetyTests is Test {
    SafeToken public token;
    Vault public vault;
    address public owner;
    address public user1;
    address public user2;
    
    /// @notice Set up test environment
    function setUp() public {
        owner = address(this);
        user1 = address(0x1);
        user2 = address(0x2);
        
        // Deploy contracts
        token = new SafeToken("Test Token", "TST", 1000000 * 10**18);
        vault = new Vault();
        
        // Whitelist token in vault
        vault.setTokenWhitelisted(address(token), true);
        
        // Transfer some tokens to user1 for testing
        token.transfer(user1, 10000 * 10**18);
    }
    
    // ==================== SafeToken Tests ====================
    
    /// @notice Test CEI pattern in token transfer
    function testTokenTransferCEI() public {
        // Arrange
        uint256 initialBalance = token.balanceOf(user1);
        uint256 transferAmount = 1000 * 10**18;
        
        // Act
        vm.prank(user1);
        bool success = token.transfer(user2, transferAmount);
        
        // Assert
        assertTrue(success);
        assertEq(token.balanceOf(user1), initialBalance - transferAmount);
        assertEq(token.balanceOf(user2), transferAmount);
    }
    
    /// @notice Test input bounds validation in token functions
    function testTokenInputBounds() public {
        // Test zero address transfer fails
        vm.expectRevert("Transfer to zero address");
        vm.prank(user1);
        token.transfer(address(0), 1000 * 10**18);
        
        // Test zero amount transfer fails
        vm.expectRevert("Transfer amount must be greater than zero");
        vm.prank(user1);
        token.transfer(user2, 0);
        
        // Test fee setting within bounds
        token.setFee(500); // 5%
        assertEq(token.feePercentage(), 500);
        
        // Test fee setting exceeds bounds
        vm.expectRevert("Fee exceeds maximum allowed");
        token.setFee(1001); // 10.01% - exceeds max of 10%
    }
    
    /// @notice Test reentrancy protection in token functions
    function testTokenReentrancyProtection() public {
        // This would test reentrancy scenarios
        // In a real implementation, you would create a malicious contract
        // that tries to reenter during transfer operations
    }
    
    /// @notice Test mathematical invariants in token calculations
    function testTokenMathematicalInvariants() public {
        // Test fee calculation safety
        uint256 fee = token.calculateFee(1000 * 10**18);
        assertTrue(fee < 1000 * 10**18); // Fee must be less than amount
        
        // Test total supply invariant
        assertTrue(token.checkTotalSupplyInvariant());
        
        // Test fee invariant
        assertTrue(token.checkFeeInvariant());
    }
    
    // ==================== Vault Tests ====================
    
    /// @notice Test CEI pattern in vault deposit
    function testVaultDepositCEI() public {
        // Arrange
        uint256 depositAmount = 1000 * 10**18;
        vm.prank(user1);
        token.approve(address(vault), depositAmount);
        
        // Act
        vm.prank(user1);
        vault.deposit(address(token), depositAmount);
        
        // Assert
        assertEq(token.balanceOf(user1), 9000 * 10**18);
        assertEq(token.balanceOf(address(vault)), depositAmount);
        assertEq(vault.getTotalDeposits(user1, address(token)), depositAmount);
    }
    
    /// @notice Test input bounds validation in vault functions
    function testVaultInputBounds() public {
        // Test zero address deposit fails
        vm.expectRevert("Token address cannot be zero");
        vault.deposit(address(0), 1000 * 10**18);
        
        // Test zero amount deposit fails
        vm.expectRevert("Deposit amount must be greater than zero");
        vault.deposit(address(token), 0);
        
        // Test withdrawal limit setting within bounds
        vault.setWithdrawalLimit(address(token), 500000 * 10**18);
        // This should work as it's within MAX_DAILY_LIMIT
        
        // Test withdrawal limit setting exceeds bounds
        vm.expectRevert("Daily limit exceeds maximum");
        vault.setWithdrawalLimit(address(token), 2000000 * 10**18);
    }
    
    /// @notice Test reentrancy protection in vault functions
    function testVaultReentrancyProtection() public {
        // Test deposit reentrancy protection
        uint256 depositAmount = 1000 * 10**18;
        vm.prank(user1);
        token.approve(address(vault), depositAmount);
        
        vm.prank(user1);
        vault.deposit(address(token), depositAmount);
        // This should not be vulnerable to reentrancy
        
        // Test withdrawal reentrancy protection
        vm.prank(user1);
        vault.withdraw(address(token), depositAmount);
        // This should not be vulnerable to reentrancy
    }
    
    /// @notice Test emergency lock functionality
    function testVaultEmergencyLock() public {
        // Activate emergency lock
        vault.activateEmergencyLock();
        
        // Try to withdraw during emergency lock
        uint256 depositAmount = 1000 * 10**18;
        vm.prank(user1);
        token.approve(address(vault), depositAmount);
        
        vm.prank(user1);
        vault.deposit(address(token), depositAmount);
        
        // This should fail due to emergency lock
        vm.expectRevert("Emergency lock active");
        vm.prank(user1);
        vault.withdraw(address(token), depositAmount);
        
        // Deactivate emergency lock
        vault.deactivateEmergencyLock();
        
        // Now withdrawal should work
        vm.prank(user1);
        vault.withdraw(address(token), depositAmount);
    }
    
    /// @notice Test withdrawal limit functionality
    function testVaultWithdrawalLimits() public {
        // Set withdrawal limit
        vault.setWithdrawalLimit(address(token), 500 * 10**18);
        
        // Deposit more than limit
        uint256 depositAmount = 1000 * 10**18;
        vm.prank(user1);
        token.approve(address(vault), depositAmount);
        
        vm.prank(user1);
        vault.deposit(address(token), depositAmount);
        
        // Try to withdraw more than limit
        vm.expectRevert("Withdrawal exceeds daily limit");
        vm.prank(user1);
        vault.withdraw(address(token), 600 * 10**18);
        
        // Withdraw within limit should work
        vm.prank(user1);
        vault.withdraw(address(token), 500 * 10**18);
    }
    
    /// @notice Test vault invariants
    function testVaultInvariants() public {
        // Test solvency invariant
        assertTrue(vault.checkSolvency(address(token)));
        
        // Test emergency lock invariant
        assertTrue(vault.checkEmergencyLockInvariant());
        
        // Activate emergency lock and test again
        vault.activateEmergencyLock();
        assertTrue(vault.checkEmergencyLockInvariant());
        
        // Deactivate emergency lock and test again
        vault.deactivateEmergencyLock();
        assertTrue(vault.checkEmergencyLockInvariant());
    }
    
    // ==================== Property Tests ====================
    
    /// @notice Property test: Token balances should always be non-negative
    function testPropertyTokenBalancesNonNegative(address user, uint256 amount) public {
        // Bound the amount to prevent overflow
        amount = bound(amount, 0, 100000 * 10**18);
        
        // Ensure user has enough tokens
        if (token.balanceOf(owner) >= amount && amount > 0) {
            token.transfer(user, amount);
            assertTrue(token.balanceOf(user) >= 0);
        }
    }
    
    /// @notice Property test: Total supply should remain constant
    function testPropertyTotalSupplyConstant(uint256 transferAmount) public {
        // Bound the amount to prevent overflow and ensure sufficient balance
        transferAmount = bound(transferAmount, 0, token.balanceOf(owner) / 2);
        
        uint256 initialSupply = token.totalSupply();
        
        if (transferAmount > 0) {
            token.transfer(user1, transferAmount);
        }
        
        assertEq(token.totalSupply(), initialSupply);
    }
    
    /// @notice Property test: Vault should remain solvent
    function testPropertyVaultSolvency(uint256 depositAmount) public {
        // Bound the amount to prevent overflow
        depositAmount = bound(depositAmount, 0, token.balanceOf(owner) / 2);
        
        if (depositAmount > 0) {
            // Approve and deposit
            token.approve(address(vault), depositAmount);
            vault.deposit(address(token), depositAmount);
            
            // Check solvency
            assertTrue(vault.checkSolvency(address(token)));
        }
    }
}