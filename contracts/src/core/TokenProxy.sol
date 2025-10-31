// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

/// @title TokenProxy
/// @notice A minimal proxy contract for the upgradeable token
/// @dev Implements ERC1967 proxy standard for upgradeable contracts
contract TokenProxy is ERC1967Proxy {
    /// @notice Constructor to initialize the proxy
    /// @param _logic The address of the implementation contract
    /// @param _data The initialization data
    constructor(address _logic, bytes memory _data) payable ERC1967Proxy(_logic, _data) {}

    /// @notice Get the current implementation address
    /// @return The implementation address
    function implementation() public view returns (address) {
        return ERC1967Utils.getImplementation();
    }

    /// @notice Get the admin address
    /// @return The admin address
    function admin() public view returns (address) {
        return ERC1967Utils.getAdmin();
    }

    /// @notice Upgrade to a new implementation
    /// @param newImplementation The new implementation address
    function upgradeTo(address newImplementation) public {
        require(msg.sender == admin(), "Only admin can upgrade");
        ERC1967Utils.upgradeToAndCall(newImplementation, "");
    }

    /// @notice Upgrade to a new implementation and call a function
    /// @param newImplementation The new implementation address
    /// @param data The call data
    function upgradeToAndCall(address newImplementation, bytes memory data) public payable {
        require(msg.sender == admin(), "Only admin can upgrade");
        ERC1967Utils.upgradeToAndCall(newImplementation, data);
    }
}