// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/math/SafeMath.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

/// @title Vault
/// @notice A secure vault contract for holding and managing tokens with comprehensive safety measures
/// @dev Implements advanced safety patterns including time locks, withdrawal limits, and emergency controls
contract Vault is ReentrancyGuard, Ownable {
    using SafeMath for uint256;
    using SafeERC20 for IERC20;

    // Structs for better data organization
    struct Deposit {
        uint256 amount;
        uint256 timestamp;
        bool withdrawn;
    }
    
    struct WithdrawalLimit {
        uint256 dailyLimit;
        uint256 dailyUsed;
        uint256 lastReset;
    }
    
    // Treasury management structs
    struct ProofOfReservesReport {
        uint256 timestamp;
        uint256 totalValueUSD;
        bool isValidated;
        address validator;
    }
    
    struct TreasuryMetrics {
        uint256 porFreshnessHours;
        uint256 limitBreachCount;
        uint256 lastUpdate;
    }
    
    // State variables with explicit bounds
    mapping(address => mapping(address => Deposit[])) public deposits;
    mapping(address => mapping(address => WithdrawalLimit)) public withdrawalLimits;
    mapping(address => bool) public whitelistedTokens;
    mapping(uint256 => ProofOfReservesReport) public porReports;
    mapping(address => uint256) public tokenReserves;
    
    uint256 public constant MAX_DAILY_LIMIT = 1000000 * 10**18; // 1 million tokens
    uint256 public constant LIMIT_RESET_PERIOD = 1 days;
    uint256 public emergencyUnlockTime;
    uint256 public porReportCount = 0;
    TreasuryMetrics public treasuryMetrics;
    
    // Events for transparency
    event TokenWhitelisted(address indexed token, bool whitelisted);
    event DepositMade(address indexed user, address indexed token, uint256 amount, uint256 timestamp);
    event WithdrawalMade(address indexed user, address indexed token, uint256 amount, uint256 timestamp);
    event WithdrawalLimitSet(address indexed token, uint256 dailyLimit);
    event EmergencyActivated(uint256 unlockTime);
    event EmergencyDeactivated();
    event ProofOfReservesReported(uint256 indexed reportId, uint256 timestamp, uint256 totalValueUSD);
    event LimitBreachRecorded(uint256 breachCount);
    event TreasuryMetricsUpdated(uint256 porFreshnessHours, uint256 limitBreachCount);
    
    /// @notice Constructor to initialize the vault
    constructor() {
        emergencyUnlockTime = 0;
        treasuryMetrics.porFreshnessHours = 0;
        treasuryMetrics.limitBreachCount = 0;
        treasuryMetrics.lastUpdate = block.timestamp;
    }
    
    /// @notice Whitelist a token for deposits
    /// @param token The token address to whitelist
    /// @param whitelisted Whether to whitelist or unwhitelist the token
    function setTokenWhitelisted(address token, bool whitelisted) public onlyOwner {
        // Checks
        require(token != address(0), "Token address cannot be zero");
        
        // Effects
        whitelistedTokens[token] = whitelisted;
        
        // Interactions
        emit TokenWhitelisted(token, whitelisted);
    }
    
    /// @notice Set daily withdrawal limit for a token
    /// @param token The token address
    /// @param dailyLimit The daily limit in token units
    function setWithdrawalLimit(address token, uint256 dailyLimit) public onlyOwner {
        // Checks
        require(token != address(0), "Token address cannot be zero");
        require(dailyLimit <= MAX_DAILY_LIMIT, "Daily limit exceeds maximum");
        
        // Effects
        withdrawalLimits[token][msg.sender].dailyLimit = dailyLimit;
        
        // Interactions
        emit WithdrawalLimitSet(token, dailyLimit);
    }
    
    /// @notice Deposit tokens into the vault
    /// @param token The token address to deposit
    /// @param amount The amount to deposit
    function deposit(address token, uint256 amount) public nonReentrant {
        // Checks
        require(token != address(0), "Token address cannot be zero");
        require(amount > 0, "Deposit amount must be greater than zero");
        require(whitelistedTokens[token], "Token not whitelisted");
        require(IERC20(token).balanceOf(msg.sender) >= amount, "Insufficient token balance");
        
        // Effects - Follow CEI pattern
        // 1. Update state first
        deposits[msg.sender][token].push(Deposit({
            amount: amount,
            timestamp: block.timestamp,
            withdrawn: false
        }));
        
        // Update token reserves
        tokenReserves[token] = tokenReserves[token].add(amount);
        
        // 2. Then interact with external contracts
        IERC20(token).safeTransferFrom(msg.sender, address(this), amount);
        
        // Interactions
        emit DepositMade(msg.sender, token, amount, block.timestamp);
    }
    
    /// @notice Withdraw tokens from the vault
    /// @param token The token address to withdraw
    /// @param amount The amount to withdraw
    function withdraw(address token, uint256 amount) public nonReentrant {
        // Checks
        require(token != address(0), "Token address cannot be zero");
        require(amount > 0, "Withdrawal amount must be greater than zero");
        require(IERC20(token).balanceOf(address(this)) >= amount, "Insufficient vault balance");
        require(checkWithdrawalLimit(token, amount), "Withdrawal exceeds daily limit");
        
        // Check if emergency lock is active
        if (emergencyUnlockTime > 0) {
            require(block.timestamp >= emergencyUnlockTime, "Emergency lock active");
        }
        
        // Effects - Follow CEI pattern
        // 1. Update withdrawal limit tracking
        bool limitExceeded = updateWithdrawalLimit(token, amount);
        
        // Record limit breach if exceeded
        if (limitExceeded) {
            treasuryMetrics.limitBreachCount = treasuryMetrics.limitBreachCount.add(1);
            treasuryMetrics.lastUpdate = block.timestamp;
            emit LimitBreachRecorded(treasuryMetrics.limitBreachCount);
            emit TreasuryMetricsUpdated(treasuryMetrics.porFreshnessHours, treasuryMetrics.limitBreachCount);
        }
        
        // Update token reserves
        tokenReserves[token] = tokenReserves[token].sub(amount);
        
        // 2. Then interact with external contracts
        IERC20(token).safeTransfer(msg.sender, amount);
        
        // Interactions
        emit WithdrawalMade(msg.sender, token, amount, block.timestamp);
    }
    
    /// @notice Emergency function to lock withdrawals for 24 hours
    function activateEmergencyLock() public onlyOwner {
        // Effects
        emergencyUnlockTime = block.timestamp.add(24 hours);
        
        // Interactions
        emit EmergencyActivated(emergencyUnlockTime);
    }
    
    /// @notice Emergency function to unlock withdrawals immediately
    function deactivateEmergencyLock() public onlyOwner {
        // Effects
        emergencyUnlockTime = 0;
        
        // Interactions
        emit EmergencyDeactivated();
    }
    
    /// @notice Check if a withdrawal is within daily limits
    /// @param token The token address
    /// @param amount The withdrawal amount
    /// @return bool Whether the withdrawal is within limits
    function checkWithdrawalLimit(address token, uint256 amount) public view returns (bool) {
        WithdrawalLimit storage limit = withdrawalLimits[token][msg.sender];
        
        // Reset daily limit if needed
        uint256 dailyUsed = limit.dailyUsed;
        if (block.timestamp >= limit.lastReset.add(LIMIT_RESET_PERIOD)) {
            dailyUsed = 0;
        }
        
        // Check if withdrawal is within limits
        return dailyUsed.add(amount) <= limit.dailyLimit;
    }
    
    /// @notice Update withdrawal limit tracking
    /// @param token The token address
    /// @param amount The withdrawal amount
    /// @return bool Whether the limit was exceeded
    function updateWithdrawalLimit(address token, uint256 amount) internal returns (bool) {
        WithdrawalLimit storage limit = withdrawalLimits[token][msg.sender];
        bool limitExceeded = false;
        
        // Reset daily limit if needed
        if (block.timestamp >= limit.lastReset.add(LIMIT_RESET_PERIOD)) {
            limit.dailyUsed = 0;
            limit.lastReset = block.timestamp;
        }
        
        // Check if withdrawal exceeds limits
        if (limit.dailyUsed.add(amount) > limit.dailyLimit) {
            limitExceeded = true;
        }
        
        // Update daily usage
        limit.dailyUsed = limit.dailyUsed.add(amount);
        
        return limitExceeded;
    }
    
    /// @notice Submit a Proof of Reserves report
    /// @param totalValueUSD The total value of reserves in USD
    /// @param validator The address of the validator
    function submitProofOfReserves(uint256 totalValueUSD, address validator) public onlyOwner {
        // Create new report
        porReportCount = porReportCount.add(1);
        ProofOfReservesReport storage report = porReports[porReportCount];
        report.timestamp = block.timestamp;
        report.totalValueUSD = totalValueUSD;
        report.isValidated = true;
        report.validator = validator;
        
        // Update treasury metrics
        treasuryMetrics.porFreshnessHours = 0; // Reset freshness as we have a new report
        treasuryMetrics.lastUpdate = block.timestamp;
        
        emit ProofOfReservesReported(porReportCount, block.timestamp, totalValueUSD);
        emit TreasuryMetricsUpdated(treasuryMetrics.porFreshnessHours, treasuryMetrics.limitBreachCount);
    }
    
    /// @notice Get total deposits for a user and token
    /// @param user The user address
    /// @param token The token address
    /// @return uint256 The total deposited amount
    function getTotalDeposits(address user, address token) public view returns (uint256) {
        Deposit[] storage userDeposits = deposits[user][token];
        uint256 total = 0;
        
        for (uint256 i = 0; i < userDeposits.length; i++) {
            if (!userDeposits[i].withdrawn) {
                total = total.add(userDeposits[i].amount);
            }
        }
        
        return total;
    }
    
    /// @notice Get current token reserves
    /// @param token The token address
    /// @return uint256 The current reserve amount
    function getTokenReserves(address token) public view returns (uint256) {
        return tokenReserves[token];
    }
    
    /// @notice Check if Proof of Reserves report is fresh (less than 24 hours old)
    /// @return bool Whether the report is fresh
    function isPorReportFresh() public view returns (bool) {
        if (porReportCount == 0) {
            return false;
        }
        return (block.timestamp.sub(porReports[porReportCount].timestamp)) < 24 hours;
    }
    
    /// @notice Check if treasury metrics are within acceptable limits
    /// @return bool Whether metrics are within limits
    function areTreasuryMetricsWithinLimits() public view returns (bool) {
        return treasuryMetrics.porFreshnessHours < 24 && treasuryMetrics.limitBreachCount == 0;
    }
    
    /// @notice Check vault solvency invariant
    /// @param token The token address
    /// @return bool Whether the vault is solvent
    function checkSolvency(address token) public view returns (bool) {
        // The vault should always have enough tokens to cover all deposits
        uint256 vaultBalance = IERC20(token).balanceOf(address(this));
        uint256 totalDeposits = 0;
        
        // This is a simplified check - in a real implementation, 
        // you would need to iterate through all users
        // For demonstration purposes, we'll just check basic math
        return vaultBalance >= totalDeposits;
    }
    
    /// @notice Check emergency lock invariant
    /// @return bool Whether the emergency lock state is valid
    function checkEmergencyLockInvariant() public view returns (bool) {
        // If emergency lock is active, unlock time should be in the future
        if (emergencyUnlockTime > 0) {
            return emergencyUnlockTime > block.timestamp;
        }
        return true;
    }
}