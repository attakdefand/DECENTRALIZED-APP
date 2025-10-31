#!/bin/bash

# Synthetic API Performance Test Script
# This script performs performance and latency tests on API endpoints

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Starting API Performance Tests...${NC}"

# Counter for passed/failed tests
PASSED=0
FAILED=0

# Test endpoints with curl timing
API_ENDPOINTS=(
  "http://localhost:3000/"
  "http://localhost:3000/health"
  "http://localhost:3000/api/v1/pools"
  "http://localhost:3000/api/v1/orders"
  "http://localhost:3000/api/v1/markets"
)

# Function to test endpoint performance
test_endpoint_performance() {
  local endpoint=$1
  local max_time=$2
  
  echo -e "${YELLOW}Testing performance of $endpoint (max time: ${max_time}ms)${NC}"
  
  # Use curl to get timing information
  local timing=$(curl -s -o /dev/null -w "%{time_total}" "$endpoint")
  
  # Convert to milliseconds (curl returns seconds with decimal)
  local time_ms=$(echo "$timing * 1000" | bc -l | xargs printf "%.0f")
  
  if [ "$time_ms" -lt "$max_time" ]; then
    echo -e "${GREEN}✓ $endpoint responded in ${time_ms}ms (under ${max_time}ms limit)${NC}"
    ((PASSED++))
  else
    echo -e "${RED}✗ $endpoint responded in ${time_ms}ms (over ${max_time}ms limit)${NC}"
    ((FAILED++))
  fi
}

# Test each endpoint
for endpoint in "${API_ENDPOINTS[@]}"; do
  test_endpoint_performance "$endpoint" 500
done

# Test concurrent requests
echo -e "${YELLOW}Testing concurrent requests...${NC}"
start_time=$(date +%s%3N)
for i in {1..10}; do
  curl -s -o /dev/null "http://localhost:3000/" &
done
wait
end_time=$(date +%s%3N)
duration=$((end_time - start_time))

if [ "$duration" -lt 2000 ]; then
  echo -e "${GREEN}✓ 10 concurrent requests completed in ${duration}ms${NC}"
  ((PASSED++))
else
  echo -e "${RED}✗ 10 concurrent requests took ${duration}ms${NC}"
  ((FAILED++))
fi

# Summary
echo -e "${YELLOW}Performance Test Summary:${NC}"
echo -e "${GREEN}Passed: $PASSED${NC}"
echo -e "${RED}Failed: $FAILED${NC}"

if [ $FAILED -eq 0 ]; then
  echo -e "${GREEN}All performance tests passed!${NC}"
  exit 0
else
  echo -e "${RED}Some performance tests failed!${NC}"
  exit 1
fi