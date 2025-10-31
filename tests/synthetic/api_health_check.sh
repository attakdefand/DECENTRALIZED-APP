#!/bin/bash

# Synthetic API Health Check Script
# This script performs basic health checks on all API endpoints

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Starting API Health Checks...${NC}"

# API endpoints to test
API_ENDPOINTS=(
  "http://localhost:3000/health"
  "http://localhost:3000/"
  "http://localhost:3000/api/v1/pools"
  "http://localhost:3000/api/v1/orders"
  "http://localhost:3000/api/v1/markets"
)

# Metrics endpoint
METRICS_ENDPOINT="http://localhost:3000/metrics"

# Counter for passed/failed tests
PASSED=0
FAILED=0

# Test each API endpoint
for endpoint in "${API_ENDPOINTS[@]}"; do
  echo -e "${YELLOW}Testing $endpoint${NC}"
  
  # Try to curl the endpoint
  if response=$(curl -s -o /dev/null -w "%{http_code}" "$endpoint"); then
    if [ "$response" -eq 200 ]; then
      echo -e "${GREEN}✓ $endpoint returned status 200${NC}"
      ((PASSED++))
    else
      echo -e "${RED}✗ $endpoint returned status $response${NC}"
      ((FAILED++))
    fi
  else
    echo -e "${RED}✗ $endpoint failed to respond${NC}"
    ((FAILED++))
  fi
done

# Test metrics endpoint
echo -e "${YELLOW}Testing $METRICS_ENDPOINT${NC}"
if response=$(curl -s -o /dev/null -w "%{http_code}" "$METRICS_ENDPOINT"); then
  if [ "$response" -eq 200 ]; then
    echo -e "${GREEN}✓ $METRICS_ENDPOINT returned status 200${NC}"
    ((PASSED++))
  else
    echo -e "${RED}✗ $METRICS_ENDPOINT returned status $response${NC}"
    ((FAILED++))
  fi
else
  echo -e "${RED}✗ $METRICS_ENDPOINT failed to respond${NC}"
  ((FAILED++))
fi

# Summary
echo -e "${YELLOW}Health Check Summary:${NC}"
echo -e "${GREEN}Passed: $PASSED${NC}"
echo -e "${RED}Failed: $FAILED${NC}"

if [ $FAILED -eq 0 ]; then
  echo -e "${GREEN}All health checks passed!${NC}"
  exit 0
else
  echo -e "${RED}Some health checks failed!${NC}"
  exit 1
fi