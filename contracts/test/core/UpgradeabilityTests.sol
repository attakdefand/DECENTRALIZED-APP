// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../../src/core/UpgradeableToken.sol";
import "../../src/core/TokenProxy.sol";
import "../../src/core/AppTimelock.sol";
import "../../src/core/GuardianMultisig.sol";

/// @title UpgradeabilityTests
/// @notice Comprehensive tests for upgradeability mechanisms
contract UpgradeabilityTests is Test {
    UpgradeableToken public tokenImplementation;
    TokenProxy public tokenProxy;
    UpgradeableToken public token;
    AppTimelock public timelock;
    GuardianMultisig public guardianMultisig;
    
    address public owner;
    address public user1;
    address public user2;
    address[] public guardians;
    
    /// @notice Set up test environment
    function setUp() public {
        owner = address(this);
        user1 = address(0x1);
        user2 = address(0x2);
        
        // Set up guardians
        guardians.push(user1);
        guardians.push(user2);
        
        // Deploy implementation
        tokenImplementation = new UpgradeableToken();
        
        // Deploy proxy with initialization
        bytes memory initData = abi.encodeWithSelector(
            UpgradeableToken.initialize.selector,
            "Test Token",
            "TST",
            1000000 * 10**18
        );
        
        tokenProxy = new TokenProxy(address(tokenImplementation), initData);
        token = UpgradeableToken(address(tokenProxy));
        
        // Deploy timelock
        address[] memory proposers = new address[](1);
        proposers[0] = owner;
        
        address[] memory executors = new address[](1);
        executors[0] = owner;
        
        timelock = new AppTimelock(24 hours, proposers, executors);
        
        // Deploy guardian multisig
        guardianMultisig = new GuardianMultisig(guardians, 2);
    }
    
    // ==================== UUPS Upgrade Tests ====================
    
    /// @notice Test UUPS initialization
    function testUUPSInitialization() public {
        // Assert token is properly initialized
        assertEq(token.name(), "Test Token");
        assertEq(token.symbol(), "TST");
        assertEq(token.totalSupply(), 1000000 * 10**18);
        assertEq(token.balanceOf(owner), 1000000 * 10**18);
    }
    
    /// @notice Test UUPS upgrade authorization
    function testUUPSAuthorization() public {
        // Deploy new implementation
        UpgradeableToken newImplementation = new UpgradeableToken();
        
        // Only owner should be able to upgrade
        vm.expectRevert("Ownable: caller is not the owner");
        vm.prank(user1);
        tokenProxy.upgradeTo(address(newImplementation));
        
        // Owner should be able to upgrade
        tokenProxy.upgradeTo(address(newImplementation));
        assertEq(tokenProxy.implementation(), address(newImplementation));
    }
    
    /// @notice Test UUPS upgrade with data
    function testUUPSUpgradeWithData() public {
        // Deploy new implementation
        UpgradeableToken newImplementation = new UpgradeableToken();
        
        // Prepare initialization data for new implementation
        bytes memory initData = abi.encodeWithSelector(
            UpgradeableToken.initialize.selector,
            "New Token",
            "NEW",
            2000000 * 10**18
        );
        
        // Owner should be able to upgrade with data
        tokenProxy.upgradeToAndCall(address(newImplementation), initData);
        assertEq(tokenProxy.implementation(), address(newImplementation));
    }
    
    // ==================== Timelock Tests ====================
    
    /// @notice Test timelock initialization
    function testTimelockInitialization() public {
        assertEq(timelock.getMinDelay(), 24 hours);
        assertTrue(timelock.hasRole(timelock.PROPOSER_ROLE(), owner));
        assertTrue(timelock.hasRole(timelock.EXECUTOR_ROLE(), owner));
    }
    
    /// @notice Test timelock schedule operation
    function testTimelockSchedule() public {
        bytes memory data = abi.encodeWithSelector(token.transfer.selector, user1, 1000 * 10**18);
        bytes32 predecessor = bytes32(0);
        bytes32 salt = bytes32(0);
        uint256 delay = 24 hours;
        
        // Schedule operation
        timelock.scheduleUpgrade(address(token), data, predecessor, salt, delay);
        
        // Operation should be scheduled
        bytes32 id = keccak256(abi.encode(address(token), data, predecessor, salt, delay));
        (bool ready, uint48 timestamp) = timelock.getOperationState(id);
        assertTrue(!ready); // Not ready yet
        assertTrue(timestamp > 0); // Scheduled
    }
    
    /// @notice Test timelock delay validation
    function testTimelockDelayValidation() public {
        bytes memory data = abi.encodeWithSelector(token.transfer.selector, user1, 1000 * 10**18);
        bytes32 predecessor = bytes32(0);
        bytes32 salt = bytes32(0);
        uint256 shortDelay = 1 hours; // Too short
        uint256 longDelay = 31 days; // Too long
        
        // Should fail with short delay
        vm.expectRevert("Delay less than minimum");
        timelock.scheduleUpgrade(address(token), data, predecessor, salt, shortDelay);
        
        // Should fail with long delay
        vm.expectRevert("Delay too long");
        timelock.scheduleUpgrade(address(token), data, predecessor, salt, longDelay);
    }
    
    // ==================== Guardian Multisig Tests ====================
    
    /// @notice Test guardian multisig initialization
    function testGuardianMultisigInitialization() public {
        assertEq(guardianMultisig.getGuardianCount(), 2);
        assertTrue(guardianMultisig.isAddressGuardian(user1));
        assertTrue(guardianMultisig.isAddressGuardian(user2));
        assertEq(guardianMultisig.requiredSignatures(), 2);
    }
    
    /// @notice Test guardian multisig proposal creation
    function testGuardianMultisigProposalCreation() public {
        bytes memory data = abi.encodeWithSelector(token.transfer.selector, user2, 1000 * 10**18);
        
        vm.prank(user1);
        bytes32 proposalId = guardianMultisig.createProposal(address(token), data);
        
        // Proposal should be created
        (uint256 voteCount, , bool executed, , ) = guardianMultisig.proposals(proposalId);
        assertEq(voteCount, 0);
        assertFalse(executed);
    }
    
    /// @notice Test guardian multisig voting
    function testGuardianMultisigVoting() public {
        bytes memory data = abi.encodeWithSelector(token.transfer.selector, user2, 1000 * 10**18);
        
        vm.prank(user1);
        bytes32 proposalId = guardianMultisig.createProposal(address(token), data);
        
        // First vote
        vm.prank(user1);
        guardianMultisig.vote(proposalId);
        
        (uint256 voteCount, , bool executed, , ) = guardianMultisig.proposals(proposalId);
        assertEq(voteCount, 1);
        assertFalse(executed);
        
        // Second vote should execute
        vm.prank(user2);
        guardianMultisig.vote(proposalId);
        
        (voteCount, , executed, , ) = guardianMultisig.proposals(proposalId);
        assertEq(voteCount, 2);
        assertTrue(executed);
    }
    
    /// @notice Test guardian multisig veto
    function testGuardianMultisigVeto() public {
        bytes memory data = abi.encodeWithSelector(token.transfer.selector, user2, 1000 * 10**18);
        
        vm.prank(user1);
        bytes32 proposalId = guardianMultisig.createProposal(address(token), data);
        
        // Admin can veto
        guardianMultisig.vetoProposal(proposalId);
        
        (, , bool executed, , ) = guardianMultisig.proposals(proposalId);
        assertTrue(executed);
    }
    
    /// @notice Test guardian multisig emergency pause
    function testGuardianMultisigEmergencyPause() public {
        // Guardian can pause
        vm.prank(user1);
        guardianMultisig.emergencyPause();
        
        // Token should be paused (if it were Pausable)
        // This is a simplified test since our token isn't pausable
    }
    
    // ==================== Integration Tests ====================
    
    /// @notice Test complete upgrade flow with timelock and guardians
    function testCompleteUpgradeFlow() public {
        // 1. Deploy new implementation
        UpgradeableToken newImplementation = new UpgradeableToken();
        
        // 2. Create upgrade proposal through guardian multisig
        bytes memory upgradeData = abi.encodeWithSelector(
            TokenProxy.upgradeTo.selector,
            address(newImplementation)
        );
        
        vm.prank(user1);
        bytes32 proposalId = guardianMultisig.createProposal(address(tokenProxy), upgradeData);
        
        // 3. Get guardian approvals
        vm.prank(user1);
        guardianMultisig.vote(proposalId);
        
        vm.prank(user2);
        guardianMultisig.vote(proposalId);
        
        // 4. Verify upgrade was executed
        assertEq(tokenProxy.implementation(), address(newImplementation));
    }
    
    // ==================== Property Tests ====================
    
    /// @notice Property test: Token supply should remain constant through upgrades
    function testPropertyTokenSupplyConstant() public {
        uint256 initialSupply = token.totalSupply();
        
        // Deploy new implementation
        UpgradeableToken newImplementation = new UpgradeableToken();
        
        // Upgrade
        tokenProxy.upgradeTo(address(newImplementation));
        
        // Supply should remain the same
        assertEq(token.totalSupply(), initialSupply);
    }
    
    /// @notice Property test: Timelock delay should be within bounds
    function testPropertyTimelockDelayBounds() public {
        uint256 minDelay = timelock.getMinDelay();
        assertTrue(minDelay >= 24 hours);
        assertTrue(minDelay <= 30 days);
    }
    
    /// @notice Property test: Guardian count should be reasonable
    function testPropertyGuardianCountReasonable() public {
        uint256 guardianCount = guardianMultisig.getGuardianCount();
        assertTrue(guardianCount > 0);
        assertTrue(guardianCount <= 10);
    }
}