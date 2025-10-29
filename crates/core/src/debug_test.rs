//! Debug test for CI/CD gatekeeping

use crate::supply_chain::*;

#[test]
fn debug_build_blocking() {
    let mut manager = SupplyChainManager::new();
    
    // Create a build
    let source = SourceInfo {
        repo_url: "https://github.com/example/test-app".to_string(),
        commit_hash: "a1b2c3d4e5f".to_string(),
        branch: "main".to_string(),
        tag: Some("v1.0.0".to_string()),
    };

    let build_config = BuildConfig {
        build_script: "build.sh".to_string(),
        environment: std::collections::HashMap::new(),
        tools: std::collections::HashMap::new(),
    };

    let artifacts = vec![Artifact {
        name: "test-app-binary".to_string(),
        uri: "artifact://test-app-v1.0.0".to_string(),
        hash: "sha256:test123".to_string(),
        size: 2048,
        signature: None,
        sbom: None,
        is_signed: false,
        created: 0,
    }];
    
    let build = manager.create_build("123", source, build_config, artifacts).unwrap();
    let build_id = build.id.clone();
    manager.store_build(build.clone()).unwrap();
    
    // Check initial state
    println!("Initial policy rules count: {}", manager.policy_rules.len());
    for (id, rule) in &manager.policy_rules {
        println!("Rule: {} - Enabled: {} - Severity: {} - Category: {}", 
                 id, rule.enabled, rule.severity, rule.category);
    }
    
    // Check build policy rules
    let stored_build = manager.get_build(&build_id).unwrap();
    println!("Build policy rules count: {}", stored_build.policy_rules.len());
    for rule in &stored_build.policy_rules {
        println!("Build Rule: {} - Enabled: {} - Severity: {} - Category: {}", 
                 rule.id, rule.enabled, rule.severity, rule.category);
    }
    
    // Add a critical vulnerability
    let critical_vuln = Vulnerability {
        id: "CVE-2023-99999".to_string(),
        severity: 9.5,
        description: "Critical vulnerability".to_string(),
        fix_available: false,
    };
    
    let scan_id = format!("scan-{}-{}", build.id, manager.current_timestamp());
    let critical_scan = SecurityScanResult {
        id: scan_id,
        build_id: build.id.clone(),
        scan_type: "sast".to_string(),
        vulnerabilities: vec![critical_vuln],
        licenses: vec!["MIT".to_string()],
        completed: manager.current_timestamp(),
        passed: false,
    };
    
    manager.security_scans.insert(critical_scan.id.clone(), critical_scan);
    
    // Record test results
    manager.record_test_results(&build.id, "unit", 100, 100, 0, 95.0).unwrap();
    
    // Evaluate policies
    let result = manager.evaluate_policies(&build.id).unwrap();
    println!("Policy evaluation result: {}", result);
    
    // Check policy results
    let updated_build = manager.get_build(&build.id).unwrap();
    println!("Build passed policies: {}", updated_build.passed_policies);
    println!("Policy results:");
    for (rule_id, passed) in &updated_build.policy_results {
        println!("  {}: {}", rule_id, passed);
    }
    
    // Check statistics
    let (total, blocked, scans, tests) = manager.get_cicd_stats();
    println!("Stats - Total: {}, Blocked: {}, Scans: {}, Tests: {}", total, blocked, scans, tests);
    
    // The build should be blocked
    assert!(!result); // Should fail due to critical vulnerability
    assert!(!updated_build.passed_policies);
    assert_eq!(blocked, 1); // One build blocked
}