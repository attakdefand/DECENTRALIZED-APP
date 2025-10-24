// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

/// @title CommitReveal
/// @notice Implements a commit-reveal scheme to prevent MEV exploitation
/// @dev This contract allows users to commit to transactions and reveal them later
contract CommitReveal is Ownable, ReentrancyGuard {
    using ECDSA for bytes32;

    struct Commitment {
        bytes32 commitmentHash;
        uint256 timestamp;
        address user;
        bool revealed;
        bytes data; // For gas optimization, we store the actual data
    }

    // Mapping from commitId to Commitment
    mapping(bytes32 => Commitment) public commitments;
    
    // Mapping from user to their stake amount
    mapping(address => uint256) public userStakes;
    
    // Configuration parameters
    uint256 public commitPhaseDuration; // Duration of commit phase in seconds
    uint256 public revealPhaseDuration; // Duration of reveal phase in seconds
    uint256 public stakeAmount; // Required stake amount
    uint256 public penaltyAmount; // Penalty for failed reveals
    
    // Events
    event CommitmentMade(bytes32 indexed commitId, address indexed user, uint256 timestamp);
    event CommitmentRevealed(bytes32 indexed commitId, address indexed user);
    event StakeAdded(address indexed user, uint256 amount);
    event StakeRemoved(address indexed user, uint256 amount);
    event StakeSlashed(address indexed user, uint256 amount);
    
    // Modifiers
    modifier hasSufficientStake() {
        require(userStakes[msg.sender] >= stakeAmount, "Insufficient stake");
        _;
    }
    
    /// @notice Constructor to initialize the contract
    /// @param _commitPhaseDuration Duration of commit phase in seconds
    /// @param _revealPhaseDuration Duration of reveal phase in seconds
    /// @param _stakeAmount Required stake amount
    /// @param _penaltyAmount Penalty for failed reveals
    constructor(
        uint256 _commitPhaseDuration,
        uint256 _revealPhaseDuration,
        uint256 _stakeAmount,
        uint256 _penaltyAmount
    ) {
        require(_commitPhaseDuration > 0, "Commit phase duration must be positive");
        require(_revealPhaseDuration > 0, "Reveal phase duration must be positive");
        require(_stakeAmount > 0, "Stake amount must be positive");
        require(_penaltyAmount > 0, "Penalty amount must be positive");
        
        commitPhaseDuration = _commitPhaseDuration;
        revealPhaseDuration = _revealPhaseDuration;
        stakeAmount = _stakeAmount;
        penaltyAmount = _penaltyAmount;
    }
    
    /// @notice Add stake to participate in commit-reveal
    function addStake() external payable {
        require(msg.value >= stakeAmount, "Insufficient stake amount");
        userStakes[msg.sender] += msg.value;
        emit StakeAdded(msg.sender, msg.value);
    }
    
    /// @notice Remove stake (only possible when no active commitments)
    function removeStake(uint256 amount) external nonReentrant {
        require(userStakes[msg.sender] >= amount, "Insufficient stake");
        userStakes[msg.sender] -= amount;
        payable(msg.sender).transfer(amount);
        emit StakeRemoved(msg.sender, amount);
    }
    
    /// @notice Commit to a transaction
    /// @param _commitmentHash Hash of the transaction data and salt
    function commit(bytes32 _commitmentHash) external hasSufficientStake nonReentrant {
        bytes32 commitId = keccak256(abi.encodePacked(_commitmentHash, msg.sender, block.timestamp));
        
        require(commitments[commitId].timestamp == 0, "Commitment already exists");
        
        commitments[commitId] = Commitment({
            commitmentHash: _commitmentHash,
            timestamp: block.timestamp,
            user: msg.sender,
            revealed: false,
            data: ""
        });
        
        emit CommitmentMade(commitId, msg.sender, block.timestamp);
    }
    
    /// @notice Reveal a committed transaction
    /// @param _data The actual transaction data
    /// @param _salt The salt used in the commitment
    function reveal(bytes memory _data, bytes32 _salt) external nonReentrant {
        bytes32 commitmentHash = keccak256(abi.encodePacked(_data, _salt));
        bytes32 commitId = keccak256(abi.encodePacked(commitmentHash, msg.sender, commitments[commitmentHash].timestamp));
        
        Commitment storage commitment = commitments[commitId];
        
        require(commitment.timestamp > 0, "No commitment found");
        require(!commitment.revealed, "Already revealed");
        require(commitment.user == msg.sender, "Not your commitment");
        require(block.timestamp >= commitment.timestamp + commitPhaseDuration, "Commit phase not ended");
        require(block.timestamp <= commitment.timestamp + commitPhaseDuration + revealPhaseDuration, "Reveal phase ended");
        require(commitmentHash == commitment.commitmentHash, "Invalid commitment");
        
        commitment.revealed = true;
        commitment.data = _data;
        
        // Process the actual transaction
        executeTransaction(_data);
        
        emit CommitmentRevealed(commitId, msg.sender);
    }
    
    /// @notice Slash stake of a user who failed to reveal
    /// @param _user The user whose stake should be slashed
    function slashStake(address _user) external onlyOwner {
        require(userStakes[_user] >= penaltyAmount, "Insufficient stake for penalty");
        userStakes[_user] -= penaltyAmount;
        emit StakeSlashed(_user, penaltyAmount);
    }
    
    /// @notice Execute the actual transaction (to be implemented based on specific use case)
    /// @param _data The transaction data to execute
    function executeTransaction(bytes memory _data) internal {
        // This is a placeholder implementation
        // In a real implementation, this would execute the actual transaction
        // based on the data provided
        // For example, it might parse the data and call other contract functions
    }
    
    /// @notice Get commitment details
    /// @param _commitId The commitment ID
    /// @return commitment The commitment details
    function getCommitment(bytes32 _commitId) external view returns (Commitment memory) {
        return commitments[_commitId];
    }
    
    /// @notice Update commit phase duration
    /// @param _duration New duration in seconds
    function updateCommitPhaseDuration(uint256 _duration) external onlyOwner {
        require(_duration > 0, "Duration must be positive");
        commitPhaseDuration = _duration;
    }
    
    /// @notice Update reveal phase duration
    /// @param _duration New duration in seconds
    function updateRevealPhaseDuration(uint256 _duration) external onlyOwner {
        require(_duration > 0, "Duration must be positive");
        revealPhaseDuration = _duration;
    }
    
    /// @notice Update stake amount
    /// @param _amount New stake amount
    function updateStakeAmount(uint256 _amount) external onlyOwner {
        require(_amount > 0, "Amount must be positive");
        stakeAmount = _amount;
    }
    
    /// @notice Update penalty amount
    /// @param _amount New penalty amount
    function updatePenaltyAmount(uint256 _amount) external onlyOwner {
        require(_amount > 0, "Amount must be positive");
        penaltyAmount = _amount;
    }
}