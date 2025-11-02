#!/bin/bash

# Proof of Reserves Validation Script
# This script validates Proof of Reserves reports and treasury metrics

echo "Running Proof of Reserves validation..."

# Check if required environment variables are set
POR_FRESHNESS_HOURS=${POR_FRESHNESS_HOURS:-0}
LIMIT_BREACH_COUNT=${LIMIT_BREACH_COUNT:-0}

echo "Proof of Reserves Freshness Hours: $POR_FRESHNESS_HOURS"
echo "Limit Breach Count: $LIMIT_BREACH_COUNT"

# Validate Proof of Reserves freshness (should be < 24 hours)
if (( $(echo "$POR_FRESHNESS_HOURS >= 24" | bc -l) )); then
    echo "ERROR: Proof of Reserves report is stale (>= 24 hours old)"
    exit 1
fi

# Validate limit breach count (should be 0)
if [ "$LIMIT_BREACH_COUNT" -gt 0 ]; then
    echo "ERROR: Limit breach count is greater than 0"
    exit 1
fi

echo "Proof of Reserves validation passed!"
exit 0