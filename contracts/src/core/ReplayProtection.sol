// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

/// @title ReplayProtection
/// @notice Implements replay protection for transactions
contract ReplayProtection {
    using ECDSA for bytes32;

    /// @dev Structure for transaction data
    struct Transaction {
        address target;
        uint256 value;
        bytes data;
        uint256 chainId;
        uint256 nonce;
        uint256 deadline;
        bytes signature;
    }

    /// @dev Mapping of nonces for each address
    mapping(address => uint256) public nonces;
    
    /// @dev Mapping of executed transaction hashes to prevent replay
    mapping(bytes32 => bool) public executedTransactions;
    
    /// @dev Mapping of permit nonces to prevent replay
    mapping(address => mapping(uint256 => bool)) public usedPermitNonces;
    
    /// @dev Events
    event TransactionExecuted(
        bytes32 indexed txHash,
        address indexed sender,
        address target,
        uint256 value,
        uint256 nonce
    );
    
    event PermitUsed(
        address indexed owner,
        uint256 indexed nonce,
        uint256 deadline
    );

    /// @notice Execute a transaction with replay protection
    /// @param txData The transaction data
    function executeTransaction(Transaction calldata txData) external {
        // Validate chain ID
        require(txData.chainId == block.chainid, "Invalid chain ID");
        
        // Validate deadline
        require(txData.deadline >= block.timestamp, "Transaction expired");
        
        // Validate nonce
        require(txData.nonce == nonces[msg.sender], "Invalid nonce");
        
        // Create transaction hash
        bytes32 txHash = keccak256(abi.encode(
            txData.target,
            txData.value,
            txData.data,
            txData.chainId,
            txData.nonce,
            txData.deadline
        ));
        
        // Prevent replay
        require(!executedTransactions[txHash], "Transaction already executed");
        executedTransactions[txHash] = true;
        
        // Validate signature
        require(validateSignature(txHash, txData.signature, msg.sender), "Invalid signature");
        
        // Increment nonce
        nonces[msg.sender]++;
        
        // Execute transaction
        (bool success, ) = txData.target.call{value: txData.value}(txData.data);
        require(success, "Transaction execution failed");
        
        emit TransactionExecuted(txHash, msg.sender, txData.target, txData.value, txData.nonce);
    }
    
    /// @notice Execute a batch of transactions with replay protection
    /// @param txData Array of transaction data
    function executeBatchTransactions(Transaction[] calldata txData) external {
        for (uint256 i = 0; i < txData.length; i++) {
            executeTransaction(txData[i]);
        }
    }
    
    /// @notice Validate and use a permit
    /// @param owner The address of the token owner
    /// @param spender The address of the spender
    /// @param value The amount of tokens to approve
    /// @param deadline The timestamp after which the permit is invalid
    /// @param nonce The nonce for the permit
    /// @param signature The signature for the permit
    function usePermit(
        address owner,
        address spender,
        uint256 value,
        uint256 deadline,
        uint256 nonce,
        bytes memory signature
    ) external {
        // Validate deadline
        require(deadline >= block.timestamp, "Permit expired");
        
        // Prevent replay
        require(!usedPermitNonces[owner][nonce], "Permit already used");
        usedPermitNonces[owner][nonce] = true;
        
        // Create permit hash
        bytes32 permitHash = keccak256(abi.encodePacked(
            "\x19\x01",
            getDomainSeparator(),
            keccak256(abi.encode(
                keccak256("Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)"),
                owner,
                spender,
                value,
                nonce,
                deadline
            ))
        ));
        
        // Validate signature
        require(validateSignature(permitHash, signature, owner), "Invalid permit signature");
        
        emit PermitUsed(owner, nonce, deadline);
        
        // In a real implementation, this would call the token contract's permit function
        // For this example, we'll just emit the event
    }
    
    /// @notice Validate a signature
    /// @param hash The hash to validate
    /// @param signature The signature to validate
    /// @param signer The expected signer
    /// @return valid Whether the signature is valid
    function validateSignature(
        bytes32 hash,
        bytes memory signature,
        address signer
    ) internal pure returns (bool) {
        // Handle both regular and EIP-2098 compact signatures
        if (signature.length == 64) {
            // Compact signature
            bytes32 r;
            bytes32 vs;
            assembly {
                r := mload(add(signature, 0x20))
                vs := mload(add(signature, 0x40))
            }
            bytes32 s = vs & bytes32(0x7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff);
            uint8 v = uint8(uint256(vs >> 255)) + 27;
            return ecrecover(hash, v, r, s) == signer;
        } else if (signature.length == 65) {
            // Regular signature
            return hash.recover(signature) == signer;
        }
        return false;
    }
    
    /// @notice Get the domain separator for EIP-712
    /// @return domainSeparator The domain separator
    function getDomainSeparator() internal view returns (bytes32) {
        return keccak256(abi.encode(
            keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
            keccak256("ReplayProtection"),
            keccak256("1"),
            block.chainid,
            address(this)
        ));
    }
    
    /// @notice Get the current nonce for an address
    /// @param account The address to get the nonce for
    /// @return nonce The current nonce
    function getNonce(address account) external view returns (uint256) {
        return nonces[account];
    }
    
    /// @notice Check if a transaction hash has been executed
    /// @param txHash The transaction hash to check
    /// @return executed Whether the transaction has been executed
    function isTransactionExecuted(bytes32 txHash) external view returns (bool) {
        return executedTransactions[txHash];
    }
    
    /// @notice Check if a permit nonce has been used
    /// @param owner The owner address
    /// @param nonce The nonce to check
    /// @return used Whether the nonce has been used
    function isPermitNonceUsed(address owner, uint256 nonce) external view returns (bool) {
        return usedPermitNonces[owner][nonce];
    }
}