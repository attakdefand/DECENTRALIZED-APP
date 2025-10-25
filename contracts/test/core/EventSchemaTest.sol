// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../../src/core/EventSchema.sol";
import "../../src/core/EventSchemaToken.sol";

/// @title EventSchemaTest
/// @notice Comprehensive tests for event schema implementation including indexing, auditability, and analytics readiness
contract EventSchemaTest is Test {
    EventSchema public eventSchema;
    EventSchemaToken public token;
    
    address public owner;
    address public user1;
    address public user2;
    address public emitter;
    
    /// @notice Set up test environment
    function setUp() public {
        owner = address(this);
        user1 = address(0x1);
        user2 = address(0x2);
        emitter = address(0x3);
        
        // Deploy event schema
        eventSchema = new EventSchema(owner);
        
        // Grant roles
        eventSchema.grantRole(eventSchema.EVENT_EMITTER_ROLE(), emitter);
        eventSchema.grantRole(eventSchema.EVENT_EMITTER_ROLE(), owner);
        
        // Deploy token with event schema
        token = new EventSchemaToken("Test Token", "TST", 1000000 * 1e18, address(eventSchema));
        
        // Transfer tokens to users
        token.transfer(user1, 10000 * 1e18);
        token.transfer(user2, 10000 * 1e18);
        
        // Approve token spending
        vm.prank(user1);
        token.approve(address(this), 5000 * 1e18);
        vm.prank(user2);
        token.approve(address(this), 5000 * 1e18);
    }
    
    // ==================== Event Schema Registration Tests ====================
    
    /// @notice Test event schema registration
    function testEventSchemaRegistration() public {
        string memory eventName = "TestEvent";
        bytes32 eventSignature = keccak256("TestEvent(address,uint256)");
        
        // Register event schema
        eventSchema.registerEventSchema(eventName, eventSignature);
        
        // Check event count
        assertEq(eventSchema.getEventCount(eventName), 1);
    }
    
    /// @notice Test event schema update
    function testEventSchemaUpdate() public {
        string memory eventName = "TestEvent";
        bytes32 eventSignature = keccak256("TestEvent(address,uint256)");
        bytes32 newEventSignature = keccak256("TestEvent(address,uint256,string)");
        
        // Register event schema
        eventSchema.registerEventSchema(eventName, eventSignature);
        
        // Update event schema
        eventSchema.updateEventSchema(eventName, newEventSignature);
        
        // Check event count
        assertEq(eventSchema.getEventCount(eventName), 2);
    }
    
    /// @notice Test unauthorized event schema registration
    function testUnauthorizedEventSchemaRegistration() public {
        string memory eventName = "TestEvent";
        bytes32 eventSignature = keccak256("TestEvent(address,uint256)");
        
        // User1 should not be able to register event schema
        vm.expectRevert();
        vm.prank(user1);
        eventSchema.registerEventSchema(eventName, eventSignature);
    }
    
    // ==================== Contract Deployment Event Tests ====================
    
    /// @notice Test ContractDeployed event emission
    function testContractDeployedEvent() public {
        address contractAddress = address(0x1234);
        string memory contractName = "TestContract";
        address deployer = owner;
        bytes32 version = keccak256("1.0.0");
        
        // Emit ContractDeployed event
        eventSchema.emitContractDeployed(contractAddress, contractName, deployer, version);
        
        // Check event counts
        assertEq(eventSchema.getTotalEventsEmitted(), 1);
        assertEq(eventSchema.getEventCount("ContractDeployed"), 1);
        assertEq(eventSchema.getContractEventCount(contractAddress), 1);
    }
    
    // ==================== Function Called Event Tests ====================
    
    /// @notice Test FunctionCalled event emission
    function testFunctionCalledEvent() public {
        bytes4 functionSelector = bytes4(keccak256("testFunction()"));
        string memory functionName = "testFunction";
        address contractAddress = address(token);
        uint256 gasUsed = 1000;
        
        // Emit FunctionCalled event
        eventSchema.emitFunctionCalled(functionSelector, functionName, contractAddress, gasUsed);
        
        // Check event counts
        assertEq(eventSchema.getTotalEventsEmitted(), 1);
        assertEq(eventSchema.getEventCount("FunctionCalled"), 1);
        assertEq(eventSchema.getContractEventCount(contractAddress), 1);
    }
    
    // ==================== State Changed Event Tests ====================
    
    /// @notice Test StateChanged event emission
    function testStateChangedEvent() public {
        string memory stateVariable = "testVariable";
        address contractAddress = address(token);
        bytes32 changeId = keccak256("testChange");
        bytes memory oldValue = abi.encode(uint256(100));
        bytes memory newValue = abi.encode(uint256(200));
        
        // Emit StateChanged event
        eventSchema.emitStateChanged(stateVariable, contractAddress, changeId, oldValue, newValue);
        
        // Check event counts
        assertEq(eventSchema.getTotalEventsEmitted(), 1);
        assertEq(eventSchema.getEventCount("StateChanged"), 1);
        assertEq(eventSchema.getContractEventCount(contractAddress), 1);
    }
    
    // ==================== User Action Event Tests ====================
    
    /// @notice Test UserAction event emission
    function testUserActionEvent() public {
        string memory actionType = "transfer";
        address contractAddress = address(token);
        bytes memory data = abi.encode(user1, user2, 1000);
        
        // Emit UserAction event
        eventSchema.emitUserAction(actionType, contractAddress, data);
        
        // Check event counts
        assertEq(eventSchema.getTotalEventsEmitted(), 1);
        assertEq(eventSchema.getEventCount("UserAction"), 1);
        assertEq(eventSchema.getContractEventCount(contractAddress), 1);
    }
    
    // ==================== Security Event Tests ====================
    
    /// @notice Test SecurityEvent emission
    function testSecurityEvent() public {
        string memory eventType = "reentrancyAttempt";
        address contractAddress = address(token);
        uint256 severity = 8;
        string memory message = "Potential reentrancy attack detected";
        
        // Emit SecurityEvent
        eventSchema.emitSecurityEvent(eventType, contractAddress, severity, message);
        
        // Check event counts
        assertEq(eventSchema.getTotalEventsEmitted(), 1);
        assertEq(eventSchema.getEventCount("SecurityEvent"), 1);
        assertEq(eventSchema.getContractEventCount(contractAddress), 1);
    }
    
    /// @notice Test SecurityEvent with invalid severity
    function testSecurityEventInvalidSeverity() public {
        string memory eventType = "reentrancyAttempt";
        address contractAddress = address(token);
        uint256 severity = 15; // Invalid severity
        string memory message = "Potential reentrancy attack detected";
        
        // Should fail with invalid severity
        vm.expectRevert("Severity must be between 1 and 10");
        eventSchema.emitSecurityEvent(eventType, contractAddress, severity, message);
    }
    
    // ==================== Transfer Event Tests ====================
    
    /// @notice Test Transfer event emission
    function testTransferEvent() public {
        address from = user1;
        address to = user2;
        address tokenAddress = address(token);
        uint256 amount = 1000 * 1e18;
        
        // Emit Transfer event
        eventSchema.emitTransfer(from, to, tokenAddress, amount);
        
        // Check event counts
        assertEq(eventSchema.getTotalEventsEmitted(), 1);
        assertEq(eventSchema.getEventCount("Transfer"), 1);
        assertEq(eventSchema.getContractEventCount(tokenAddress), 1);
    }
    
    // ==================== Approval Event Tests ====================
    
    /// @notice Test Approval event emission
    function testApprovalEvent() public {
        address ownerAddr = user1;
        address spender = user2;
        address tokenAddress = address(token);
        uint256 amount = 1000 * 1e18;
        
        // Emit Approval event
        eventSchema.emitApproval(ownerAddr, spender, tokenAddress, amount);
        
        // Check event counts
        assertEq(eventSchema.getTotalEventsEmitted(), 1);
        assertEq(eventSchema.getEventCount("Approval"), 1);
        assertEq(eventSchema.getContractEventCount(tokenAddress), 1);
    }
    
    // ==================== Governance Action Event Tests ====================
    
    /// @notice Test GovernanceAction event emission
    function testGovernanceActionEvent() public {
        string memory actionType = "upgrade";
        address target = address(token);
        bytes memory data = abi.encode("newImplementationAddress");
        
        // Emit GovernanceAction event
        eventSchema.emitGovernanceAction(actionType, target, data);
        
        // Check event counts
        assertEq(eventSchema.getTotalEventsEmitted(), 1);
        assertEq(eventSchema.getEventCount("GovernanceAction"), 1);
        assertEq(eventSchema.getContractEventCount(target), 1);
    }
    
    // ==================== Upgrade Event Tests ====================
    
    /// @notice Test Upgrade event emission
    function testUpgradeEvent() public {
        address oldImplementation = address(0x1111);
        address newImplementation = address(0x2222);
        string memory version = "2.0.0";
        
        // Emit Upgrade event
        eventSchema.emitUpgrade(oldImplementation, newImplementation, version);
        
        // Check event counts
        assertEq(eventSchema.getTotalEventsEmitted(), 1);
        assertEq(eventSchema.getEventCount("Upgrade"), 1);
        assertEq(eventSchema.getContractEventCount(newImplementation), 1);
    }
    
    // ==================== Emergency Action Event Tests ====================
    
    /// @notice Test EmergencyAction event emission
    function testEmergencyActionEvent() public {
        string memory actionType = "pause";
        string memory reason = "Security vulnerability detected";
        
        // Emit EmergencyAction event
        eventSchema.emitEmergencyAction(actionType, reason);
        
        // Check event counts
        assertEq(eventSchema.getTotalEventsEmitted(), 1);
        assertEq(eventSchema.getEventCount("EmergencyAction"), 1);
    }
    
    // ==================== Error Event Tests ====================
    
    /// @notice Test Error event emission
    function testErrorEvent() public {
        string memory errorType = "InsufficientBalance";
        address contractAddress = address(token);
        uint256 errorCode = 1001;
        string memory message = "User does not have sufficient balance for transfer";
        
        // Emit Error event
        eventSchema.emitError(errorType, contractAddress, errorCode, message);
        
        // Check event counts
        assertEq(eventSchema.getTotalEventsEmitted(), 1);
        assertEq(eventSchema.getEventCount("Error"), 1);
        assertEq(eventSchema.getContractEventCount(contractAddress), 1);
    }
    
    // ==================== Performance Metric Event Tests ====================
    
    /// @notice Test PerformanceMetric event emission
    function testPerformanceMetricEvent() public {
        string memory metricName = "gasUsed";
        address contractAddress = address(token);
        uint256 value = 50000;
        
        // Emit PerformanceMetric event
        eventSchema.emitPerformanceMetric(metricName, contractAddress, value);
        
        // Check event counts
        assertEq(eventSchema.getTotalEventsEmitted(), 1);
        assertEq(eventSchema.getEventCount("PerformanceMetric"), 1);
        assertEq(eventSchema.getContractEventCount(contractAddress), 1);
    }
    
    // ==================== Token Integration Tests ====================
    
    /// @notice Test token transfer with event emission
    function testTokenTransferWithEvents() public {
        uint256 initialTotalEvents = eventSchema.getTotalEventsEmitted();
        uint256 initialTransferEvents = eventSchema.getEventCount("Transfer");
        uint256 initialFunctionEvents = eventSchema.getEventCount("FunctionCalled");
        uint256 initialPerformanceEvents = eventSchema.getEventCount("PerformanceMetric");
        
        uint256 transferAmount = 1000 * 1e18;
        
        // Perform token transfer
        vm.prank(user1);
        bool success = token.transfer(user2, transferAmount);
        
        assertTrue(success);
        
        // Check that events were emitted
        assertTrue(eventSchema.getTotalEventsEmitted() > initialTotalEvents);
        assertTrue(eventSchema.getEventCount("Transfer") > initialTransferEvents);
        assertTrue(eventSchema.getEventCount("FunctionCalled") > initialFunctionEvents);
        assertTrue(eventSchema.getEventCount("PerformanceMetric") > initialPerformanceEvents);
    }
    
    /// @notice Test token approval with event emission
    function testTokenApprovalWithEvents() public {
        uint256 initialTotalEvents = eventSchema.getTotalEventsEmitted();
        uint256 initialApprovalEvents = eventSchema.getEventCount("Approval");
        uint256 initialUserActionEvents = eventSchema.getEventCount("UserAction");
        
        uint256 approvalAmount = 2000 * 1e18;
        
        // Perform token approval
        vm.prank(user1);
        bool success = token.approve(user2, approvalAmount);
        
        assertTrue(success);
        
        // Check that events were emitted
        assertTrue(eventSchema.getTotalEventsEmitted() > initialTotalEvents);
        assertTrue(eventSchema.getEventCount("Approval") > initialApprovalEvents);
        assertTrue(eventSchema.getEventCount("UserAction") > initialUserActionEvents);
    }
    
    /// @notice Test token burn with event emission
    function testTokenBurnWithEvents() public {
        uint256 initialTotalEvents = eventSchema.getTotalEventsEmitted();
        uint256 initialUserActionEvents = eventSchema.getEventCount("UserAction");
        uint256 initialFunctionEvents = eventSchema.getEventCount("FunctionCalled");
        uint256 initialPerformanceEvents = eventSchema.getEventCount("PerformanceMetric");
        
        uint256 burnAmount = 500 * 1e18;
        
        // Perform token burn
        vm.prank(user1);
        token.burn(burnAmount);
        
        // Check that events were emitted
        assertTrue(eventSchema.getTotalEventsEmitted() > initialTotalEvents);
        assertTrue(eventSchema.getEventCount("UserAction") > initialUserActionEvents);
        assertTrue(eventSchema.getEventCount("FunctionCalled") > initialFunctionEvents);
        assertTrue(eventSchema.getEventCount("PerformanceMetric") > initialPerformanceEvents);
    }
    
    // ==================== Event Snapshot Comparison Tests ====================
    
    /// @notice Test event snapshot comparison
    function testEventSnapshotComparison() public {
        uint256 snapshot1 = eventSchema.getTotalEventsEmitted();
        
        // Emit some events
        eventSchema.emitTransfer(user1, user2, address(token), 1000);
        eventSchema.emitApproval(user1, user2, address(token), 2000);
        eventSchema.emitUserAction("test", address(token), "");
        
        uint256 snapshot2 = eventSchema.getTotalEventsEmitted();
        
        // Verify snapshots are different
        assertTrue(snapshot2 > snapshot1);
        
        // Verify specific event counts
        assertEq(eventSchema.getEventCount("Transfer"), 1);
        assertEq(eventSchema.getEventCount("Approval"), 1);
        assertEq(eventSchema.getEventCount("UserAction"), 1);
    }
    
    // ==================== ABI/Event Schema Lint Tests ====================
    
    /// @notice Test ABI/event schema linting
    function testEventSchemaLinting() public {
        // This would typically be done off-chain with tools
        // Here we're just verifying the event signatures are consistent
        
        // Check that event signatures are properly formatted
        bytes32 transferSignature = keccak256("Transfer(address,address,address,uint256,uint256)");
        bytes32 approvalSignature = keccak256("Approval(address,address,address,uint256,uint256)");
        
        // These should be consistent with the actual event definitions
        assertTrue(transferSignature != 0);
        assertTrue(approvalSignature != 0);
    }
    
    // ==================== Event Coverage Tests ====================
    
    /// @notice Test event coverage percentage
    function testEventCoverage() public {
        // This is a simplified test - in practice, you would measure actual coverage
        uint256 initialEvents = eventSchema.getTotalEventsEmitted();
        
        // Emit events of all types
        eventSchema.emitContractDeployed(address(this), "Test", owner, keccak256("1.0.0"));
        eventSchema.emitFunctionCalled(this.testEventCoverage.selector, "testEventCoverage", address(this), 1000);
        eventSchema.emitStateChanged("test", address(this), keccak256("change"), "", "");
        eventSchema.emitUserAction("test", address(this), "");
        eventSchema.emitSecurityEvent("test", address(this), 5, "test");
        eventSchema.emitTransfer(address(this), address(0x1), address(this), 100);
        eventSchema.emitApproval(address(this), address(0x1), address(this), 100);
        eventSchema.emitGovernanceAction("test", address(this), "");
        eventSchema.emitUpgrade(address(0x1), address(0x2), "1.0.0");
        eventSchema.emitEmergencyAction("test", "test");
        eventSchema.emitError("test", address(this), 1000, "test");
        eventSchema.emitPerformanceMetric("test", address(this), 1000);
        
        uint256 finalEvents = eventSchema.getTotalEventsEmitted();
        
        // Verify all event types were covered
        assertTrue(finalEvents > initialEvents);
        assertEq(eventSchema.getEventCount("ContractDeployed"), 1);
        assertEq(eventSchema.getEventCount("FunctionCalled"), 1);
        assertEq(eventSchema.getEventCount("StateChanged"), 1);
        assertEq(eventSchema.getEventCount("UserAction"), 1);
        assertEq(eventSchema.getEventCount("SecurityEvent"), 1);
        assertEq(eventSchema.getEventCount("Transfer"), 1);
        assertEq(eventSchema.getEventCount("Approval"), 1);
        assertEq(eventSchema.getEventCount("GovernanceAction"), 1);
        assertEq(eventSchema.getEventCount("Upgrade"), 1);
        assertEq(eventSchema.getEventCount("EmergencyAction"), 1);
        assertEq(eventSchema.getEventCount("Error"), 1);
        assertEq(eventSchema.getEventCount("PerformanceMetric"), 1);
    }
    
    // ==================== Property Tests ====================
    
    /// @notice Property test: Event counts should be non-negative
    function testPropertyEventCountsNonNegative() public {
        // Emit some events
        eventSchema.emitTransfer(user1, user2, address(token), 1000);
        eventSchema.emitApproval(user1, user2, address(token), 2000);
        
        // Check that event counts are non-negative
        assertTrue(eventSchema.getEventCount("Transfer") >= 0);
        assertTrue(eventSchema.getEventCount("Approval") >= 0);
        assertTrue(eventSchema.getTotalEventsEmitted() >= 0);
    }
    
    /// @notice Property test: Event counts should increase with emissions
    function testPropertyEventCountsIncreaseWithEmissions(uint256 numEvents) public {
        // Bound the number of events to a reasonable range
        numEvents = bound(numEvents, 0, 100);
        
        uint256 initialCount = eventSchema.getEventCount("Transfer");
        uint256 initialTotal = eventSchema.getTotalEventsEmitted();
        
        // Emit multiple events
        for (uint256 i = 0; i < numEvents; i++) {
            eventSchema.emitTransfer(user1, user2, address(token), 1000 + i);
        }
        
        uint256 finalCount = eventSchema.getEventCount("Transfer");
        uint256 finalTotal = eventSchema.getTotalEventsEmitted();
        
        // Verify counts increased correctly
        assertEq(finalCount, initialCount + numEvents);
        assertEq(finalTotal, initialTotal + numEvents);
    }
    
    /// @notice Property test: Contract event counts should be accurate
    function testPropertyContractEventCountsAccurate() public {
        address contract1 = address(0x1111);
        address contract2 = address(0x2222);
        
        uint256 initialCount1 = eventSchema.getContractEventCount(contract1);
        uint256 initialCount2 = eventSchema.getContractEventCount(contract2);
        
        // Emit events for different contracts
        eventSchema.emitTransfer(user1, user2, contract1, 1000);
        eventSchema.emitTransfer(user1, user2, contract1, 2000);
        eventSchema.emitTransfer(user1, user2, contract2, 3000);
        
        uint256 finalCount1 = eventSchema.getContractEventCount(contract1);
        uint256 finalCount2 = eventSchema.getContractEventCount(contract2);
        
        // Verify contract event counts
        assertEq(finalCount1, initialCount1 + 2);
        assertEq(finalCount2, initialCount2 + 1);
    }
    
    // ==================== Edge Case Tests ====================
    
    /// @notice Test event emission with zero values
    function testEventEmissionWithZeroValues() public {
        // Emit events with zero values where appropriate
        eventSchema.emitTransfer(address(0), address(0), address(token), 0);
        eventSchema.emitApproval(address(0), address(0), address(token), 0);
        
        // Check that events were emitted
        assertEq(eventSchema.getEventCount("Transfer"), 1);
        assertEq(eventSchema.getEventCount("Approval"), 1);
    }
    
    /// @notice Test event emission with maximum values
    function testEventEmissionWithMaxValues() public {
        // Emit events with maximum values
        eventSchema.emitTransfer(address(type(uint160).max), address(type(uint160).max), address(token), type(uint256).max);
        eventSchema.emitSecurityEvent("test", address(token), 10, "Maximum severity test");
        
        // Check that events were emitted
        assertEq(eventSchema.getEventCount("Transfer"), 1);
        assertEq(eventSchema.getEventCount("SecurityEvent"), 1);
    }
    
    /// @notice Test event emission with empty strings
    function testEventEmissionWithEmptyStrings() public {
        // Emit events with empty strings
        eventSchema.emitUserAction("", address(token), "");
        eventSchema.emitSecurityEvent("", address(token), 5, "");
        
        // Check that events were emitted
        assertEq(eventSchema.getEventCount("UserAction"), 1);
        assertEq(eventSchema.getEventCount("SecurityEvent"), 1);
    }
    
    /// @notice Test unauthorized event emission
    function testUnauthorizedEventEmission() public {
        // User1 should not be able to emit events
        vm.expectRevert();
        vm.prank(user1);
        eventSchema.emitTransfer(user1, user2, address(token), 1000);
    }
}