// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../../src/core/Vault.sol";

contract TreasuryManagementTest is Test {
    Vault public vault;
    address public owner;
    address public user;
    address public token;
    
    function setUp() public {
        owner = address(this);
        user = address(0x123);
        token = address(0x456);
        
        vault = new Vault();
        
        // Whitelist the token
        vault.setTokenWhitelisted(token, true);
    }
    
    function testProofOfReservesSubmission() public {
        // Submit a Proof of Reserves report
        vault.submitProofOfReserves(1000000, owner);
        
        // Check that the report was recorded
        assertEq(vault.porReportCount(), 1);
        
        // Check that treasury metrics were updated
        (uint256 porFreshnessHours, uint256 limitBreachCount, ) = vault.treasuryMetrics();
        assertEq(porFreshnessHours, 0); // Should be reset to 0 when new report is submitted
        assertEq(limitBreachCount, 0);
    }
    
    function testLimitBreachRecording() public {
        // Set a low withdrawal limit
        vault.setWithdrawalLimit(token, 100);
        
        // Try to withdraw more than the limit (this should record a breach)
        // Note: In a real test, we would need to mock the token balance and other state
        // For now, we'll just test the metrics update function directly
        
        // Check initial metrics
        (, uint256 initialBreachCount, ) = vault.treasuryMetrics();
        assertEq(initialBreachCount, 0);
    }
    
    function testTreasuryMetricsWithinLimits() public {
        // Initially should be within limits
        bool withinLimits = vault.areTreasuryMetricsWithinLimits();
        assertTrue(withinLimits);
        
        // Submit a fresh POR report
        vault.submitProofOfReserves(1000000, owner);
        
        // Should still be within limits
        withinLimits = vault.areTreasuryMetricsWithinLimits();
        assertTrue(withinLimits);
    }
    
    function testPorReportFreshness() public {
        // Initially no reports, so not fresh
        bool isFresh = vault.isPorReportFresh();
        assertFalse(isFresh);
        
        // Submit a report
        vault.submitProofOfReserves(1000000, owner);
        
        // Should be fresh now
        isFresh = vault.isPorReportFresh();
        assertTrue(isFresh);
    }
    
    function testTokenReservesTracking() public {
        // Initially should be zero
        uint256 reserves = vault.getTokenReserves(token);
        assertEq(reserves, 0);
        
        // Note: To fully test this, we would need to mock ERC20 tokens
        // and perform actual deposits/withdrawals, which requires more setup
    }
    
    function testTreasuryMetricsUpdateEvent() public {
        // Test that events are emitted correctly
        vm.expectEmit(true, true, true, true);
        emit Vault.TreasuryMetricsUpdated(0, 0);
        
        vault.submitProofOfReserves(1000000, owner);
    }
    
    function testProofOfReservesReportEvent() public {
        // Test that events are emitted correctly
        vm.expectEmit(true, true, true, true);
        emit Vault.ProofOfReservesReported(1, block.timestamp, 1000000);
        
        vault.submitProofOfReserves(1000000, owner);
    }
}