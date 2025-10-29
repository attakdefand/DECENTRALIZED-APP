//! Runtime Drift Control Simulation Tests for Supply Chain Security
//!
//! This file contains realistic simulation scenarios for testing runtime drift control
//! functionality in a software supply chain security context.

use core::supply_chain::{
    SupplyChainManager, ApprovedManifest, RunningContainer
};
use std::collections::HashMap;

/// Simulate a realistic web application deployment scenario
#[test]
fn test_web_app_deployment_drift_simulation() {
    let mut manager = SupplyChainManager::new();
    
    // Create an approved manifest for a typical web application
    let expected_processes = vec![
        "nginx".to_string(),
        "app-server".to_string(),
        "database-proxy".to_string()
    ];
    
    let expected_files = vec![
        "/etc/nginx/nginx.conf".to_string(),
        "/app/config.json".to_string(),
        "/var/log/app.log".to_string()
    ];
    
    let expected_network = vec![
        "tcp:80".to_string(),
        "tcp:443".to_string(),
        "tcp:5432".to_string() // Database connection
    ];
    
    let mut expected_env = HashMap::new();
    expected_env.insert("PORT".to_string(), "8080".to_string());
    expected_env.insert("DATABASE_URL".to_string(), "postgresql://db:5432/app".to_string());
    expected_env.insert("ENV".to_string(), "production".to_string());
    expected_env.insert("LOG_LEVEL".to_string(), "info".to_string());
    
    // Create and store the approved manifest
    let manifest = manager.create_approved_manifest(
        "web-application",
        "v1.2.3",
        expected_processes,
        expected_files,
        expected_network,
        expected_env,
    );
    let manifest_id = manifest.id.clone();
    assert!(manager.store_approved_manifest(manifest).is_ok());
    
    // Simulate a legitimate running container that matches the manifest
    let legitimate_processes = vec![
        "nginx".to_string(),
        "app-server".to_string(),
        "database-proxy".to_string()
    ];
    
    let legitimate_files = vec![
        "/etc/nginx/nginx.conf".to_string(),
        "/app/config.json".to_string(),
        "/var/log/app.log".to_string()
        // No temporary files to keep it clean
    ];
    
    let legitimate_network = vec![
        "tcp:80".to_string(),
        "tcp:443".to_string(),
        "tcp:5432".to_string()
    ];
    
    let mut legitimate_env = HashMap::new();
    legitimate_env.insert("PORT".to_string(), "8080".to_string());
    legitimate_env.insert("DATABASE_URL".to_string(), "postgresql://db:5432/app".to_string());
    legitimate_env.insert("ENV".to_string(), "production".to_string());
    legitimate_env.insert("LOG_LEVEL".to_string(), "info".to_string());
    // All expected environment variables present
    
    let legitimate_container = manager.create_running_container(
        "web-app-container-001",
        "web-application-instance-001",
        "web-application:v1.2.3",
        legitimate_processes,
        legitimate_files,
        legitimate_network,
        legitimate_env,
        &manifest_id,
    );
    assert!(manager.store_running_container(legitimate_container).is_ok());
    
    // Check for drift - should find no deviations
    let report = manager.check_runtime_drift("web-app-container-001").unwrap();
    
    // Should not be considered a drift incident for exact match
    assert!(!report.is_drift_incident);
    assert!(!report.is_sneaky_container);
    
    println!("Legitimate container drift report:");
    println!("  Process deviations: {}", report.process_deviations.len());
    println!("  File deviations: {}", report.file_deviations.len());
    println!("  Network deviations: {}", report.network_deviations.len());
    println!("  Environment variable deviations: {}", report.env_var_deviations.len());
}

/// Simulate a sneaky container infiltration scenario
#[test]
fn test_sneaky_container_infiltration_simulation() {
    let mut manager = SupplyChainManager::new();
    
    // Create an approved manifest for a typical microservice
    let expected_processes = vec![
        "api-server".to_string(),
        "health-check".to_string()
    ];
    
    let expected_files = vec![
        "/app/config.yaml".to_string(),
        "/var/log/service.log".to_string()
    ];
    
    let expected_network = vec![
        "tcp:8080".to_string(),
        "udp:53".to_string() // DNS lookups
    ];
    
    let mut expected_env = HashMap::new();
    expected_env.insert("SERVICE_PORT".to_string(), "8080".to_string());
    expected_env.insert("LOG_LEVEL".to_string(), "info".to_string());
    
    // Create and store the approved manifest
    let manifest = manager.create_approved_manifest(
        "payment-service",
        "v2.1.0",
        expected_processes,
        expected_files,
        expected_network,
        expected_env,
    );
    let manifest_id = manifest.id.clone();
    assert!(manager.store_approved_manifest(manifest).is_ok());
    
    // Simulate a sneaky container with many unauthorized processes, files, and connections
    let mut sneaky_processes = vec![
        "api-server".to_string(), // Legitimate process
        "health-check".to_string() // Legitimate process
    ];
    
    // Add many suspicious processes
    for i in 0..10 {
        sneaky_processes.push(format!("miner-{}", i));
        sneaky_processes.push(format!("reverse-shell-{}", i));
    }
    sneaky_processes.push("crypto-miner".to_string());
    sneaky_processes.push("port-scanner".to_string());
    sneaky_processes.push("data-exfiltrator".to_string());
    
    let mut sneaky_files = vec![
        "/app/config.yaml".to_string() // Legitimate file
    ];
    
    // Add many suspicious files
    for i in 0..20 {
        sneaky_files.push(format!("/tmp/malicious-{}.sh", i));
        sneaky_files.push(format!("/var/tmp/backdoor-{}.bin", i));
    }
    sneaky_files.push("/etc/passwd.bak".to_string());
    sneaky_files.push("/root/.ssh/authorized_keys".to_string());
    
    let mut sneaky_network = vec![
        "tcp:8080".to_string() // Legitimate connection
    ];
    
    // Add many suspicious network connections
    for i in 8000..8050 {
        sneaky_network.push(format!("tcp:{}", i));
    }
    sneaky_network.push("tcp:4444".to_string()); // Common reverse shell port
    sneaky_network.push("udp:5353".to_string()); // Suspicious DNS port
    sneaky_network.push("tcp:9001".to_string()); // Common backdoor port
    
    let mut sneaky_env = HashMap::new();
    sneaky_env.insert("SERVICE_PORT".to_string(), "8080".to_string()); // Legitimate
    // Add many suspicious environment variables
    for i in 0..15 {
        sneaky_env.insert(format!("MALICIOUS_VAR_{}", i), format!("payload_{}", i));
    }
    sneaky_env.insert("C2_SERVER".to_string(), "evil.com".to_string());
    sneaky_env.insert("ENCRYPTION_KEY".to_string(), "super_secret_key".to_string());
    
    let sneaky_container = manager.create_running_container(
        "sneaky-container-001",
        "compromised-payment-service",
        "payment-service:v2.1.0",
        sneaky_processes,
        sneaky_files,
        sneaky_network,
        sneaky_env,
        &manifest_id,
    );
    assert!(manager.store_running_container(sneaky_container).is_ok());
    
    // Check for drift - should identify as sneaky container
    let report = manager.check_runtime_drift("sneaky-container-001").unwrap();
    
    // Should be identified as both drift incident and sneaky container
    assert!(report.is_drift_incident);
    assert!(report.is_sneaky_container);
    
    println!("Sneaky container drift report:");
    println!("  Process deviations: {}", report.process_deviations.len());
    println!("  File deviations: {}", report.file_deviations.len());
    println!("  Network deviations: {}", report.network_deviations.len());
    println!("  Environment variable deviations: {}", report.env_var_deviations.len());
    
    // Verify we have significant deviations
    assert!(report.process_deviations.len() > 10);
    assert!(report.file_deviations.len() > 15);
    assert!(report.network_deviations.len() > 20);
    assert!(report.env_var_deviations.len() > 10);
}

/// Simulate a gradual drift scenario over time
#[test]
fn test_gradual_drift_over_time_simulation() {
    let mut manager = SupplyChainManager::new();
    
    // Create an approved manifest for a data processing service
    let expected_processes = vec![
        "data-processor".to_string(),
        "scheduler".to_string()
    ];
    
    let expected_files = vec![
        "/app/processor.conf".to_string(),
        "/data/input/".to_string(),
        "/data/output/".to_string()
    ];
    
    let expected_network = vec![
        "tcp:9090".to_string(), // Internal service communication
        "tcp:2181".to_string()  // Zookeeper
    ];
    
    let mut expected_env = HashMap::new();
    expected_env.insert("PROCESSOR_THREADS".to_string(), "4".to_string());
    expected_env.insert("INPUT_PATH".to_string(), "/data/input/".to_string());
    expected_env.insert("OUTPUT_PATH".to_string(), "/data/output/".to_string());
    
    // Create and store the approved manifest
    let manifest = manager.create_approved_manifest(
        "data-processor",
        "v1.0.5",
        expected_processes,
        expected_files,
        expected_network,
        expected_env,
    );
    let manifest_id = manifest.id.clone();
    assert!(manager.store_approved_manifest(manifest).is_ok());
    
    // Day 1: Normal operation
    let day1_processes = vec![
        "data-processor".to_string(),
        "scheduler".to_string()
    ];
    
    let day1_files = vec![
        "/app/processor.conf".to_string(),
        "/data/input/".to_string(),
        "/data/output/".to_string()
    ];
    
    let day1_network = vec![
        "tcp:9090".to_string(),
        "tcp:2181".to_string()
    ];
    
    let mut day1_env = HashMap::new();
    day1_env.insert("PROCESSOR_THREADS".to_string(), "4".to_string());
    day1_env.insert("INPUT_PATH".to_string(), "/data/input/".to_string());
    day1_env.insert("OUTPUT_PATH".to_string(), "/data/output/".to_string());
    
    let day1_container = manager.create_running_container(
        "data-processor-001",
        "data-processor-instance",
        "data-processor:v1.0.5",
        day1_processes,
        day1_files,
        day1_network,
        day1_env,
        &manifest_id,
    );
    assert!(manager.store_running_container(day1_container).is_ok());
    
    // Check for drift on day 1 - should be normal
    let day1_report = manager.check_runtime_drift("data-processor-001").unwrap();
    assert!(!day1_report.is_drift_incident);
    assert!(!day1_report.is_sneaky_container);
    
    // Day 5: Some suspicious activity
    let day5_processes = vec![
        "data-processor".to_string(),
        "scheduler".to_string(),
        "suspicious-downloader".to_string() // New suspicious process
    ];
    
    let day5_files = vec![
        "/app/processor.conf".to_string(),
        "/data/input/".to_string(),
        "/data/output/".to_string(),
        "/tmp/suspicious-payload.exe".to_string() // New suspicious file
    ];
    
    let day5_network = vec![
        "tcp:9090".to_string(),
        "tcp:2181".to_string(),
        "tcp:1337".to_string() // Suspicious connection
    ];
    
    let mut day5_env = HashMap::new();
    day5_env.insert("PROCESSOR_THREADS".to_string(), "4".to_string());
    day5_env.insert("INPUT_PATH".to_string(), "/data/input/".to_string());
    day5_env.insert("OUTPUT_PATH".to_string(), "/data/output/".to_string());
    day5_env.insert("CALLBACK_URL".to_string(), "http://malicious-site.com/callback".to_string()); // Suspicious env var
    
    // Update the container
    let day5_container = manager.create_running_container(
        "data-processor-001",
        "data-processor-instance",
        "data-processor:v1.0.5",
        day5_processes,
        day5_files,
        day5_network,
        day5_env,
        &manifest_id,
    );
    assert!(manager.store_running_container(day5_container).is_ok());
    
    // Check for drift on day 5 - should detect some deviations
    let day5_report = manager.check_runtime_drift("data-processor-001").unwrap();
    assert!(day5_report.is_drift_incident);
    assert!(!day5_report.is_sneaky_container); // Not enough deviations to be sneaky yet
    
    println!("Day 5 drift report:");
    println!("  Process deviations: {}", day5_report.process_deviations.len());
    println!("  File deviations: {}", day5_report.file_deviations.len());
    println!("  Network deviations: {}", day5_report.network_deviations.len());
    println!("  Environment variable deviations: {}", day5_report.env_var_deviations.len());
    
    // Day 10: Full compromise
    let day10_processes = vec![
        "data-processor".to_string(),
        "scheduler".to_string(),
        "suspicious-downloader".to_string(),
        "crypto-miner".to_string(),
        "reverse-shell".to_string(),
        "data-exfiltrator".to_string()
    ];
    
    let mut day10_files = vec![
        "/app/processor.conf".to_string(),
        "/data/input/".to_string(),
        "/data/output/".to_string()
    ];
    
    // Add many suspicious files
    for i in 0..10 {
        day10_files.push(format!("/tmp/malicious-{}.sh", i));
    }
    day10_files.push("/etc/passwd.bak".to_string());
    day10_files.push("/root/.ssh/authorized_keys".to_string());
    
    let mut day10_network = vec![
        "tcp:9090".to_string(),
        "tcp:2181".to_string()
    ];
    
    // Add many suspicious connections
    for i in 3000..3020 {
        day10_network.push(format!("tcp:{}", i));
    }
    day10_network.push("tcp:4444".to_string());
    day10_network.push("tcp:9001".to_string());
    
    let mut day10_env = HashMap::new();
    day10_env.insert("PROCESSOR_THREADS".to_string(), "4".to_string());
    day10_env.insert("INPUT_PATH".to_string(), "/data/input/".to_string());
    day10_env.insert("OUTPUT_PATH".to_string(), "/data/output/".to_string());
    day10_env.insert("CALLBACK_URL".to_string(), "http://malicious-site.com/callback".to_string());
    
    // Add many suspicious environment variables
    for i in 0..8 {
        day10_env.insert(format!("MALICIOUS_VAR_{}", i), format!("payload_{}", i));
    }
    day10_env.insert("C2_SERVER".to_string(), "evil.com".to_string());
    day10_env.insert("ENCRYPTION_KEY".to_string(), "super_secret_key".to_string());
    
    // Update the container
    let day10_container = manager.create_running_container(
        "data-processor-001",
        "data-processor-instance",
        "data-processor:v1.0.5",
        day10_processes,
        day10_files,
        day10_network,
        day10_env,
        &manifest_id,
    );
    assert!(manager.store_running_container(day10_container).is_ok());
    
    // Check for drift on day 10 - should detect significant deviations
    let day10_report = manager.check_runtime_drift("data-processor-001").unwrap();
    assert!(day10_report.is_drift_incident);
    assert!(day10_report.is_sneaky_container); // Now enough deviations to be considered sneaky
    
    println!("Day 10 drift report:");
    println!("  Process deviations: {}", day10_report.process_deviations.len());
    println!("  File deviations: {}", day10_report.file_deviations.len());
    println!("  Network deviations: {}", day10_report.network_deviations.len());
    println!("  Environment variable deviations: {}", day10_report.env_var_deviations.len());
    
    // Verify increasing deviation counts
    assert!(day10_report.process_deviations.len() > day5_report.process_deviations.len());
    assert!(day10_report.file_deviations.len() > day5_report.file_deviations.len());
    assert!(day10_report.network_deviations.len() > day5_report.network_deviations.len());
    assert!(day10_report.env_var_deviations.len() > day5_report.env_var_deviations.len());
}

/// Test drift incidents per week reporting with multiple containers
#[test]
fn test_drift_incidents_weekly_reporting_simulation() {
    let mut manager = SupplyChainManager::new();
    
    // Create manifests for different services
    let web_manifest = manager.create_approved_manifest(
        "web-service",
        "v1.0.0",
        vec!["nginx".to_string(), "app-server".to_string()],
        vec!["/etc/nginx/nginx.conf".to_string()],
        vec!["tcp:80".to_string(), "tcp:443".to_string()],
        HashMap::new(),
    );
    let web_manifest_id = web_manifest.id.clone();
    assert!(manager.store_approved_manifest(web_manifest).is_ok());
    
    let api_manifest = manager.create_approved_manifest(
        "api-service",
        "v2.0.0",
        vec!["api-server".to_string()],
        vec!["/app/config.json".to_string()],
        vec!["tcp:8080".to_string()],
        HashMap::new(),
    );
    let api_manifest_id = api_manifest.id.clone();
    assert!(manager.store_approved_manifest(api_manifest).is_ok());
    
    // Create multiple containers with different drift patterns
    // Normal web container
    let normal_web = manager.create_running_container(
        "web-001",
        "web-instance-001",
        "web-service:v1.0.0",
        vec!["nginx".to_string(), "app-server".to_string()],
        vec!["/etc/nginx/nginx.conf".to_string()],
        vec!["tcp:80".to_string(), "tcp:443".to_string()],
        HashMap::new(),
        &web_manifest_id,
    );
    assert!(manager.store_running_container(normal_web).is_ok());
    
    // Drifted web container
    let drifted_web = manager.create_running_container(
        "web-002",
        "web-instance-002",
        "web-service:v1.0.0",
        vec!["nginx".to_string(), "app-server".to_string(), "suspicious-process".to_string()],
        vec!["/etc/nginx/nginx.conf".to_string(), "/tmp/malicious.sh".to_string()],
        vec!["tcp:80".to_string(), "tcp:443".to_string(), "tcp:1337".to_string()],
        HashMap::new(),
        &web_manifest_id,
    );
    assert!(manager.store_running_container(drifted_web).is_ok());
    
    // Normal API container
    let normal_api = manager.create_running_container(
        "api-001",
        "api-instance-001",
        "api-service:v2.0.0",
        vec!["api-server".to_string()],
        vec!["/app/config.json".to_string()],
        vec!["tcp:8080".to_string()],
        HashMap::new(),
        &api_manifest_id,
    );
    assert!(manager.store_running_container(normal_api).is_ok());
    
    // Sneaky API container
    let mut sneaky_processes = vec!["api-server".to_string()];
    for i in 0..5 {
        sneaky_processes.push(format!("miner-{}", i));
    }
    
    let sneaky_api = manager.create_running_container(
        "api-002",
        "api-instance-002",
        "api-service:v2.0.0",
        sneaky_processes,
        vec!["/app/config.json".to_string()],
        vec!["tcp:8080".to_string()],
        HashMap::new(),
        &api_manifest_id,
    );
    assert!(manager.store_running_container(sneaky_api).is_ok());
    
    // Check drift for all containers
    manager.check_runtime_drift("web-001").unwrap(); // Normal - no incident
    manager.check_runtime_drift("web-002").unwrap(); // Drift incident
    manager.check_runtime_drift("api-001").unwrap(); // Normal - no incident
    manager.check_runtime_drift("api-002").unwrap(); // Sneaky container (drift incident)
    
    // Check weekly reporting
    let incidents = manager.get_drift_incidents_per_week();
    assert_eq!(incidents, 2); // Two drift incidents (web-002 and api-002)
    
    println!("Weekly drift incidents: {}", incidents);
    
    // Check statistics
    let (total_checks, drift_incidents, sneaky_containers, _) = manager.get_drift_stats();
    assert_eq!(total_checks, 4);
    assert_eq!(drift_incidents, 2);
    assert_eq!(sneaky_containers, 1);
    
    println!("Drift statistics:");
    println!("  Total checks: {}", total_checks);
    println!("  Drift incidents: {}", drift_incidents);
    println!("  Sneaky containers: {}", sneaky_containers);
}