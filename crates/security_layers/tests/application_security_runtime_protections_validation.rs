//! Application Security Runtime Protections Features Validation Tests (Line 14)
//!
//! This module contains tests that specifically validate the Runtime Protections features from line 14 
//! of the web3_protection_layers.csv file:
//! 3,Application Security,Runtime Protections,WAF / RASP,"WAF rulesets (OWASP Top 10), runtime self-protection hooks in app","Block exploit patterns pre-database","WAF block events, rule hit rate"

use security_layers::application_security::*;

/// Test that validates the specific Runtime Protections features from line 14 of web3_protection_layers.csv
#[test]
fn test_application_security_runtime_protections_line_14() {
    println!("Testing Application Security Runtime Protections features from line 14 of web3_protection_layers.csv...");
    
    // Test Line 14: Application Security, Runtime Protections, WAF / RASP
    // "WAF rulesets (OWASP Top 10), runtime self-protection hooks in app"
    // "Block exploit patterns pre-database"
    // "WAF block events, rule hit rate"
    test_runtime_protections_features();
    
    println!("All Application Security Runtime Protections features from line 14 validated successfully!");
}

/// Test Application Security, Runtime Protections, WAF / RASP
/// Component/Mechanism: "WAF rulesets (OWASP Top 10), runtime self-protection hooks in app"
/// Goal: "Block exploit patterns pre-database"
/// Evidence/Telemetry: "WAF block events, rule hit rate"
fn test_runtime_protections_features() {
    println!("Testing Application Security, Runtime Protections, WAF / RASP...");
    
    // Test WAF rulesets (OWASP Top 10)
    test_waf_owasp_rulesets();
    
    // Test runtime self-protection hooks in app
    test_rasp_hooks();
    
    // Test Evidence/Telemetry Collection
    test_runtime_protections_evidence_collection();
    
    println!("✓ Runtime protections features validated");
}

/// Test WAF rulesets (OWASP Top 10)
/// Component/Mechanism: "WAF rulesets (OWASP Top 10)"
/// Goal: "Block exploit patterns pre-database"
fn test_waf_owasp_rulesets() {
    println!("  Testing WAF rulesets (OWASP Top 10)...");
    
    let mut waf = WebApplicationFirewall::new();
    
    // Test SQL Injection detection
    let sql_request = HttpRequest {
        raw_data: "SELECT * FROM users WHERE id = '1'; DROP TABLE users;".to_string(),
        method: "POST".to_string(),
        path: "/api/users".to_string(),
        headers: std::collections::HashMap::new(),
        body: "".to_string(),
        query_params: std::collections::HashMap::new(),
        client_ip: "127.0.0.1".to_string(),
    };
    
    let result = waf.check_request(&sql_request);
    match result {
        WafResult::Blocked { rule_id, category, .. } => {
            assert_eq!(rule_id, "INJ-001");
            assert_eq!(category, "injection");
        },
        _ => panic!("Expected SQL injection request to be blocked"),
    }
    
    // Test XSS detection
    let xss_request = HttpRequest {
        raw_data: "<script>alert('XSS')</script>".to_string(),
        method: "GET".to_string(),
        path: "/search".to_string(),
        headers: std::collections::HashMap::new(),
        body: "".to_string(),
        query_params: std::collections::HashMap::new(),
        client_ip: "127.0.0.1".to_string(),
    };
    
    let result = waf.check_request(&xss_request);
    match result {
        WafResult::Blocked { rule_id, category, .. } => {
            assert_eq!(rule_id, "XSS-001");
            assert_eq!(category, "xss");
        },
        _ => panic!("Expected XSS request to be blocked"),
    }
    
    // Test legitimate request is allowed
    let legitimate_request = HttpRequest {
        raw_data: "SELECT name, email FROM users WHERE active = 1".to_string(),
        method: "GET".to_string(),
        path: "/api/users".to_string(),
        headers: std::collections::HashMap::new(),
        body: "".to_string(),
        query_params: std::collections::HashMap::new(),
        client_ip: "127.0.0.1".to_string(),
    };
    
    let result = waf.check_request(&legitimate_request);
    match result {
        WafResult::Allowed => {},
        _ => panic!("Expected legitimate request to be allowed"),
    }
    
    println!("    ✓ WAF OWASP rulesets validated");
}

/// Test runtime self-protection hooks in app
/// Component/Mechanism: "runtime self-protection hooks in app"
/// Goal: "Block exploit patterns pre-database"
fn test_rasp_hooks() {
    println!("  Testing runtime self-protection hooks in app...");
    
    let mut rasp = RuntimeSelfProtection::new();
    
    // Test SQL injection protection
    let result = rasp.check_sql_injection("SELECT * FROM users WHERE id = '1' OR '1'='1';");
    match result {
        RaspResult::Blocked { protection_type, .. } => {
            assert_eq!(protection_type, "sql_injection");
        },
        _ => panic!("Expected SQL injection to be blocked"),
    }
    
    // Test path traversal protection
    let result = rasp.check_file_access("../../etc/passwd");
    match result {
        RaspResult::Blocked { protection_type, .. } => {
            assert_eq!(protection_type, "path_traversal");
        },
        _ => panic!("Expected path traversal to be blocked"),
    }
    
    // Test command execution protection
    let result = rasp.check_command_execution("rm -rf /");
    match result {
        RaspResult::Blocked { protection_type, .. } => {
            assert_eq!(protection_type, "command_execution");
        },
        _ => panic!("Expected command execution to be blocked"),
    }
    
    // Test legitimate operations are allowed
    let result = rasp.check_sql_injection("SELECT name, email FROM users WHERE active = 1");
    match result {
        RaspResult::Allowed => {},
        _ => panic!("Expected legitimate SQL query to be allowed"),
    }
    
    let result = rasp.check_file_access("/home/user/documents/file.txt");
    match result {
        RaspResult::Allowed => {},
        _ => panic!("Expected legitimate file access to be allowed"),
    }
    
    let result = rasp.check_command_execution("ls -la");
    match result {
        RaspResult::Allowed => {},
        _ => panic!("Expected legitimate command to be allowed"),
    }
    
    println!("    ✓ RASP hooks validated");
}

/// Test Evidence/Telemetry Collection
/// Evidence/Telemetry: "WAF block events, rule hit rate"
fn test_runtime_protections_evidence_collection() {
    println!("  Testing Evidence/Telemetry Collection...");
    
    let mut waf = WebApplicationFirewall::new();
    
    // Create requests that will trigger different rules
    let sql_request = HttpRequest {
        raw_data: "SELECT * FROM users WHERE id = '1';".to_string(),
        method: "POST".to_string(),
        path: "/api/users".to_string(),
        headers: std::collections::HashMap::new(),
        body: "".to_string(),
        query_params: std::collections::HashMap::new(),
        client_ip: "127.0.0.1".to_string(),
    };
    
    let xss_request = HttpRequest {
        raw_data: "<script>alert('XSS')</script>".to_string(),
        method: "GET".to_string(),
        path: "/search".to_string(),
        headers: std::collections::HashMap::new(),
        body: "".to_string(),
        query_params: std::collections::HashMap::new(),
        client_ip: "127.0.0.1".to_string(),
    };
    
    // Trigger rules
    waf.check_request(&sql_request);
    waf.check_request(&xss_request);
    waf.check_request(&xss_request); // Trigger XSS rule again
    
    // Test WAF block events
    assert_eq!(waf.get_block_events("injection"), 1);
    assert_eq!(waf.get_block_events("xss"), 2);
    
    // Test rule hit counts
    assert_eq!(waf.get_rule_hit_count("INJ-001"), 1);
    assert_eq!(waf.get_rule_hit_count("XSS-001"), 2);
    
    // Test rule hit rate
    let rates = waf.get_rule_hit_rate();
    assert_eq!(rates.len(), 2);
    assert!(rates.contains_key("injection"));
    assert!(rates.contains_key("xss"));
    
    // Test RASP protection events
    let mut rasp = RuntimeSelfProtection::new();
    rasp.check_sql_injection("SELECT * FROM users WHERE id = '1' OR '1'='1';");
    rasp.check_file_access("../../etc/passwd");
    rasp.check_command_execution("rm -rf /");
    
    assert_eq!(rasp.get_protection_events("sql_injection"), 1);
    assert_eq!(rasp.get_protection_events("path_traversal"), 1);
    assert_eq!(rasp.get_protection_events("command_execution"), 1);
    
    println!("    ✓ Evidence/telemetry collection validated");
}