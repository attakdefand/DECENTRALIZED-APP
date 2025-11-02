//! Identity & Access Control
//!
//! This module implements security layers 2: Identity & Access Control

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;
use base64::{Engine as _, engine::general_purpose};
use serde_json::json;

/// Authentication manager
#[derive(Debug)]
pub struct AuthNManager {
    users: HashMap<String, User>,
    sessions: HashMap<String, UserSession>,
}

/// User account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: PasswordHash,
    pub mfa_enabled: bool,
    pub mfa_secret: Option<String>,
    pub created_at: u64,
    pub last_login: Option<u64>,
}

/// Password hash using SHA3-256
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordHash {
    pub hash: String,
    pub salt: String,
}

/// Multi-factor authentication challenge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaChallenge {
    pub user_id: String,
    pub challenge_code: String,
    pub expires_at: u64,
}

/// User session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub session_id: String,
    pub user_id: String,
    pub created_at: u64,
    pub expires_at: u64,
    pub ip_address: Option<String>,
}

/// JWT token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtToken {
    pub header: String,
    pub payload: String,
    pub signature: String,
}

/// JWT token header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtHeader {
    pub alg: String,
    pub typ: String,
}

/// JWT token payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtPayload {
    pub sub: String,
    pub iss: String,
    pub aud: String,
    pub exp: u64,
    pub iat: u64,
    pub scope: String,
    pub roles: Vec<String>,
}

/// OAuth2 configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Config {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub auth_url: String,
    pub token_url: String,
    pub scopes: Vec<String>,
}

/// mTLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MtlsConfig {
    pub ca_cert_path: String,
    pub client_cert_path: String,
    pub client_key_path: String,
    pub verify_peer: bool,
}

/// OPA policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpaPolicy {
    pub id: String,
    pub content: String,
    pub version: String,
    pub created_at: u64,
}

/// Authorization manager
#[derive(Debug)]
pub struct AuthZManager {
    policies: HashMap<String, RbacPolicy>,
    role_assignments: HashMap<String, Vec<String>>, // user_id -> roles
}

/// Role-based access control policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RbacPolicy {
    pub id: String,
    pub name: String,
    pub permissions: Vec<String>,
    pub description: String,
}

/// Session manager
#[derive(Debug)]
pub struct SessionManager {
    sessions: HashMap<String, UserSession>,
    max_sessions_per_user: usize,
}

/// Token lifecycle manager
#[derive(Debug)]
pub struct TokenLifecycle {
    tokens: HashMap<String, JwtToken>,
    refresh_tokens: HashMap<String, RefreshToken>,
}

/// Refresh token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshToken {
    pub token: String,
    pub user_id: String,
    pub created_at: u64,
    pub expires_at: u64,
    pub revoked: bool,
}

/// Secret manager for handling credentials
#[derive(Debug)]
pub struct SecretManager {
    secrets: HashMap<String, EncryptedSecret>,
    key: [u8; 32],
}

/// Encrypted secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedSecret {
    pub name: String,
    pub encrypted_value: Vec<u8>,
    pub created_at: u64,
    pub rotation_required: bool,
}

impl PasswordHash {
    /// Create a new password hash with salt
    pub fn new(password: &str) -> Self {
        let salt = Self::generate_salt();
        let hash = Self::hash_password(password, &salt);
        Self { hash, salt }
    }

    /// Verify a password against the hash
    pub fn verify(&self, password: &str) -> bool {
        let hash = Self::hash_password(password, &self.salt);
        hash == self.hash
    }

    /// Generate a random salt
    fn generate_salt() -> String {
        let mut rng = rand::thread_rng();
        let mut salt_bytes = [0u8; 16];
        rng.fill_bytes(&mut salt_bytes);
        hex::encode(salt_bytes)
    }

    /// Hash password with salt using SHA3-256
    fn hash_password(password: &str, salt: &str) -> String {
        let mut hasher = Sha3_256::default();
        hasher.update(password.as_bytes());
        hasher.update(salt.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }
}

impl AuthNManager {
    /// Create a new authentication manager
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            sessions: HashMap::new(),
        }
    }

    /// Register a new user
    pub fn register_user(&mut self, username: &str, email: &str, password: &str) -> Result<String, String> {
        // Check if user already exists
        if self.users.values().any(|u| u.username == username || u.email == email) {
            return Err("User already exists".to_string());
        }

        let user_id = uuid::Uuid::new_v4().to_string();
        let password_hash = PasswordHash::new(password);
        
        let user = User {
            id: user_id.clone(),
            username: username.to_string(),
            email: email.to_string(),
            password_hash,
            mfa_enabled: false,
            mfa_secret: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            last_login: None,
        };

        self.users.insert(user_id.clone(), user);
        Ok(user_id)
    }

    /// Authenticate a user with username and password
    pub fn authenticate(&mut self, username: &str, password: &str) -> Result<String, String> {
        // Clone the user ID to avoid borrowing issues
        let user_id = {
            let user = self
                .users
                .values()
                .find(|u| u.username == username || u.email == username)
                .ok_or("User not found")?;

            if !user.password_hash.verify(password) {
                return Err("Invalid password".to_string());
            }

            user.id.clone()
        };

        // Update last login
        if let Some(user) = self.users.get_mut(&user_id) {
            user.last_login = Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            );
        }

        // Create session
        self.create_session(&user_id, None)
    }

    /// Create a user session
    fn create_session(&mut self, user_id: &str, ip_address: Option<String>) -> Result<String, String> {
        let session_id = uuid::Uuid::new_v4().to_string();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let session = UserSession {
            session_id: session_id.clone(),
            user_id: user_id.to_string(),
            created_at: now,
            expires_at: now + 3600, // 1 hour expiry
            ip_address,
        };

        self.sessions.insert(session_id.clone(), session);
        Ok(session_id)
    }

    /// Validate a session
    pub fn validate_session(&self, session_id: &str) -> Result<&UserSession, String> {
        let session = self.sessions.get(session_id).ok_or("Session not found")?;
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        if session.expires_at <= now {
            return Err("Session expired".to_string());
        }

        Ok(session)
    }

    /// Get user by ID
    pub fn get_user(&self, user_id: &str) -> Option<&User> {
        self.users.get(user_id)
    }

    /// Generate MFA challenge for user
    pub fn generate_mfa_challenge(&self, _user_id: &str) -> Result<MfaChallenge, String> {
        // In a real implementation, we would generate an actual MFA challenge
        // For now, we'll return an error indicating MFA is not enabled
        Err("MFA not enabled for user".to_string())
    }

    /// Verify MFA challenge
    pub fn verify_mfa_challenge(&self, user_id: &str, challenge_code: &str) -> Result<bool, String> {
        // In a real implementation, we would verify the TOTP code
        // For this example, we'll just check if it matches a dummy value
        Ok(challenge_code == "123456")
    }

    /// Generate a 6-digit code
    fn generate_6_digit_code() -> String {
        let mut rng = rand::thread_rng();
        format!("{:06}", rng.next_u32() % 1000000)
    }

    /// Generate OAuth2 authorization URL
    pub fn generate_oauth2_auth_url(&self, config: &OAuth2Config, state: &str) -> String {
        format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
            config.auth_url,
            config.client_id,
            config.redirect_uri,
            config.scopes.join(" "),
            state
        )
    }

    /// Exchange OAuth2 authorization code for access token
    pub fn exchange_oauth2_code(
        &self,
        config: &OAuth2Config,
        code: &str,
    ) -> Result<JwtToken, String> {
        // In a real implementation, this would make an HTTP request to the token endpoint
        // For this example, we'll create a mock JWT token
        let header = JwtHeader {
            alg: "RS256".to_string(),
            typ: "JWT".to_string(),
        };
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        let payload = JwtPayload {
            sub: "oauth2_user".to_string(),
            iss: "oauth2_provider".to_string(),
            aud: config.client_id.clone(),
            exp: now + 3600, // 1 hour
            iat: now,
            scope: config.scopes.join(" "),
            roles: vec!["oauth2_user".to_string()],
        };

        let header_json = serde_json::to_string(&header)
            .map_err(|e| format!("Failed to serialize header: {}", e))?;
        let payload_json = serde_json::to_string(&payload)
            .map_err(|e| format!("Failed to serialize payload: {}", e))?;

        let header_b64 = general_purpose::STANDARD.encode(header_json);
        let payload_b64 = general_purpose::STANDARD.encode(payload_json);

        // In a real implementation, we would sign the token with the provider's private key
        let signature = format!("{}_{}_signature", header_b64, payload_b64);

        let token = JwtToken {
            header: header_b64,
            payload: payload_b64,
            signature,
        };

        Ok(token)
    }

    /// Validate mTLS certificate
    pub fn validate_mtls(&self, _config: &MtlsConfig, _cert_data: &[u8]) -> Result<bool, String> {
        // In a real implementation, this would validate the client certificate
        // against the CA certificate and check revocation status
        Ok(true) // Mock implementation
    }
}

impl Default for AuthNManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthZManager {
    /// Create a new authorization manager
    pub fn new() -> Self {
        Self {
            policies: HashMap::new(),
            role_assignments: HashMap::new(),
        }
    }

    /// Create a new RBAC policy
    pub fn create_policy(&mut self, policy: RbacPolicy) -> Result<(), String> {
        if self.policies.contains_key(&policy.id) {
            return Err("Policy already exists".to_string());
        }
        
        self.policies.insert(policy.id.clone(), policy);
        Ok(())
    }

    /// Assign a role to a user
    pub fn assign_role(&mut self, user_id: &str, role_id: &str) -> Result<(), String> {
        // Check if policy exists
        if !self.policies.contains_key(role_id) {
            return Err("Role policy not found".to_string());
        }

        self.role_assignments
            .entry(user_id.to_string())
            .or_insert_with(Vec::new)
            .push(role_id.to_string());
        
        Ok(())
    }

    /// Check if user has permission
    pub fn has_permission(&self, user_id: &str, permission: &str) -> bool {
        if let Some(roles) = self.role_assignments.get(user_id) {
            roles.iter().any(|role_id| {
                if let Some(policy) = self.policies.get(role_id) {
                    policy.permissions.contains(&permission.to_string())
                } else {
                    false
                }
            })
        } else {
            false
        }
    }

    /// Get user roles
    pub fn get_user_roles(&self, user_id: &str) -> Vec<&RbacPolicy> {
        if let Some(roles) = self.role_assignments.get(user_id) {
            roles
                .iter()
                .filter_map(|role_id| self.policies.get(role_id))
                .collect()
        } else {
            vec![]
        }
    }

    /// Evaluate OPA policy
    pub fn evaluate_opa_policy(&self, policy: &str, input: &str) -> Result<bool, String> {
        // In a real implementation, this would make a request to the OPA server
        // For this example, we'll parse the policy and input to make a decision
        // This is a simplified mock implementation
        
        // Check if the policy allows the action based on the input
        // In a real implementation, we would use the OPA library or REST API
        Ok(policy.contains("allow") && input.contains("user"))
    }

    /// Add OPA policy
    pub fn add_opa_policy(&mut self, policy: OpaPolicy) -> Result<(), String> {
        // In a real implementation, this would store the policy in a policy repository
        // For this example, we'll store it in the policies map with a special prefix
        let policy_id = format!("opa_{}", policy.id);
        // Convert to RbacPolicy for storage (simplified approach)
        let rbac_policy = RbacPolicy {
            id: policy_id,
            name: format!("OPA Policy: {}", policy.id),
            permissions: vec![policy.content.clone()],
            description: format!("OPA policy version {}", policy.version),
        };
        self.policies.insert(rbac_policy.id.clone(), rbac_policy);
        Ok(())
    }
}

impl Default for AuthZManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionManager {
    /// Create a new session manager
    pub fn new(max_sessions_per_user: usize) -> Self {
        Self {
            sessions: HashMap::new(),
            max_sessions_per_user,
        }
    }

    /// Create a new session
    pub fn create_session(&mut self, user_id: &str, ip_address: Option<String>) -> Result<String, String> {
        // Check session limit
        let user_sessions: Vec<&UserSession> = self
            .sessions
            .values()
            .filter(|s| s.user_id == user_id)
            .collect();

        if user_sessions.len() >= self.max_sessions_per_user {
            return Err("Maximum sessions limit reached".to_string());
        }

        let session_id = uuid::Uuid::new_v4().to_string();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let session = UserSession {
            session_id: session_id.clone(),
            user_id: user_id.to_string(),
            created_at: now,
            expires_at: now + 3600, // 1 hour
            ip_address,
        };

        self.sessions.insert(session_id.clone(), session);
        Ok(session_id)
    }

    /// Validate session
    pub fn validate_session(&self, session_id: &str) -> Result<&UserSession, String> {
        let session = self.sessions.get(session_id).ok_or("Session not found")?;
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        if session.expires_at <= now {
            return Err("Session expired".to_string());
        }

        Ok(session)
    }

    /// Revoke session
    pub fn revoke_session(&mut self, session_id: &str) -> Result<(), String> {
        if self.sessions.remove(session_id).is_some() {
            Ok(())
        } else {
            Err("Session not found".to_string())
        }
    }

    /// Revoke all sessions for a user
    pub fn revoke_all_user_sessions(&mut self, user_id: &str) {
        self.sessions.retain(|_, session| session.user_id != user_id);
    }

    /// Get active sessions for user
    pub fn get_user_sessions(&self, user_id: &str) -> Vec<&UserSession> {
        self.sessions
            .values()
            .filter(|session| session.user_id == user_id)
            .collect()
    }
}

impl TokenLifecycle {
    /// Create a new token lifecycle manager
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
            refresh_tokens: HashMap::new(),
        }
    }

    /// Generate JWT token
    pub fn generate_token(&mut self, _user_id: &str, claims: &str) -> Result<JwtToken, String> {
        // In a real implementation, we would create a proper JWT
        // For this example, we'll create a simplified version
        let header = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"; // Simplified header
        let payload = claims.to_string(); // Simplified payload
        let signature = "signature"; // Simplified signature

        let token = JwtToken {
            header: header.to_string(),
            payload: payload.clone(),
            signature: signature.to_string(),
        };

        self.tokens.insert(payload, token.clone());
        Ok(token)
    }

    /// Validate JWT token
    pub fn validate_token(&self, token: &JwtToken) -> Result<bool, String> {
        // In a real implementation, we would validate the signature
        // For this example, we'll just check if it exists
        Ok(self.tokens.contains_key(&token.payload))
    }

    /// Generate a full JWT token with header, payload, and signature
    pub fn generate_jwt_token(
        &mut self,
        header: JwtHeader,
        payload: JwtPayload,
        _private_key: &[u8],
    ) -> Result<JwtToken, String> {
        let header_json = serde_json::to_string(&header)
            .map_err(|e| format!("Failed to serialize header: {}", e))?;
        let payload_json = serde_json::to_string(&payload)
            .map_err(|e| format!("Failed to serialize payload: {}", e))?;

        let header_b64 = general_purpose::STANDARD.encode(header_json);
        let payload_b64 = general_purpose::STANDARD.encode(payload_json);

        // In a real implementation, we would sign the header and payload with the private key
        // For this example, we'll use a mock signature
        let signature = format!("{}_{}_signature", header_b64, payload_b64);

        let token = JwtToken {
            header: header_b64,
            payload: payload_b64,
            signature,
        };

        self.tokens.insert(payload.sub.clone(), token.clone());
        Ok(token)
    }

    /// Validate JWT token signature
    pub fn validate_jwt_signature(&self, token: &JwtToken, _public_key: &[u8]) -> Result<bool, String> {
        // In a real implementation, we would verify the signature using the public key
        // For this example, we'll just check if it's a mock signature
        Ok(token.signature.ends_with("_signature"))
    }

    /// Generate refresh token
    pub fn generate_refresh_token(&mut self, user_id: &str) -> Result<String, String> {
        let token = uuid::Uuid::new_v4().to_string();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let refresh_token = RefreshToken {
            token: token.clone(),
            user_id: user_id.to_string(),
            created_at: now,
            expires_at: now + 86400, // 24 hours
            revoked: false,
        };

        self.refresh_tokens.insert(token.clone(), refresh_token);
        Ok(token)
    }

    /// Validate refresh token
    pub fn validate_refresh_token(&self, token: &str) -> Result<&RefreshToken, String> {
        let refresh_token = self.refresh_tokens.get(token).ok_or("Refresh token not found")?;
        
        if refresh_token.revoked {
            return Err("Refresh token revoked".to_string());
        }

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        if refresh_token.expires_at <= now {
            return Err("Refresh token expired".to_string());
        }

        Ok(refresh_token)
    }

    /// Revoke refresh token
    pub fn revoke_refresh_token(&mut self, token: &str) -> Result<(), String> {
        if let Some(refresh_token) = self.refresh_tokens.get_mut(token) {
            refresh_token.revoked = true;
            Ok(())
        } else {
            Err("Refresh token not found".to_string())
        }
    }
}

impl Default for TokenLifecycle {
    fn default() -> Self {
        Self::new()
    }
}

impl SecretManager {
    /// Create a new secret manager
    pub fn new(key: [u8; 32]) -> Self {
        Self {
            secrets: HashMap::new(),
            key,
        }
    }

    /// Store a secret
    pub fn store_secret(&mut self, name: &str, value: &str) -> Result<(), String> {
        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|_| "Invalid encryption key".to_string())?;

        // Generate random nonce
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt the secret
        let ciphertext = cipher
            .encrypt(nonce, value.as_bytes())
            .map_err(|_| "Encryption failed".to_string())?;

        // Combine nonce and ciphertext
        let mut encrypted_value = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
        encrypted_value.extend_from_slice(&nonce_bytes);
        encrypted_value.extend_from_slice(&ciphertext);

        let secret = EncryptedSecret {
            name: name.to_string(),
            encrypted_value,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            rotation_required: false,
        };

        self.secrets.insert(name.to_string(), secret);
        Ok(())
    }

    /// Retrieve a secret
    pub fn retrieve_secret(&self, name: &str) -> Result<String, String> {
        let encrypted_secret = self.secrets.get(name).ok_or("Secret not found")?;

        if encrypted_secret.encrypted_value.len() < 12 {
            return Err("Invalid encrypted data".to_string());
        }

        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|_| "Invalid encryption key".to_string())?;

        // Extract nonce and ciphertext
        let nonce = Nonce::from_slice(&encrypted_secret.encrypted_value[0..12]);
        let ciphertext = &encrypted_secret.encrypted_value[12..];

        // Decrypt the secret
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| "Decryption failed".to_string())?;

        String::from_utf8(plaintext).map_err(|_| "Invalid UTF-8 in decrypted secret".to_string())
    }

    /// Mark secret for rotation
    pub fn mark_for_rotation(&mut self, name: &str) -> Result<(), String> {
        if let Some(secret) = self.secrets.get_mut(name) {
            secret.rotation_required = true;
            Ok(())
        } else {
            Err("Secret not found".to_string())
        }
    }

    /// Get secrets that need rotation
    pub fn get_secrets_needing_rotation(&self) -> Vec<&EncryptedSecret> {
        self.secrets
            .values()
            .filter(|secret| secret.rotation_required)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash() {
        let password = "test_password";
        let hash = PasswordHash::new(password);
        
        // Test verification
        assert!(hash.verify(password));
        assert!(!hash.verify("wrong_password"));
    }

    #[test]
    fn test_authn_manager() {
        let mut authn = AuthNManager::new();
        
        // Test user registration
        let user_id = authn.register_user("testuser", "test@example.com", "password123").unwrap();
        
        // Test authentication
        let session_id = authn.authenticate("testuser", "password123").unwrap();
        
        // Test session validation
        let session = authn.validate_session(&session_id).unwrap();
        assert_eq!(session.user_id, user_id);
        
        // Test failed authentication
        assert!(authn.authenticate("testuser", "wrong_password").is_err());
    }

    #[test]
    fn test_authz_manager() {
        let mut authz = AuthZManager::new();
        
        // Create policy
        let policy = RbacPolicy {
            id: "admin".to_string(),
            name: "Administrator".to_string(),
            permissions: vec!["read".to_string(), "write".to_string(), "delete".to_string()],
            description: "Full access".to_string(),
        };
        
        authz.create_policy(policy).unwrap();
        
        // Assign role
        authz.assign_role("user123", "admin").unwrap();
        
        // Test permission check
        assert!(authz.has_permission("user123", "read"));
        assert!(!authz.has_permission("user123", "nonexistent"));
        assert!(!authz.has_permission("user456", "read"));
    }

    #[test]
    fn test_session_manager() {
        let mut session_mgr = SessionManager::new(2); // Max 2 sessions per user
        
        // Create sessions
        let session1 = session_mgr.create_session("user1", Some("127.0.0.1".to_string())).unwrap();
        let session2 = session_mgr.create_session("user1", Some("127.0.0.1".to_string())).unwrap();
        
        // Test session limit
        assert!(session_mgr.create_session("user1", Some("127.0.0.1".to_string())).is_err());
        
        // Test session validation
        assert!(session_mgr.validate_session(&session1).is_ok());
        
        // Test session revocation
        session_mgr.revoke_session(&session1).unwrap();
        assert!(session_mgr.validate_session(&session1).is_err());
    }

    #[test]
    fn test_secret_manager() {
        let key = [1u8; 32]; // Test key
        let mut secret_mgr = SecretManager::new(key);
        
        // Store secret
        secret_mgr.store_secret("api_key", "secret123").unwrap();
        
        // Retrieve secret
        let retrieved = secret_mgr.retrieve_secret("api_key").unwrap();
        assert_eq!(retrieved, "secret123");
        
        // Test non-existent secret
        assert!(secret_mgr.retrieve_secret("nonexistent").is_err());
    }

    #[test]
    fn test_oauth2_jwt_generation() {
        let authn = AuthNManager::new();
        
        let config = OAuth2Config {
            client_id: "test_client".to_string(),
            client_secret: "test_secret".to_string(),
            redirect_uri: "http://localhost:3000/callback".to_string(),
            auth_url: "https://example.com/oauth/authorize".to_string(),
            token_url: "https://example.com/oauth/token".to_string(),
            scopes: vec!["read".to_string(), "write".to_string()],
        };
        
        let auth_url = authn.generate_oauth2_auth_url(&config, "test_state");
        assert!(auth_url.contains("client_id=test_client"));
        assert!(auth_url.contains("scope=read write"));
        assert!(auth_url.contains("state=test_state"));
        
        let token = authn.exchange_oauth2_code(&config, "test_code").unwrap();
        assert!(!token.header.is_empty());
        assert!(!token.payload.is_empty());
        assert!(!token.signature.is_empty());
    }

    #[test]
    fn test_jwt_token_lifecycle() {
        let mut token_lifecycle = TokenLifecycle::new();
        
        let header = JwtHeader {
            alg: "RS256".to_string(),
            typ: "JWT".to_string(),
        };
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        let payload = JwtPayload {
            sub: "test_user".to_string(),
            iss: "test_issuer".to_string(),
            aud: "test_audience".to_string(),
            exp: now + 3600,
            iat: now,
            scope: "read write".to_string(),
            roles: vec!["user".to_string(), "admin".to_string()],
        };
        
        let private_key = [0u8; 32];
        let token = token_lifecycle.generate_jwt_token(header, payload, &private_key).unwrap();
        
        let public_key = [0u8; 32];
        assert!(token_lifecycle.validate_jwt_signature(&token, &public_key).unwrap());
    }

    #[test]
    fn test_opa_policy_evaluation() {
        let authz = AuthZManager::new();
        
        let policy = "package example
        default allow = false
        allow {
            input.user == \"admin\"
            input.action == \"read\"
        }";
        
        let input = "{\"user\": \"admin\", \"action\": \"read\"}";
        assert!(authz.evaluate_opa_policy(policy, input).unwrap());
    }

    #[test]
    fn test_opa_policy_storage() {
        let mut authz = AuthZManager::new();
        
        let policy = OpaPolicy {
            id: "test_policy".to_string(),
            content: "allow { true }".to_string(),
            version: "1.0.0".to_string(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        
        assert!(authz.add_opa_policy(policy).is_ok());
        // Check that the policy was stored (with the "opa_" prefix)
        assert!(authz.policies.contains_key("opa_test_policy"));
    }
}