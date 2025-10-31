//! Application Security Dependency Safety Features Validation Tests (Line 13)
//!
//! This module contains tests that specifically validate the Dependency Safety features from line 13 
//! of the web3_protection_layers.csv file:
//! 3,Application Security,Dependency Safety,SAST/SCA,"Static code scanning, dependency vulnerability scan, SBOM, license scan","Stop known-vuln libs from shipping","Critical vuln count, unresolved vuln age"

use security_layers::application_security::*;
use std::time::SystemTime;

/// Test that validates the specific Dependency Safety features from line 13 of web3_protection_layers.csv
#[test]
fn test_application_security_dependency_safety_line_13() {
    println!("Testing Application Security Dependency Safety features from line 13 of web3_protection_layers.csv...");
    
    // Test Line 13: Application Security, Dependency Safety, SAST/SCA
    // "Static code scanning, dependency vulnerability scan, SBOM, license scan"
    // "Stop known-vuln libs from shipping"
    // "Critical vuln count, unresolved vuln age"
    test_dependency_safety_features();
    
    println!("All Application Security Dependency Safety features from line 13 validated successfully!");
}

/// Test Application Security, Dependency Safety, SAST/SCA
/// Component/Mechanism: "Static code scanning, dependency vulnerability scan, SBOM, license scan"
/// Goal: "Stop known-vuln libs from shipping"
/// Evidence/Telemetry: "Critical vuln count, unresolved vuln age"
fn test_dependency_safety_features() {
    println!("Testing Application Security, Dependency Safety, SAST/SCA...");
    
    // Test Static Code Scanning
    test_static_code_scanning();
    
    // Test Dependency Vulnerability Scanning
    test_dependency_vulnerability_scanning();
    
    // Test SBOM Generation
    test_sbom_generation();
    
    // Test License Scanning
    test_license_scanning();
    
    // Test Evidence/Telemetry Collection
    test_evidence_collection();
    
    println!("✓ Dependency safety features validated");
}

/// Test Static Code Scanning
/// Component/Mechanism: "Static code scanning"
/// Goal: "Stop known-vuln libs from shipping"
fn test_static_code_scanning() {
    println!("  Testing Static Code Scanning...");
    
    let safety = DependencySafety::new();
    
    // Perform static analysis on a code file
    let issues = safety.perform_static_analysis("src/main.rs");
    
    // Should find some simulated issues
    assert!(!issues.is_empty());
    assert!(issues.len() >= 1);
    
    // Check that issues have proper structure
    for issue in &issues {
        assert!(!issue.id.is_empty());
        assert!(!issue.description.is_empty());
        assert!(!issue.file.is_empty());
        assert!(issue.line > 0);
    }
    
    println!("    ✓ Static code scanning validated");
}

/// Test Dependency Vulnerability Scanning
/// Component/Mechanism: "dependency vulnerability scan"
/// Goal: "Stop known-vuln libs from shipping"
fn test_dependency_vulnerability_scanning() {
    println!("  Testing Dependency Vulnerability Scanning...");
    
    let mut safety = DependencySafety::new();
    
    // Add vulnerabilities to the database
    let vuln1 = Vulnerability {
        id: "CVE-2023-12345".to_string(),
        package_name: "vulnerable-package".to_string(),
        affected_versions: vec!["1.0.0".to_string(), "1.0.1".to_string()],
        severity: Severity::Critical,
        description: "A critical security vulnerability".to_string(),
        published_date: SystemTime::now(),
    };
    safety.add_vulnerability(vuln1);
    
    let vuln2 = Vulnerability {
        id: "CVE-2023-54321".to_string(),
        package_name: "another-vuln-package".to_string(),
        affected_versions: vec!["2.0.0".to_string()],
        severity: Severity::High,
        description: "A high severity vulnerability".to_string(),
        published_date: SystemTime::now(),
    };
    safety.add_vulnerability(vuln2);
    
    // Create dependencies to scan
    let dependencies = vec![
        security_layers::application_security::Dependency {
            name: "vulnerable-package".to_string(),
            version: "1.0.1".to_string(),
            license: Some("MIT".to_string()),
        },
        security_layers::application_security::Dependency {
            name: "safe-package".to_string(),
            version: "2.0.0".to_string(),
            license: Some("Apache-2.0".to_string()),
        },
        security_layers::application_security::Dependency {
            name: "another-vuln-package".to_string(),
            version: "2.0.0".to_string(),
            license: Some("BSD-3-Clause".to_string()),
        }
    ];
    
    // Scan dependencies
    let findings = safety.scan_dependencies(&dependencies);
    
    // Should find two vulnerabilities
    assert_eq!(findings.len(), 2);
    
    // Check that findings have proper structure
    let mut found_critical = false;
    let mut found_high = false;
    
    for finding in &findings {
        assert!(!finding.dependency.name.is_empty());
        assert!(!finding.vulnerability.id.is_empty());
        assert!(!finding.vulnerability.description.is_empty());
        
        if finding.vulnerability.id == "CVE-2023-12345" {
            assert_eq!(finding.severity, Severity::Critical);
            found_critical = true;
        } else if finding.vulnerability.id == "CVE-2023-54321" {
            assert_eq!(finding.severity, Severity::High);
            found_high = true;
        }
    }
    
    assert!(found_critical);
    assert!(found_high);
    
    println!("    ✓ Dependency vulnerability scanning validated");
}

/// Test SBOM Generation
/// Component/Mechanism: "SBOM"
/// Goal: "Stop known-vuln libs from shipping"
fn test_sbom_generation() {
    println!("  Testing SBOM Generation...");
    
    let mut safety = DependencySafety::new();
    
    // Add components to SBOM
    let component1 = security_layers::application_security::Component {
        id: "component-1".to_string(),
        name: "example-component".to_string(),
        version: "1.0.0".to_string(),
        license: Some("MIT".to_string()),
        supplier: Some("Example Corp".to_string()),
    };
    safety.add_component(component1);
    
    let component2 = security_layers::application_security::Component {
        id: "component-2".to_string(),
        name: "another-component".to_string(),
        version: "2.0.0".to_string(),
        license: Some("Apache-2.0".to_string()),
        supplier: Some("Another Corp".to_string()),
    };
    safety.add_component(component2);
    
    // Generate SBOM
    let sbom = safety.generate_sbom("test-project");
    
    assert_eq!(sbom.project_name, "test-project");
    assert_eq!(sbom.components.len(), 2);
    
    // Check that components have proper structure
    let mut found_component1 = false;
    let mut found_component2 = false;
    
    for component in &sbom.components {
        assert!(!component.id.is_empty());
        assert!(!component.name.is_empty());
        assert!(!component.version.is_empty());
        
        if component.id == "component-1" {
            assert_eq!(component.name, "example-component");
            assert_eq!(component.version, "1.0.0");
            assert_eq!(component.license, Some("MIT".to_string()));
            assert_eq!(component.supplier, Some("Example Corp".to_string()));
            found_component1 = true;
        } else if component.id == "component-2" {
            assert_eq!(component.name, "another-component");
            assert_eq!(component.version, "2.0.0");
            assert_eq!(component.license, Some("Apache-2.0".to_string()));
            assert_eq!(component.supplier, Some("Another Corp".to_string()));
            found_component2 = true;
        }
    }
    
    assert!(found_component1);
    assert!(found_component2);
    
    println!("    ✓ SBOM generation validated");
}

/// Test License Scanning
/// Component/Mechanism: "license scan"
/// Goal: "Stop known-vuln libs from shipping"
fn test_license_scanning() {
    println!("  Testing License Scanning...");
    
    let mut safety = DependencySafety::new();
    
    // Add license policies
    let gpl_policy = LicensePolicy {
        license_id: "GPL-3.0".to_string(),
        allowed: false,
        description: "GPL license not allowed due to licensing restrictions".to_string(),
    };
    safety.add_license_policy(gpl_policy);
    
    let agpl_policy = LicensePolicy {
        license_id: "AGPL-3.0".to_string(),
        allowed: false,
        description: "AGPL license not allowed due to licensing restrictions".to_string(),
    };
    safety.add_license_policy(agpl_policy);
    
    // Create dependencies to scan
    let dependencies = vec![
        security_layers::application_security::Dependency {
            name: "allowed-package".to_string(),
            version: "1.0.0".to_string(),
            license: Some("MIT".to_string()),
        },
        security_layers::application_security::Dependency {
            name: "restricted-package-1".to_string(),
            version: "2.0.0".to_string(),
            license: Some("GPL-3.0".to_string()),
        },
        security_layers::application_security::Dependency {
            name: "restricted-package-2".to_string(),
            version: "3.0.0".to_string(),
            license: Some("AGPL-3.0".to_string()),
        },
        security_layers::application_security::Dependency {
            name: "another-allowed-package".to_string(),
            version: "1.5.0".to_string(),
            license: Some("Apache-2.0".to_string()),
        }
    ];
    
    // Scan licenses
    let findings = safety.scan_licenses(&dependencies);
    
    // Should find two license violations
    assert_eq!(findings.len(), 2);
    
    // Check that findings have proper structure
    let mut found_gpl_violation = false;
    let mut found_agpl_violation = false;
    
    for finding in &findings {
        assert!(!finding.dependency.name.is_empty());
        assert!(!finding.license_id.is_empty());
        assert!(finding.violation);
        
        if finding.license_id == "GPL-3.0" {
            assert_eq!(finding.dependency.name, "restricted-package-1");
            found_gpl_violation = true;
        } else if finding.license_id == "AGPL-3.0" {
            assert_eq!(finding.dependency.name, "restricted-package-2");
            found_agpl_violation = true;
        }
    }
    
    assert!(found_gpl_violation);
    assert!(found_agpl_violation);
    
    println!("    ✓ License scanning validated");
}

/// Test Evidence/Telemetry Collection
/// Evidence/Telemetry: "Critical vuln count, unresolved vuln age"
fn test_evidence_collection() {
    println!("  Testing Evidence/Telemetry Collection...");
    
    let safety = DependencySafety::new();
    
    // Create some vulnerability findings for statistics
    let findings = vec![
        VulnerabilityFinding {
            dependency: security_layers::application_security::Dependency {
                name: "critical-package".to_string(),
                version: "1.0.0".to_string(),
                license: Some("MIT".to_string()),
            },
            vulnerability: Vulnerability {
                id: "CVE-2023-00001".to_string(),
                package_name: "critical-package".to_string(),
                affected_versions: vec!["1.0.0".to_string()],
                severity: Severity::Critical,
                description: "Critical vulnerability".to_string(),
                published_date: SystemTime::now(),
            },
            severity: Severity::Critical,
        },
        VulnerabilityFinding {
            dependency: security_layers::application_security::Dependency {
                name: "high-package".to_string(),
                version: "2.0.0".to_string(),
                license: Some("Apache-2.0".to_string()),
            },
            vulnerability: Vulnerability {
                id: "CVE-2023-00002".to_string(),
                package_name: "high-package".to_string(),
                affected_versions: vec!["2.0.0".to_string()],
                severity: Severity::High,
                description: "High vulnerability".to_string(),
                published_date: SystemTime::now(),
            },
            severity: Severity::High,
        },
        VulnerabilityFinding {
            dependency: security_layers::application_security::Dependency {
                name: "another-critical-package".to_string(),
                version: "1.5.0".to_string(),
                license: Some("BSD-3-Clause".to_string()),
            },
            vulnerability: Vulnerability {
                id: "CVE-2023-00003".to_string(),
                package_name: "another-critical-package".to_string(),
                affected_versions: vec!["1.5.0".to_string()],
                severity: Severity::Critical,
                description: "Another critical vulnerability".to_string(),
                published_date: SystemTime::now(),
            },
            severity: Severity::Critical,
        }
    ];
    
    // Test critical vulnerability count
    let critical_count = safety.get_critical_vuln_count(&findings);
    assert_eq!(critical_count, 2); // Two critical vulnerabilities
    
    // Test unresolved vulnerability age
    let ages = safety.get_unresolved_vuln_age(&findings);
    assert_eq!(ages.len(), 3); // Three vulnerabilities
    
    // All ages should be non-negative
    for age in &ages {
        assert!(*age >= 0);
    }
    
    // Record scan results for telemetry
    let scan_result = ScanResult {
        timestamp: SystemTime::now(),
        vuln_count: findings.len(),
        license_violations: 0,
        critical_vuln_count: critical_count,
    };
    
    let mut safety_with_results = DependencySafety::new();
    safety_with_results.record_scan_results("scan-001", scan_result);
    
    // Retrieve scan results
    let retrieved_result = safety_with_results.get_scan_results("scan-001");
    assert!(retrieved_result.is_some());
    let retrieved_result = retrieved_result.unwrap();
    assert_eq!(retrieved_result.vuln_count, 3);
    assert_eq!(retrieved_result.critical_vuln_count, 2);
    
    println!("    ✓ Evidence/telemetry collection validated");
}