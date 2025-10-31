// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

/// @title EventSchema
/// @notice A comprehensive event schema implementation with proper indexing and structured logging
/// @dev Implements standardized event schemas with consistent naming, indexed parameters, and comprehensive logging
contract EventSchema is AccessControl, ReentrancyGuard {
    // Role definitions
    bytes32 public constant EVENT_ADMIN_ROLE = keccak256("EVENT_ADMIN_ROLE");
    bytes32 public constant EVENT_EMITTER_ROLE = keccak256("EVENT_EMITTER_ROLE");
    
    // Event schemas with proper indexing for analytics and auditability
    event ContractDeployed(
        address indexed contractAddress,
        string contractName,
        address indexed deployer,
        uint256 timestamp,
        bytes32 indexed version
    );
    
    event FunctionCalled(
        bytes4 indexed functionSelector,
        string functionName,
        address indexed caller,
        address indexed contractAddress,
        uint256 timestamp,
        uint256 gasUsed
    );
    
    event StateChanged(
        string indexed stateVariable,
        address indexed contractAddress,
        bytes32 indexed changeId,
        address changer,
        bytes oldValue,
        bytes newValue,
        uint256 timestamp
    );
    
    event UserAction(
        string indexed actionType,
        address indexed user,
        address indexed contractAddress,
        uint256 timestamp,
        bytes data
    );
    
    event SecurityEvent(
        string indexed eventType,
        address indexed contractAddress,
        address indexed actor,
        uint256 severity,
        string message,
        uint256 timestamp
    );
    
    event Transfer(
        address indexed from,
        address indexed to,
        address indexed token,
        uint256 amount,
        uint256 timestamp
    );
    
    event Approval(
        address indexed owner,
        address indexed spender,
        address indexed token,
        uint256 amount,
        uint256 timestamp
    );
    
    event GovernanceAction(
        string indexed actionType,
        address indexed executor,
        address indexed target,
        uint256 timestamp,
        bytes data
    );
    
    event Upgrade(
        address indexed oldImplementation,
        address indexed newImplementation,
        address indexed admin,
        uint256 timestamp,
        string version
    );
    
    event EmergencyAction(
        string indexed actionType,
        address indexed executor,
        uint256 timestamp,
        string reason
    );
    
    event Error(
        string indexed errorType,
        address indexed contractAddress,
        uint256 indexed errorCode,
        string message,
        uint256 timestamp
    );
    
    event PerformanceMetric(
        string indexed metricName,
        address indexed contractAddress,
        uint256 value,
        uint256 timestamp
    );
    
    // Event tracking
    uint256 public totalEventsEmitted;
    mapping(string => uint256) public eventCounts;
    mapping(address => uint256) public contractEventCounts;
    
    // Events for tracking event schema itself
    event EventSchemaRegistered(
        string indexed eventName,
        bytes32 indexed eventSignature,
        address indexed registrar,
        uint256 timestamp
    );
    
    event EventSchemaUpdated(
        string indexed eventName,
        bytes32 indexed eventSignature,
        address indexed updater,
        uint256 timestamp
    );
    
    /// @notice Constructor to initialize the event schema
    /// @param admin The admin address
    constructor(address admin) {
        require(admin != address(0), "Admin cannot be zero address");
        
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(EVENT_ADMIN_ROLE, admin);
        _grantRole(EVENT_EMITTER_ROLE, admin);
        
        totalEventsEmitted = 0;
    }
    
    /// @notice Register a new event schema
    /// @param eventName The name of the event
    /// @param eventSignature The signature of the event
    function registerEventSchema(string memory eventName, bytes32 eventSignature) public onlyRole(EVENT_ADMIN_ROLE) {
        emit EventSchemaRegistered(eventName, eventSignature, msg.sender, block.timestamp);
        eventCounts[eventName] += 1;
    }
    
    /// @notice Update an existing event schema
    /// @param eventName The name of the event
    /// @param eventSignature The signature of the event
    function updateEventSchema(string memory eventName, bytes32 eventSignature) public onlyRole(EVENT_ADMIN_ROLE) {
        emit EventSchemaUpdated(eventName, eventSignature, msg.sender, block.timestamp);
        eventCounts[eventName] += 1;
    }
    
    /// @notice Emit a ContractDeployed event
    /// @param contractAddress The address of the deployed contract
    /// @param contractName The name of the contract
    /// @param deployer The address of the deployer
    /// @param version The version of the contract
    function emitContractDeployed(
        address contractAddress,
        string memory contractName,
        address deployer,
        bytes32 version
    ) public onlyRole(EVENT_EMITTER_ROLE) {
        emit ContractDeployed(contractAddress, contractName, deployer, block.timestamp, version);
        totalEventsEmitted += 1;
        eventCounts["ContractDeployed"] += 1;
        contractEventCounts[contractAddress] += 1;
    }
    
    /// @notice Emit a FunctionCalled event
    /// @param functionSelector The function selector
    /// @param functionName The name of the function
    /// @param contractAddress The address of the contract
    /// @param gasUsed The gas used in the call
    function emitFunctionCalled(
        bytes4 functionSelector,
        string memory functionName,
        address contractAddress,
        uint256 gasUsed
    ) public onlyRole(EVENT_EMITTER_ROLE) {
        emit FunctionCalled(functionSelector, functionName, msg.sender, contractAddress, block.timestamp, gasUsed);
        totalEventsEmitted += 1;
        eventCounts["FunctionCalled"] += 1;
        contractEventCounts[contractAddress] += 1;
    }
    
    /// @notice Emit a StateChanged event
    /// @param stateVariable The name of the state variable
    /// @param contractAddress The address of the contract
    /// @param changeId The ID of the change
    /// @param oldValue The old value (encoded)
    /// @param newValue The new value (encoded)
    function emitStateChanged(
        string memory stateVariable,
        address contractAddress,
        bytes32 changeId,
        bytes memory oldValue,
        bytes memory newValue
    ) public onlyRole(EVENT_EMITTER_ROLE) {
        emit StateChanged(stateVariable, contractAddress, changeId, msg.sender, oldValue, newValue, block.timestamp);
        totalEventsEmitted += 1;
        eventCounts["StateChanged"] += 1;
        contractEventCounts[contractAddress] += 1;
    }
    
    /// @notice Emit a UserAction event
    /// @param actionType The type of action
    /// @param contractAddress The address of the contract
    /// @param data Additional data
    function emitUserAction(
        string memory actionType,
        address contractAddress,
        bytes memory data
    ) public onlyRole(EVENT_EMITTER_ROLE) {
        emit UserAction(actionType, msg.sender, contractAddress, block.timestamp, data);
        totalEventsEmitted += 1;
        eventCounts["UserAction"] += 1;
        contractEventCounts[contractAddress] += 1;
    }
    
    /// @notice Emit a SecurityEvent event
    /// @param eventType The type of security event
    /// @param contractAddress The address of the contract
    /// @param severity The severity level (1-10)
    /// @param message The event message
    function emitSecurityEvent(
        string memory eventType,
        address contractAddress,
        uint256 severity,
        string memory message
    ) public onlyRole(EVENT_EMITTER_ROLE) {
        require(severity >= 1 && severity <= 10, "Severity must be between 1 and 10");
        emit SecurityEvent(eventType, contractAddress, msg.sender, severity, message, block.timestamp);
        totalEventsEmitted += 1;
        eventCounts["SecurityEvent"] += 1;
        contractEventCounts[contractAddress] += 1;
    }
    
    /// @notice Emit a Transfer event
    /// @param from The sender address
    /// @param to The recipient address
    /// @param token The token address
    /// @param amount The transfer amount
    function emitTransfer(
        address from,
        address to,
        address token,
        uint256 amount
    ) public onlyRole(EVENT_EMITTER_ROLE) {
        emit Transfer(from, to, token, amount, block.timestamp);
        totalEventsEmitted += 1;
        eventCounts["Transfer"] += 1;
        contractEventCounts[token] += 1;
    }
    
    /// @notice Emit an Approval event
    /// @param owner The owner address
    /// @param spender The spender address
    /// @param token The token address
    /// @param amount The approval amount
    function emitApproval(
        address owner,
        address spender,
        address token,
        uint256 amount
    ) public onlyRole(EVENT_EMITTER_ROLE) {
        emit Approval(owner, spender, token, amount, block.timestamp);
        totalEventsEmitted += 1;
        eventCounts["Approval"] += 1;
        contractEventCounts[token] += 1;
    }
    
    /// @notice Emit a GovernanceAction event
    /// @param actionType The type of governance action
    /// @param target The target address
    /// @param data Additional data
    function emitGovernanceAction(
        string memory actionType,
        address target,
        bytes memory data
    ) public onlyRole(EVENT_EMITTER_ROLE) {
        emit GovernanceAction(actionType, msg.sender, target, block.timestamp, data);
        totalEventsEmitted += 1;
        eventCounts["GovernanceAction"] += 1;
        contractEventCounts[target] += 1;
    }
    
    /// @notice Emit an Upgrade event
    /// @param oldImplementation The old implementation address
    /// @param newImplementation The new implementation address
    /// @param version The version string
    function emitUpgrade(
        address oldImplementation,
        address newImplementation,
        string memory version
    ) public onlyRole(EVENT_EMITTER_ROLE) {
        emit Upgrade(oldImplementation, newImplementation, msg.sender, block.timestamp, version);
        totalEventsEmitted += 1;
        eventCounts["Upgrade"] += 1;
        contractEventCounts[newImplementation] += 1;
    }
    
    /// @notice Emit an EmergencyAction event
    /// @param actionType The type of emergency action
    /// @param reason The reason for the action
    function emitEmergencyAction(
        string memory actionType,
        string memory reason
    ) public onlyRole(EVENT_EMITTER_ROLE) {
        emit EmergencyAction(actionType, msg.sender, block.timestamp, reason);
        totalEventsEmitted += 1;
        eventCounts["EmergencyAction"] += 1;
    }
    
    /// @notice Emit an Error event
    /// @param errorType The type of error
    /// @param contractAddress The address of the contract
    /// @param errorCode The error code
    /// @param message The error message
    function emitError(
        string memory errorType,
        address contractAddress,
        uint256 errorCode,
        string memory message
    ) public onlyRole(EVENT_EMITTER_ROLE) {
        emit Error(errorType, contractAddress, errorCode, message, block.timestamp);
        totalEventsEmitted += 1;
        eventCounts["Error"] += 1;
        contractEventCounts[contractAddress] += 1;
    }
    
    /// @notice Emit a PerformanceMetric event
    /// @param metricName The name of the metric
    /// @param contractAddress The address of the contract
    /// @param value The metric value
    function emitPerformanceMetric(
        string memory metricName,
        address contractAddress,
        uint256 value
    ) public onlyRole(EVENT_EMITTER_ROLE) {
        emit PerformanceMetric(metricName, contractAddress, value, block.timestamp);
        totalEventsEmitted += 1;
        eventCounts["PerformanceMetric"] += 1;
        contractEventCounts[contractAddress] += 1;
    }
    
    /// @notice Get the count of a specific event type
    /// @param eventName The name of the event
    /// @return uint256 The count of the event
    function getEventCount(string memory eventName) public view returns (uint256) {
        return eventCounts[eventName];
    }
    
    /// @notice Get the count of events for a specific contract
    /// @param contractAddress The address of the contract
    /// @return uint256 The count of events for the contract
    function getContractEventCount(address contractAddress) public view returns (uint256) {
        return contractEventCounts[contractAddress];
    }
    
    /// @notice Get the total number of events emitted
    /// @return uint256 The total number of events
    function getTotalEventsEmitted() public view returns (uint256) {
        return totalEventsEmitted;
    }
}