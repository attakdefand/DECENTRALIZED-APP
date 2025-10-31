// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/governance/TimelockController.sol";

/// @title AppTimelock
/// @notice A timelock controller for secure contract upgrades
/// @dev Implements OpenZeppelin's TimelockController with custom configuration
contract AppTimelock is TimelockController {
    uint256 public constant MINIMUM_DELAY = 24 hours;
    uint256 public constant MAXIMUM_DELAY = 30 days;

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

    /// @notice Execute an upgrade operation
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

    /// @notice Version tracking
    /// @return The current version
    function version() public pure returns (string memory) {
        return "1.0.0";
    }
}