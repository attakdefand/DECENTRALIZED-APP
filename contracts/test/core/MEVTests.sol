// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../../src/core/CommitReveal.sol";
import "../../src/core/BatchAuction.sol";

contract MEVTests is Test {
    CommitReveal public commitReveal;
    BatchAuction public batchAuction;
    
    address public owner = address(1);
    address public user1 = address(2);
    address public user2 = address(3);
    address public feeRecipient = address(4);
    
    function setUp() public {
        vm.startPrank(owner);
        commitReveal = new CommitReveal(5, 5, 1 ether, 0.1 ether, 12);
        batchAuction = new BatchAuction(12, feeRecipient, 10); // 12 seconds, 10 basis points fee
        vm.stopPrank();
    }
    
    function testCommitRevealFlow() public {
        vm.startPrank(user1);
        
        // Add stake
        vm.deal(user1, 2 ether);
        commitReveal.addStake{value: 1 ether}();
        
        // Commit phase
        bytes memory data = abi.encodeWithSignature("transfer(address,uint256)", user2, 100);
        bytes32 salt = keccak256("salt");
        bytes32 commitmentHash = keccak256(abi.encodePacked(data, salt));
        
        commitReveal.commit(commitmentHash, 100000, 20 gwei);
        
        // Move time forward to end commit phase
        vm.warp(block.timestamp + 6);
        
        // Reveal phase
        commitReveal.reveal(data, salt);
        
        vm.stopPrank();
    }
    
    function testFailedReveal() public {
        vm.startPrank(user1);
        
        // Add stake
        vm.deal(user1, 2 ether);
        commitReveal.addStake{value: 1 ether}();
        
        // Commit phase
        bytes memory data = abi.encodeWithSignature("transfer(address,uint256)", user2, 100);
        bytes32 salt = keccak256("salt");
        bytes32 commitmentHash = keccak256(abi.encodePacked(data, salt));
        
        commitReveal.commit(commitmentHash, 100000, 20 gwei);
        
        // Move time forward to end both phases
        vm.warp(block.timestamp + 12);
        
        // Try to reveal after reveal phase ended - should fail
        vm.expectRevert("Reveal phase ended");
        commitReveal.reveal(data, salt);
        
        vm.stopPrank();
    }
    
    function testBatchAuctionFlow() public {
        // Submit orders
        vm.startPrank(user1);
        // Mock token transfers would be needed here in a real test
        // For now, we'll just test the batch execution logic
        vm.stopPrank();
        
        // Move time forward to allow batch execution
        vm.warp(block.timestamp + 13);
        
        // Execute batch
        batchAuction.executeBatch();
        
        // Verify batch was executed
        (uint256 batchId, uint256 startTime, uint256 endTime, , bool executed, uint256 clearingPrice) = batchAuction.batches(1);
        assertEq(executed, true);
    }
    
    function testSandwichBounds() public {
        vm.startPrank(user1);
        
        // Add stake
        vm.deal(user1, 2 ether);
        commitReveal.addStake{value: 1 ether}();
        
        // Commit phase
        bytes memory data = abi.encodeWithSignature("transfer(address,uint256)", user2, 100);
        bytes32 salt = keccak256("salt");
        bytes32 commitmentHash = keccak256(abi.encodePacked(data, salt));
        
        commitReveal.commit(commitmentHash, 100000, 20 gwei);
        
        // Move time forward to end commit phase
        vm.warp(block.timestamp + 6);
        
        // Reveal phase
        commitReveal.reveal(data, salt);
        
        // Try to reveal again immediately - should fail due to anti-sandwich window
        bytes memory data2 = abi.encodeWithSignature("transfer(address,uint256)", user2, 200);
        bytes32 salt2 = keccak256("salt2");
        bytes32 commitmentHash2 = keccak256(abi.encodePacked(data2, salt2));
        
        commitReveal.commit(commitmentHash2, 100000, 20 gwei);
        
        // Move time forward but not enough to pass anti-sandwich window
        vm.warp(block.timestamp + 1);
        
        // Try to reveal - should fail
        vm.expectRevert("Anti-sandwich window active");
        commitReveal.reveal(data2, salt2);
        
        vm.stopPrank();
    }
    
    function testCommitRevealCorrectness() public {
        vm.startPrank(user1);
        
        // Add stake
        vm.deal(user1, 2 ether);
        commitReveal.addStake{value: 1 ether}();
        
        // Commit with valid data
        bytes memory data = abi.encodeWithSignature("transfer(address,uint256)", user2, 100);
        bytes32 salt = keccak256("salt");
        bytes32 commitmentHash = keccak256(abi.encodePacked(data, salt));
        
        commitReveal.commit(commitmentHash, 100000, 20 gwei);
        
        // Move time forward
        vm.warp(block.timestamp + 6);
        
        // Reveal with correct data
        commitReveal.reveal(data, salt);
        
        // Try to reveal again - should fail
        vm.expectRevert("Already revealed");
        commitReveal.reveal(data, salt);
        
        vm.stopPrank();
    }
    
    function testGasLimitBounds() public {
        vm.startPrank(user1);
        
        // Add stake
        vm.deal(user1, 2 ether);
        commitReveal.addStake{value: 1 ether}();
        
        // Try to commit with gas limit below minimum - should fail
        bytes memory data = abi.encodeWithSignature("transfer(address,uint256)", user2, 100);
        bytes32 salt = keccak256("salt");
        bytes32 commitmentHash = keccak256(abi.encodePacked(data, salt));
        
        vm.expectRevert("Gas limit out of bounds");
        commitReveal.commit(commitmentHash, 1000, 20 gwei);
        
        // Try to commit with gas limit above maximum - should fail
        vm.expectRevert("Gas limit out of bounds");
        commitReveal.commit(commitmentHash, 100000000, 20 gwei);
        
        // Commit with valid gas limit - should succeed
        commitReveal.commit(commitmentHash, 100000, 20 gwei);
        
        vm.stopPrank();
    }
    
    function testBatchIntervalRespected() public {
        vm.startPrank(user1);
        
        // Add stake
        vm.deal(user1, 2 ether);
        commitReveal.addStake{value: 1 ether}();
        
        // Commit phase
        bytes memory data = abi.encodeWithSignature("transfer(address,uint256)", user2, 100);
        bytes32 salt = keccak256("salt");
        bytes32 commitmentHash = keccak256(abi.encodePacked(data, salt));
        
        commitReveal.commit(commitmentHash, 100000, 20 gwei);
        
        // Try to commit again immediately - should fail due to batch interval
        bytes memory data2 = abi.encodeWithSignature("transfer(address,uint256)", user2, 200);
        bytes32 salt2 = keccak256("salt2");
        bytes32 commitmentHash2 = keccak256(abi.encodePacked(data2, salt2));
        
        vm.expectRevert("Batch interval not respected");
        commitReveal.commit(commitmentHash2, 100000, 20 gwei);
        
        vm.stopPrank();
    }
    
    function testFuzzCommitReveal(uint256 gasLimit, uint256 maxFeePerGas) public {
        // Constrain inputs to valid ranges
        vm.assume(gasLimit >= 50000 && gasLimit <= 10000000);
        vm.assume(maxFeePerGas > 0 && maxFeePerGas <= 1000 gwei);
        
        vm.startPrank(user1);
        
        // Add stake
        vm.deal(user1, 2 ether);
        commitReveal.addStake{value: 1 ether}();
        
        // Commit phase
        bytes memory data = abi.encodeWithSignature("transfer(address,uint256)", user2, 100);
        bytes32 salt = keccak256("salt");
        bytes32 commitmentHash = keccak256(abi.encodePacked(data, salt));
        
        commitReveal.commit(commitmentHash, gasLimit, maxFeePerGas);
        
        // Move time forward to end commit phase
        vm.warp(block.timestamp + 6);
        
        // Reveal phase
        commitReveal.reveal(data, salt);
        
        vm.stopPrank();
    }
    
    function testInvariantCommitRevealState() public {
        // Test that commitments are properly tracked
        vm.startPrank(user1);
        
        // Add stake
        vm.deal(user1, 2 ether);
        commitReveal.addStake{value: 1 ether}();
        
        // Commit phase
        bytes memory data = abi.encodeWithSignature("transfer(address,uint256)", user2, 100);
        bytes32 salt = keccak256("salt");
        bytes32 commitmentHash = keccak256(abi.encodePacked(data, salt));
        
        commitReveal.commit(commitmentHash, 100000, 20 gwei);
        
        // Check commitment exists
        bytes32 commitId = keccak256(abi.encodePacked(commitmentHash, user1, block.timestamp, block.number));
        CommitReveal.Commitment memory commitment = commitReveal.getCommitment(commitId);
        assertEq(commitment.commitmentHash, commitmentHash);
        assertEq(commitment.user, user1);
        assertFalse(commitment.revealed);
        
        // Move time forward to end commit phase
        vm.warp(block.timestamp + 6);
        
        // Reveal phase
        commitReveal.reveal(data, salt);
        
        // Check commitment is now revealed
        commitment = commitReveal.getCommitment(commitId);
        assertTrue(commitment.revealed);
        
        vm.stopPrank();
    }
}