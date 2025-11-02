#[cfg(test)]
mod regex_tests {
    use regex::Regex;

    #[test]
    fn test_sql_injection_pattern() {
        // Test the fixed regex pattern
        // More specific pattern that looks for SQL injection attempts with quotes or semicolons
        let pattern = Regex::new(r#"(?i)(union\s+select|select.*['"];|insert\s+into|update.*set|delete\s+from|drop\s+table|create\s+table|alter\s+table|exec\s+|execute\s+)"#).unwrap();
        
        // Test cases that should match (SQL injection attempts)
        assert!(pattern.is_match("SELECT * FROM users WHERE id = '1';"));
        assert!(pattern.is_match("UNION SELECT username, password FROM admin_users;"));
        assert!(pattern.is_match("DELETE FROM users WHERE id = '1';"));
        
        // Test cases that should not match (legitimate queries)
        assert!(!pattern.is_match("SELECT * FROM users WHERE id = 1"));
        assert!(!pattern.is_match("SELECT name, email FROM users"));
    }
}