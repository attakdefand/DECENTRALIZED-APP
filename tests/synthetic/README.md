# Synthetic API Tests

This directory contains synthetic tests that periodically check the health and functionality of the API endpoints.

## Test Scripts

- `api_health_check.sh` - Basic health check of all services
- `api_functionality_test.sh` - More comprehensive tests of API functionality
- `api_performance_test.sh` - Performance and latency tests

## Running Tests

### Manual Execution

```bash
# Run health checks
./tests/synthetic/api_health_check.sh

# Run functionality tests
./tests/synthetic/api_functionality_test.sh

# Run performance tests
./tests/synthetic/api_performance_test.sh
```

### Scheduled Execution

The tests can be scheduled to run periodically using cron jobs or Kubernetes cron jobs.

## Test Results

Test results are logged and can be pushed to Prometheus for monitoring.