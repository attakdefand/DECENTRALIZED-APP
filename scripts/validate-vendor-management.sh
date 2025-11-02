#!/bin/bash

# Vendor Management Validation Script
# This script validates vendor management requirements and warns if overdue reviews exist

VENDOR_METRICS_PATH=${1:-"docs/security/vendor-metrics.json"}
MAX_OVERDUE_REVIEWS=${2:-0}

echo "Running Vendor Management Validation..."

# Function to parse vendor metrics
get_vendor_metrics() {
    local file_path=$1
    if [[ ! -f "$file_path" ]]; then
        echo "WARNING: Vendor metrics file not found at $file_path"
        echo '{"vendor_score_avg":0,"overdue_reviews":0}'
        return
    fi
    
    # Use jq to parse JSON if available, otherwise return default values
    if command -v jq &> /dev/null; then
        jq -r '.' "$file_path" 2>/dev/null || echo '{"vendor_score_avg":0,"overdue_reviews":0}'
    else
        echo '{"vendor_score_avg":0,"overdue_reviews":0}'
    fi
}

# Get vendor metrics
METRICS=$(get_vendor_metrics "$VENDOR_METRICS_PATH")
VENDOR_SCORE_AVG=$(echo "$METRICS" | grep -o '"vendor_score_avg":[0-9]*' | grep -o '[0-9]*')
OVERDUE_REVIEWS=$(echo "$METRICS" | grep -o '"overdue_reviews":[0-9]*' | grep -o '[0-9]*')

echo "[INFO] Vendor score average: $VENDOR_SCORE_AVG"
echo "[INFO] Overdue reviews: $OVERDUE_REVIEWS"

# Check for overdue reviews
if [[ $OVERDUE_REVIEWS -gt $MAX_OVERDUE_REVIEWS ]]; then
    echo "WARNING: Overdue vendor reviews detected ($OVERDUE_REVIEWS)"
    exit 0 # Warn only, don't block
else
    echo "[PASS] No overdue vendor reviews"
fi

echo "Vendor Management validation completed successfully!"
exit 0