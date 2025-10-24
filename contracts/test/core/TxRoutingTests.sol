// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../../src/core/ReplayProtection.sol";

contract TxRoutingTests is Test {
    ReplayProtection public replayProtection;
    
    address public owner = address(1);
    address public user1 = address(2);
    address public user2 = address(3);
    address public target = address(4);
    
    function setUp() public {
        replayProtection = new ReplayProtection();
    }
    
    function testReplayProtection() public {
        vm.startPrank(user1);
        
        // Create transaction data
        ReplayProtection.Transaction memory txData = ReplayProtection.Transaction({
            target: target,
            value: 1 ether,
            data: abi.encodeWithSignature("transfer(address,uint256)", user2, 0.5 ether),
            chainId: block.chainid,
            nonce: replayProtection.getNonce(user1),
            deadline: block.timestamp + 3600, // 1 hour from now
            signature: new bytes(65) // Dummy signature
        });
        
        // Execute transaction
        replayProtection.executeTransaction(txData);
        
        // Verify nonce was incremented
        assertEq(replayProtection.getNonce(user1), 1);
        
        // Try to replay the same transaction - should fail
        vm.expectRevert("Transaction already executed");
        replayProtection.executeTransaction(txData);
        
        vm.stopPrank();
    }
    
    function testChainIdValidation() public {
        vm.startPrank(user1);
        
        // Create transaction with wrong chain ID
        ReplayProtection.Transaction memory txData = ReplayProtection.Transaction({
            target: target,
            value: 1 ether,
            data: abi.encodeWithSignature("transfer(address,uint256)", user2, 0.5 ether),
            chainId: block.chainid + 1, // Wrong chain ID
            nonce: replayProtection.getNonce(user1),
            deadline: block.timestamp + 3600,
            signature: new bytes(65)
        });
        
        // Try to execute transaction with wrong chain ID - should fail
        vm.expectRevert("Invalid chain ID");
        replayProtection.executeTransaction(txData);
        
        vm.stopPrank();
    }
    
    function testDeadlineValidation() public {
        vm.startPrank(user1);
        
        // Create expired transaction
        ReplayProtection.Transaction memory txData = ReplayProtection.Transaction({
            target: target,
            value: 1 ether,
            data: abi.encodeWithSignature("transfer(address,uint256)", user2, 0.5 ether),
            chainId: block.chainid,
            nonce: replayProtection.getNonce(user1),
            deadline: block.timestamp - 3600, // Expired 1 hour ago
            signature: new bytes(65)
        });
        
        // Try to execute expired transaction - should fail
        vm.expectRevert("Transaction expired");
        replayProtection.executeTransaction(txData);
        
        vm.stopPrank();
    }
    
    function testNonceValidation() public {
        vm.startPrank(user1);
        
        // Create transaction with wrong nonce
        ReplayProtection.Transaction memory txData = ReplayProtection.Transaction({
            target: target,
            value: 1 ether,
            data: abi.encodeWithSignature("transfer(address,uint256)", user2, 0.5 ether),
            chainId: block.chainid,
            nonce: replayProtection.getNonce(user1) + 1, // Wrong nonce
            deadline: block.timestamp + 3600,
            signature: new bytes(65)
        });
        
        // Try to execute transaction with wrong nonce - should fail
        vm.expectRevert("Invalid nonce");
        replayProtection.executeTransaction(txData);
        
        vm.stopPrank();
    }
    
    function testBatchTransactions() public {
        vm.startPrank(user1);
        
        // Create multiple transactions
        ReplayProtection.Transaction[] memory txData = new ReplayProtection.Transaction[](2);
        
        txData[0] = ReplayProtection.Transaction({
            target: target,
            value: 0.5 ether,
            data: abi.encodeWithSignature("transfer(address,uint256)", user2, 0.25 ether),
            chainId: block.chainid,
            nonce: replayProtection.getNonce(user1),
            deadline: block.timestamp + 3600,
            signature: new bytes(65)
        });
        
        txData[1] = ReplayProtection.Transaction({
            target: target,
            value: 0.5 ether,
            data: abi.encodeWithSignature("transfer(address,uint256)", user2, 0.25 ether),
            chainId: block.chainid,
            nonce: replayProtection.getNonce(user1) + 1,
            deadline: block.timestamp + 3600,
            signature: new bytes(65)
        });
        
        // Execute batch transactions
        replayProtection.executeBatchTransactions(txData);
        
        // Verify nonces were incremented
        assertEq(replayProtection.getNonce(user1), 2);
        
        vm.stopPrank();
    }
    
    function testPermitUsage() public {
        vm.startPrank(user1);
        
        // Use a permit
        replayProtection.usePermit(
            user1, // owner
            user2, // spender
            1000, // value
            block.timestamp + 3600, // deadline
            1, // nonce
            new bytes(65) // signature
        );
        
        // Verify permit was marked as used
        assertTrue(replayProtection.isPermitNonceUsed(user1, 1));
        
        // Try to use the same permit again - should fail
        vm.expectRevert("Permit already used");
        replayProtection.usePermit(
            user1, // owner
            user2, // spender
            1000, // value
            block.timestamp + 3600, // deadline
            1, // nonce
            new bytes(65) // signature
        );
        
        vm.stopPrank();
    }
    
    function testPermitExpiration() public {
        vm.startPrank(user1);
        
        // Try to use an expired permit - should fail
        vm.expectRevert("Permit expired");
        replayProtection.usePermit(
            user1, // owner
            user2, // spender
            1000, // value
            block.timestamp - 3600, // expired deadline
            1, // nonce
            new bytes(65) // signature
        );
        
        vm.stopPrank();
    }
}