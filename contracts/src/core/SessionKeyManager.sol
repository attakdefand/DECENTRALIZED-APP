// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

/// @title SessionKeyManager
/// @notice Manages session keys for account abstraction with scope and time limits
contract SessionKeyManager is Ownable {
    using ECDSA for bytes32;

    struct SessionKey {
        address owner;
        bytes32 permissions;
        uint64 validAfter;
        uint64 validUntil;
        uint32 useCount;
        uint32 maxUses;
        bool revoked;
    }

    // Mapping from session ID to session key
    mapping(bytes32 => SessionKey) public sessionKeys;
    
    // Mapping from owner to their session key count
    mapping(address => uint256) public ownerSessionCount;
    
    // Events
    event SessionKeyCreated(
        bytes32 indexed sessionId,
        address indexed owner,
        bytes32 permissions,
        uint64 validAfter,
        uint64 validUntil,
        uint32 maxUses
    );
    
    event SessionKeyUsed(bytes32 indexed sessionId, address indexed owner);
    event SessionKeyRevoked(bytes32 indexed sessionId, address indexed owner);
    
    /// @notice Create a new session key
    /// @param sessionId Unique identifier for the session key
    /// @param permissions Permissions encoded as bytes32
    /// @param validAfter Timestamp when the key becomes valid
    /// @param validUntil Timestamp when the key expires
    /// @param maxUses Maximum number of times the key can be used
    function createSessionKey(
        bytes32 sessionId,
        bytes32 permissions,
        uint64 validAfter,
        uint64 validUntil,
        uint32 maxUses
    ) external {
        require(validUntil > validAfter, "Invalid time bounds");
        require(maxUses > 0, "Max uses must be positive");
        require(sessionKeys[sessionId].owner == address(0), "Session ID already exists");
        
        sessionKeys[sessionId] = SessionKey({
            owner: msg.sender,
            permissions: permissions,
            validAfter: validAfter,
            validUntil: validUntil,
            useCount: 0,
            maxUses: maxUses,
            revoked: false
        });
        
        ownerSessionCount[msg.sender]++;
        
        emit SessionKeyCreated(sessionId, msg.sender, permissions, validAfter, validUntil, maxUses);
    }
    
    /// @notice Validate a session key for an operation
    /// @param sessionId The session key ID
    /// @param operationHash Hash of the operation to validate
    /// @return valid Whether the session key is valid for this operation
    function validateSessionKey(
        bytes32 sessionId,
        bytes32 operationHash
    ) external view returns (bool valid) {
        SessionKey storage key = sessionKeys[sessionId];
        
        // Check if key exists and is not revoked
        if (key.owner == address(0) || key.revoked) {
            return false;
        }
        
        // Check time bounds
        if (block.timestamp < key.validAfter || block.timestamp > key.validUntil) {
            return false;
        }
        
        // Check usage count
        if (key.useCount >= key.maxUses) {
            return false;
        }
        
        // Check permissions (simplified - in a real implementation, this would be more complex)
        // For now, we'll just check if the operation hash matches the permissions
        // A real implementation would decode the permissions and check against the operation
        if (key.permissions != bytes32(0) && key.permissions != operationHash) {
            return false;
        }
        
        return true;
    }
    
    /// @notice Use a session key (increments use count)
    /// @param sessionId The session key ID
    function useSessionKey(bytes32 sessionId) external {
        SessionKey storage key = sessionKeys[sessionId];
        
        require(key.owner != address(0), "Session key does not exist");
        require(!key.revoked, "Session key revoked");
        require(block.timestamp >= key.validAfter, "Session not active yet");
        require(block.timestamp <= key.validUntil, "Session expired");
        require(key.useCount < key.maxUses, "Session usage limit exceeded");
        
        key.useCount++;
        
        emit SessionKeyUsed(sessionId, key.owner);
    }
    
    /// @notice Revoke a session key
    /// @param sessionId The session key ID
    function revokeSessionKey(bytes32 sessionId) external {
        SessionKey storage key = sessionKeys[sessionId];
        require(key.owner == msg.sender || msg.sender == owner(), "Not owner or admin");
        require(!key.revoked, "Already revoked");
        
        key.revoked = true;
        ownerSessionCount[key.owner]--;
        
        emit SessionKeyRevoked(sessionId, key.owner);
    }
    
    /// @notice Get session key details
    /// @param sessionId The session key ID
    /// @return sessionKey The session key details
    function getSessionKey(bytes32 sessionId) external view returns (SessionKey memory) {
        return sessionKeys[sessionId];
    }
    
    /// @notice Check if a session key is valid
    /// @param sessionId The session key ID
    /// @return valid Whether the session key is valid
    function isSessionKeyValid(bytes32 sessionId) external view returns (bool) {
        SessionKey storage key = sessionKeys[sessionId];
        return (
            key.owner != address(0) &&
            !key.revoked &&
            block.timestamp >= key.validAfter &&
            block.timestamp <= key.validUntil &&
            key.useCount < key.maxUses
        );
    }
}