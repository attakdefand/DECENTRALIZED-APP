// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "./EventSchema.sol";

/// @title EventSchemaToken
/// @notice An ERC20 token implementation with comprehensive event schema integration
/// @dev Implements standardized event schemas with consistent naming, indexed parameters, and comprehensive logging
contract EventSchemaToken is ERC20, Ownable, ReentrancyGuard {
    EventSchema public eventSchema;
    uint256 public constant MAX_SUPPLY = 1000000000 * 10**18; // 1 billion tokens
    uint256 public constant MAX_FEE = 1000; // 10% in basis points
    uint256 public feePercentage;
    address public feeRecipient;
    
    /// @notice Constructor to initialize the token
    /// @param name The name of the token
    /// @param symbol The symbol of the token
    /// @param initialSupply The initial supply of tokens
    /// @param _eventSchema The event schema contract address
    constructor(
        string memory name,
        string memory symbol,
        uint256 initialSupply,
        address _eventSchema
    ) ERC20(name, symbol) {
        require(_eventSchema != address(0), "Event schema cannot be zero address");
        eventSchema = EventSchema(_eventSchema);
        
        // Checks
        require(bytes(name).length > 0, "Name cannot be empty");
        require(bytes(symbol).length > 0, "Symbol cannot be empty");
        require(initialSupply <= MAX_SUPPLY, "Initial supply exceeds maximum");
        
        // Effects
        _mint(msg.sender, initialSupply);
        feePercentage = 0;
        feeRecipient = msg.sender;
        
        // Emit deployment event
        eventSchema.emitContractDeployed(
            address(this),
            "EventSchemaToken",
            msg.sender,
            keccak256(abi.encodePacked("1.0.0"))
        );
    }
    
    /// @notice Set the fee percentage for transfers
    /// @param fee The new fee percentage in basis points (0-1000 for 0-10%)
    function setFee(uint256 fee) public onlyOwner {
        // Checks
        require(fee <= MAX_FEE, "Fee exceeds maximum allowed");
        
        // Effects
        uint256 oldFee = feePercentage;
        feePercentage = fee;
        
        // Emit state change event
        eventSchema.emitStateChanged(
            "feePercentage",
            address(this),
            keccak256(abi.encodePacked("setFee", block.timestamp)),
            abi.encode(oldFee),
            abi.encode(fee),
            block.timestamp
        );
        
        // Emit user action event
        eventSchema.emitUserAction(
            "setFee",
            address(this),
            abi.encode(fee)
        );
    }
    
    /// @notice Set the fee recipient address
    /// @param recipient The new fee recipient address
    function setFeeRecipient(address recipient) public onlyOwner {
        // Checks
        require(recipient != address(0), "Recipient cannot be zero address");
        
        // Effects
        address oldRecipient = feeRecipient;
        feeRecipient = recipient;
        
        // Emit state change event
        eventSchema.emitStateChanged(
            "feeRecipient",
            address(this),
            keccak256(abi.encodePacked("setFeeRecipient", block.timestamp)),
            abi.encode(oldRecipient),
            abi.encode(recipient),
            block.timestamp
        );
        
        // Emit user action event
        eventSchema.emitUserAction(
            "setFeeRecipient",
            address(this),
            abi.encode(recipient)
        );
    }
    
    /// @notice Transfer tokens with fee deduction
    /// @param to The recipient address
    /// @param amount The amount to transfer
    /// @return bool Success status
    function transfer(address to, uint256 amount) public override nonReentrant returns (bool) {
        // Record gas at start
        uint256 gasStart = gasleft();
        
        // Checks
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
        
        // Record gas used
        uint256 gasUsed = gasStart - gasleft();
        
        // Emit transfer event
        eventSchema.emitTransfer(
            msg.sender,
            to,
            address(this),
            amount
        );
        
        // Emit function called event
        eventSchema.emitFunctionCalled(
            this.transfer.selector,
            "transfer",
            address(this),
            gasUsed
        );
        
        // Emit performance metric
        eventSchema.emitPerformanceMetric(
            "transferGasUsed",
            address(this),
            gasUsed
        );
        
        // Interactions - Events are emitted by _transfer
        return true;
    }
    
    /// @notice Transfer tokens from one address to another with fee deduction
    /// @param from The sender address
    /// @param to The recipient address
    /// @param amount The amount to transfer
    /// @return bool Success status
    function transferFrom(address from, address to, uint256 amount) public override nonReentrant returns (bool) {
        // Record gas at start
        uint256 gasStart = gasleft();
        
        // Checks
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
        
        // Record gas used
        uint256 gasUsed = gasStart - gasleft();
        
        // Emit transfer event
        eventSchema.emitTransfer(
            from,
            to,
            address(this),
            amount
        );
        
        // Emit function called event
        eventSchema.emitFunctionCalled(
            this.transferFrom.selector,
            "transferFrom",
            address(this),
            gasUsed
        );
        
        // Emit performance metric
        eventSchema.emitPerformanceMetric(
            "transferFromGasUsed",
            address(this),
            gasUsed
        );
        
        // Interactions - Events are emitted by _transfer
        return true;
    }
    
    /// @notice Approve spending allowance
    /// @param spender The spender address
    /// @param amount The amount to approve
    /// @return bool Success status
    function approve(address spender, uint256 amount) public override returns (bool) {
        bool result = super.approve(spender, amount);
        
        // Emit approval event
        eventSchema.emitApproval(
            msg.sender,
            spender,
            address(this),
            amount
        );
        
        // Emit user action event
        eventSchema.emitUserAction(
            "approve",
            address(this),
            abi.encode(spender, amount)
        );
        
        return result;
    }
    
    /// @notice Increase spending allowance
    /// @param spender The spender address
    /// @param addedValue The amount to add to the allowance
    /// @return bool Success status
    function increaseAllowance(address spender, uint256 addedValue) public override returns (bool) {
        bool result = super.increaseAllowance(spender, addedValue);
        
        // Emit approval event
        eventSchema.emitApproval(
            msg.sender,
            spender,
            address(this),
            allowance(msg.sender, spender)
        );
        
        // Emit user action event
        eventSchema.emitUserAction(
            "increaseAllowance",
            address(this),
            abi.encode(spender, addedValue)
        );
        
        return result;
    }
    
    /// @notice Decrease spending allowance
    /// @param spender The spender address
    /// @param subtractedValue The amount to subtract from the allowance
    /// @return bool Success status
    function decreaseAllowance(address spender, uint256 subtractedValue) public override returns (bool) {
        bool result = super.decreaseAllowance(spender, subtractedValue);
        
        // Emit approval event
        eventSchema.emitApproval(
            msg.sender,
            spender,
            address(this),
            allowance(msg.sender, spender)
        );
        
        // Emit user action event
        eventSchema.emitUserAction(
            "decreaseAllowance",
            address(this),
            abi.encode(spender, subtractedValue)
        );
        
        return result;
    }
    
    /// @notice Burn tokens from the caller's balance
    /// @param amount The amount to burn
    function burn(uint256 amount) public nonReentrant {
        // Record gas at start
        uint256 gasStart = gasleft();
        
        // Checks
        require(amount > 0, "Burn amount must be greater than zero");
        require(balanceOf(msg.sender) >= amount, "Insufficient balance");
        
        // Effects - Follow CEI pattern
        _burn(msg.sender, amount);
        
        // Record gas used
        uint256 gasUsed = gasStart - gasleft();
        
        // Emit user action event
        eventSchema.emitUserAction(
            "burn",
            address(this),
            abi.encode(amount)
        );
        
        // Emit function called event
        eventSchema.emitFunctionCalled(
            this.burn.selector,
            "burn",
            address(this),
            gasUsed
        );
        
        // Emit performance metric
        eventSchema.emitPerformanceMetric(
            "burnGasUsed",
            address(this),
            gasUsed
        );
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
    
    /// @notice Check all invariants
    /// @return bool Whether all invariants hold
    function checkAllInvariants() public view returns (bool) {
        return checkTotalSupplyInvariant() && checkFeeInvariant();
    }
}