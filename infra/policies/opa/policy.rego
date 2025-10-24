# OPA policy for the decentralized application

package dex

# Default decision
default allow = false

# Allow access to public endpoints
allow {
    input.method = "GET"
    input.path = "/"
}

allow {
    input.method = "GET"
    input.path = "/health"
}

# Allow access to API endpoints with proper authentication
allow {
    input.method = "GET"
    startswith(input.path, "/api/v1/")
    input.headers.authorization != ""
}

# Allow access to metrics endpoints
allow {
    input.method = "GET"
    input.path = "/metrics"
}