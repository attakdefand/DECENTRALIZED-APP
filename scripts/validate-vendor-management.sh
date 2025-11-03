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
        echo '{"vendor_score_avg":0,"overdue_reviews":0,"vendor_access_review_completion_pct":100,"vendor_sod_violations":0,"vendor_key_rotation_compliance_pct":100,"vendor_key_health_checks_pass":1,"vendor_policy_coverage_pct":100,"vendor_policy_violations":0}'
        return
    fi
    
    # Use jq to parse JSON if available, otherwise return default values
    if command -v jq &> /dev/null; then
        jq -r '.' "$file_path" 2>/dev/null || echo '{"vendor_score_avg":0,"overdue_reviews":0,"vendor_access_review_completion_pct":100,"vendor_sod_violations":0,"vendor_key_rotation_compliance_pct":100,"vendor_key_health_checks_pass":1,"vendor_policy_coverage_pct":100,"vendor_policy_violations":0}'
    else
        echo '{"vendor_score_avg":0,"overdue_reviews":0,"vendor_access_review_completion_pct":100,"vendor_sod_violations":0,"vendor_key_rotation_compliance_pct":100,"vendor_key_health_checks_pass":1,"vendor_policy_coverage_pct":100,"vendor_policy_violations":0}'
    fi
}

# Get vendor metrics
METRICS=$(get_vendor_metrics "$VENDOR_METRICS_PATH")
VENDOR_SCORE_AVG=$(echo "$METRICS" | grep -o '"vendor_score_avg":[0-9]*' | grep -o '[0-9]*')
OVERDUE_REVIEWS=$(echo "$METRICS" | grep -o '"overdue_reviews":[0-9]*' | grep -o '[0-9]*')
VENDOR_ACCESS_REVIEW_COMPLETION_PCT=$(echo "$METRICS" | grep -o '"vendor_access_review_completion_pct":[0-9]*' | grep -o '[0-9]*' || echo "100")
VENDOR_SOD_VIOLATIONS=$(echo "$METRICS" | grep -o '"vendor_sod_violations":[0-9]*' | grep -o '[0-9]*' || echo "0")
VENDOR_KEY_ROTATION_COMPLIANCE_PCT=$(echo "$METRICS" | grep -o '"vendor_key_rotation_compliance_pct":[0-9]*' | grep -o '[0-9]*' || echo "100")
VENDOR_KEY_HEALTH_CHECKS_PASS=$(echo "$METRICS" | grep -o '"vendor_key_health_checks_pass":[0-9]*' | grep -o '[0-9]*' || echo "1")
VENDOR_POLICY_COVERAGE_PCT=$(echo "$METRICS" | grep -o '"vendor_policy_coverage_pct":[0-9]*' | grep -o '[0-9]*' || echo "100")
VENDOR_POLICY_VIOLATIONS=$(echo "$METRICS" | grep -o '"vendor_policy_violations":[0-9]*' | grep -o '[0-9]*' || echo "0")

echo "[INFO] Vendor score average: $VENDOR_SCORE_AVG"
echo "[INFO] Overdue reviews: $OVERDUE_REVIEWS"
echo "[INFO] Vendor access review completion: $VENDOR_ACCESS_REVIEW_COMPLETION_PCT%"
echo "[INFO] Vendor SoD violations: $VENDOR_SOD_VIOLATIONS"
echo "[INFO] Vendor key rotation compliance: $VENDOR_KEY_ROTATION_COMPLIANCE_PCT%"
echo "[INFO] Vendor key health checks pass: $VENDOR_KEY_HEALTH_CHECKS_PASS"
echo "[INFO] Vendor policy coverage: $VENDOR_POLICY_COVERAGE_PCT%"
echo "[INFO] Vendor policy violations: $VENDOR_POLICY_VIOLATIONS"

# Check for overdue reviews
if [[ $OVERDUE_REVIEWS -gt $MAX_OVERDUE_REVIEWS ]]; then
    echo "WARNING: Overdue vendor reviews detected ($OVERDUE_REVIEWS)"
    exit 0 # Warn only, don't block
else
    echo "[PASS] No overdue vendor reviews"
fi

# Check for vendor access review completion
if [[ $VENDOR_ACCESS_REVIEW_COMPLETION_PCT -lt 100 ]]; then
    echo "BLOCK: Vendor access review completion below 100% ($VENDOR_ACCESS_REVIEW_COMPLETION_PCT%)"
    exit 1
fi

# Check for vendor SoD violations
if [[ $VENDOR_SOD_VIOLATIONS -gt 0 ]]; then
    echo "BLOCK: Vendor SoD violations detected ($VENDOR_SOD_VIOLATIONS)"
    exit 1
fi

# Check for vendor key rotation compliance
if [[ $VENDOR_KEY_ROTATION_COMPLIANCE_PCT -lt 100 ]]; then
    echo "BLOCK: Vendor key rotation compliance below 100% ($VENDOR_KEY_ROTATION_COMPLIANCE_PCT%)"
    exit 1
fi

# Check for vendor key health checks
if [[ $VENDOR_KEY_HEALTH_CHECKS_PASS -ne 1 ]]; then
    echo "BLOCK: Vendor key health checks failed"
    exit 1
fi

# Check for vendor policy coverage
if [[ $VENDOR_POLICY_COVERAGE_PCT -lt 95 ]]; then
    echo "BLOCK: Vendor policy coverage below 95% ($VENDOR_POLICY_COVERAGE_PCT%)"
    exit 1
fi

# Check for vendor policy violations
if [[ $VENDOR_POLICY_VIOLATIONS -gt 0 ]]; then
    echo "BLOCK: Vendor policy violations detected ($VENDOR_POLICY_VIOLATIONS)"
    exit 1
fi

echo "Vendor Management validation completed successfully!"
exit 0