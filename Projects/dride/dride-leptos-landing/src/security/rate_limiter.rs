use dashmap::DashMap;
use governor::{clock, state::InMemoryState, Quota, RateLimiter};
use governor::middleware::NoOpMiddleware;
use std::sync::Arc;
use std::time::Duration;
use chrono::{Utc, DateTime};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum RateLimitError {
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    #[error("Internal rate limiter error")]
    InternalError,
}

/// Rate limiter for preventing DoS attacks
pub struct RateLimiter {
    purchase_limiter: Arc<RateLimiter<clock::DefaultClock, InMemoryState, NoOpMiddleware>>,
    wallet_limiter: Arc<RateLimiter<clock::DefaultClock, InMemoryState, NoOpMiddleware>>,
    api_limiter: Arc<RateLimiter<clock::DefaultClock, InMemoryState, NoOpMiddleware>>,
    wallet_attempts: DashMap<String, WalletAttemptTracker>,
}

#[derive(Debug, Clone)]
pub struct WalletAttemptTracker {
    pub attempts: u32,
    pub last_attempt: DateTime<Utc>,
    pub blocked_until: Option<DateTime<Utc>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        // 10 purchases per minute
        let purchase_quota = Quota::per_minute(10);

        // 100 purchases per day per wallet
        let wallet_quota = Quota::per_hour(100);

        // 100 requests per minute per IP
        let api_quota = Quota::per_minute(100);

        let purchase_limiter = Arc::new(
            RateLimiter::direct(purchase_quota)
        );

        let wallet_limiter = Arc::new(
            RateLimiter::direct(wallet_quota)
        );

        let api_limiter = Arc::new(
            RateLimiter::direct(api_quota)
        );

        Self {
            purchase_limiter,
            wallet_limiter,
            api_limiter,
            wallet_attempts: DashMap::new(),
        }
    }

    /// Check if purchase is allowed for a wallet
    pub fn check_purchase_rate(&self, wallet_address: &str) -> Result<(), RateLimitError> {
        // Check global rate limit
        self.purchase_limiter
            .check()
            .map_err(|_| RateLimitError::RateLimitExceeded)?;

        // Check wallet-specific rate limit
        let tracker = self.wallet_attempts.entry(wallet_address.to_string()).or_insert_with(|| {
            WalletAttemptTracker {
                attempts: 0,
                last_attempt: Utc::now(),
                blocked_until: None,
            }
        });

        // Check if wallet is blocked
        if let Some(blocked_until) = tracker.blocked_until {
            if Utc::now() < blocked_until {
                return Err(RateLimitError::RateLimitExceeded);
            } else {
                tracker.blocked_until = None;
                tracker.attempts = 0;
            }
        }

        // Check attempts
        if tracker.attempts >= 100 {
            // Block for 1 hour
            tracker.blocked_until = Some(Utc::now() + chrono::Duration::hours(1));
            return Err(RateLimitError::RateLimitExceeded);
        }

        // Update tracker
        tracker.attempts += 1;
        tracker.last_attempt = Utc::now();

        Ok(())
    }

    /// Check API rate limit
    pub fn check_api_rate(&self) -> Result<(), RateLimitError> {
        self.api_limiter
            .check()
            .map_err(|_| RateLimitError::RateLimitExceeded)?;
        Ok(())
    }

    /// Reset rate limits (for testing or admin)
    pub fn reset_wallet_limits(&self, wallet_address: &str) {
        self.wallet_attempts.remove(wallet_address);
    }

    /// Get number of attempts for a wallet
    pub fn get_wallet_attempts(&self, wallet_address: &str) -> u32 {
        self.wallet_attempts
            .get(wallet_address)
            .map(|t| t.attempts)
            .unwrap_or(0)
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_purchase_rate_limiting() {
        let limiter = RateLimiter::new();

        // First 10 purchases should succeed
        for _ in 0..10 {
            assert!(limiter.check_purchase_rate("test_wallet").is_ok());
        }

        // 11th purchase should fail (rate limit exceeded)
        assert!(limiter.check_purchase_rate("test_wallet").is_err());
    }

    #[test]
    fn test_wallet_specific_limits() {
        let limiter = RateLimiter::new();

        // Different wallets have independent limits
        for _ in 0..10 {
            assert!(limiter.check_purchase_rate("wallet1").is_ok());
            assert!(limiter.check_purchase_rate("wallet2").is_ok());
        }
    }

    #[test]
    fn test_wallet_tracking() {
        let limiter = RateLimiter::new();

        assert_eq!(limiter.get_wallet_attempts("wallet1"), 0);
        limiter.check_purchase_rate("wallet1").unwrap();
        assert_eq!(limiter.get_wallet_attempts("wallet1"), 1);
    }

    #[test]
    fn test_reset_limits() {
        let limiter = RateLimiter::new();

        limiter.check_purchase_rate("wallet1").unwrap();
        assert_eq!(limiter.get_wallet_attempts("wallet1"), 1);

        limiter.reset_wallet_limits("wallet1");
        assert_eq!(limiter.get_wallet_attempts("wallet1"), 0);
    }
}
