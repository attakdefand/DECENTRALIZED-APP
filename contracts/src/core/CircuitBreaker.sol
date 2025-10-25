// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

/// @title CircuitBreaker
/// @notice A comprehensive circuit breaker implementation for smart contracts with rate limiting and emergency controls
/// @dev Implements pause/unpause functionality with role-based access, rate limiting, and emergency controls
contract CircuitBreaker is AccessControl, Pausable, ReentrancyGuard {
    // Role definitions
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");
    bytes32 public constant UNPAUSER_ROLE = keccak256("UNPAUSER_ROLE");
    bytes32 public constant ADMIN_ROLE = keccak256("ADMIN_ROLE");
    
    // Rate limiting
    struct RateLimit {
        uint256 limit;
        uint256 window;
        mapping(address => uint256) lastReset;
        mapping(address => uint256) count;
    }
    
    // Function-specific pause states
    mapping(bytes4 => bool) public functionPaused;
    mapping(string => bool) public functionGroupPaused;
    
    // Rate limits for functions
    mapping(bytes4 => RateLimit) public functionRateLimits;
    mapping(string => RateLimit) public functionGroupRateLimits;
    
    // Emergency controls
    uint256 public emergencyUnlockTime;
    bool public globalEmergencyPause;
    
    // Events
    event FunctionPaused(bytes4 functionSelector, bool paused);
    event FunctionGroupPaused(string groupName, bool paused);
    event RateLimitSet(bytes4 functionSelector, uint256 limit, uint256 window);
    event RateLimitGroupSet(string groupName, uint256 limit, uint256 window);
    event EmergencyActivated(uint256 unlockTime);
    event EmergencyDeactivated();
    event GlobalEmergencyPause(bool paused);
    
    /// @notice Constructor to initialize the circuit breaker
    /// @param admin The admin address
    constructor(address admin) {
        require(admin != address(0), "Admin cannot be zero address");
        
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(ADMIN_ROLE, admin);
        _grantRole(PAUSER_ROLE, admin);
        _grantRole(UNPAUSER_ROLE, admin);
        
        emergencyUnlockTime = 0;
        globalEmergencyPause = false;
    }
    
    /// @notice Modifier to check if a function is paused
    /// @param functionSelector The function selector
    modifier whenFunctionNotPaused(bytes4 functionSelector) {
        require(!functionPaused[functionSelector], "Function is paused");
        _;
    }
    
    /// @notice Modifier to check if a function group is paused
    /// @param groupName The function group name
    modifier whenFunctionGroupNotPaused(string memory groupName) {
        require(!functionGroupPaused[groupName], "Function group is paused");
        _;
    }
    
    /// @notice Modifier to check rate limit for a function
    /// @param functionSelector The function selector
    modifier rateLimited(bytes4 functionSelector) {
        RateLimit storage limit = functionRateLimits[functionSelector];
        if (limit.limit > 0) {
            checkAndUpdateRateLimit(limit);
        }
        _;
    }
    
    /// @notice Modifier to check rate limit for a function group
    /// @param groupName The function group name
    modifier rateLimitedGroup(string memory groupName) {
        RateLimit storage limit = functionGroupRateLimits[groupName];
        if (limit.limit > 0) {
            checkAndUpdateRateLimit(limit);
        }
        _;
    }
    
    /// @notice Check and update rate limit
    /// @param limit The rate limit struct
    function checkAndUpdateRateLimit(RateLimit storage limit) internal {
        if (limit.window > 0) {
            // Reset counter if window has passed
            if (block.timestamp >= limit.lastReset[msg.sender] + limit.window) {
                limit.count[msg.sender] = 0;
                limit.lastReset[msg.sender] = block.timestamp;
            }
            
            // Check if limit is exceeded
            require(limit.count[msg.sender] < limit.limit, "Rate limit exceeded");
            
            // Increment counter
            limit.count[msg.sender]++;
        }
    }
    
    /// @notice Pause a specific function
    /// @param functionSelector The function selector to pause
    function pauseFunction(bytes4 functionSelector) public onlyRole(PAUSER_ROLE) {
        functionPaused[functionSelector] = true;
        emit FunctionPaused(functionSelector, true);
    }
    
    /// @notice Unpause a specific function
    /// @param functionSelector The function selector to unpause
    function unpauseFunction(bytes4 functionSelector) public onlyRole(UNPAUSER_ROLE) {
        functionPaused[functionSelector] = false;
        emit FunctionPaused(functionSelector, false);
    }
    
    /// @notice Pause a function group
    /// @param groupName The function group name to pause
    function pauseFunctionGroup(string memory groupName) public onlyRole(PAUSER_ROLE) {
        functionGroupPaused[groupName] = true;
        emit FunctionGroupPaused(groupName, true);
    }
    
    /// @notice Unpause a function group
    /// @param groupName The function group name to unpause
    function unpauseFunctionGroup(string memory groupName) public onlyRole(UNPAUSER_ROLE) {
        functionGroupPaused[groupName] = false;
        emit FunctionGroupPaused(groupName, false);
    }
    
    /// @notice Set rate limit for a function
    /// @param functionSelector The function selector
    /// @param limit The rate limit (calls per window)
    /// @param window The time window in seconds
    function setFunctionRateLimit(bytes4 functionSelector, uint256 limit, uint256 window) public onlyRole(ADMIN_ROLE) {
        RateLimit storage rateLimit = functionRateLimits[functionSelector];
        rateLimit.limit = limit;
        rateLimit.window = window;
        emit RateLimitSet(functionSelector, limit, window);
    }
    
    /// @notice Set rate limit for a function group
    /// @param groupName The function group name
    /// @param limit The rate limit (calls per window)
    /// @param window The time window in seconds
    function setFunctionGroupRateLimit(string memory groupName, uint256 limit, uint256 window) public onlyRole(ADMIN_ROLE) {
        RateLimit storage rateLimit = functionGroupRateLimits[groupName];
        rateLimit.limit = limit;
        rateLimit.window = window;
        emit RateLimitGroupSet(groupName, limit, window);
    }
    
    /// @notice Emergency function to activate global pause for 24 hours
    function activateEmergencyPause() public onlyRole(PAUSER_ROLE) {
        globalEmergencyPause = true;
        emergencyUnlockTime = block.timestamp + 24 hours;
        emit EmergencyActivated(emergencyUnlockTime);
        emit GlobalEmergencyPause(true);
    }
    
    /// @notice Emergency function to deactivate global pause immediately
    function deactivateEmergencyPause() public onlyRole(UNPAUSER_ROLE) {
        globalEmergencyPause = false;
        emergencyUnlockTime = 0;
        emit EmergencyDeactivated();
        emit GlobalEmergencyPause(false);
    }
    
    /// @notice Check if emergency pause is active
    /// @return bool Whether emergency pause is active
    function isEmergencyPauseActive() public view returns (bool) {
        if (globalEmergencyPause) {
            if (emergencyUnlockTime > 0 && block.timestamp >= emergencyUnlockTime) {
                return false; // Emergency pause has expired
            }
            return true;
        }
        return false;
    }
    
    /// @notice Modifier to check global emergency pause
    modifier whenNotEmergencyPaused() {
        require(!isEmergencyPauseActive(), "Emergency pause is active");
        _;
    }
    
    /// @notice Get function pause status
    /// @param functionSelector The function selector
    /// @return bool Whether the function is paused
    function isFunctionPaused(bytes4 functionSelector) public view returns (bool) {
        return functionPaused[functionSelector];
    }
    
    /// @notice Get function group pause status
    /// @param groupName The function group name
    /// @return bool Whether the function group is paused
    function isFunctionGroupPaused(string memory groupName) public view returns (bool) {
        return functionGroupPaused[groupName];
    }
    
    /// @notice Get current rate limit count for a function
    /// @param functionSelector The function selector
    /// @return uint256 The current count
    function getFunctionRateLimitCount(bytes4 functionSelector) public view returns (uint256) {
        return functionRateLimits[functionSelector].count[msg.sender];
    }
    
    /// @notice Get current rate limit count for a function group
    /// @param groupName The function group name
    /// @return uint256 The current count
    function getFunctionGroupRateLimitCount(string memory groupName) public view returns (uint256) {
        return functionGroupRateLimits[groupName].count[msg.sender];
    }
}