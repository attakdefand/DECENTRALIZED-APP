#!/bin/bash

# Script to run synthetic tests and push results to Prometheus Pushgateway
# This script should be run periodically (e.g., via cron)

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Running synthetic tests and pushing metrics...${NC}"

# Pushgateway endpoint
PUSHGATEWAY_URL="http://localhost:9091/metrics/job/synthetic_tests"

# Run health check test
echo -e "${YELLOW}Running health check test...${NC}"
if ./api_health_check.sh > /tmp/health_check.log 2>&1; then
  HEALTH_STATUS=1
  echo -e "${GREEN}Health check passed${NC}"
else
  HEALTH_STATUS=0
  echo -e "${RED}Health check failed${NC}"
fi

# Run functionality test
echo -e "${YELLOW}Running functionality test...${NC}"
if ./api_functionality_test.sh > /tmp/functionality_test.log 2>&1; then
  FUNCTIONALITY_STATUS=1
  echo -e "${GREEN}Functionality test passed${NC}"
else
  FUNCTIONALITY_STATUS=0
  echo -e "${RED}Functionality test failed${NC}"
fi

# Run performance test
echo -e "${YELLOW}Running performance test...${NC}"
if ./api_performance_test.sh > /tmp/performance_test.log 2>&1; then
  PERFORMANCE_STATUS=1
  echo -e "${GREEN}Performance test passed${NC}"
else
  PERFORMANCE_STATUS=0
  echo -e "${RED}Performance test failed${NC}"
fi

# Create metrics data
cat > /tmp/synthetic_metrics.txt <<EOF
# HELP synthetic_test_success Status of synthetic tests (1 = success, 0 = failure)
# TYPE synthetic_test_success gauge
synthetic_test_success{test="health_check"} $HEALTH_STATUS
synthetic_test_success{test="functionality"} $FUNCTIONALITY_STATUS
synthetic_test_success{test="performance"} $PERFORMANCE_STATUS
EOF

# Push metrics to Pushgateway
echo -e "${YELLOW}Pushing metrics to Pushgateway...${NC}"
if curl -s -X POST --data-binary @/tmp/synthetic_metrics.txt $PUSHGATEWAY_URL; then
  echo -e "${GREEN}Metrics pushed successfully${NC}"
else
  echo -e "${RED}Failed to push metrics${NC}"
  exit 1
fi

# Cleanup
rm -f /tmp/synthetic_metrics.txt /tmp/health_check.log /tmp/functionality_test.log /tmp/performance_test.log

echo -e "${GREEN}Synthetic tests completed and metrics pushed!${NC}"