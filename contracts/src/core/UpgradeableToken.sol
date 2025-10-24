// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts-upgradeable/token/ERC20/ERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/security/ReentrancyGuardUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/math/SafeMathUpgradeable.sol";

/// @title UpgradeableToken
/// @notice An upgradeable ERC20 token implementation using UUPS pattern
/// @dev Implements upgradeability with security controls and storage layout management
contract UpgradeableToken is ERC20Upgradeable, OwnableUpgradeable, UUPSUpgradeable, ReentrancyGuardUpgradeable {
    using SafeMathUpgradeable for uint256;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @notice Initialize the contract
    /// @param name The token name
    /// @param symbol The token symbol
    /// @param initialSupply The initial supply
    function initialize(
        string memory name,
        string memory symbol,
        uint256 initialSupply
    ) initializer public {
        __ERC20_init(name, symbol);
        __Ownable_init();
        __UUPSUpgradeable_init();
        __ReentrancyGuard_init();
        
        // Checks
        require(bytes(name).length > 0, "Name cannot be empty");
        require(bytes(symbol).length > 0, "Symbol cannot be empty");
        
        // Effects
        _mint(msg.sender, initialSupply);
    }

    /// @notice Set a new name for the token (upgradeable feature)
    /// @param newName The new name
    function setName(string memory newName) public onlyOwner {
        // Checks
        require(bytes(newName).length > 0, "Name cannot be empty");
        
        // Effects
        // In a real implementation, you would need a custom name storage
        // This is just a placeholder to demonstrate upgradeable features
    }

    /// @notice Burn tokens from the caller's balance
    /// @param amount The amount to burn
    function burn(uint256 amount) public nonReentrant {
        // Checks
        require(amount > 0, "Burn amount must be greater than zero");
        require(balanceOf(msg.sender) >= amount, "Insufficient balance");
        
        // Effects
        _burn(msg.sender, amount);
    }

    /// @notice Required function for UUPS upgradeability
    /// @param newImplementation The address of the new implementation
    function _authorizeUpgrade(address newImplementation)
        internal
        override
        onlyOwner
    {}

    /// @notice Version tracking for upgrade management
    /// @return The current version
    function version() public pure returns (string memory) {
        return "1.0.0";
    }

    /// @notice Storage layout validation function
    /// @dev This function helps verify storage layout during upgrades
    /// @return slot0 The value at storage slot 0
    /// @return slot1 The value at storage slot 1
    function getStorageLayout() public view returns (uint256 slot0, uint256 slot1) {
        // This is a simplified example for demonstration
        // In practice, you would check specific storage slots
        slot0 = 0;
        slot1 = 0;
    }
}