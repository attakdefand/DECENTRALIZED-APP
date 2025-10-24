// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

/// @title SimpleEntryPoint
/// @notice Simplified EntryPoint contract for handling UserOperations in account abstraction
contract SimpleEntryPoint {
    using ECDSA for bytes32;
    using SafeERC20 for IERC20;

    struct UserOperation {
        address sender;
        uint256 nonce;
        bytes initCode;
        bytes callData;
        uint256 callGasLimit;
        uint256 verificationGasLimit;
        uint256 preVerificationGas;
        uint256 maxFeePerGas;
        uint256 maxPriorityFeePerGas;
        bytes paymasterAndData;
        bytes signature;
    }

    struct UserOpInfo {
        bytes32 userOpHash;
        uint256 prefund;
        address sender;
        address paymaster;
    }

    // State variables
    uint256 public nonceCounter;
    mapping(address => uint256) public senderNonce;
    mapping(bytes32 => bool) public userOpHashes; // To prevent replay attacks
    
    // Events
    event UserOperationEvent(
        bytes32 indexed userOpHash,
        address indexed sender,
        address indexed paymaster,
        uint256 nonce,
        bool success
    );
    
    event UserOperationRevertReason(
        bytes32 indexed userOpHash,
        address indexed sender,
        uint256 nonce,
        bytes revertReason
    );

    /// @notice Handle a batch of UserOperations
    /// @param ops Array of UserOperations
    /// @param beneficiary Address to receive gas payments
    function handleOps(UserOperation[] calldata ops, address payable beneficiary) public {
        uint256 opsLength = ops.length;
        UserOpInfo[] memory opInfos = new UserOpInfo[](opsLength);
        
        // Validate all operations first
        for (uint256 i = 0; i < opsLength; i++) {
            UserOperation calldata op = ops[i];
            bytes32 userOpHash = getUserOpHash(op);
            
            // Prevent replay attacks
            require(!userOpHashes[userOpHash], "UserOp already executed");
            
            // Validate nonce
            require(op.nonce == senderNonce[op.sender]++, "Invalid nonce");
            
            // Validate signature
            require(validateSignature(op, userOpHash), "Invalid signature");
            
            // Store operation info
            opInfos[i] = UserOpInfo({
                userOpHash: userOpHash,
                prefund: op.preVerificationGas * op.maxFeePerGas,
                sender: op.sender,
                paymaster: getPaymaster(op)
            });
            
            userOpHashes[userOpHash] = true;
        }
        
        // Execute all operations
        for (uint256 i = 0; i < opsLength; i++) {
            UserOperation calldata op = ops[i];
            bool success = executeUserOp(op);
            
            emit UserOperationEvent(
                opInfos[i].userOpHash,
                op.sender,
                opInfos[i].paymaster,
                op.nonce,
                success
            );
        }
        
        // Handle gas payments
        handleGasPayments(opInfos, beneficiary);
    }
    
    /// @notice Validate a UserOperation's signature
    /// @param userOp The UserOperation to validate
    /// @param userOpHash The hash of the UserOperation
    /// @return valid Whether the signature is valid
    function validateSignature(UserOperation calldata userOp, bytes32 userOpHash) internal view returns (bool) {
        // For simplicity, we're using ECDSA signature validation
        // In a real implementation, this would support ERC-1271 contract signatures
        address signer = userOpHash.toEthSignedMessageHash().recover(userOp.signature);
        return signer == userOp.sender;
    }
    
    /// @notice Execute a UserOperation
    /// @param userOp The UserOperation to execute
    /// @return success Whether the execution was successful
    function executeUserOp(UserOperation calldata userOp) internal returns (bool) {
        // Deploy account if needed
        if (userOp.initCode.length > 0) {
            deployAccount(userOp.initCode);
        }
        
        // Execute the call
        (bool success, bytes memory result) = userOp.sender.call{gas: userOp.callGasLimit}(
            abi.encodePacked(userOp.callData)
        );
        
        if (!success) {
            emit UserOperationRevertReason(
                getUserOpHash(userOp),
                userOp.sender,
                userOp.nonce,
                result
            );
        }
        
        return success;
    }
    
    /// @notice Deploy an account using initCode
    /// @param initCode The initialization code
    function deployAccount(bytes calldata initCode) internal {
        address sender = getAddressFromInitCode(initCode);
        if (sender.code.length > 0) return; // Already deployed
        
        bytes memory code = initCode[20:];
        assembly {
            sender := create2(0, add(code, 0x20), mload(code), 0)
        }
        
        require(sender != address(0), "Account deployment failed");
    }
    
    /// @notice Get the address from initCode
    /// @param initCode The initialization code
    /// @return addr The address
    function getAddressFromInitCode(bytes calldata initCode) internal pure returns (address addr) {
        bytes32 salt = bytes32(0);
        bytes memory code = initCode[20:];
        bytes32 hash = keccak256(abi.encodePacked(bytes1(0xff), address(this), salt, keccak256(code)));
        assembly {
            addr := and(0xffffffffffffffffffffffffffffffffffffffff, shr(96, hash))
        }
    }
    
    /// @notice Get the paymaster address from paymasterAndData
    /// @param userOp The UserOperation
    /// @return paymaster The paymaster address
    function getPaymaster(UserOperation calldata userOp) internal pure returns (address paymaster) {
        if (userOp.paymasterAndData.length >= 20) {
            assembly {
                paymaster := shr(96, mload(add(userOp.paymasterAndData, 20)))
            }
        }
    }
    
    /// @notice Handle gas payments for executed operations
    /// @param opInfos Array of UserOpInfo
    /// @param beneficiary Address to receive payments
    function handleGasPayments(UserOpInfo[] memory opInfos, address payable beneficiary) internal {
        // Simplified gas payment handling
        // In a real implementation, this would be more complex
        uint256 totalPayment = 0;
        for (uint256 i = 0; i < opInfos.length; i++) {
            totalPayment += opInfos[i].prefund;
        }
        
        // Transfer payment to beneficiary
        beneficiary.transfer(totalPayment);
    }
    
    /// @notice Get the hash of a UserOperation
    /// @param userOp The UserOperation
    /// @return hash The hash
    function getUserOpHash(UserOperation calldata userOp) public view returns (bytes32) {
        return keccak256(abi.encode(
            userOp.sender,
            userOp.nonce,
            keccak256(userOp.initCode),
            keccak256(userOp.callData),
            userOp.callGasLimit,
            userOp.verificationGasLimit,
            userOp.preVerificationGas,
            userOp.maxFeePerGas,
            userOp.maxPriorityFeePerGas,
            keccak256(userOp.paymasterAndData)
        ));
    }
    
    /// @notice Get the nonce for a sender
    /// @param sender The sender address
    /// @return nonce The nonce
    function getNonce(address sender) external view returns (uint256) {
        return senderNonce[sender];
    }
}