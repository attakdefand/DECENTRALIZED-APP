#!/bin/bash

# Synthetic API Functionality Test Script
# This script performs more comprehensive tests of API functionality

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Starting API Functionality Tests...${NC}"

# Counter for passed/failed tests
PASSED=0
FAILED=0

# Test root endpoint
echo -e "${YELLOW}Testing root endpoint${NC}"
if response=$(curl -s -w "%{http_code}" -o /tmp/response.txt "http://localhost:3000/"); then
  if [ "$response" -eq 200 ]; then
    content=$(cat /tmp/response.txt)
    if [[ "$content" == *"Decentralized Application API Service"* ]]; then
      echo -e "${GREEN}✓ Root endpoint returned expected content${NC}"
      ((PASSED++))
    else
      echo -e "${RED}✗ Root endpoint returned unexpected content: $content${NC}"
      ((FAILED++))
    fi
  else
    echo -e "${RED}✗ Root endpoint returned status $response${NC}"
    ((FAILED++))
  fi
else
  echo -e "${RED}✗ Root endpoint failed to respond${NC}"
  ((FAILED++))
fi

# Test health endpoint
echo -e "${YELLOW}Testing health endpoint${NC}"
if response=$(curl -s -w "%{http_code}" -o /tmp/response.txt "http://localhost:3000/health"); then
  if [ "$response" -eq 200 ]; then
    content=$(cat /tmp/response.txt)
    if [[ "$content" == "OK" ]]; then
      echo -e "${GREEN}✓ Health endpoint returned expected content${NC}"
      ((PASSED++))
    else
      echo -e "${RED}✗ Health endpoint returned unexpected content: $content${NC}"
      ((FAILED++))
    fi
  else
    echo -e "${RED}✗ Health endpoint returned status $response${NC}"
    ((FAILED++))
  fi
else
  echo -e "${RED}✗ Health endpoint failed to respond${NC}"
  ((FAILED++))
fi

# Test pools endpoint
echo -e "${YELLOW}Testing pools endpoint${NC}"
if response=$(curl -s -w "%{http_code}" -o /tmp/response.txt "http://localhost:3000/api/v1/pools"); then
  if [ "$response" -eq 200 ]; then
    content=$(cat /tmp/response.txt)
    if [[ "$content" == *"Pool data would be returned here"* ]]; then
      echo -e "${GREEN}✓ Pools endpoint returned expected content${NC}"
      ((PASSED++))
    else
      echo -e "${RED}✗ Pools endpoint returned unexpected content: $content${NC}"
      ((FAILED++))
    fi
  else
    echo -e "${RED}✗ Pools endpoint returned status $response${NC}"
    ((FAILED++))
  fi
else
  echo -e "${RED}✗ Pools endpoint failed to respond${NC}"
  ((FAILED++))
fi

# Test orders endpoint
echo -e "${YELLOW}Testing orders endpoint${NC}"
if response=$(curl -s -w "%{http_code}" -o /tmp/response.txt "http://localhost:3000/api/v1/orders"); then
  if [ "$response" -eq 200 ]; then
    content=$(cat /tmp/response.txt)
    if [[ "$content" == *"Order data would be returned here"* ]]; then
      echo -e "${GREEN}✓ Orders endpoint returned expected content${NC}"
      ((PASSED++))
    else
      echo -e "${RED}✗ Orders endpoint returned unexpected content: $content${NC}"
      ((FAILED++))
    fi
  else
    echo -e "${RED}✗ Orders endpoint returned status $response${NC}"
    ((FAILED++))
  fi
else
  echo -e "${RED}✗ Orders endpoint failed to respond${NC}"
  ((FAILED++))
fi

# Test markets endpoint
echo -e "${YELLOW}Testing markets endpoint${NC}"
if response=$(curl -s -w "%{http_code}" -o /tmp/response.txt "http://localhost:3000/api/v1/markets"); then
  if [ "$response" -eq 200 ]; then
    content=$(cat /tmp/response.txt)
    if [[ "$content" == *"Market data would be returned here"* ]]; then
      echo -e "${GREEN}✓ Markets endpoint returned expected content${NC}"
      ((PASSED++))
    else
      echo -e "${RED}✗ Markets endpoint returned unexpected content: $content${NC}"
      ((FAILED++))
    fi
  else
    echo -e "${RED}✗ Markets endpoint returned status $response${NC}"
    ((FAILED++))
  fi
else
  echo -e "${RED}✗ Markets endpoint failed to respond${NC}"
  ((FAILED++))
fi

# Test metrics endpoint
echo -e "${YELLOW}Testing metrics endpoint${NC}"
if response=$(curl -s -w "%{http_code}" -o /tmp/response.txt "http://localhost:3000/metrics"); then
  if [ "$response" -eq 200 ]; then
    content=$(cat /tmp/response.txt)
    if [[ "$content" == *"http_requests_total"* ]] && [[ "$content" == *"http_request_duration_seconds"* ]]; then
      echo -e "${GREEN}✓ Metrics endpoint contains expected metrics${NC}"
      ((PASSED++))
    else
      echo -e "${RED}✗ Metrics endpoint missing expected metrics${NC}"
      ((FAILED++))
    fi
  else
    echo -e "${RED}✗ Metrics endpoint returned status $response${NC}"
    ((FAILED++))
  fi
else
  echo -e "${RED}✗ Metrics endpoint failed to respond${NC}"
  ((FAILED++))
fi

# Summary
echo -e "${YELLOW}Functionality Test Summary:${NC}"
echo -e "${GREEN}Passed: $PASSED${NC}"
echo -e "${RED}Failed: $FAILED${NC}"

if [ $FAILED -eq 0 ]; then
  echo -e "${GREEN}All functionality tests passed!${NC}"
  exit 0
else
  echo -e "${RED}Some functionality tests failed!${NC}"
  exit 1
fi