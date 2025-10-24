//! Integration tests for testing assurance functionality

use testing::{
    CoverageReport, TestConfig, TestResult, TestStatus, TestSuite, TestType, TestingAssurance,
};

/// Integration test for the complete testing assurance workflow
#[test]
fn test_complete_testing_assurance_workflow() {
    println!("Starting complete testing assurance workflow test");

    // 1. Create testing assurance configuration
    let config = TestConfig {
        enable_unit_tests: true,
        enable_fuzz_tests: true,
        enable_invariant_tests: true,
        enable_chaos_tests: true,
        min_coverage_percent: 80.0,
        enable_mainnet_fork: true,
    };

    // 2. Create testing assurance manager
    let mut assurance = TestingAssurance::new(config);
    println!("✓ Testing assurance manager created");

    // 3. Record test results
    let unit_test_result = TestResult {
        id: "unit-test-001".to_string(),
        name: "test_addition".to_string(),
        test_type: TestType::Unit,
        status: TestStatus::Passed,
        duration_ms: 50,
        coverage_percent: 95.0,
        timestamp: 1234567890,
        error_message: None,
    };

    let fuzz_test_result = TestResult {
        id: "fuzz-test-001".to_string(),
        name: "test_input_fuzzing".to_string(),
        test_type: TestType::Fuzz,
        status: TestStatus::Passed,
        duration_ms: 1200,
        coverage_percent: 88.0,
        timestamp: 1234567895,
        error_message: None,
    };

    let invariant_test_result = TestResult {
        id: "invariant-test-001".to_string(),
        name: "test_state_invariants".to_string(),
        test_type: TestType::Invariant,
        status: TestStatus::Passed,
        duration_ms: 800,
        coverage_percent: 92.0,
        timestamp: 1234567900,
        error_message: None,
    };

    assert!(assurance.record_test_result(unit_test_result).is_ok());
    assert!(assurance.record_test_result(fuzz_test_result).is_ok());
    assert!(assurance.record_test_result(invariant_test_result).is_ok());
    println!("✓ Test results recorded");

    // 4. Record test suites
    let unit_test_suite = TestSuite {
        id: "unit-suite-001".to_string(),
        name: "core_unit_tests".to_string(),
        test_type: TestType::Unit,
        results: vec![],
        total_tests: 50,
        passed_tests: 50,
        failed_tests: 0,
        skipped_tests: 0,
        average_coverage: 92.0,
        timestamp: 1234567910,
    };

    let integration_test_suite = TestSuite {
        id: "integration-suite-001".to_string(),
        name: "integration_tests".to_string(),
        test_type: TestType::Integration,
        results: vec![],
        total_tests: 20,
        passed_tests: 18,
        failed_tests: 2,
        skipped_tests: 0,
        average_coverage: 85.0,
        timestamp: 1234567920,
    };

    assert!(assurance.record_test_suite(unit_test_suite).is_ok());
    assert!(assurance.record_test_suite(integration_test_suite).is_ok());
    println!("✓ Test suites recorded");

    // 5. Record coverage reports
    let core_coverage = CoverageReport {
        file_path: "src/core.rs".to_string(),
        line_coverage: 95.0,
        branch_coverage: 90.0,
        function_coverage: 98.0,
        lines_covered: 190,
        lines_total: 200,
        branches_covered: 90,
        branches_total: 100,
        functions_covered: 49,
        functions_total: 50,
    };

    let utils_coverage = CoverageReport {
        file_path: "src/utils.rs".to_string(),
        line_coverage: 85.0,
        branch_coverage: 80.0,
        function_coverage: 90.0,
        lines_covered: 170,
        lines_total: 200,
        branches_covered: 80,
        branches_total: 100,
        functions_covered: 18,
        functions_total: 20,
    };

    assert!(assurance.record_coverage_report(core_coverage).is_ok());
    assert!(assurance.record_coverage_report(utils_coverage).is_ok());
    println!("✓ Coverage reports recorded");

    // 6. Verify test statistics
    let (total, passed, failed, skipped) = assurance.get_test_statistics();
    assert_eq!(total, 70);
    assert_eq!(passed, 68);
    assert_eq!(failed, 2);
    assert_eq!(skipped, 0);
    println!("✓ Test statistics verified");

    // 7. Check coverage meets minimum
    assert!(assurance.coverage_meets_minimum());
    println!("✓ Coverage meets minimum requirements");

    // 8. Generate evidence bundle
    let bundle = assurance.generate_evidence_bundle().unwrap();
    assert_eq!(bundle.test_suites.len(), 2);
    assert_eq!(bundle.coverage_reports.len(), 2);
    assert!(bundle.signature.is_none()); // In this implementation
    println!("✓ Evidence bundle generated");

    // 9. Verify test results by type
    let unit_results = assurance.get_results_by_type(TestType::Unit);
    assert_eq!(unit_results.len(), 1);

    let fuzz_results = assurance.get_results_by_type(TestType::Fuzz);
    assert_eq!(fuzz_results.len(), 1);
    println!("✓ Test results by type verified");

    println!("Complete testing assurance workflow test passed!");
}

/// Integration test for testing assurance with high coverage requirements
#[test]
fn test_testing_assurance_high_coverage() {
    println!("Starting testing assurance high coverage test");

    // 1. Create testing assurance configuration with high coverage requirements
    let config = TestConfig {
        enable_unit_tests: true,
        enable_fuzz_tests: true,
        enable_invariant_tests: true,
        enable_chaos_tests: true,
        min_coverage_percent: 95.0, // High coverage requirement
        enable_mainnet_fork: true,
    };

    // 2. Create testing assurance manager
    let mut assurance = TestingAssurance::new(config);
    println!("✓ Testing assurance manager created with high coverage requirements");

    // 3. Record test suites with high coverage
    let high_coverage_suite = TestSuite {
        id: "high-coverage-suite-001".to_string(),
        name: "high_coverage_tests".to_string(),
        test_type: TestType::Unit,
        results: vec![],
        total_tests: 100,
        passed_tests: 100,
        failed_tests: 0,
        skipped_tests: 0,
        average_coverage: 98.0,
        timestamp: 1234567930,
    };

    assert!(assurance.record_test_suite(high_coverage_suite).is_ok());
    println!("✓ High coverage test suite recorded");

    // 4. Record coverage reports with high coverage
    let high_coverage_report = CoverageReport {
        file_path: "src/high_coverage.rs".to_string(),
        line_coverage: 98.0,
        branch_coverage: 96.0,
        function_coverage: 100.0,
        lines_covered: 490,
        lines_total: 500,
        branches_covered: 192,
        branches_total: 200,
        functions_covered: 50,
        functions_total: 50,
    };

    assert!(assurance
        .record_coverage_report(high_coverage_report)
        .is_ok());
    println!("✓ High coverage report recorded");

    // 5. Verify coverage meets high requirements
    assert!(assurance.coverage_meets_minimum());
    println!("✓ High coverage requirements met");

    // 6. Verify all tests pass
    assert!(assurance.all_tests_pass());
    println!("✓ All tests pass with high coverage");

    println!("Testing assurance high coverage test passed!");
}

/// Integration test for testing assurance error handling
#[test]
fn test_testing_assurance_error_handling() {
    println!("Starting testing assurance error handling test");

    // 1. Create testing assurance configuration
    let config = TestConfig {
        enable_unit_tests: true,
        enable_fuzz_tests: true,
        enable_invariant_tests: true,
        enable_chaos_tests: true,
        min_coverage_percent: 80.0,
        enable_mainnet_fork: true,
    };

    // 2. Create testing assurance manager
    let mut assurance = TestingAssurance::new(config);
    println!("✓ Testing assurance manager created");

    // 3. Record test result with failure
    let failed_test_result = TestResult {
        id: "failed-test-001".to_string(),
        name: "test_division_by_zero".to_string(),
        test_type: TestType::Unit,
        status: TestStatus::Failed,
        duration_ms: 75,
        coverage_percent: 90.0,
        timestamp: 1234567940,
        error_message: Some("Division by zero error".to_string()),
    };

    assert!(assurance.record_test_result(failed_test_result).is_ok());
    println!("✓ Failed test result recorded");

    // 4. Record test suite with failures
    let failing_test_suite = TestSuite {
        id: "failing-suite-001".to_string(),
        name: "failing_tests".to_string(),
        test_type: TestType::Unit,
        results: vec![],
        total_tests: 10,
        passed_tests: 7,
        failed_tests: 3,
        skipped_tests: 0,
        average_coverage: 85.0,
        timestamp: 1234567950,
    };

    assert!(assurance.record_test_suite(failing_test_suite).is_ok());
    println!("✓ Test suite with failures recorded");

    // 5. Record coverage reports with low coverage
    let low_coverage_report = CoverageReport {
        file_path: "src/low_coverage.rs".to_string(),
        line_coverage: 70.0,
        branch_coverage: 65.0,
        function_coverage: 75.0,
        lines_covered: 140,
        lines_total: 200,
        branches_covered: 65,
        branches_total: 100,
        functions_covered: 15,
        functions_total: 20,
    };

    assert!(assurance
        .record_coverage_report(low_coverage_report)
        .is_ok());
    println!("✓ Low coverage report recorded");

    // 6. Verify all tests don't pass (due to failures)
    assert!(!assurance.all_tests_pass());
    println!("✓ All tests correctly identified as not passing");

    // 7. Verify coverage doesn't meet minimum (due to low coverage)
    assert!(!assurance.coverage_meets_minimum());
    println!("✓ Low coverage correctly identified");

    println!("Testing assurance error handling test passed!");
}
