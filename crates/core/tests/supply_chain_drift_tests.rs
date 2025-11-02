//! Runtime Drift Control Tests for Supply Chain Security

use core::supply_chain::{
    SupplyChainManager, SupplyChainError
};

/// Test drift control initialization
#[test]
fn test_drift_control_initialization() {
    let manager = SupplyChainManager::new();
    assert_eq!(manager.approved_manifests.len(), 0);
    assert_eq!(manager.drift_reports.len(), 0);
    assert_eq!(manager.running_containers.len(), 0);
    
    // Check drift statistics
    let (total_checks, incidents, sneaky, manifests) = manager.get_drift_stats();
    assert_eq!(total_checks, 0);
    assert_eq!(incidents, 0);
    assert_eq!(sneaky, 0);
    assert_eq!(manifests, 0);
}

/// Test approved manifest creation and storage
#[test]
fn test_approved_manifest_creation_and_storage() {
    let mut manager = SupplyChainManager::new();
    
    // Create expected values
    let expected_processes = vec!["nginx".to_string(), "app-server".to_string()];
    let expected_files = vec!["/etc/nginx/nginx.conf".to_string(), "/app/config.json".to_string()];
    let expected_network = vec!["tcp:80".to_string(), "tcp:443".to_string()];
    let mut expected_env = std::collections::HashMap::new();
    expected_env.insert("PORT".to_string(), "8080".to_string());
    expected_env.insert("ENV".to_string(), "production".to_string());
    
    // Create approved manifest
    let manifest = manager.create_approved_manifest(
        "web-service",
        "1.0.0",
        expected_processes.clone(),
        expected_files.clone(),
        expected_network.clone(),
        expected_env.clone(),
    );
    
    assert_eq!(manifest.name, "web-service");
    assert_eq!(manifest.version, "1.0.0");
    assert_eq!(manifest.expected_processes, expected_processes);
    assert_eq!(manifest.expected_file_changes, expected_files);
    assert_eq!(manifest.expected_network_connections, expected_network);
    assert_eq!(manifest.expected_env_vars, expected_env);
    
    // Store manifest
    assert!(manager.store_approved_manifest(manifest.clone()).is_ok());
    assert_eq!(manager.approved_manifests.len(), 1);
    
    // Retrieve manifest
    let retrieved = manager.get_approved_manifest(&manifest.id).unwrap();
    assert_eq!(retrieved.id, manifest.id);
    assert_eq!(retrieved.name, "web-service");
    
    // Check statistics
    let (_, _, _, manifests) = manager.get_drift_stats();
    assert_eq!(manifests, 1);
}

/// Test running container creation and storage
#[test]
fn test_running_container_creation_and_storage() {
    let mut manager = SupplyChainManager::new();
    
    // Create container data
    let processes = vec!["nginx".to_string(), "app-server".to_string()];
    let file_changes = vec!["/var/log/app.log".to_string()];
    let network_connections = vec!["tcp:80".to_string()];
    let mut env_vars = std::collections::HashMap::new();
    env_vars.insert("PORT".to_string(), "8080".to_string());
    env_vars.insert("ENV".to_string(), "production".to_string());
    
    // Create running container
    let container = manager.create_running_container(
        "container-123",
        "web-service-container",
        "web-service:1.0.0",
        processes.clone(),
        file_changes.clone(),
        network_connections.clone(),
        env_vars.clone(),
        "manifest-web-service-1.0.0",
    );
    
    assert_eq!(container.id, "container-123");
    assert_eq!(container.name, "web-service-container");
    assert_eq!(container.image, "web-service:1.0.0");
    assert_eq!(container.processes, processes);
    assert_eq!(container.file_changes, file_changes);
    assert_eq!(container.network_connections, network_connections);
    assert_eq!(container.env_vars, env_vars);
    assert_eq!(container.manifest_id, "manifest-web-service-1.0.0");
    
    // Store container
    assert!(manager.store_running_container(container.clone()).is_ok());
    assert_eq!(manager.running_containers.len(), 1);
    
    // Retrieve container
    let retrieved = manager.get_running_container("container-123").unwrap();
    assert_eq!(retrieved.id, "container-123");
    assert_eq!(retrieved.name, "web-service-container");
}

/// Test drift check with no deviations
#[test]
fn test_drift_check_no_deviations() {
    let mut manager = SupplyChainManager::new();
    
    // Create approved manifest
    let expected_processes = vec!["nginx".to_string(), "app-server".to_string()];
    let expected_files = vec!["/etc/nginx/nginx.conf".to_string(), "/app/config.json".to_string()];
    let expected_network = vec!["tcp:80".to_string(), "tcp:443".to_string()];
    let mut expected_env = std::collections::HashMap::new();
    expected_env.insert("PORT".to_string(), "8080".to_string());
    expected_env.insert("ENV".to_string(), "production".to_string());
    
    let manifest = manager.create_approved_manifest(
        "web-service",
        "1.0.0",
        expected_processes.clone(),
        expected_files.clone(),
        expected_network.clone(),
        expected_env.clone(),
    );
    assert!(manager.store_approved_manifest(manifest.clone()).is_ok());
    
    // Create running container with same values as manifest
    let container = manager.create_running_container(
        "container-123",
        "web-service-container",
        "web-service:1.0.0",
        expected_processes,
        expected_files,
        expected_network,
        expected_env,
        &manifest.id,
    );
    assert!(manager.store_running_container(container.clone()).is_ok());
    
    // Check for drift - should find no deviations
    let report = manager.check_runtime_drift("container-123").unwrap();
    
    assert!(!report.is_drift_incident);
    assert!(!report.is_sneaky_container);
    assert_eq!(report.process_deviations.len(), 0);
    assert_eq!(report.file_deviations.len(), 0);
    assert_eq!(report.network_deviations.len(), 0);
    assert_eq!(report.env_var_deviations.len(), 0);
    
    // Check that report is stored
    assert_eq!(manager.drift_reports.len(), 1);
    let stored_report = manager.get_drift_report(&report.id).unwrap();
    assert_eq!(stored_report.id, report.id);
    
    // Check statistics
    let (total_checks, incidents, sneaky, _) = manager.get_drift_stats();
    assert_eq!(total_checks, 1);
    assert_eq!(incidents, 0);
    assert_eq!(sneaky, 0);
}

/// Test drift check with deviations
#[test]
fn test_drift_check_with_deviations() {
    let mut manager = SupplyChainManager::new();
    
    // Create approved manifest
    let expected_processes = vec!["nginx".to_string(), "app-server".to_string()];
    let expected_files = vec!["/etc/nginx/nginx.conf".to_string(), "/app/config.json".to_string()];
    let expected_network = vec!["tcp:80".to_string(), "tcp:443".to_string()];
    let mut expected_env = std::collections::HashMap::new();
    expected_env.insert("PORT".to_string(), "8080".to_string());
    expected_env.insert("ENV".to_string(), "production".to_string());
    
    let manifest = manager.create_approved_manifest(
        "web-service",
        "1.0.0",
        expected_processes.clone(),
        expected_files.clone(),
        expected_network.clone(),
        expected_env.clone(),
    );
    assert!(manager.store_approved_manifest(manifest.clone()).is_ok());
    
    // Create running container with deviations
    let actual_processes = vec!["nginx".to_string(), "app-server".to_string(), "malicious-process".to_string()];
    let actual_files = vec!["/etc/nginx/nginx.conf".to_string(), "/app/config.json".to_string(), "/tmp/malicious.sh".to_string()];
    let actual_network = vec!["tcp:80".to_string(), "tcp:443".to_string(), "tcp:1337".to_string()];
    let mut actual_env = std::collections::HashMap::new();
    actual_env.insert("PORT".to_string(), "8080".to_string()); // Same value
    actual_env.insert("ENV".to_string(), "hacked".to_string()); // Different value
    actual_env.insert("MALICIOUS_VAR".to_string(), "evil".to_string()); // Extra variable
    
    let container = manager.create_running_container(
        "container-123",
        "web-service-container",
        "web-service:1.0.0",
        actual_processes,
        actual_files,
        actual_network,
        actual_env,
        &manifest.id,
    );
    assert!(manager.store_running_container(container.clone()).is_ok());
    
    // Check for drift - should find deviations
    let report = manager.check_runtime_drift("container-123").unwrap();
    
    assert!(report.is_drift_incident);
    assert!(!report.is_sneaky_container); // Not enough deviations to be considered sneaky
    assert_eq!(report.process_deviations.len(), 1);
    assert_eq!(report.file_deviations.len(), 1);
    assert_eq!(report.network_deviations.len(), 1);
    assert_eq!(report.env_var_deviations.len(), 2); // One changed value, one extra variable
    
    // Check specific deviations
    assert!(report.process_deviations.contains(&"malicious-process".to_string()));
    assert!(report.file_deviations.contains(&"/tmp/malicious.sh".to_string()));
    assert!(report.network_deviations.contains(&"tcp:1337".to_string()));
    assert!(report.env_var_deviations.iter().any(|e| e.contains("ENV") && e.contains("hacked")));
    assert!(report.env_var_deviations.iter().any(|e| e.contains("MALICIOUS_VAR")));
    
    // Check statistics
    let (total_checks, incidents, sneaky, _) = manager.get_drift_stats();
    assert_eq!(total_checks, 1);
    assert_eq!(incidents, 1);
    assert_eq!(sneaky, 0);
}

/// Test drift check for sneaky container
#[test]
fn test_drift_check_sneaky_container() {
    let mut manager = SupplyChainManager::new();
    
    // Create approved manifest
    let expected_processes = vec!["nginx".to_string(), "app-server".to_string()];
    let expected_files = vec!["/etc/nginx/nginx.conf".to_string(), "/app/config.json".to_string()];
    let expected_network = vec!["tcp:80".to_string(), "tcp:443".to_string()];
    let mut expected_env = std::collections::HashMap::new();
    expected_env.insert("PORT".to_string(), "8080".to_string());
    expected_env.insert("ENV".to_string(), "production".to_string());
    
    let manifest = manager.create_approved_manifest(
        "web-service",
        "1.0.0",
        expected_processes,
        expected_files,
        expected_network,
        expected_env,
    );
    assert!(manager.store_approved_manifest(manifest.clone()).is_ok());
    
    // Create running container with many deviations (sneaky container)
    let mut sneaky_processes = vec![];
    for i in 0..5 {
        sneaky_processes.push(format!("sneaky-process-{}", i));
    }
    
    let mut sneaky_files = vec![];
    for i in 0..10 {
        sneaky_files.push(format!("/tmp/sneaky-file-{}.tmp", i));
    }
    
    let mut sneaky_network = vec![];
    for i in 0..6 {
        sneaky_network.push(format!("tcp:{}", 3000 + i));
    }
    
    let mut sneaky_env = std::collections::HashMap::new();
    for i in 0..8 {
        sneaky_env.insert(format!("SNEAKY_VAR_{}", i), format!("value_{}", i));
    }
    
    let container = manager.create_running_container(
        "container-123",
        "web-service-container",
        "web-service:1.0.0",
        sneaky_processes,
        sneaky_files,
        sneaky_network,
        sneaky_env,
        &manifest.id,
    );
    assert!(manager.store_running_container(container.clone()).is_ok());
    
    // Check for drift - should identify as sneaky container
    let report = manager.check_runtime_drift("container-123").unwrap();
    
    assert!(report.is_drift_incident);
    assert!(report.is_sneaky_container); // Enough deviations to be considered sneaky
    
    // Check statistics
    let (total_checks, incidents, sneaky, _) = manager.get_drift_stats();
    assert_eq!(total_checks, 1);
    assert_eq!(incidents, 1);
    assert_eq!(sneaky, 1);
}

/// Test drift check with missing manifest
#[test]
fn test_drift_check_missing_manifest() {
    let mut manager = SupplyChainManager::new();
    
    // Create running container without storing the manifest
    let processes = vec!["nginx".to_string()];
    let files = vec!["/etc/nginx/nginx.conf".to_string()];
    let network = vec!["tcp:80".to_string()];
    let mut env = std::collections::HashMap::new();
    env.insert("PORT".to_string(), "8080".to_string());
    
    let container = manager.create_running_container(
        "container-123",
        "web-service-container",
        "web-service:1.0.0",
        processes,
        files,
        network,
        env,
        "non-existent-manifest",
    );
    assert!(manager.store_running_container(container.clone()).is_ok());
    
    // Check for drift - should fail due to missing manifest
    let result = manager.check_runtime_drift("container-123");
    assert!(result.is_err());
    
    match result.unwrap_err() {
        SupplyChainError::ConfigurationError(msg) => {
            assert!(msg.contains("Approved manifest not found"));
        }
        _ => panic!("Expected ConfigurationError"),
    }
}

/// Test drift check with missing container
#[test]
fn test_drift_check_missing_container() {
    let mut manager = SupplyChainManager::new();  // Fixed: made manager mutable
    
    // Check for drift on non-existent container - should fail
    let result = manager.check_runtime_drift("non-existent-container");
    assert!(result.is_err());
    
    match result.unwrap_err() {
        SupplyChainError::ConfigurationError(msg) => {
            assert!(msg.contains("Container not found"));
        }
        _ => panic!("Expected ConfigurationError"),
    }
}

/// Test drift reports for specific manifest
#[test]
fn test_drift_reports_for_manifest() {
    let mut manager = SupplyChainManager::new();
    
    // Create approved manifest
    let expected_processes = vec!["nginx".to_string()];
    let expected_files = vec!["/etc/nginx/nginx.conf".to_string()];
    let expected_network = vec!["tcp:80".to_string()];
    let expected_env = std::collections::HashMap::new();
    
    let manifest = manager.create_approved_manifest(
        "web-service",
        "1.0.0",
        expected_processes.clone(),
        expected_files.clone(),
        expected_network.clone(),
        expected_env.clone(),
    );
    let manifest_id = manifest.id.clone();
    assert!(manager.store_approved_manifest(manifest).is_ok());
    
    // Create another manifest for comparison
    let other_manifest = manager.create_approved_manifest(
        "other-service",
        "1.0.0",
        expected_processes.clone(),
        expected_files.clone(),
        expected_network.clone(),
        expected_env.clone(),
    );
    let other_manifest_id = other_manifest.id.clone();
    assert!(manager.store_approved_manifest(other_manifest).is_ok());
    
    // Create containers with deviations
    let actual_processes = vec!["nginx".to_string(), "malicious-process".to_string()];
    
    let container1 = manager.create_running_container(
        "container-1",
        "web-service-container-1",
        "web-service:1.0.0",
        actual_processes.clone(),
        expected_files.clone(),
        expected_network.clone(),
        expected_env.clone(),
        &manifest_id,
    );
    assert!(manager.store_running_container(container1).is_ok());
    
    let container2 = manager.create_running_container(
        "container-2",
        "web-service-container-2",
        "web-service:1.0.0",
        actual_processes,
        expected_files.clone(),
        expected_network.clone(),
        expected_env.clone(),
        &manifest_id,
    );
    assert!(manager.store_running_container(container2).is_ok());
    
    let container3 = manager.create_running_container(
        "container-3",
        "other-service-container",
        "other-service:1.0.0",
        vec!["nginx".to_string(), "suspicious-process".to_string()],
        expected_files,
        expected_network,
        expected_env,
        &other_manifest_id,
    );
    assert!(manager.store_running_container(container3).is_ok());
    
    // Check for drift on all containers
    manager.check_runtime_drift("container-1").unwrap();
    manager.check_runtime_drift("container-2").unwrap();
    manager.check_runtime_drift("container-3").unwrap();
    
    // Check reports for specific manifest
    let manifest_reports = manager.get_drift_reports_for_manifest(&manifest_id);
    assert_eq!(manifest_reports.len(), 2);
    
    let other_manifest_reports = manager.get_drift_reports_for_manifest(&other_manifest_id);
    assert_eq!(other_manifest_reports.len(), 1);
    
    // Check statistics
    let (total_checks, incidents, sneaky, _) = manager.get_drift_stats();
    assert_eq!(total_checks, 3);
    assert_eq!(incidents, 3);
    assert_eq!(sneaky, 0); // Not enough deviations to be considered sneaky
}

/// Test drift incidents per week reporting
#[test]
fn test_drift_incidents_per_week() {
    let mut manager = SupplyChainManager::new();
    
    // Create approved manifest
    let expected_processes = vec!["nginx".to_string()];
    let expected_files = vec!["/etc/nginx/nginx.conf".to_string()];
    let expected_network = vec!["tcp:80".to_string()];
    let expected_env = std::collections::HashMap::new();
    
    let manifest = manager.create_approved_manifest(
        "web-service",
        "1.0.0",
        expected_processes.clone(),
        expected_files.clone(),
        expected_network.clone(),
        expected_env.clone(),
    );
    let manifest_id = manifest.id.clone();
    assert!(manager.store_approved_manifest(manifest).is_ok());
    
    // Create container with deviations
    let container = manager.create_running_container(
        "container-123",
        "web-service-container",
        "web-service:1.0.0",
        vec!["nginx".to_string(), "malicious-process".to_string()],
        expected_files,
        expected_network,
        expected_env,
        &manifest_id,
    );
    assert!(manager.store_running_container(container).is_ok());
    
    // Check for drift
    manager.check_runtime_drift("container-123").unwrap();
    
    // Check drift incidents per week
    let incidents = manager.get_drift_incidents_per_week();
    assert_eq!(incidents, 1);
}