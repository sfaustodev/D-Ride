use dride_leptos_landing::security::rate_limiter::{RateLimiter, RateLimitError};

#[test]
fn test_purchase_rate_limiting() {
    let limiter = RateLimiter::new();

    // First 10 purchases should succeed
    for i in 0..10 {
        assert!(limiter.check_purchase_rate(&format!("wallet{}", i)).is_ok());
    }

    // 11th purchase should fail (rate limit exceeded)
    assert!(matches!(
        limiter.check_purchase_rate("test_wallet"),
        Err(RateLimitError::RateLimitExceeded)
    ));
}

#[test]
fn test_wallet_specific_limits() {
    let limiter = RateLimiter::new();

    // Different wallets have independent limits
    for _ in 0..10 {
        assert!(limiter.check_purchase_rate("wallet1").is_ok());
        assert!(limiter.check_purchase_rate("wallet2").is_ok());
    }

    // Each wallet reached their individual limit
    assert!(matches!(
        limiter.check_purchase_rate("wallet1"),
        Err(RateLimitError::RateLimitExceeded)
    ));
    assert!(matches!(
        limiter.check_purchase_rate("wallet2"),
        Err(RateLimitError::RateLimitExceeded)
    ));
}

#[test]
fn test_api_rate_limiting() {
    let limiter = RateLimiter::new();

    // First 100 requests should succeed
    for _ in 0..100 {
        assert!(limiter.check_api_rate().is_ok());
    }

    // 101st request should fail
    assert!(matches!(
        limiter.check_api_rate(),
        Err(RateLimitError::RateLimitExceeded)
    ));
}

#[test]
fn test_wallet_tracking() {
    let limiter = RateLimiter::new();

    assert_eq!(limiter.get_wallet_attempts("wallet1"), 0);

    limiter.check_purchase_rate("wallet1").unwrap();
    assert_eq!(limiter.get_wallet_attempts("wallet1"), 1);

    limiter.check_purchase_rate("wallet1").unwrap();
    assert_eq!(limiter.get_wallet_attempts("wallet1"), 2);
}

#[test]
fn test_reset_limits() {
    let limiter = RateLimiter::new();

    limiter.check_purchase_rate("wallet1").unwrap();
    assert_eq!(limiter.get_wallet_attempts("wallet1"), 1);

    limiter.reset_wallet_limits("wallet1");
    assert_eq!(limiter.get_wallet_attempts("wallet1"), 0);
}
