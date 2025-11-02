// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

/// @title GuardianMultisig
/// @notice A multisig contract for guardian approvals with emergency controls
/// @dev Implements multisig functionality with pause and veto capabilities
contract GuardianMultisig is AccessControl, Pausable {
    bytes32 public constant GUARDIAN_ROLE = keccak256("GUARDIAN_ROLE");
    bytes32 public constant ADMIN_ROLE = keccak256("ADMIN_ROLE");
    
    uint256 public requiredSignatures;
    uint256 public constant MAX_GUARDIANS = 10;
    
    struct Proposal {
        uint256 voteCount;
        mapping(address => bool) hasVoted;
        bool executed;
        bytes data;
        address target;
        uint256 createdAt;
        address[] voters;
    }
    
    struct VoteRecord {
        address voter;
        uint256 timestamp;
        bytes32 proposalId;
    }
    
    mapping(bytes32 => Proposal) public proposals;
    mapping(address => bool) public isGuardian;
    mapping(bytes32 => VoteRecord[]) public proposalVoteHistory;
    address[] public guardians;
    
    event GuardianAdded(address indexed guardian);
    event GuardianRemoved(address indexed guardian);
    event ProposalCreated(bytes32 indexed proposalId, address target, bytes data, uint256 timestamp);
    event ProposalExecuted(bytes32 indexed proposalId, uint256 executionTime);
    event ProposalVetoed(bytes32 indexed proposalId, address indexed admin);
    event RequiredSignaturesChanged(uint256 oldRequired, uint256 newRequired);
    event VoteCast(bytes32 indexed proposalId, address indexed voter, uint256 timestamp);
    event VoteHistoryRecorded(bytes32 indexed proposalId, address voter, uint256 timestamp);

    /// @notice Constructor to initialize the guardian multisig
    /// @param _guardians The initial guardian addresses
    /// @param _requiredSignatures The number of required signatures
    constructor(address[] memory _guardians, uint256 _requiredSignatures) {
        require(_guardians.length > 0, "At least one guardian required");
        require(_guardians.length <= MAX_GUARDIANS, "Too many guardians");
        require(_requiredSignatures > 0, "At least one signature required");
        require(_requiredSignatures <= _guardians.length, "Required signatures exceed guardians");
        
        requiredSignatures = _requiredSignatures;
        
        // Grant admin role to deployer
        _grantRole(ADMIN_ROLE, msg.sender);
        
        // Add guardians
        for (uint256 i = 0; i < _guardians.length; i++) {
            require(_guardians[i] != address(0), "Invalid guardian address");
            require(!isGuardian[_guardians[i]], "Duplicate guardian");
            
            isGuardian[_guardians[i]] = true;
            _grantRole(GUARDIAN_ROLE, _guardians[i]);
        }
        
        guardians = _guardians;
    }

    /// @notice Add a new guardian
    /// @param guardian The guardian address to add
    function addGuardian(address guardian) public onlyRole(ADMIN_ROLE) {
        require(guardian != address(0), "Invalid guardian address");
        require(!isGuardian[guardian], "Guardian already exists");
        require(guardians.length < MAX_GUARDIANS, "Maximum guardians reached");
        
        isGuardian[guardian] = true;
        _grantRole(GUARDIAN_ROLE, guardian);
        guardians.push(guardian);
        
        emit GuardianAdded(guardian);
    }

    /// @notice Remove a guardian
    /// @param guardian The guardian address to remove
    function removeGuardian(address guardian) public onlyRole(ADMIN_ROLE) {
        require(isGuardian[guardian], "Guardian does not exist");
        require(guardians.length > requiredSignatures, "Cannot remove below required threshold");
        
        isGuardian[guardian] = false;
        _revokeRole(GUARDIAN_ROLE, guardian);
        
        // Remove from guardians array
        for (uint256 i = 0; i < guardians.length; i++) {
            if (guardians[i] == guardian) {
                guardians[i] = guardians[guardians.length - 1];
                guardians.pop();
                break;
            }
        }
        
        emit GuardianRemoved(guardian);
    }

    /// @notice Update required signatures
    /// @param _requiredSignatures The new required signatures count
    function updateRequiredSignatures(uint256 _requiredSignatures) public onlyRole(ADMIN_ROLE) {
        require(_requiredSignatures > 0, "At least one signature required");
        require(_requiredSignatures <= guardians.length, "Required signatures exceed guardians");
        
        uint256 oldRequired = requiredSignatures;
        requiredSignatures = _requiredSignatures;
        
        emit RequiredSignaturesChanged(oldRequired, _requiredSignatures);
    }

    /// @notice Create a new proposal
    /// @param target The target contract address
    /// @param data The call data
    /// @return bytes32 The proposal ID
    function createProposal(address target, bytes memory data) public onlyRole(GUARDIAN_ROLE) returns (bytes32) {
        require(target != address(0), "Invalid target address");
        
        bytes32 proposalId = keccak256(abi.encodePacked(target, data, block.timestamp));
        
        Proposal storage proposal = proposals[proposalId];
        proposal.data = data;
        proposal.target = target;
        proposal.createdAt = block.timestamp;
        
        emit ProposalCreated(proposalId, target, data, block.timestamp);
        
        return proposalId;
    }

    /// @notice Vote on a proposal
    /// @param proposalId The proposal ID
    function vote(bytes32 proposalId) public onlyRole(GUARDIAN_ROLE) {
        Proposal storage proposal = proposals[proposalId];
        require(!proposal.hasVoted[msg.sender], "Already voted");
        require(!proposal.executed, "Proposal already executed");
        
        proposal.hasVoted[msg.sender] = true;
        proposal.voteCount += 1;
        proposal.voters.push(msg.sender);
        
        // Record vote in history
        VoteRecord memory voteRecord = VoteRecord({
            voter: msg.sender,
            timestamp: block.timestamp,
            proposalId: proposalId
        });
        proposalVoteHistory[proposalId].push(voteRecord);
        
        emit VoteCast(proposalId, msg.sender, block.timestamp);
        emit VoteHistoryRecorded(proposalId, msg.sender, block.timestamp);
        
        // Execute if enough votes
        if (proposal.voteCount >= requiredSignatures) {
            _executeProposal(proposalId);
        }
    }

    /// @notice Veto a proposal (emergency function)
    /// @param proposalId The proposal ID
    function vetoProposal(bytes32 proposalId) public onlyRole(ADMIN_ROLE) {
        Proposal storage proposal = proposals[proposalId];
        require(!proposal.executed, "Proposal already executed");
        
        proposal.executed = true;
        emit ProposalVetoed(proposalId, msg.sender);
    }

    /// @notice Execute a proposal
    /// @param proposalId The proposal ID
    function _executeProposal(bytes32 proposalId) internal {
        Proposal storage proposal = proposals[proposalId];
        require(!proposal.executed, "Proposal already executed");
        
        proposal.executed = true;
        
        // Execute the call
        (bool success, ) = proposal.target.call(proposal.data);
        require(success, "Proposal execution failed");
        
        emit ProposalExecuted(proposalId, block.timestamp);
    }

    /// @notice Emergency pause function
    function emergencyPause() public onlyRole(GUARDIAN_ROLE) {
        _pause();
    }

    /// @notice Emergency unpause function
    function emergencyUnpause() public onlyRole(ADMIN_ROLE) {
        _unpause();
    }

    /// @notice Get guardian count
    /// @return uint256 The number of guardians
    function getGuardianCount() public view returns (uint256) {
        return guardians.length;
    }

    /// @notice Check if an address is a guardian
    /// @param account The address to check
    /// @return bool Whether the address is a guardian
    function isAddressGuardian(address account) public view returns (bool) {
        return isGuardian[account];
    }

    /// @notice Get vote history for a proposal
    /// @param proposalId The proposal ID
    /// @return VoteRecord[] The vote history
    function getProposalVoteHistory(bytes32 proposalId) public view returns (VoteRecord[] memory) {
        return proposalVoteHistory[proposalId];
    }

    /// @notice Get voters for a proposal
    /// @param proposalId The proposal ID
    /// @return address[] The list of voters
    function getProposalVoters(bytes32 proposalId) public view returns (address[] memory) {
        return proposals[proposalId].voters;
    }

    /// @notice Version tracking
    /// @return The current version
    function version() public pure returns (string memory) {
        return "1.0.0";
    }
}