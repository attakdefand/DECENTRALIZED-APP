// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts-upgradeable/token/ERC20/ERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/security/ReentrancyGuardUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/security/PausableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/math/SafeMathUpgradeable.sol";

/// @title EnhancedUpgradeableToken
/// @notice An enhanced upgradeable ERC20 token implementation using UUPS pattern with additional security features
/// @dev Implements upgradeability with security controls, storage layout management, and immutable invariants
contract EnhancedUpgradeableToken is 
    ERC20Upgradeable, 
    OwnableUpgradeable, 
    UUPSUpgradeable, 
    ReentrancyGuardUpgradeable,
    PausableUpgradeable
{
    using SafeMathUpgradeable for uint256;

    // Storage layout version tracking
    uint256 private constant _STORAGE_LAYOUT_VERSION = 1;
    
    // Token configuration
    uint256 public constant MAX_SUPPLY = 1000000000 * 10**18; // 1 billion tokens
    uint256 public constant MAX_FEE = 1000; // 10% in basis points
    
    // Token state variables (carefully ordered for storage layout)
    uint256 public feePercentage;
    address public feeRecipient;
    bool public transferPaused;
    
    // Upgrade tracking
    string private _version;
    uint256 private _storageLayoutVersion;
    
    // Events
    event FeeChanged(uint256 oldFee, uint256 newFee);
    event FeeRecipientChanged(address oldRecipient, address newRecipient);
    event TransferPaused(bool paused);
    event VersionUpdated(string oldVersion, string newVersion);
    
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
        __Pausable_init();
        
        // Checks
        require(bytes(name).length > 0, "Name cannot be empty");
        require(bytes(symbol).length > 0, "Symbol cannot be empty");
        require(initialSupply <= MAX_SUPPLY, "Initial supply exceeds maximum");
        
        // Effects
        _mint(msg.sender, initialSupply);
        feePercentage = 0;
        feeRecipient = msg.sender;
        transferPaused = false;
        _version = "1.0.0";
        _storageLayoutVersion = _STORAGE_LAYOUT_VERSION;
    }

    /// @notice Set the fee percentage for transfers
    /// @param fee The new fee percentage in basis points (0-1000 for 0-10%)
    function setFee(uint256 fee) public onlyOwner {
        // Checks
        require(fee <= MAX_FEE, "Fee exceeds maximum allowed");
        
        // Effects
        uint256 oldFee = feePercentage;
        feePercentage = fee;
        
        // Interactions
        emit FeeChanged(oldFee, fee);
    }

    /// @notice Set the fee recipient address
    /// @param recipient The new fee recipient address
    function setFeeRecipient(address recipient) public onlyOwner {
        // Checks
        require(recipient != address(0), "Recipient cannot be zero address");
        
        // Effects
        address oldRecipient = feeRecipient;
        feeRecipient = recipient;
        
        // Interactions
        emit FeeRecipientChanged(oldRecipient, recipient);
    }

    /// @notice Pause or unpause transfers
    /// @param paused Whether to pause transfers
    function setTransferPaused(bool paused) public onlyOwner {
        transferPaused = paused;
        emit TransferPaused(paused);
    }

    /// @notice Transfer tokens with fee deduction
    /// @param to The recipient address
    /// @param amount The amount to transfer
    /// @return bool Success status
    function transfer(address to, uint256 amount) public override whenNotPaused nonReentrant returns (bool) {
        // Checks
        require(!transferPaused, "Transfers are paused");
        require(to != address(0), "Transfer to zero address");
        require(amount > 0, "Transfer amount must be greater than zero");
        require(balanceOf(msg.sender) >= amount, "Insufficient balance");
        
        // Calculate fee
        uint256 fee = calculateFee(amount);
        uint256 netAmount = amount - fee;
        
        // Effects - Follow CEI pattern
        _transfer(msg.sender, to, netAmount);
        if (fee > 0) {
            _transfer(msg.sender, feeRecipient, fee);
        }
        
        // Interactions - Events are emitted by _transfer
        return true;
    }

    /// @notice Transfer tokens from one address to another with fee deduction
    /// @param from The sender address
    /// @param to The recipient address
    /// @param amount The amount to transfer
    /// @return bool Success status
    function transferFrom(address from, address to, uint256 amount) public override whenNotPaused nonReentrant returns (bool) {
        // Checks
        require(!transferPaused, "Transfers are paused");
        require(from != address(0), "Transfer from zero address");
        require(to != address(0), "Transfer to zero address");
        require(amount > 0, "Transfer amount must be greater than zero");
        require(balanceOf(from) >= amount, "Insufficient balance");
        require(allowance(from, msg.sender) >= amount, "Insufficient allowance");
        
        // Calculate fee
        uint256 fee = calculateFee(amount);
        uint256 netAmount = amount - fee;
        
        // Effects - Follow CEI pattern
        _transfer(from, to, netAmount);
        _approve(from, msg.sender, allowance(from, msg.sender) - amount);
        if (fee > 0) {
            _transfer(from, feeRecipient, fee);
        }
        
        // Interactions - Events are emitted by _transfer
        return true;
    }

    /// @notice Burn tokens from the caller's balance
    /// @param amount The amount to burn
    function burn(uint256 amount) public nonReentrant {
        // Checks
        require(amount > 0, "Burn amount must be greater than zero");
        require(balanceOf(msg.sender) >= amount, "Insufficient balance");
        
        // Effects - Follow CEI pattern
        _burn(msg.sender, amount);
    }

    /// @notice Calculate fee for a transfer amount
    /// @param amount The transfer amount
    /// @return uint256 The calculated fee
    function calculateFee(uint256 amount) public view returns (uint256) {
        // Mathematical invariant: fee calculation must be safe
        if (feePercentage == 0) {
            return 0;
        }
        
        // Safe calculation with bounds checking
        uint256 fee = (amount * feePercentage) / 10000;
        require(fee < amount, "Fee cannot exceed transfer amount");
        return fee;
    }

    /// @notice Get the total supply invariant check
    /// @return bool Whether the total supply invariant holds
    function checkTotalSupplyInvariant() public view returns (bool) {
        // The total supply should never exceed MAX_SUPPLY
        return totalSupply() <= MAX_SUPPLY;
    }

    /// @notice Get the fee invariant check
    /// @return bool Whether the fee invariant holds
    function checkFeeInvariant() public view returns (bool) {
        // The fee should never exceed MAX_FEE
        return feePercentage <= MAX_FEE;
    }

    /// @notice Required function for UUPS upgradeability
    /// @param newImplementation The address of the new implementation
    function _authorizeUpgrade(address newImplementation)
        internal
        override
        onlyOwner
    {
        // Additional validation can be added here
        // For example, checking that the new implementation is valid
        // or that it maintains certain invariants
    }

    /// @notice Version tracking for upgrade management
    /// @return The current version
    function version() public view returns (string memory) {
        return _version;
    }

    /// @notice Update the contract version
    /// @param newVersion The new version string
    function updateVersion(string memory newVersion) public onlyOwner {
        string memory oldVersion = _version;
        _version = newVersion;
        emit VersionUpdated(oldVersion, newVersion);
    }

    /// @notice Get the storage layout version
    /// @return The storage layout version
    function storageLayoutVersion() public view returns (uint256) {
        return _storageLayoutVersion;
    }

    /// @notice Storage layout validation function
    /// @dev This function helps verify storage layout during upgrades
    /// @return slot0 The value at storage slot 0
    /// @return slot1 The value at storage slot 1
    /// @return slot2 The value at storage slot 2
    function getStorageLayout() public view returns (uint256 slot0, uint256 slot1, uint256 slot2) {
        // This is a simplified example for demonstration
        // In practice, you would check specific storage slots that are critical
        assembly {
            slot0 := sload(0)
            slot1 := sload(1)
            slot2 := sload(2)
        }
    }

    /// @notice Check if all invariants are preserved
    /// @return bool Whether all invariants hold
    function checkAllInvariants() public view returns (bool) {
        return checkTotalSupplyInvariant() && checkFeeInvariant();
    }

    /// @notice Emergency pause function
    function emergencyPause() public onlyOwner {
        _pause();
    }

    /// @notice Emergency unpause function
    function emergencyUnpause() public onlyOwner {
        _unpause();
    }
}