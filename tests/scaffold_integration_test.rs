//! Integration tests for the repository scaffold implementation

#[cfg(test)]
mod scaffold_tests {
    use std::path::Path;

    #[test]
    fn test_contracts_directory_structure() {
        // Test that contracts directory has the required subdirectories
        assert!(Path::new("contracts/src").exists(), "contracts/src directory missing");
        assert!(Path::new("contracts/script").exists(), "contracts/script directory missing");
        assert!(Path::new("contracts/test").exists(), "contracts/test directory missing");
    }

    #[test]
    fn test_services_directory_structure() {
        // Test that services directory has the required services
        assert!(Path::new("services/aa-bundler").exists(), "services/aa-bundler directory missing");
    }

    #[test]
    fn test_infra_directory_structure() {
        // Test that infra directory has the required subdirectories
        assert!(Path::new("infra/compose").exists(), "infra/compose directory missing");
        assert!(Path::new("infra/k8s").exists(), "infra/k8s directory missing");
        assert!(Path::new("infra/policies").exists(), "infra/policies directory missing");
    }

    #[test]
    fn test_docs_directory_structure() {
        // Test that docs directory has the required subdirectories
        assert!(Path::new("docs/api").exists(), "docs/api directory missing");
    }

    #[test]
    fn test_tests_directory_structure() {
        // Test that tests directory has the required subdirectories
        assert!(Path::new("tests/e2e").exists(), "tests/e2e directory missing");
        assert!(Path::new("tests/perf").exists(), "tests/perf directory missing");
        assert!(Path::new("tests/chaos").exists(), "tests/chaos directory missing");
    }
}