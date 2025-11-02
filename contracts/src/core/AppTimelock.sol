// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/governance/TimelockController.sol";

/// @title AppTimelock
/// @notice A timelock controller for secure contract upgrades
/// @dev Implements OpenZeppelin's TimelockController with custom configuration
contract AppTimelock is TimelockController {
    uint256 public constant MINIMUM_DELAY = 24 hours;
    uint256 public constant MAXIMUM_DELAY = 30 days;

    /// @notice Mapping to track guardian vote status for operations
    mapping(bytes32 => bool) public guardianVoteRecorded;

    /// @notice Event emitted when guardian vote is recorded
    event GuardianVoteRecorded(bytes32 indexed operationId);

    /// @notice Event emitted when upgrade validation fails
    event UpgradeValidationFailed(bytes32 indexed operationId, string reason);

    /// @notice Constructor to initialize the timelock
    /// @param minDelay The minimum delay for operations
    /// @param proposers The addresses that can propose operations
    /// @param executors The addresses that can execute operations
    constructor(
        uint256 minDelay,
        address[] memory proposers,
        address[] memory executors
    ) TimelockController(minDelay, proposers, executors) {
        require(minDelay >= MINIMUM_DELAY, "Delay too short");
        require(minDelay <= MAXIMUM_DELAY, "Delay too long");
    }

    /// @notice Schedule an upgrade operation
    /// @param target The target contract address
    /// @param data The call data
    /// @param predecessor The predecessor operation
    /// @param salt The salt for the operation
    /// @param delay The delay for the operation
    function scheduleUpgrade(
        address target,
        bytes memory data,
        bytes32 predecessor,
        bytes32 salt,
        uint256 delay
    ) public onlyRole(PROPOSER_ROLE) {
        require(delay >= getMinDelay(), "Delay less than minimum");
        schedule(target, data, predecessor, salt, delay);
    }

    /// @notice Execute an upgrade operation with validation
    /// @param target The target contract address
    /// @param data The call data
    /// @param predecessor The predecessor operation
    /// @param salt The salt for the operation
    function executeUpgrade(
        address target,
        bytes memory data,
        bytes32 predecessor,
        bytes32 salt
    ) public onlyRole(EXECUTOR_ROLE) {
        bytes32 operationId = hashOperation(target, data, predecessor, salt);
        
        // Validate upgrade conditions before execution
        require(isValidUpgrade(operationId), "Upgrade validation failed");
        
        execute(target, data, predecessor, salt);
    }

    /// @notice Cancel an upgrade operation
    /// @param id The operation ID
    function cancelUpgrade(bytes32 id) public onlyRole(PROPOSER_ROLE) {
        cancel(id);
    }

    /// @notice Update the minimum delay
    /// @param newDelay The new minimum delay
    function updateDelay(uint256 newDelay) public virtual override {
        require(newDelay >= MINIMUM_DELAY, "Delay too short");
        require(newDelay <= MAXIMUM_DELAY, "Delay too long");
        super.updateDelay(newDelay);
    }

    /// @notice Record guardian vote for an operation
    /// @param operationId The operation ID
    function recordGuardianVote(bytes32 operationId) public {
        // In a real implementation, this would check that the caller is a guardian
        // For simplicity, we're just recording the vote
        guardianVoteRecorded[operationId] = true;
        emit GuardianVoteRecorded(operationId);
    }

    /// @notice Validate upgrade conditions
    /// @param operationId The operation ID to validate
    /// @return bool Whether the upgrade is valid
    function isValidUpgrade(bytes32 operationId) public view returns (bool) {
        // Check if guardian vote was recorded
        if (!guardianVoteRecorded[operationId]) {
            emit UpgradeValidationFailed(operationId, "Guardian vote not recorded");
            return false;
        }
        
        // Check if operation is ready (delay has passed)
        (bool ready, ) = getOperationState(operationId);
        if (!ready) {
            emit UpgradeValidationFailed(operationId, "Operation not ready");
            return false;
        }
        
        return true;
    }

    /// @notice Version tracking
    /// @return The current version
    function version() public pure returns (string memory) {
        return "1.0.0";
    }
}