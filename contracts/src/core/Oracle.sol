// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./SafeToken.sol";

/**
 * @title Oracle
 * @notice Oracle contract for price feeds with TWAP, medianization, and outlier detection
 */
contract Oracle {
    // Price feed structure
    struct PriceFeed {
        uint256 price;
        uint256 timestamp;
        bool isValid;
    }

    // TWAP data point
    struct TWAPDataPoint {
        uint256 price;
        uint256 timestamp;
    }

    // Events
    event PriceUpdated(address indexed token, uint256 price, uint256 timestamp);
    event FeedStatusUpdated(address indexed publisher, bool isValid);
    event EmergencyShutdown(bool isShutdown);
    event FallbackActivated(address indexed fallbackOracle);
    event ConnectorAllowlistUpdated(address indexed connector, bool isAllowed);

    // State variables
    mapping(address => mapping(address => PriceFeed[])) public priceFeeds; // token => publisher => feeds
    mapping(address => bool) public isValidPublisher;
    mapping(address => uint256) public lastValidPrice; // token => price
    mapping(address => TWAPDataPoint[]) public twapHistory; // token => TWAP history
    mapping(address => bool) public connectorAllowlist; // connector => isAllowed
    address[] public publishers;
    address public owner;
    bool public isEmergencyShutdown;
    address public fallbackOracle;
    uint256 public constant MAX_FEEDS = 1000;
    uint256 public constant TWAP_WINDOW = 30 minutes;
    uint256 public stalenessThreshold;
    uint256 public outlierThreshold; // Percentage deviation threshold (scaled by 1e18)
    uint256 public enhancedDeviationThreshold; // Enhanced deviation threshold (scaled by 1e18)

    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner");
        _;
    }

    modifier notShutdown() {
        require(!isEmergencyShutdown, "Oracle shutdown");
        _;
    }

    constructor(uint256 _stalenessThreshold, uint256 _outlierThreshold) {
        owner = msg.sender;
        stalenessThreshold = _stalenessThreshold;
        outlierThreshold = _outlierThreshold;
        enhancedDeviationThreshold = 2 * 1e18; // 2% enhanced deviation threshold
        isEmergencyShutdown = false;
    }

    /**
     * @notice Add a valid price publisher
     * @param publisher Publisher address
     */
    function addPublisher(address publisher) external onlyOwner {
        require(publisher != address(0), "Invalid publisher");
        require(!isValidPublisher[publisher], "Already a publisher");

        isValidPublisher[publisher] = true;
        publishers.push(publisher);

        emit FeedStatusUpdated(publisher, true);
    }

    /**
     * @notice Remove a price publisher
     * @param publisher Publisher address
     */
    function removePublisher(address publisher) external onlyOwner {
        require(isValidPublisher[publisher], "Not a publisher");

        isValidPublisher[publisher] = false;

        // Remove from publishers array
        for (uint256 i = 0; i < publishers.length; i++) {
            if (publishers[i] == publisher) {
                publishers[i] = publishers[publishers.length - 1];
                publishers.pop();
                break;
            }
        }

        emit FeedStatusUpdated(publisher, false);
    }

    /**
     * @notice Set fallback oracle
     * @param _fallbackOracle Fallback oracle address
     */
    function setFallbackOracle(address _fallbackOracle) external onlyOwner {
        fallbackOracle = _fallbackOracle;
        emit FallbackActivated(_fallbackOracle);
    }

    /**
     * @notice Set staleness threshold
     * @param _stalenessThreshold New staleness threshold
     */
    function setStalenessThreshold(uint256 _stalenessThreshold) external onlyOwner {
        stalenessThreshold = _stalenessThreshold;
    }

    /**
     * @notice Set outlier threshold
     * @param _outlierThreshold New outlier threshold (scaled by 1e18)
     */
    function setOutlierThreshold(uint256 _outlierThreshold) external onlyOwner {
        outlierThreshold = _outlierThreshold;
    }

    /**
     * @notice Set enhanced deviation threshold
     * @param _enhancedDeviationThreshold New enhanced deviation threshold (scaled by 1e18)
     */
    function setEnhancedDeviationThreshold(uint256 _enhancedDeviationThreshold) external onlyOwner {
        enhancedDeviationThreshold = _enhancedDeviationThreshold;
    }

    /**
     * @notice Add connector to allowlist
     * @param connector Connector address
     * @param isAllowed Whether connector is allowed
     */
    function updateConnectorAllowlist(address connector, bool isAllowed) external onlyOwner {
        connectorAllowlist[connector] = isAllowed;
        emit ConnectorAllowlistUpdated(connector, isAllowed);
    }

    /**
     * @notice Publish a price feed
     * @param token Token address
     * @param price Price (scaled by 1e18)
     */
    function publishPrice(address token, uint256 price) external notShutdown {
        require(isValidPublisher[msg.sender], "Not a valid publisher");
        require(connectorAllowlist[msg.sender], "Connector not allowed");
        require(token != address(0), "Invalid token");
        require(price > 0, "Price must be positive");

        // Add new price feed
        PriceFeed[] storage feeds = priceFeeds[token][msg.sender];
        require(feeds.length < MAX_FEEDS, "Too many feeds");

        feeds.push(PriceFeed({
            price: price,
            timestamp: block.timestamp,
            isValid: true
        }));

        // Update last valid price
        lastValidPrice[token] = price;

        // Add to TWAP history
        twapHistory[token].push(TWAPDataPoint({
            price: price,
            timestamp: block.timestamp
        }));

        // Clean up old TWAP data
        _cleanupTWAPHistory(token);

        emit PriceUpdated(token, price, block.timestamp);
    }

    /**
     * @notice Get the current price for a token
     * @param token Token address
     * @return price Current price (scaled by 1e18)
     * @return timestamp Timestamp of the price
     */
    function getPrice(address token) external view returns (uint256 price, uint256 timestamp) {
        if (isEmergencyShutdown) {
            revert("Oracle shutdown");
        }

        // Try to get median price from active publishers
        uint256[] memory prices = _getValidPrices(token);
        
        if (prices.length == 0) {
            // No valid prices, try fallback oracle
            if (fallbackOracle != address(0)) {
                try Oracle(fallbackOracle).getPrice(token) returns (uint256 fallbackPrice, uint256 fallbackTimestamp) {
                    return (fallbackPrice, fallbackTimestamp);
                } catch {
                    revert("No valid price available");
                }
            }
            revert("No valid price available");
        }

        // Calculate median price
        price = _calculateMedian(prices);
        timestamp = block.timestamp;

        // Check for staleness
        if (block.timestamp - timestamp > stalenessThreshold) {
            revert("Price is stale");
        }

        return (price, timestamp);
    }

    /**
     * @notice Get the TWAP price for a token
     * @param token Token address
     * @param window Time window for TWAP calculation
     * @return price TWAP price (scaled by 1e18)
     * @return timestamp Timestamp of the TWAP calculation
     */
    function getTWAPPrice(address token, uint256 window) external view returns (uint256 price, uint256 timestamp) {
        if (isEmergencyShutdown) {
            revert("Oracle shutdown");
        }

        TWAPDataPoint[] storage history = twapHistory[token];
        require(history.length > 0, "No TWAP history");

        uint256 endTime = block.timestamp;
        uint256 startTime = endTime - window;
        
        // Find the start index
        uint256 startIndex = 0;
        for (uint256 i = 0; i < history.length; i++) {
            if (history[i].timestamp >= startTime) {
                startIndex = i;
                break;
            }
        }

        // Calculate TWAP
        uint256 sum = 0;
        uint256 count = 0;
        
        for (uint256 i = startIndex; i < history.length; i++) {
            sum += history[i].price;
            count++;
        }

        require(count > 0, "No data points in window");
        
        price = sum / count;
        timestamp = endTime;

        return (price, timestamp);
    }

    /**
     * @notice Emergency shutdown of the oracle
     */
    function emergencyShutdown() external onlyOwner {
        isEmergencyShutdown = true;
        emit EmergencyShutdown(true);
    }

    /**
     * @notice Resume oracle operations after emergency shutdown
     */
    function resumeOperations() external onlyOwner {
        isEmergencyShutdown = false;
        emit EmergencyShutdown(false);
    }

    /**
     * @notice Get valid prices from active publishers
     * @param token Token address
     * @return prices Array of valid prices
     */
    function _getValidPrices(address token) internal view returns (uint256[] memory prices) {
        uint256 validCount = 0;
        
        // Count valid prices
        for (uint256 i = 0; i < publishers.length; i++) {
            address publisher = publishers[i];
            if (!isValidPublisher[publisher]) continue;
            if (!connectorAllowlist[publisher]) continue; // Check allowlist
            
            PriceFeed[] storage feeds = priceFeeds[token][publisher];
            if (feeds.length == 0) continue;
            
            PriceFeed storage latestFeed = feeds[feeds.length - 1];
            if (!latestFeed.isValid) continue;
            
            // Check for staleness
            if (block.timestamp - latestFeed.timestamp > stalenessThreshold) continue;
            
            validCount++;
        }
        
        // Collect valid prices
        prices = new uint256[](validCount);
        uint256 index = 0;
        
        for (uint256 i = 0; i < publishers.length; i++) {
            address publisher = publishers[i];
            if (!isValidPublisher[publisher]) continue;
            if (!connectorAllowlist[publisher]) continue; // Check allowlist
            
            PriceFeed[] storage feeds = priceFeeds[token][publisher];
            if (feeds.length == 0) continue;
            
            PriceFeed storage latestFeed = feeds[feeds.length - 1];
            if (!latestFeed.isValid) continue;
            
            // Check for staleness
            if (block.timestamp - latestFeed.timestamp > stalenessThreshold) continue;
            
            // Check for outliers using basic threshold
            if (lastValidPrice[token] > 0) {
                uint256 deviation = _calculateDeviation(latestFeed.price, lastValidPrice[token]);
                if (deviation > outlierThreshold) continue;
            }
            
            // Check for enhanced deviation using statistical methods
            if (!_isWithinEnhancedDeviationBounds(token, latestFeed.price)) continue;
            
            prices[index] = latestFeed.price;
            index++;
        }
        
        return prices;
    }

    /**
     * @notice Check if price is within enhanced deviation bounds using statistical methods
     * @param token Token address
     * @param price Price to check
     * @return isWithinBounds True if price is within enhanced deviation bounds
     */
    function _isWithinEnhancedDeviationBounds(address token, uint256 price) internal view returns (bool isWithinBounds) {
        // Collect recent prices for statistical analysis
        uint256[] memory recentPrices = new uint256[](publishers.length);
        uint256 validPriceCount = 0;
        
        for (uint256 i = 0; i < publishers.length; i++) {
            address publisher = publishers[i];
            if (!isValidPublisher[publisher]) continue;
            if (!connectorAllowlist[publisher]) continue; // Check allowlist
            
            PriceFeed[] storage feeds = priceFeeds[token][publisher];
            if (feeds.length == 0) continue;
            
            PriceFeed storage latestFeed = feeds[feeds.length - 1];
            if (!latestFeed.isValid) continue;
            
            // Check for staleness
            if (block.timestamp - latestFeed.timestamp > stalenessThreshold) continue;
            
            recentPrices[validPriceCount] = latestFeed.price;
            validPriceCount++;
        }
        
        // Need at least 3 prices for statistical analysis
        if (validPriceCount < 3) {
            return true; // Not enough data, allow the price
        }
        
        // Calculate mean and standard deviation
        uint256 sum = 0;
        for (uint256 i = 0; i < validPriceCount; i++) {
            sum += recentPrices[i];
        }
        uint256 mean = sum / validPriceCount;
        
        uint256 varianceSum = 0;
        for (uint256 i = 0; i < validPriceCount; i++) {
            uint256 diff = recentPrices[i] > mean ? recentPrices[i] - mean : mean - recentPrices[i];
            varianceSum += diff * diff;
        }
        uint256 variance = varianceSum / validPriceCount;
        
        // Calculate standard deviation (simplified)
        uint256 stdDev = _sqrt(variance);
        
        // Check if price is within enhanced deviation bounds (mean ± stdDev * threshold)
        uint256 lowerBound = mean > stdDev ? mean - stdDev : 0;
        uint256 upperBound = mean + stdDev;
        
        // Apply the enhanced deviation threshold multiplier
        uint256 thresholdAdjustedLower = (lowerBound * (1e18 - enhancedDeviationThreshold)) / 1e18;
        uint256 thresholdAdjustedUpper = (upperBound * (1e18 + enhancedDeviationThreshold)) / 1e18;
        
        return price >= thresholdAdjustedLower && price <= thresholdAdjustedUpper;
    }

    /**
     * @notice Calculate square root (simplified implementation)
     * @param x Number to calculate square root of
     * @return y Square root of x
     */
    function _sqrt(uint256 x) internal pure returns (uint256 y) {
        if (x == 0) return 0;
        y = x;
        uint256 z = (y + x / y) / 2;
        while (z < y) {
            y = z;
            z = (y + x / y) / 2;
        }
    }

    /**
     * @notice Calculate median of an array
     * @param array Array of values
     * @return median Median value
     */
    function _calculateMedian(uint256[] memory array) internal pure returns (uint256 median) {
        require(array.length > 0, "Empty array");
        
        // Sort array (bubble sort for simplicity, in production use more efficient algorithm)
        for (uint256 i = 0; i < array.length - 1; i++) {
            for (uint256 j = 0; j < array.length - i - 1; j++) {
                if (array[j] > array[j + 1]) {
                    uint256 temp = array[j];
                    array[j] = array[j + 1];
                    array[j + 1] = temp;
                }
            }
        }
        
        if (array.length % 2 == 0) {
            // Even number of elements
            median = (array[array.length / 2 - 1] + array[array.length / 2]) / 2;
        } else {
            // Odd number of elements
            median = array[array.length / 2];
        }
        
        return median;
    }

    /**
     * @notice Calculate deviation between two prices
     * @param price1 First price
     * @param price2 Second price
     * @return deviation Deviation percentage (scaled by 1e18)
     */
    function _calculateDeviation(uint256 price1, uint256 price2) internal pure returns (uint256 deviation) {
        if (price2 == 0) return 0;
        
        if (price1 > price2) {
            deviation = ((price1 - price2) * 1e18) / price2;
        } else {
            deviation = ((price2 - price1) * 1e18) / price2;
        }
        
        return deviation;
    }

    /**
     * @notice Clean up old TWAP history data
     * @param token Token address
     */
    function _cleanupTWAPHistory(address token) internal {
        TWAPDataPoint[] storage history = twapHistory[token];
        uint256 cutoffTime = block.timestamp - TWAP_WINDOW * 2; // Keep 2x window data
        
        // Find the first index that is within the cutoff time
        uint256 startIndex = 0;
        for (uint256 i = 0; i < history.length; i++) {
            if (history[i].timestamp >= cutoffTime) {
                startIndex = i;
                break;
            }
        }
        
        // Remove old data
        if (startIndex > 0) {
            for (uint256 i = 0; i < history.length - startIndex; i++) {
                history[i] = history[i + startIndex];
            }
            for (uint256 i = 0; i < startIndex; i++) {
                history.pop();
            }
        }
    }

    /**
     * @notice Get number of active publishers
     * @return count Number of active publishers
     */
    function getPublisherCount() external view returns (uint256 count) {
        for (uint256 i = 0; i < publishers.length; i++) {
            if (isValidPublisher[publishers[i]] && connectorAllowlist[publishers[i]]) {
                count++;
            }
        }
        return count;
    }

    /**
     * @notice Check if quorum is met (at least 2 publishers)
     * @return isQuorumMet True if quorum is met
     */
    function isQuorumMet(address token) external view returns (bool isQuorumMet) {
        uint256 validCount = 0;
        
        for (uint256 i = 0; i < publishers.length; i++) {
            address publisher = publishers[i];
            if (!isValidPublisher[publisher]) continue;
            if (!connectorAllowlist[publisher]) continue; // Check allowlist
            
            PriceFeed[] storage feeds = priceFeeds[token][publisher];
            if (feeds.length == 0) continue;
            
            PriceFeed storage latestFeed = feeds[feeds.length - 1];
            if (!latestFeed.isValid) continue;
            
            // Check for staleness
            if (block.timestamp - latestFeed.timestamp > stalenessThreshold) continue;
            
            validCount++;
        }
        
        return validCount >= 2;
    }
}