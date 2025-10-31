// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

/// @title CommitReveal
/// @notice Implements a commit-reveal scheme to prevent MEV exploitation with enhanced fairness features
/// @dev This contract allows users to commit to transactions and reveal them later with anti-sandwich protections
contract CommitReveal is Ownable, ReentrancyGuard {
    using ECDSA for bytes32;

    struct Commitment {
        bytes32 commitmentHash;
        uint256 timestamp;
        uint256 blockNumber;
        address user;
        bool revealed;
        bytes data;
        uint256 gasLimit;
        uint256 maxFeePerGas;
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
    uint256 public minGasLimit; // Minimum gas limit for transactions
    uint256 public maxGasLimit; // Maximum gas limit for transactions
    
    // Anti-sandwich protection
    uint256 public antiSandwichWindow; // Window in seconds to prevent sandwich attacks
    mapping(address => uint256) public lastRevealTime; // Track last reveal time per user
    
    // Batch processing
    uint256 public batchInterval; // Interval between batches in seconds
    uint256 public lastBatchEndTime; // Timestamp of last batch end
    
    // Events
    event CommitmentMade(bytes32 indexed commitId, address indexed user, uint256 timestamp, uint256 blockNumber);
    event CommitmentRevealed(bytes32 indexed commitId, address indexed user, uint256 gasUsed);
    event StakeAdded(address indexed user, uint256 amount);
    event StakeRemoved(address indexed user, uint256 amount);
    event StakeSlashed(address indexed user, uint256 amount);
    event BatchExecuted(uint256 batchId, uint256 clearingPrice, uint256 executionTime);
    event AntiSandwichViolation(address indexed user, uint256 lastRevealTime, uint256 currentTime);
    
    // Modifiers
    modifier hasSufficientStake() {
        require(userStakes[msg.sender] >= stakeAmount, "Insufficient stake");
        _;
    }
    
    modifier notInAntiSandwichWindow() {
        require(block.timestamp >= lastRevealTime[msg.sender] + antiSandwichWindow, "Anti-sandwich window active");
        _;
    }
    
    modifier batchIntervalRespected() {
        require(block.timestamp >= lastBatchEndTime + batchInterval, "Batch interval not respected");
        _;
    }
    
    /// @notice Constructor to initialize the contract
    /// @param _commitPhaseDuration Duration of commit phase in seconds
    /// @param _revealPhaseDuration Duration of reveal phase in seconds
    /// @param _stakeAmount Required stake amount
    /// @param _penaltyAmount Penalty for failed reveals
    /// @param _batchInterval Interval between batches in seconds
    constructor(
        uint256 _commitPhaseDuration,
        uint256 _revealPhaseDuration,
        uint256 _stakeAmount,
        uint256 _penaltyAmount,
        uint256 _batchInterval
    ) {
        require(_commitPhaseDuration > 0, "Commit phase duration must be positive");
        require(_revealPhaseDuration > 0, "Reveal phase duration must be positive");
        require(_stakeAmount > 0, "Stake amount must be positive");
        require(_penaltyAmount > 0, "Penalty amount must be positive");
        require(_batchInterval > 0, "Batch interval must be positive");
        
        commitPhaseDuration = _commitPhaseDuration;
        revealPhaseDuration = _revealPhaseDuration;
        stakeAmount = _stakeAmount;
        penaltyAmount = _penaltyAmount;
        batchInterval = _batchInterval;
        minGasLimit = 50000;
        maxGasLimit = 10000000;
        antiSandwichWindow = 2; // 2 seconds anti-sandwich window
        lastBatchEndTime = block.timestamp;
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
    
    /// @notice Commit to a transaction with gas parameters
    /// @param _commitmentHash Hash of the transaction data and salt
    /// @param _gasLimit Gas limit for the transaction
    /// @param _maxFeePerGas Maximum fee per gas for the transaction
    function commit(bytes32 _commitmentHash, uint256 _gasLimit, uint256 _maxFeePerGas) external hasSufficientStake nonReentrant batchIntervalRespected {
        require(_gasLimit >= minGasLimit && _gasLimit <= maxGasLimit, "Gas limit out of bounds");
        require(_maxFeePerGas > 0, "Max fee per gas must be positive");
        
        bytes32 commitId = keccak256(abi.encodePacked(_commitmentHash, msg.sender, block.timestamp, block.number));
        
        require(commitments[commitId].timestamp == 0, "Commitment already exists");
        
        commitments[commitId] = Commitment({
            commitmentHash: _commitmentHash,
            timestamp: block.timestamp,
            blockNumber: block.number,
            user: msg.sender,
            revealed: false,
            data: "",
            gasLimit: _gasLimit,
            maxFeePerGas: _maxFeePerGas
        });
        
        emit CommitmentMade(commitId, msg.sender, block.timestamp, block.number);
    }
    
    /// @notice Reveal a committed transaction with anti-sandwich protection
    /// @param _data The actual transaction data
    /// @param _salt The salt used in the commitment
    function reveal(bytes memory _data, bytes32 _salt) external nonReentrant notInAntiSandwichWindow {
        bytes32 commitmentHash = keccak256(abi.encodePacked(_data, _salt));
        bytes32 commitId = keccak256(abi.encodePacked(commitmentHash, msg.sender, commitments[commitmentHash].timestamp, commitments[commitmentHash].blockNumber));
        
        Commitment storage commitment = commitments[commitId];
        
        require(commitment.timestamp > 0, "No commitment found");
        require(!commitment.revealed, "Already revealed");
        require(commitment.user == msg.sender, "Not your commitment");
        require(block.timestamp >= commitment.timestamp + commitPhaseDuration, "Commit phase not ended");
        require(block.timestamp <= commitment.timestamp + commitPhaseDuration + revealPhaseDuration, "Reveal phase ended");
        require(commitmentHash == commitment.commitmentHash, "Invalid commitment");
        
        commitment.revealed = true;
        commitment.data = _data;
        
        // Update last reveal time for anti-sandwich protection
        lastRevealTime[msg.sender] = block.timestamp;
        
        // Process the actual transaction
        uint256 gasBefore = gasleft();
        executeTransaction(_data);
        uint256 gasUsed = gasBefore - gasleft();
        
        emit CommitmentRevealed(commitId, msg.sender, gasUsed);
    }
    
    /// @notice Slash stake of a user who failed to reveal
    /// @param _user The user whose stake should be slashed
    function slashStake(address _user) external onlyOwner {
        require(userStakes[_user] >= penaltyAmount, "Insufficient stake for penalty");
        userStakes[_user] -= penaltyAmount;
        emit StakeSlashed(_user, penaltyAmount);
    }
    
    /// @notice Execute a batch of transactions with uniform clearing
    function executeBatch() external {
        require(block.timestamp >= lastBatchEndTime + batchInterval, "Batch interval not reached");
        
        // In a real implementation, this would collect all commitments
        // and execute them at a uniform clearing price
        // For now, we'll just update the batch end time
        
        uint256 batchId = block.timestamp / batchInterval;
        uint256 clearingPrice = 1e18; // Placeholder clearing price
        
        lastBatchEndTime = block.timestamp;
        
        emit BatchExecuted(batchId, clearingPrice, block.timestamp);
    }
    
    /// @notice Execute the actual transaction (to be implemented based on specific use case)
    /// @param _data The transaction data to execute
    function executeTransaction(bytes memory _data) internal {
        // This is a placeholder implementation
        // In a real implementation, this would execute the actual transaction
        // based on the data provided
        // For example, it might parse the data and call other contract functions
        
        // Simulate some processing
        assembly {
            let ptr := add(_data, 0x20)
            let len := mload(_data)
            // In a real implementation, this would actually execute the transaction
        }
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
    
    /// @notice Update batch interval
    /// @param _interval New batch interval in seconds
    function updateBatchInterval(uint256 _interval) external onlyOwner {
        require(_interval > 0, "Interval must be positive");
        batchInterval = _interval;
    }
    
    /// @notice Update anti-sandwich window
    /// @param _window New anti-sandwich window in seconds
    function updateAntiSandwichWindow(uint256 _window) external onlyOwner {
        antiSandwichWindow = _window;
    }
    
    /// @notice Update gas limits
    /// @param _minGasLimit New minimum gas limit
    /// @param _maxGasLimit New maximum gas limit
    function updateGasLimits(uint256 _minGasLimit, uint256 _maxGasLimit) external onlyOwner {
        require(_minGasLimit > 0, "Min gas limit must be positive");
        require(_maxGasLimit > _minGasLimit, "Max gas limit must be greater than min");
        minGasLimit = _minGasLimit;
        maxGasLimit = _maxGasLimit;
    }
}