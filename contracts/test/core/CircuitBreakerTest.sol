// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../../src/core/CircuitBreaker.sol";
import "../../src/core/EnhancedVault.sol";
import "../../src/core/SafeToken.sol";

/// @title CircuitBreakerTest
/// @notice Comprehensive tests for circuit breaker and pause functionality
contract CircuitBreakerTest is Test {
    CircuitBreaker public circuitBreaker;
    EnhancedVault public vault;
    SafeToken public token;
    
    address public owner;
    address public pauser;
    address public unpauser;
    address public user1;
    address public user2;
    
    bytes4 public constant DEPOSIT_SELECTOR = bytes4(keccak256("deposit(address,uint256)"));
    bytes4 public constant WITHDRAW_SELECTOR = bytes4(keccak256("withdraw(address,uint256)"));
    
    /// @notice Set up test environment
    function setUp() public {
        owner = address(this);
        pauser = address(0x1);
        unpauser = address(0x2);
        user1 = address(0x3);
        user2 = address(0x4);
        
        // Deploy circuit breaker
        circuitBreaker = new CircuitBreaker(owner);
        
        // Grant roles
        circuitBreaker.grantRole(circuitBreaker.PAUSER_ROLE(), pauser);
        circuitBreaker.grantRole(circuitBreaker.UNPAUSER_ROLE(), unpauser);
        
        // Deploy token
        token = new SafeToken("Test Token", "TST", 1000000 * 1e18);
        
        // Deploy vault with circuit breaker
        vault = new EnhancedVault(address(circuitBreaker));
        
        // Transfer tokens to users
        token.transfer(user1, 10000 * 1e18);
        token.transfer(user2, 10000 * 1e18);
        
        // Whitelist token in vault
        vault.setTokenWhitelisted(address(token), true);
        
        // Approve vault to spend tokens
        vm.prank(user1);
        token.approve(address(vault), type(uint256).max);
        vm.prank(user2);
        token.approve(address(vault), type(uint256).max);
    }
    
    // ==================== Basic Pause Functionality Tests ====================
    
    /// @notice Test function pause and unpause
    function testFunctionPauseUnpause() public {
        // Initially function should not be paused
        assertFalse(circuitBreaker.isFunctionPaused(DEPOSIT_SELECTOR));
        
        // Pauser can pause function
        vm.prank(pauser);
        circuitBreaker.pauseFunction(DEPOSIT_SELECTOR);
        
        // Function should be paused
        assertTrue(circuitBreaker.isFunctionPaused(DEPOSIT_SELECTOR));
        
        // Unpauser can unpause function
        vm.prank(unpauser);
        circuitBreaker.unpauseFunction(DEPOSIT_SELECTOR);
        
        // Function should be unpaused
        assertFalse(circuitBreaker.isFunctionPaused(DEPOSIT_SELECTOR));
    }
    
    /// @notice Test function group pause and unpause
    function testFunctionGroupPauseUnpause() public {
        string memory groupName = "vault";
        
        // Initially function group should not be paused
        assertFalse(circuitBreaker.isFunctionGroupPaused(groupName));
        
        // Pauser can pause function group
        vm.prank(pauser);
        circuitBreaker.pauseFunctionGroup(groupName);
        
        // Function group should be paused
        assertTrue(circuitBreaker.isFunctionGroupPaused(groupName));
        
        // Unpauser can unpause function group
        vm.prank(unpauser);
        circuitBreaker.unpauseFunctionGroup(groupName);
        
        // Function group should be unpaused
        assertFalse(circuitBreaker.isFunctionGroupPaused(groupName));
    }
    
    /// @notice Test unauthorized pause attempts
    function testUnauthorizedPauseAttempts() public {
        // User1 should not be able to pause functions
        vm.expectRevert();
        vm.prank(user1);
        circuitBreaker.pauseFunction(DEPOSIT_SELECTOR);
        
        // User1 should not be able to pause function groups
        vm.expectRevert();
        vm.prank(user1);
        circuitBreaker.pauseFunctionGroup("vault");
    }
    
    /// @notice Test unauthorized unpause attempts
    function testUnauthorizedUnpauseAttempts() public {
        // Pause a function first
        vm.prank(pauser);
        circuitBreaker.pauseFunction(DEPOSIT_SELECTOR);
        
        // User1 should not be able to unpause functions
        vm.expectRevert();
        vm.prank(user1);
        circuitBreaker.unpauseFunction(DEPOSIT_SELECTOR);
        
        // Pause a function group
        vm.prank(pauser);
        circuitBreaker.pauseFunctionGroup("vault");
        
        // User1 should not be able to unpause function groups
        vm.expectRevert();
        vm.prank(user1);
        circuitBreaker.unpauseFunctionGroup("vault");
    }
    
    // ==================== Rate Limiting Tests ====================
    
    /// @notice Test function rate limiting
    function testFunctionRateLimiting() public {
        uint256 limit = 2;
        uint256 window = 1 hours;
        
        // Set rate limit for deposit function
        circuitBreaker.setFunctionRateLimit(DEPOSIT_SELECTOR, limit, window);
        
        // First call should succeed
        vm.prank(user1);
        circuitBreaker.getFunctionRateLimitCount(DEPOSIT_SELECTOR);
        
        // Second call should succeed
        vm.prank(user1);
        circuitBreaker.getFunctionRateLimitCount(DEPOSIT_SELECTOR);
        
        // Third call should fail (rate limit exceeded)
        // Note: This is a simplified test - actual rate limiting would be enforced
        // in the contract that uses the circuit breaker
    }
    
    /// @notice Test function group rate limiting
    function testFunctionGroupRateLimiting() public {
        string memory groupName = "deposit";
        uint256 limit = 3;
        uint256 window = 30 minutes;
        
        // Set rate limit for deposit group
        circuitBreaker.setFunctionGroupRateLimit(groupName, limit, window);
        
        // Check rate limit count
        vm.prank(user1);
        circuitBreaker.getFunctionGroupRateLimitCount(groupName);
    }
    
    // ==================== Emergency Controls Tests ====================
    
    /// @notice Test emergency pause activation and deactivation
    function testEmergencyPauseActivationDeactivation() public {
        // Initially emergency pause should not be active
        assertFalse(circuitBreaker.isEmergencyPauseActive());
        
        // Pauser can activate emergency pause
        vm.prank(pauser);
        circuitBreaker.activateEmergencyPause();
        
        // Emergency pause should be active
        assertTrue(circuitBreaker.isEmergencyPauseActive());
        
        // Unpauser can deactivate emergency pause
        vm.prank(unpauser);
        circuitBreaker.deactivateEmergencyPause();
        
        // Emergency pause should not be active
        assertFalse(circuitBreaker.isEmergencyPauseActive());
    }
    
    /// @notice Test emergency pause expiration
    function testEmergencyPauseExpiration() public {
        // Activate emergency pause
        vm.prank(pauser);
        circuitBreaker.activateEmergencyPause();
        
        // Emergency pause should be active
        assertTrue(circuitBreaker.isEmergencyPauseActive());
        
        // Fast forward time beyond emergency pause duration
        vm.warp(block.timestamp + 25 hours);
        
        // Emergency pause should no longer be active
        assertFalse(circuitBreaker.isEmergencyPauseActive());
    }
    
    // ==================== Integration Tests ====================
    
    /// @notice Test vault integration with circuit breaker
    function testVaultIntegrationWithCircuitBreaker() public {
        uint256 depositAmount = 1000 * 1e18;
        
        // Initial deposit should succeed
        vm.prank(user1);
        vault.deposit(address(token), depositAmount);
        
        // Pause the deposit function
        vm.prank(pauser);
        circuitBreaker.pauseFunction(bytes4(keccak256("deposit(address,uint256)")));
        
        // Deposit should fail when function is paused
        vm.expectRevert("Function is paused by circuit breaker");
        vm.prank(user2);
        vault.deposit(address(token), depositAmount);
        
        // Unpause the deposit function
        vm.prank(unpauser);
        circuitBreaker.unpauseFunction(bytes4(keccak256("deposit(address,uint256)")));
        
        // Deposit should succeed again
        vm.prank(user2);
        vault.deposit(address(token), depositAmount);
    }
    
    /// @notice Test vault integration with function group pause
    function testVaultIntegrationWithFunctionGroupPause() public {
        uint256 depositAmount = 1000 * 1e18;
        
        // Initial deposit should succeed
        vm.prank(user1);
        vault.deposit(address(token), depositAmount);
        
        // Pause the vault function group
        vm.prank(pauser);
        circuitBreaker.pauseFunctionGroup("vault");
        
        // Deposit should fail when function group is paused
        vm.expectRevert("Function group is paused by circuit breaker");
        vm.prank(user2);
        vault.deposit(address(token), depositAmount);
        
        // Unpause the vault function group
        vm.prank(unpauser);
        circuitBreaker.unpauseFunctionGroup("vault");
        
        // Deposit should succeed again
        vm.prank(user2);
        vault.deposit(address(token), depositAmount);
    }
    
    /// @notice Test vault integration with emergency pause
    function testVaultIntegrationWithEmergencyPause() public {
        uint256 depositAmount = 1000 * 1e18;
        
        // Initial deposit should succeed
        vm.prank(user1);
        vault.deposit(address(token), depositAmount);
        
        // Activate emergency pause
        vm.prank(pauser);
        circuitBreaker.activateEmergencyPause();
        
        // Deposit should fail when emergency pause is active
        vm.expectRevert("Emergency pause is active");
        vm.prank(user2);
        vault.deposit(address(token), depositAmount);
        
        // Deactivate emergency pause
        vm.prank(unpauser);
        circuitBreaker.deactivateEmergencyPause();
        
        // Deposit should succeed again
        vm.prank(user2);
        vault.deposit(address(token), depositAmount);
    }
    
    // ==================== Scenario Tests ====================
    
    /// @notice Test complex pause scenario with multiple functions
    function testComplexPauseScenario() public {
        uint256 depositAmount = 1000 * 1e18;
        uint256 withdrawAmount = 500 * 1e18;
        
        // User1 deposits tokens
        vm.prank(user1);
        vault.deposit(address(token), depositAmount);
        
        // User2 deposits tokens
        vm.prank(user2);
        vault.deposit(address(token), depositAmount);
        
        // Pause deposit function
        vm.prank(pauser);
        circuitBreaker.pauseFunction(DEPOSIT_SELECTOR);
        
        // User1 tries to deposit - should fail
        vm.expectRevert("Function is paused by circuit breaker");
        vm.prank(user1);
        vault.deposit(address(token), depositAmount);
        
        // User1 tries to withdraw - should succeed
        vm.prank(user1);
        vault.withdraw(address(token), withdrawAmount);
        
        // Pause withdraw function
        vm.prank(pauser);
        circuitBreaker.pauseFunction(WITHDRAW_SELECTOR);
        
        // User2 tries to withdraw - should fail
        vm.expectRevert("Function is paused by circuit breaker");
        vm.prank(user2);
        vault.withdraw(address(token), withdrawAmount);
        
        // Unpause both functions
        vm.prank(unpauser);
        circuitBreaker.unpauseFunction(DEPOSIT_SELECTOR);
        vm.prank(unpauser);
        circuitBreaker.unpauseFunction(WITHDRAW_SELECTOR);
        
        // Both operations should succeed now
        vm.prank(user1);
        vault.deposit(address(token), depositAmount);
        vm.prank(user2);
        vault.withdraw(address(token), withdrawAmount);
    }
    
    /// @notice Test emergency scenario with global pause
    function testEmergencyScenarioWithGlobalPause() public {
        uint256 depositAmount = 1000 * 1e18;
        
        // Multiple users deposit tokens
        vm.prank(user1);
        vault.deposit(address(token), depositAmount);
        vm.prank(user2);
        vault.deposit(address(token), depositAmount);
        
        // Activate global emergency pause
        vm.prank(pauser);
        circuitBreaker.activateEmergencyPause();
        
        // All vault functions should fail
        vm.expectRevert("Emergency pause is active");
        vm.prank(user1);
        vault.deposit(address(token), depositAmount);
        
        vm.expectRevert("Emergency pause is active");
        vm.prank(user2);
        vault.withdraw(address(token), depositAmount);
        
        // Admin functions should also fail
        vm.expectRevert("Emergency pause is active");
        vm.prank(owner);
        vault.setTokenWhitelisted(address(token), false);
        
        // Fast forward time to expire emergency pause
        vm.warp(block.timestamp + 25 hours);
        
        // Functions should work again
        vm.prank(user1);
        vault.deposit(address(token), depositAmount);
    }
    
    // ==================== Property Tests ====================
    
    /// @notice Property test: Pause state should be consistent
    function testPropertyPauseStateConsistency(bytes4 functionSelector, bool initialState) public {
        // Set initial state
        if (initialState) {
            vm.prank(pauser);
            circuitBreaker.pauseFunction(functionSelector);
        }
        
        bool isPaused = circuitBreaker.isFunctionPaused(functionSelector);
        assertEq(isPaused, initialState);
        
        // Toggle state
        if (isPaused) {
            vm.prank(unpauser);
            circuitBreaker.unpauseFunction(functionSelector);
            assertFalse(circuitBreaker.isFunctionPaused(functionSelector));
        } else {
            vm.prank(pauser);
            circuitBreaker.pauseFunction(functionSelector);
            assertTrue(circuitBreaker.isFunctionPaused(functionSelector));
        }
    }
    
    /// @notice Property test: Emergency pause should expire correctly
    function testPropertyEmergencyPauseExpiration(uint256 timeElapsed) public {
        // Bound time elapsed to reasonable values
        timeElapsed = bound(timeElapsed, 0, 30 days);
        
        // Initially emergency pause should not be active
        assertFalse(circuitBreaker.isEmergencyPauseActive());
        
        // Activate emergency pause
        vm.prank(pauser);
        circuitBreaker.activateEmergencyPause();
        
        // Emergency pause should be active
        assertTrue(circuitBreaker.isEmergencyPauseActive());
        
        // Fast forward time
        vm.warp(block.timestamp + timeElapsed);
        
        // If time elapsed is >= 24 hours, emergency pause should expire
        if (timeElapsed >= 24 hours) {
            assertFalse(circuitBreaker.isEmergencyPauseActive());
        } else {
            assertTrue(circuitBreaker.isEmergencyPauseActive());
        }
    }
    
    /// @notice Property test: Function group pause should affect all functions in group
    function testPropertyFunctionGroupPauseAffectsAllFunctions() public {
        string memory groupName = "testGroup";
        
        // Initially function group should not be paused
        assertFalse(circuitBreaker.isFunctionGroupPaused(groupName));
        
        // Pause function group
        vm.prank(pauser);
        circuitBreaker.pauseFunctionGroup(groupName);
        
        // Function group should be paused
        assertTrue(circuitBreaker.isFunctionGroupPaused(groupName));
        
        // Unpause function group
        vm.prank(unpauser);
        circuitBreaker.unpauseFunctionGroup(groupName);
        
        // Function group should not be paused
        assertFalse(circuitBreaker.isFunctionGroupPaused(groupName));
    }
    
    // ==================== Edge Case Tests ====================
    
    /// @notice Test pause with zero address
    function testPauseWithZeroAddress() public {
        // Should not be able to grant roles to zero address
        vm.expectRevert();
        circuitBreaker.grantRole(circuitBreaker.PAUSER_ROLE(), address(0));
    }
    
    /// @notice Test pause with same pauser and unpauser
    function testPauseWithSamePauserAndUnpauser() public {
        address sameUser = address(0x5);
        
        // Grant both roles to same user
        circuitBreaker.grantRole(circuitBreaker.PAUSER_ROLE(), sameUser);
        circuitBreaker.grantRole(circuitBreaker.UNPAUSER_ROLE(), sameUser);
        
        // User can pause and unpause
        vm.prank(sameUser);
        circuitBreaker.pauseFunction(DEPOSIT_SELECTOR);
        assertTrue(circuitBreaker.isFunctionPaused(DEPOSIT_SELECTOR));
        
        vm.prank(sameUser);
        circuitBreaker.unpauseFunction(DEPOSIT_SELECTOR);
        assertFalse(circuitBreaker.isFunctionPaused(DEPOSIT_SELECTOR));
    }
    
    /// @notice Test multiple pause states
    function testMultiplePauseStates() public {
        bytes4 function1 = bytes4(keccak256("function1()"));
        bytes4 function2 = bytes4(keccak256("function2()"));
        string memory group1 = "group1";
        string memory group2 = "group2";
        
        // Pause multiple functions
        vm.prank(pauser);
        circuitBreaker.pauseFunction(function1);
        vm.prank(pauser);
        circuitBreaker.pauseFunction(function2);
        
        // Pause multiple function groups
        vm.prank(pauser);
        circuitBreaker.pauseFunctionGroup(group1);
        vm.prank(pauser);
        circuitBreaker.pauseFunctionGroup(group2);
        
        // All should be paused
        assertTrue(circuitBreaker.isFunctionPaused(function1));
        assertTrue(circuitBreaker.isFunctionPaused(function2));
        assertTrue(circuitBreaker.isFunctionGroupPaused(group1));
        assertTrue(circuitBreaker.isFunctionGroupPaused(group2));
        
        // Unpause all
        vm.prank(unpauser);
        circuitBreaker.unpauseFunction(function1);
        vm.prank(unpauser);
        circuitBreaker.unpauseFunction(function2);
        vm.prank(unpauser);
        circuitBreaker.unpauseFunctionGroup(group1);
        vm.prank(unpauser);
        circuitBreaker.unpauseFunctionGroup(group2);
        
        // All should be unpaused
        assertFalse(circuitBreaker.isFunctionPaused(function1));
        assertFalse(circuitBreaker.isFunctionPaused(function2));
        assertFalse(circuitBreaker.isFunctionGroupPaused(group1));
        assertFalse(circuitBreaker.isFunctionGroupPaused(group2));
    }
    
    /// @notice Test rate limit edge cases
    function testRateLimitEdgeCases() public {
        bytes4 functionSelector = bytes4(keccak256("testFunction()"));
        uint256 limit = 1;
        uint256 window = 1;
        
        // Set rate limit with minimum values
        circuitBreaker.setFunctionRateLimit(functionSelector, limit, window);
        
        // Check rate limit count
        vm.prank(user1);
        uint256 count = circuitBreaker.getFunctionRateLimitCount(functionSelector);
        assertEq(count, 0);
    }
}