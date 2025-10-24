//! Tests for the repository scaffold implementation

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_contracts_structure() {
        // Test that contracts directory has the required subdirectories
        assert!(Path::new("contracts/src").exists());
        assert!(Path::new("contracts/script").exists());
        assert!(Path::new("contracts/test").exists());
        
        // Test that each subdirectory has at least a README.md
        assert!(Path::new("contracts/src/README.md").exists());
        assert!(Path::new("contracts/script/README.md").exists());
        assert!(Path::new("contracts/test/README.md").exists());
    }

    #[test]
    fn test_services_structure() {
        // Test that services directory has the required services
        assert!(Path::new("services/api-rs").exists());
        assert!(Path::new("services/indexer-rs").exists());
        assert!(Path::new("services/keepers-rs").exists());
        assert!(Path::new("services/aa-bundler").exists());
        
        // Test that AA Bundler service has the required files
        assert!(Path::new("services/aa-bundler/Cargo.toml").exists());
        assert!(Path::new("services/aa-bundler/src/main.rs").exists());
        assert!(Path::new("services/aa-bundler/Dockerfile").exists());
    }

    #[test]
    fn test_infra_structure() {
        // Test that infra directory has the required subdirectories
        assert!(Path::new("infra/compose").exists());
        assert!(Path::new("infra/k8s").exists());
        assert!(Path::new("infra/policies").exists());
        
        // Test that compose directory has the docker-compose.yml
        assert!(Path::new("infra/compose/docker-compose.yml").exists());
        
        // Test that k8s directory has the required structure
        assert!(Path::new("infra/k8s/helm").exists());
        assert!(Path::new("infra/k8s/kustomize").exists());
        
        // Test that policies directory has the required structure
        assert!(Path::new("infra/policies/opa").exists());
        assert!(Path::new("infra/policies/cedar").exists());
    }

    #[test]
    fn test_docs_structure() {
        // Test that docs directory has the required subdirectories and files
        assert!(Path::new("docs/api").exists());
        assert!(Path::new("docs/api/openapi.yaml").exists());
        assert!(Path::new("docs/api/sdk/ts").exists());
        assert!(Path::new("docs/api/sdk/rust").exists());
    }

    #[test]
    fn test_tests_structure() {
        // Test that tests directory has the required subdirectories
        assert!(Path::new("tests/e2e").exists());
        assert!(Path::new("tests/perf").exists());
        assert!(Path::new("tests/chaos").exists());
        
        // Test that each subdirectory has at least a README.md
        assert!(Path::new("tests/e2e/README.md").exists());
        assert!(Path::new("tests/perf/README.md").exists());
        assert!(Path::new("tests/chaos/README.md").exists());
    }

    #[test]
    fn test_docker_compose_includes_new_services() {
        // Read the docker-compose.yml file
        let docker_compose_content = fs::read_to_string("infra/compose/docker-compose.yml")
            .expect("Failed to read docker-compose.yml");
        
        // Check that it includes the new services
        assert!(docker_compose_content.contains("aa-bundler"));
        assert!(docker_compose_content.contains("mev-monitor"));
    }

    #[test]
    fn test_prometheus_config_includes_new_services() {
        // Read the prometheus.yml file
        let prometheus_content = fs::read_to_string("infra/prometheus.yml")
            .expect("Failed to read prometheus.yml");
        
        // Check that it includes the new service
        assert!(prometheus_content.contains("aa-bundler-service"));
    }
}