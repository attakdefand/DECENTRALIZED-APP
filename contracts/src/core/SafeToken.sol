// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/math/SafeMath.sol";

/// @title SafeToken
/// @notice A sample ERC20 token implementation with comprehensive safety measures
/// @dev Implements CEI pattern, input bounds validation, reentrancy protection, and mathematical invariants
contract SafeToken is ERC20, Ownable, ReentrancyGuard {
    using SafeMath for uint256;

    // State variables with explicit bounds
    uint256 public constant MAX_SUPPLY = 1000000000 * 10**18; // 1 billion tokens
    uint256 public constant MAX_FEE = 1000; // 10% in basis points
    uint256 public feePercentage;
    address public feeRecipient;
    
    // Events for transparency
    event FeeChanged(uint256 oldFee, uint256 newFee);
    event FeeRecipientChanged(address oldRecipient, address newRecipient);
    event TokensBurned(address indexed from, uint256 amount);
    
    /// @notice Constructor to initialize the token
    /// @param name The name of the token
    /// @param symbol The symbol of the token
    /// @param initialSupply The initial supply of tokens
    constructor(
        string memory name,
        string memory symbol,
        uint256 initialSupply
    ) ERC20(name, symbol) {
        // Checks
        require(bytes(name).length > 0, "Name cannot be empty");
        require(bytes(symbol).length > 0, "Symbol cannot be empty");
        require(initialSupply <= MAX_SUPPLY, "Initial supply exceeds maximum");
        
        // Effects
        _mint(msg.sender, initialSupply);
        feePercentage = 0;
        feeRecipient = msg.sender;
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
    
    /// @notice Transfer tokens with fee deduction
    /// @param to The recipient address
    /// @param amount The amount to transfer
    /// @return bool Success status
    function transfer(address to, uint256 amount) public override nonReentrant returns (bool) {
        // Checks
        require(to != address(0), "Transfer to zero address");
        require(amount > 0, "Transfer amount must be greater than zero");
        require(balanceOf(msg.sender) >= amount, "Insufficient balance");
        
        // Calculate fee
        uint256 fee = calculateFee(amount);
        uint256 netAmount = amount.sub(fee);
        
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
    function transferFrom(address from, address to, uint256 amount) public override nonReentrant returns (bool) {
        // Checks
        require(from != address(0), "Transfer from zero address");
        require(to != address(0), "Transfer to zero address");
        require(amount > 0, "Transfer amount must be greater than zero");
        require(balanceOf(from) >= amount, "Insufficient balance");
        require(allowance(from, msg.sender) >= amount, "Insufficient allowance");
        
        // Calculate fee
        uint256 fee = calculateFee(amount);
        uint256 netAmount = amount.sub(fee);
        
        // Effects - Follow CEI pattern
        _transfer(from, to, netAmount);
        _approve(from, msg.sender, allowance(from, msg.sender).sub(amount));
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
        
        // Interactions
        emit TokensBurned(msg.sender, amount);
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
        uint256 fee = amount.mul(feePercentage).div(10000);
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
}