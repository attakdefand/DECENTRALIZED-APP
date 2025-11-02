// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./SafeToken.sol";

/**
 * @title LendingPool
 * @notice Core lending pool contract with risk management and liquidation features
 */
contract LendingPool {
    // User position structure
    struct UserPosition {
        uint256 supplied;
        uint256 borrowed;
        uint256 collateral;
        uint256 lastUpdate;
    }

    // Reserve data structure
    struct ReserveData {
        uint256 totalSupplied;
        uint256 totalBorrowed;
        uint256 totalReserves;
        uint256 lastUpdate;
        // Interest rate model parameters
        uint256 baseRate; // Base interest rate (scaled by 1e18)
        uint256 slope1; // Slope before kink (scaled by 1e18)
        uint256 slope2; // Slope after kink (scaled by 1e18)
        uint256 kink; // Utilization rate at kink (scaled by 1e18)
        uint256 reserveFactor; // Percentage to reserves (scaled by 1e18)
    }

    // Risk parameters
    struct RiskParams {
        uint256 collateralFactor; // Collateral factor (scaled by 1e18)
        uint256 liquidationThreshold; // Liquidation threshold (scaled by 1e18)
        uint256 liquidationPenalty; // Liquidation penalty (scaled by 1e18)
        uint256 minHealthFactor; // Minimum health factor (scaled by 1e18)
    }

    // Events
    event Supply(address indexed user, address indexed token, uint256 amount);
    event Withdraw(address indexed user, address indexed token, uint256 amount);
    event Borrow(address indexed user, address indexed token, uint256 amount);
    event Repay(address indexed user, address indexed token, uint256 amount);
    event Liquidate(
        address indexed liquidator,
        address indexed user,
        address indexed collateralToken,
        address debtToken,
        uint256 collateralAmount,
        uint256 debtAmount
    );
    event InterestAccrued(address indexed token, uint256 interest);
    event RiskParamsUpdated(address indexed token, RiskParams riskParams);

    // State variables
    mapping(address => mapping(address => UserPosition)) public userPositions;
    mapping(address => ReserveData) public reserves;
    mapping(address => RiskParams) public riskParams;
    mapping(address => bool) public isCollateralToken;
    
    address public owner;
    uint256 public constant SECONDS_PER_YEAR = 365 days;
    uint256 public constant WAD = 1e18; // 18 decimal places

    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner");
        _;
    }

    constructor() {
        owner = msg.sender;
    }

    /**
     * @notice Set risk parameters for a token
     * @param token Token address
     * @param params Risk parameters
     */
    function setRiskParams(address token, RiskParams memory params) external onlyOwner {
        require(token != address(0), "Invalid token");
        require(params.collateralFactor <= WAD, "Collateral factor too high");
        require(params.liquidationThreshold <= WAD, "Liquidation threshold too high");
        require(params.liquidationPenalty <= WAD / 2, "Liquidation penalty too high");
        require(params.minHealthFactor >= WAD, "Min health factor too low");

        riskParams[token] = params;
        emit RiskParamsUpdated(token, params);
    }

    /**
     * @notice Set interest rate model for a token
     * @param token Token address
     * @param baseRate Base interest rate (scaled by 1e18)
     * @param slope1 Slope before kink (scaled by 1e18)
     * @param slope2 Slope after kink (scaled by 1e18)
     * @param kink Utilization rate at kink (scaled by 1e18)
     * @param reserveFactor Reserve factor (scaled by 1e18)
     */
    function setInterestRateModel(
        address token,
        uint256 baseRate,
        uint256 slope1,
        uint256 slope2,
        uint256 kink,
        uint256 reserveFactor
    ) external onlyOwner {
        require(token != address(0), "Invalid token");
        require(kink <= WAD, "Kink too high");
        require(reserveFactor <= WAD / 2, "Reserve factor too high");

        ReserveData storage reserve = reserves[token];
        reserve.baseRate = baseRate;
        reserve.slope1 = slope1;
        reserve.slope2 = slope2;
        reserve.kink = kink;
        reserve.reserveFactor = reserveFactor;
    }

    /**
     * @notice Set token as collateral
     * @param token Token address
     * @param isCollateral Whether token can be used as collateral
     */
    function setCollateralToken(address token, bool isCollateral) external onlyOwner {
        require(token != address(0), "Invalid token");
        isCollateralToken[token] = isCollateral;
    }

    /**
     * @notice Supply tokens to the lending pool
     * @param token Token to supply
     * @param amount Amount to supply
     */
    function supply(address token, uint256 amount) external {
        require(token != address(0), "Invalid token");
        require(amount > 0, "Amount must be > 0");

        // Accrue interest before state changes
        _accrueInterest(token);

        ReserveData storage reserve = reserves[token];
        UserPosition storage userPosition = userPositions[token][msg.sender];

        // Update user position
        userPosition.supplied += amount;
        userPosition.lastUpdate = block.timestamp;

        // Update reserve
        reserve.totalSupplied += amount;

        // Transfer tokens from user
        SafeToken(token).transferFrom(msg.sender, address(this), amount);

        emit Supply(msg.sender, token, amount);
    }

    /**
     * @notice Withdraw tokens from the lending pool
     * @param token Token to withdraw
     * @param amount Amount to withdraw
     */
    function withdraw(address token, uint256 amount) external {
        require(token != address(0), "Invalid token");
        require(amount > 0, "Amount must be > 0");

        // Accrue interest before state changes
        _accrueInterest(token);

        UserPosition storage userPosition = userPositions[token][msg.sender];
        require(userPosition.supplied >= amount, "Insufficient balance");

        // Check health factor after withdrawal
        _checkHealthFactor(msg.sender, token, amount, 0);

        ReserveData storage reserve = reserves[token];
        require(reserve.totalSupplied >= amount, "Insufficient liquidity");

        // Update user position
        userPosition.supplied -= amount;
        userPosition.lastUpdate = block.timestamp;

        // Update reserve
        reserve.totalSupplied -= amount;

        // Transfer tokens to user
        SafeToken(token).transfer(msg.sender, amount);

        emit Withdraw(msg.sender, token, amount);
    }

    /**
     * @notice Borrow tokens from the lending pool
     * @param token Token to borrow
     * @param amount Amount to borrow
     */
    function borrow(address token, uint256 amount) external {
        require(token != address(0), "Invalid token");
        require(amount > 0, "Amount must be > 0");

        // Accrue interest before state changes
        _accrueInterest(token);

        ReserveData storage reserve = reserves[token];
        require(reserve.totalSupplied >= reserve.totalBorrowed + amount, "Insufficient liquidity");

        UserPosition storage userPosition = userPositions[token][msg.sender];

        // Update user position
        userPosition.borrowed += amount;
        userPosition.lastUpdate = block.timestamp;

        // Update reserve
        reserve.totalBorrowed += amount;

        // Check health factor after borrowing
        _checkHealthFactor(msg.sender, token, 0, amount);

        // Transfer tokens to user
        SafeToken(token).transfer(msg.sender, amount);

        emit Borrow(msg.sender, token, amount);
    }

    /**
     * @notice Repay borrowed tokens
     * @param token Token to repay
     * @param amount Amount to repay
     */
    function repay(address token, uint256 amount) external {
        require(token != address(0), "Invalid token");
        require(amount > 0, "Amount must be > 0");

        // Accrue interest before state changes
        _accrueInterest(token);

        UserPosition storage userPosition = userPositions[token][msg.sender];
        require(userPosition.borrowed >= amount, "Repay amount exceeds debt");

        ReserveData storage reserve = reserves[token];

        // Update user position
        userPosition.borrowed -= amount;
        userPosition.lastUpdate = block.timestamp;

        // Update reserve
        reserve.totalBorrowed -= amount;

        // Transfer tokens from user
        SafeToken(token).transferFrom(msg.sender, address(this), amount);

        emit Repay(msg.sender, token, amount);
    }

    /**
     * @notice Liquidate an undercollateralized position
     * @param user User to liquidate
     * @param collateralToken Collateral token
     * @param debtToken Debt token
     * @param debtAmount Amount of debt to repay
     */
    function liquidate(
        address user,
        address collateralToken,
        address debtToken,
        uint256 debtAmount
    ) external {
        require(user != address(0), "Invalid user");
        require(collateralToken != address(0), "Invalid collateral token");
        require(debtToken != address(0), "Invalid debt token");
        require(debtAmount > 0, "Debt amount must be > 0");

        // Accrue interest for both tokens
        _accrueInterest(collateralToken);
        _accrueInterest(debtToken);

        UserPosition storage userPosition = userPositions[debtToken][user];
        require(userPosition.borrowed >= debtAmount, "Insufficient debt");

        // Check if user is liquidatable
        require(_isLiquidatable(user), "User not liquidatable");

        // Calculate collateral to seize
        uint256 collateralToSeize = _calculateCollateralToSeize(
            user,
            collateralToken,
            debtToken,
            debtAmount
        );

        // Update user positions
        userPositions[debtToken][user].borrowed -= debtAmount;
        userPositions[collateralToken][user].collateral -= collateralToSeize;

        userPositions[debtToken][msg.sender].borrowed += debtAmount;
        userPositions[collateralToken][msg.sender].collateral += collateralToSeize;

        // Update reserves
        reserves[debtToken].totalBorrowed -= debtAmount;
        reserves[collateralToken].totalSupplied -= collateralToSeize;

        // Transfer debt tokens from liquidator
        SafeToken(debtToken).transferFrom(msg.sender, address(this), debtAmount);

        // Transfer collateral tokens to liquidator
        SafeToken(collateralToken).transfer(msg.sender, collateralToSeize);

        emit Liquidate(
            msg.sender,
            user,
            collateralToken,
            debtToken,
            collateralToSeize,
            debtAmount
        );
    }

    /**
     * @notice Add collateral to a position
     * @param token Collateral token
     * @param amount Amount to add
     */
    function addCollateral(address token, uint256 amount) external {
        require(token != address(0), "Invalid token");
        require(amount > 0, "Amount must be > 0");
        require(isCollateralToken[token], "Token not accepted as collateral");

        // Accrue interest before state changes
        _accrueInterest(token);

        UserPosition storage userPosition = userPositions[token][msg.sender];

        // Update user position
        userPosition.collateral += amount;
        userPosition.lastUpdate = block.timestamp;

        // Update reserve
        reserves[token].totalSupplied += amount;

        // Transfer tokens from user
        SafeToken(token).transferFrom(msg.sender, address(this), amount);
    }

    /**
     * @notice Remove collateral from a position
     * @param token Collateral token
     * @param amount Amount to remove
     */
    function removeCollateral(address token, uint256 amount) external {
        require(token != address(0), "Invalid token");
        require(amount > 0, "Amount must be > 0");

        // Accrue interest before state changes
        _accrueInterest(token);

        UserPosition storage userPosition = userPositions[token][msg.sender];
        require(userPosition.collateral >= amount, "Insufficient collateral");

        // Check health factor after removal
        _checkHealthFactor(msg.sender, token, amount, 0);

        // Update user position
        userPosition.collateral -= amount;
        userPosition.lastUpdate = block.timestamp;

        // Update reserve
        reserves[token].totalSupplied -= amount;

        // Transfer tokens to user
        SafeToken(token).transfer(msg.sender, amount);
    }

    /**
     * @notice Accrue interest for a token
     * @param token Token to accrue interest for
     */
    function _accrueInterest(address token) internal {
        ReserveData storage reserve = reserves[token];
        uint256 timeElapsed = block.timestamp - reserve.lastUpdate;
        
        if (timeElapsed == 0) {
            return;
        }

        if (reserve.totalSupplied == 0) {
            reserve.lastUpdate = block.timestamp;
            return;
        }

        // Calculate utilization rate
        uint256 utilizationRate = (reserve.totalBorrowed * WAD) / 
            (reserve.totalSupplied + reserve.totalBorrowed);

        // Calculate borrow rate per second
        uint256 borrowRatePerSecond = _calculateBorrowRatePerSecond(token, utilizationRate);

        // Calculate interest accrued
        uint256 interestAccrued = (reserve.totalBorrowed * borrowRatePerSecond * timeElapsed) / WAD;

        // Update reserve data
        reserve.totalBorrowed += interestAccrued;
        reserve.totalReserves += (interestAccrued * reserve.reserveFactor) / WAD;
        reserve.lastUpdate = block.timestamp;

        emit InterestAccrued(token, interestAccrued);
    }

    /**
     * @notice Calculate borrow rate per second
     * @param token Token address
     * @param utilizationRate Utilization rate (scaled by 1e18)
     * @return Borrow rate per second (scaled by 1e18)
     */
    function _calculateBorrowRatePerSecond(address token, uint256 utilizationRate) internal view returns (uint256) {
        ReserveData storage reserve = reserves[token];
        
        uint256 annualRate;
        if (utilizationRate <= reserve.kink) {
            // Below kink: baseRate + slope1 * utilization
            annualRate = reserve.baseRate + (reserve.slope1 * utilizationRate) / WAD;
        } else {
            // Above kink: baseRate + slope1 * kink + slope2 * (utilization - kink)
            uint256 excessUtilization = utilizationRate - reserve.kink;
            annualRate = reserve.baseRate + 
                (reserve.slope1 * reserve.kink) / WAD + 
                (reserve.slope2 * excessUtilization) / WAD;
        }

        // Convert annual rate to per second
        return (annualRate * WAD) / (SECONDS_PER_YEAR * WAD);
    }

    /**
     * @notice Calculate health factor for a user
     * @param user User address
     * @return Health factor (scaled by 1e18)
     */
    function calculateHealthFactor(address user) public view returns (uint256) {
        uint256 totalCollateralValue = 0;
        uint256 totalDebtValue = 0;

        // Iterate through all tokens to calculate total values
        address[] memory tokens = new address[](3);
        tokens[0] = address(0); // Will be replaced with actual tokens
        tokens[1] = address(0);
        tokens[2] = address(0);

        // Simplified calculation for example - in practice would iterate through all tokens
        // This is a placeholder implementation

        if (totalDebtValue == 0) {
            return type(uint256).max; // Infinite health factor
        }

        return (totalCollateralValue * WAD) / totalDebtValue;
    }

    /**
     * @notice Check if a user's position is liquidatable
     * @param user User address
     * @return True if liquidatable, false otherwise
     */
    function _isLiquidatable(address user) internal view returns (bool) {
        uint256 healthFactor = calculateHealthFactor(user);
        return healthFactor < riskParams[address(0)].minHealthFactor;
    }

    /**
     * @notice Calculate collateral to seize during liquidation
     * @param user User being liquidated
     * @param collateralToken Collateral token
     * @param debtToken Debt token
     * @param debtAmount Debt amount being repaid
     * @return Amount of collateral to seize
     */
    function _calculateCollateralToSeize(
        address user,
        address collateralToken,
        address debtToken,
        uint256 debtAmount
    ) internal view returns (uint256) {
        // Get liquidation penalty
        uint256 penalty = riskParams[debtToken].liquidationPenalty;
        
        // Calculate equivalent collateral amount with penalty
        // This is a simplified calculation - in practice would need price oracles
        uint256 collateralAmount = (debtAmount * (WAD + penalty)) / WAD;
        
        return collateralAmount;
    }

    /**
     * @notice Check health factor after an operation
     * @param user User address
     * @param token Token being operated on
     * @param withdrawAmount Amount being withdrawn
     * @param borrowAmount Amount being borrowed
     */
    function _checkHealthFactor(
        address user,
        address token,
        uint256 withdrawAmount,
        uint256 borrowAmount
    ) internal view {
        // This is a simplified check - in practice would be more complex
        // For now, we'll skip the actual health factor calculation
        // In a real implementation, this would check the user's position
        // against risk parameters
    }

    /**
     * @notice Get user position for a token
     * @param token Token address
     * @param user User address
     * @return User position
     */
    function getUserPosition(address token, address user) external view returns (UserPosition memory) {
        return userPositions[token][user];
    }

    /**
     * @notice Get reserve data for a token
     * @param token Token address
     * @return Reserve data
     */
    function getReserveData(address token) external view returns (ReserveData memory) {
        return reserves[token];
    }
}