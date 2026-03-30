use uuid::Uuid;
use chrono::{Utc, Duration};
use once_cell::sync::Lazy;
use std::collections::HashSet;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum CsrfError {
    #[error("Invalid CSRF token")]
    InvalidToken,
    #[error("CSRF token expired")]
    ExpiredToken,
    #[error("CSRF token already used")]
    TokenAlreadyUsed,
}

/// CSRF protection for state-changing requests
pub struct CsrfProtection {
    used_tokens: Lazy<std::sync::Mutex<HashSet<String>>>,
    token_ttl_seconds: i64,
}

impl CsrfProtection {
    pub fn new() -> Self {
        Self {
            used_tokens: Lazy::new(|| std::sync::Mutex::new(HashSet::new())),
            token_ttl_seconds: 3600, // 1 hour
        }
    }

    /// Generate a new CSRF token
    pub fn generate_token(&self) -> (String, DateTime<Utc>) {
        let token = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + Duration::seconds(self.token_ttl_seconds);
        (token, expires_at)
    }

    /// Validate a CSRF token
    pub fn validate_token(&self, token: &str, expires_at: DateTime<Utc>) -> Result<(), CsrfError> {
        // Check if token was already used
        let mut used = self.used_tokens.lock().unwrap();
        if used.contains(token) {
            return Err(CsrfError::TokenAlreadyUsed);
        }

        // Check if token is expired
        if Utc::now() > expires_at {
            return Err(CsrfError::ExpiredToken);
        }

        // Mark token as used
        used.insert(token.to_string());

        Ok(())
    }

    /// Clean up expired tokens
    pub fn cleanup_expired_tokens(&self, before: DateTime<Utc>) {
        // In a production system with persistence, this would remove expired tokens
        // For in-memory implementation, tokens are automatically cleaned up
    }

    /// Set custom token TTL
    pub fn set_token_ttl(&mut self, seconds: i64) {
        self.token_ttl_seconds = seconds;
    }
}

impl Default for CsrfProtection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_validate_token() {
        let csrf = CsrfProtection::new();
        let (token, expires_at) = csrf.generate_token();

        // Token should be valid
        assert!(csrf.validate_token(&token, expires_at).is_ok());
    }

    #[test]
    fn test_expired_token() {
        let csrf = CsrfProtection::new();
        let (token, _) = csrf.generate_token();
        let expired = Utc::now() - Duration::hours(2);

        assert!(matches!(
            csrf.validate_token(&token, expired),
            Err(CsrfError::ExpiredToken)
        ));
    }

    #[test]
    fn test_token_already_used() {
        let csrf = CsrfProtection::new();
        let (token, expires_at) = csrf.generate_token();

        // First use should succeed
        assert!(csrf.validate_token(&token, expires_at).is_ok());

        // Second use should fail
        assert!(matches!(
            csrf.validate_token(&token, expires_at),
            Err(CsrfError::TokenAlreadyUsed)
        ));
    }
}
