# On-chain Safety Invariants

This document defines the invariants and safety properties for the smart contracts in the DECENTRALIZED-APP project.

## Overview

On-chain safety is critical for decentralized applications. This document specifies the invariants that must hold true for all smart contract implementations, including CEI guards, input bounds, reentrancy protections, and mathematical invariants.

## CEI Guards (Checks-Effects-Interactions)

### Definition
CEI is a security pattern that ensures state changes are made before external calls to prevent reentrancy attacks.

### Implementation Requirements
1. **Checks**: Validate all inputs and preconditions first
2. **Effects**: Update all state variables before external calls
3. **Interactions**: Make external calls last

### Examples
```solidity
// Correct CEI implementation
function transfer(address to, uint256 amount) public {
    // Checks
    require(to != address(0), "Invalid address");
    require(balance[msg.sender] >= amount, "Insufficient balance");
    
    // Effects
    balance[msg.sender] -= amount;
    balance[to] += amount;
    
    // Interactions
    emit Transfer(msg.sender, to, amount);
}
```

## Input Bounds

### Definition
Input bounds define valid ranges for all function parameters to prevent invalid operations.

### Implementation Requirements
1. **Range Validation**: Check that numeric inputs are within acceptable ranges
2. **Address Validation**: Verify that addresses are not zero or invalid
3. **String Validation**: Validate string lengths and content
4. **Array Validation**: Check array lengths and contents

### Examples
```solidity
// Input bounds validation
function setFee(uint256 fee) public onlyOwner {
    require(fee <= 10000, "Fee must be <= 10000 (100%)");
    feePercentage = fee;
}

function setName(string memory name) public {
    require(bytes(name).length > 0, "Name cannot be empty");
    require(bytes(name).length <= 32, "Name too long");
    userName[msg.sender] = name;
}
```

## Reentrancy Guards

### Definition
Reentrancy guards prevent malicious contracts from calling back into a function before the initial execution completes.

### Implementation Requirements
1. **Mutex Pattern**: Use a boolean flag to track function execution
2. **OpenZeppelin ReentrancyGuard**: Use battle-tested implementations
3. **Pull-over-Push Pattern**: Transfer funds to recipients rather than pushing

### Examples
```solidity
// Using OpenZeppelin ReentrancyGuard
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

contract Vault is ReentrancyGuard {
    mapping(address => uint256) public balances;
    
    function deposit() public payable nonReentrant {
        balances[msg.sender] += msg.value;
    }
    
    function withdraw(uint256 amount) public nonReentrant {
        require(balances[msg.sender] >= amount, "Insufficient balance");
        balances[msg.sender] -= amount;
        payable(msg.sender).transfer(amount);
    }
}
```

## Mathematical Invariants

### Definition
Mathematical invariants ensure that calculations are safe and do not overflow or underflow.

### Implementation Requirements
1. **SafeMath**: Use libraries that prevent overflow/underflow
2. **Bounds Checking**: Validate results of calculations
3. **Precision Handling**: Manage decimal precision correctly
4. **Rounding Direction**: Specify rounding behavior

### Examples
```solidity
// Using SafeMath for overflow protection
import "@openzeppelin/contracts/utils/math/SafeMath.sol";

contract Calculator {
    using SafeMath for uint256;
    
    function add(uint256 a, uint256 b) public pure returns (uint256) {
        return a.add(b); // Safe addition
    }
    
    function multiply(uint256 a, uint256 b) public pure returns (uint256) {
        return a.mul(b); // Safe multiplication
    }
}
```

## State Invariants

### Definition
State invariants ensure that the contract's state variables maintain consistent relationships.

### Implementation Requirements
1. **Balance Invariants**: Total supply equals sum of all balances
2. **Permission Invariants**: Only authorized addresses can perform actions
3. **Time Invariants**: Operations only valid during specific time windows
4. **Consistency Invariants**: Related state variables remain consistent

### Examples
```solidity
// State invariant checks
contract Token {
    mapping(address => uint256) public balances;
    uint256 public totalSupply;
    
    function transfer(address to, uint256 amount) public {
        // State invariant: balances must sum to totalSupply
        require(balances[msg.sender] >= amount, "Insufficient balance");
        
        balances[msg.sender] -= amount;
        balances[to] += amount;
        
        // Verify invariant still holds
        assert(totalSupply == getTotalBalances());
    }
    
    function getTotalBalances() private view returns (uint256) {
        // Implementation to sum all balances
        // This is a simplified example
        return totalSupply; // In a real implementation, this would sum all balances
    }
}
```

## Access Control Invariants

### Definition
Access control invariants ensure that only authorized entities can perform specific actions.

### Implementation Requirements
1. **Role-Based Access**: Define roles and permissions clearly
2. **Ownership Transfers**: Secure ownership transfer mechanisms
3. **Time-Based Access**: Access that varies with time
4. **Multi-Signature Requirements**: Actions requiring multiple approvals

### Examples
```solidity
// Access control invariants
import "@openzeppelin/contracts/access/Ownable.sol";

contract ManagedContract is Ownable {
    mapping(address => bool) public operators;
    
    modifier onlyOperator() {
        require(operators[msg.sender] || msg.sender == owner(), "Not authorized");
        _;
    }
    
    function addOperator(address operator) public onlyOwner {
        operators[operator] = true;
    }
    
    function removeOperator(address operator) public onlyOwner {
        operators[operator] = false;
    }
    
    function criticalFunction() public onlyOperator {
        // Only operators or owner can call this
    }
}
```

## Event Logging Invariants

### Definition
Event logging invariants ensure that all critical state changes are properly logged for transparency and debugging.

### Implementation Requirements
1. **Comprehensive Logging**: Log all significant state changes
2. **Consistent Naming**: Use consistent event names and parameters
3. **Indexed Parameters**: Index frequently queried parameters
4. **Data Integrity**: Include sufficient data for reconstruction

### Examples
```solidity
// Event logging invariants
contract Exchange {
    event Trade(
        address indexed trader,
        address indexed tokenIn,
        address indexed tokenOut,
        uint256 amountIn,
        uint256 amountOut,
        uint256 timestamp
    );
    
    function swap(address tokenIn, address tokenOut, uint256 amountIn) public {
        // Perform swap logic
        
        emit Trade(
            msg.sender,
            tokenIn,
            tokenOut,
            amountIn,
            amountOut,
            block.timestamp
        );
    }
}
```

## Testing Requirements

### Unit Tests
1. **Positive Tests**: Verify correct behavior with valid inputs
2. **Negative Tests**: Ensure proper rejection of invalid inputs
3. **Edge Case Tests**: Test boundary conditions
4. **State Transition Tests**: Verify state changes

### Property Tests
1. **Invariant Tests**: Verify that invariants hold after operations
2. **Fuzz Tests**: Test with random inputs
3. **Differential Tests**: Compare implementations
4. **Model-Based Tests**: Test against formal models

### Integration Tests
1. **Cross-Contract Tests**: Test interactions between contracts
2. **Upgrade Tests**: Test upgrade mechanisms
3. **Gas Usage Tests**: Verify gas consumption limits
4. **Reentrancy Tests**: Test reentrancy scenarios

## Verification Tools

### Static Analysis
1. **Slither**: Automated static analysis
2. **Mythril**: Symbolic execution
3. **Securify**: Formal verification
4. **Oyente**: Vulnerability detection

### Formal Verification
1. **Certora Prover**: Formal verification
2. **CertiK**: Security analysis
3. **Consensys Diligence**: Security auditing
4. **Trail of Bits**: Security research

## Audit Requirements

### Pre-Audit Checklist
1. [ ] All invariants documented
2. [ ] Unit tests cover all functions
3. [ ] Property tests verify invariants
4. [ ] Static analysis tools run
5. [ ] Gas optimization completed
6. [ ] Documentation complete

### Post-Audit Actions
1. [ ] Address all critical issues
2. [ ] Verify fixes with tests
3. [ ] Run regression tests
4. [ ] Update documentation
5. [ ] Obtain final audit report

## Continuous Monitoring

### Runtime Checks
1. **Assertion Monitoring**: Track assertion failures
2. **Gas Usage Monitoring**: Monitor gas consumption
3. **Event Monitoring**: Track critical events
4. **Anomaly Detection**: Detect unusual patterns

### Incident Response
1. **Alerting**: Notify on invariant violations
2. **Rollback Procedures**: Emergency response plans
3. **Post-Incident Analysis**: Root cause analysis
4. **Prevention Measures**: Implement preventive controls

## References

1. **OpenZeppelin Contracts**: https://github.com/OpenZeppelin/openzeppelin-contracts
2. **Solidity Documentation**: https://docs.soliditylang.org/
3. **Ethereum Smart Contract Best Practices**: https://consensys.github.io/smart-contract-best-practices/
4. **Slither Documentation**: https://github.com/crytic/slither
5. **Mythril Documentation**: https://mythril-classic.readthedocs.io/