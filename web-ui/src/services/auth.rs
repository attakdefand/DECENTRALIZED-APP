//! Authentication service
//!
//! This module provides authentication functionality for the web UI.

use serde::{Deserialize, Serialize};
use gloo_storage::{LocalStorage, Storage};
use jwt::{SignWithKey, VerifyWithKey};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use wasm_bindgen::JsValue;

/// Authentication token structure
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AuthToken {
    pub user_id: String,
    pub username: String,
    pub exp: u64,
}

/// Authentication service
pub struct AuthService {
    secret_key: Hmac<Sha256>,
}

impl AuthService {
    /// Create a new authentication service
    pub fn new(secret: &str) -> Self {
        let secret_key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())
            .expect("HMAC can take key of any size");
        Self { secret_key }
    }

    /// Generate a JWT token for a user
    pub fn generate_token(&self, user_id: &str, username: &str) -> Result<String, anyhow::Error> {
        let exp = js_sys::Date::now() as u64 + (3600 * 1000); // 1 hour expiration
        let claims = AuthToken {
            user_id: user_id.to_string(),
            username: username.to_string(),
            exp,
        };
        
        let token_str = claims.sign_with_key(&self.secret_key)?;
        Ok(token_str)
    }

    /// Verify a JWT token
    pub fn verify_token(&self, token: &str) -> Result<AuthToken, anyhow::Error> {
        let claims: AuthToken = token.verify_with_key(&self.secret_key)?;
        
        // Check if token is expired
        if claims.exp < js_sys::Date::now() as u64 {
            return Err(anyhow::Error::msg("Token expired"));
        }
        
        Ok(claims)
    }

    /// Store token in local storage
    pub fn store_token(token: &str) -> Result<(), gloo_storage::errors::StorageError> {
        LocalStorage::set("auth_token", token)
    }

    /// Retrieve token from local storage
    pub fn get_stored_token() -> Option<String> {
        LocalStorage::get("auth_token").ok()
    }

    /// Clear stored token
    pub fn clear_token() {
        LocalStorage::delete("auth_token")
    }

    /// Add authentication header to requests
    pub fn add_auth_header(&self, headers: &mut web_sys::Headers, token: &str) -> Result<(), JsValue> {
        headers.set("Authorization", &format!("Bearer {}", token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_auth_service_creation() {
        let auth = AuthService::new("test_secret");
        assert!(true); // Just test that it can be created
    }

    #[wasm_bindgen_test]
    fn test_token_generation_and_verification() {
        let auth = AuthService::new("test_secret");
        let token = auth.generate_token("user123", "testuser");
        assert!(token.is_ok());
        
        let verified = auth.verify_token(&token.unwrap());
        assert!(verified.is_ok());
    }
}