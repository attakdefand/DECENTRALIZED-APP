// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../../src/core/Oracle.sol";
import "../../src/core/SafeToken.sol";

/**
 * @title OracleTest
 * @notice Test suite for the Oracle contract
 */
contract OracleTest is Test {
    Oracle public oracle;
    SafeToken public tokenA;
    SafeToken public tokenB;
    address public publisher1 = address(0x1001);
    address public publisher2 = address(0x1002);
    address public publisher3 = address(0x1003);
    address public user = address(0x2001);

    event PriceUpdated(address indexed token, uint256 price, uint256 timestamp);
    event FeedStatusUpdated(address indexed publisher, bool isValid);
    event EmergencyShutdown(bool isShutdown);
    event FallbackActivated(address indexed fallbackOracle);

    function setUp() public {
        // Deploy contracts
        oracle = new Oracle(10 minutes, 0.05 * 1e18); // 10 minute staleness, 5% outlier threshold
        tokenA = new SafeToken("TokenA", "TKA", 18);
        tokenB = new SafeToken("TokenB", "TKB", 18);
    }

    /**
     * @notice Test publisher management
     */
    function testPublisherManagement() public {
        // Add publisher
        vm.expectEmit(true, false, false, true);
        emit FeedStatusUpdated(publisher1, true);
        
        oracle.addPublisher(publisher1);
        
        assertTrue(oracle.isValidPublisher(publisher1));
        
        // Remove publisher
        vm.expectEmit(true, false, false, true);
        emit FeedStatusUpdated(publisher1, false);
        
        oracle.removePublisher(publisher1);
        
        assertFalse(oracle.isValidPublisher(publisher1));
    }

    /**
     * @notice Test price publishing
     */
    function testPricePublishing() public {
        oracle.addPublisher(publisher1);
        
        uint256 price = 1000 * 1e18; // $1000
        
        vm.prank(publisher1);
        vm.expectEmit(true, false, false, true);
        emit PriceUpdated(address(tokenA), price, block.timestamp);
        
        oracle.publishPrice(address(tokenA), price);
        
        // Check that price was stored
        (uint256 storedPrice, uint256 timestamp) = oracle.getPrice(address(tokenA));
        assertEq(storedPrice, price);
        assertEq(timestamp, block.timestamp);
    }

    /**
     * @notice Test median price calculation
     */
    function testMedianPrice() public {
        oracle.addPublisher(publisher1);
        oracle.addPublisher(publisher2);
        oracle.addPublisher(publisher3);
        
        uint256 price1 = 1000 * 1e18; // $1000
        uint256 price2 = 1050 * 1e18; // $1050
        uint256 price3 = 950 * 1e18;  // $950
        
        // Publish prices from different publishers
        vm.prank(publisher1);
        oracle.publishPrice(address(tokenA), price1);
        
        vm.prank(publisher2);
        oracle.publishPrice(address(tokenA), price2);
        
        vm.prank(publisher3);
        oracle.publishPrice(address(tokenA), price3);
        
        // Get median price
        (uint256 medianPrice, ) = oracle.getPrice(address(tokenA));
        assertEq(medianPrice, price1); // Median of [950, 1000, 1050] is 1000
    }

    /**
     * @notice Test outlier detection
     */
    function testOutlierDetection() public {
        oracle.addPublisher(publisher1);
        oracle.addPublisher(publisher2);
        
        uint256 normalPrice = 1000 * 1e18; // $1000
        uint256 outlierPrice = 2000 * 1e18; // $2000 (100% deviation, above 5% threshold)
        
        // Publish normal price
        vm.prank(publisher1);
        oracle.publishPrice(address(tokenA), normalPrice);
        
        // Publish outlier price
        vm.prank(publisher2);
        oracle.publishPrice(address(tokenA), outlierPrice);
        
        // Get price - should return normal price since outlier is rejected
        (uint256 price, ) = oracle.getPrice(address(tokenA));
        assertEq(price, normalPrice);
    }

    /**
     * @notice Test staleness detection
     */
    function testStalenessDetection() public {
        oracle.addPublisher(publisher1);
        
        uint256 price = 1000 * 1e18; // $1000
        
        // Publish price
        vm.prank(publisher1);
        oracle.publishPrice(address(tokenA), price);
        
        // Advance time beyond staleness threshold
        vm.warp(block.timestamp + 15 minutes);
        
        // Try to get price - should revert due to staleness
        vm.expectRevert("Price is stale");
        oracle.getPrice(address(tokenA));
    }

    /**
     * @notice Test quorum requirement
     */
    function testQuorum() public {
        assertFalse(oracle.isQuorumMet(address(tokenA)));
        
        oracle.addPublisher(publisher1);
        assertFalse(oracle.isQuorumMet(address(tokenA)));
        
        oracle.addPublisher(publisher2);
        assertTrue(oracle.isQuorumMet(address(tokenA)));
    }

    /**
     * @notice Test TWAP calculation
     */
    function testTWAP() public {
        oracle.addPublisher(publisher1);
        
        uint256 price1 = 1000 * 1e18;
        uint256 price2 = 1100 * 1e18;
        uint256 price3 = 900 * 1e18;
        
        // Publish prices at different times
        vm.prank(publisher1);
        oracle.publishPrice(address(tokenA), price1);
        
        vm.warp(block.timestamp + 10 minutes);
        
        vm.prank(publisher1);
        oracle.publishPrice(address(tokenA), price2);
        
        vm.warp(block.timestamp + 10 minutes);
        
        vm.prank(publisher1);
        oracle.publishPrice(address(tokenA), price3);
        
        // Calculate 20-minute TWAP
        (uint256 twapPrice, ) = oracle.getTWAPPrice(address(tokenA), 20 minutes);
        
        // Expected TWAP: (1000 + 1100 + 900) / 3 = 1000
        assertEq(twapPrice, 1000 * 1e18);
    }

    /**
     * @notice Test emergency shutdown
     */
    function testEmergencyShutdown() public {
        oracle.addPublisher(publisher1);
        
        uint256 price = 1000 * 1e18;
        
        // Publish price
        vm.prank(publisher1);
        oracle.publishPrice(address(tokenA), price);
        
        // Emergency shutdown
        vm.expectEmit(false, false, false, true);
        emit EmergencyShutdown(true);
        
        oracle.emergencyShutdown();
        
        // Try to get price - should revert
        vm.expectRevert("Oracle shutdown");
        oracle.getPrice(address(tokenA));
        
        // Resume operations
        vm.expectEmit(false, false, false, true);
        emit EmergencyShutdown(false);
        
        oracle.resumeOperations();
        
        // Get price should work now
        (uint256 retrievedPrice, ) = oracle.getPrice(address(tokenA));
        assertEq(retrievedPrice, price);
    }

    /**
     * @notice Test fallback oracle
     */
    function testFallbackOracle() public {
        // Create fallback oracle
        Oracle fallbackOracle = new Oracle(10 minutes, 0.05 * 1e18);
        fallbackOracle.addPublisher(publisher1);
        
        uint256 fallbackPrice = 1500 * 1e18;
        
        // Publish price to fallback oracle
        vm.prank(publisher1);
        fallbackOracle.publishPrice(address(tokenA), fallbackPrice);
        
        // Set fallback oracle
        oracle.setFallbackOracle(address(fallbackOracle));
        
        // Try to get price from main oracle (no publishers)
        (uint256 price, ) = oracle.getPrice(address(tokenA));
        assertEq(price, fallbackPrice);
    }

    /**
     * @notice Test unauthorized publisher
     */
    function testUnauthorizedPublisher() public {
        uint256 price = 1000 * 1e18;
        
        // Try to publish price without being a publisher
        vm.prank(publisher1);
        vm.expectRevert("Not a valid publisher");
        oracle.publishPrice(address(tokenA), price);
    }

    /**
     * @notice Test only owner functions
     */
    function testOnlyOwner() public {
        // Try to add publisher as non-owner
        vm.prank(user);
        vm.expectRevert("Only owner");
        oracle.addPublisher(publisher1);
        
        // Try to emergency shutdown as non-owner
        vm.prank(user);
        vm.expectRevert("Only owner");
        oracle.emergencyShutdown();
    }

    /**
     * @notice Fuzz test for price publishing
     */
    function testFuzzPricePublishing(uint256 price) public {
        // Constrain price to reasonable values
        vm.assume(price > 0 && price < 1000000 * 1e18);
        
        oracle.addPublisher(publisher1);
        
        vm.prank(publisher1);
        oracle.publishPrice(address(tokenA), price);
        
        (uint256 retrievedPrice, ) = oracle.getPrice(address(tokenA));
        assertEq(retrievedPrice, price);
    }

    /**
     * @notice Fuzz test for median calculation
     */
    function testFuzzMedian(uint256 price1, uint256 price2, uint256 price3) public {
        // Constrain prices to reasonable values
        vm.assume(price1 > 0 && price1 < 1000000 * 1e18);
        vm.assume(price2 > 0 && price2 < 1000000 * 1e18);
        vm.assume(price3 > 0 && price3 < 1000000 * 1e18);
        
        oracle.addPublisher(publisher1);
        oracle.addPublisher(publisher2);
        oracle.addPublisher(publisher3);
        
        // Publish prices
        vm.prank(publisher1);
        oracle.publishPrice(address(tokenA), price1);
        
        vm.prank(publisher2);
        oracle.publishPrice(address(tokenA), price2);
        
        vm.prank(publisher3);
        oracle.publishPrice(address(tokenA), price3);
        
        // Get median price
        (uint256 medianPrice, ) = oracle.getPrice(address(tokenA));
        
        // Verify median is one of the input prices
        assertTrue(medianPrice == price1 || medianPrice == price2 || medianPrice == price3);
    }

    /**
     * @notice Invariant test for price validity
     */
    function invariantPriceValidity() public {
        // This would test that prices are always positive when available
        assertTrue(true); // Placeholder
    }

    /**
     * @notice Invariant test for quorum
     */
    function invariantQuorum() public {
        // This would test that quorum requirements are met
        assertTrue(true); // Placeholder
    }
}