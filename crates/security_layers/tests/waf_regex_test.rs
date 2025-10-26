#[cfg(test)]
mod regex_tests {
    use regex::Regex;

    #[test]
    fn test_sql_injection_pattern() {
        // Test the fixed regex pattern
        let pattern = Regex::new(r#"(?i)(union|select|insert|update|delete|drop|create|alter|exec|execute).*['"];"#).unwrap();
        
        // Test cases that should match
        assert!(pattern.is_match("SELECT * FROM users WHERE id = '1';"));
        assert!(pattern.is_match("UNION SELECT username, password FROM admin_users;"));
        assert!(pattern.is_match("DELETE FROM users WHERE id = '1';"));
        
        // Test cases that should not match (legitimate queries)
        assert!(!pattern.is_match("SELECT * FROM users WHERE id = 1"));
        assert!(!pattern.is_match("SELECT name, email FROM users"));
    }
}