# Test cases for OPA policy

package dex

test_allow_public_endpoints {
    allow with input as {"method": "GET", "path": "/"}
}

test_allow_health_endpoint {
    allow with input as {"method": "GET", "path": "/health"}
}

test_allow_api_with_auth {
    allow with input as {
        "method": "GET",
        "path": "/api/v1/pools",
        "headers": {"authorization": "Bearer token123"}
    }
}

test_deny_api_without_auth {
    not allow with input as {
        "method": "GET",
        "path": "/api/v1/pools",
        "headers": {}
    }
}

test_allow_metrics {
    allow with input as {"method": "GET", "path": "/metrics"}
}