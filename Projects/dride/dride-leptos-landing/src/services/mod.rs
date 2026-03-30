mod presale_service;

pub use presale_service::*;

// Re-export for backward compatibility with existing service module
#[allow(dead_code)]
pub mod presale_service {
    use thiserror::Error;

    #[derive(Debug, Clone, Error)]
    pub enum PresaleError {
        #[error("Presale not active")]
        NotActive,

        #[error("Presale is paused")]
        Paused,

        #[error("Invalid wallet address")]
        InvalidWallet(String),

        #[error("Invalid amount")]
        InvalidAmount(String),

        #[error("Invalid input")]
        InvalidInput(String),

        #[error("Transaction failed")]
        TransactionFailed(String),

        #[error("Rate limit exceeded")]
        RateLimitExceeded(String),

        #[error("Network error")]
        NetworkError(String),

        #[error("Presale ended")]
        PresaleEnded,
    }

    #[derive(Debug, Clone)]
    pub struct PurchaseResult {
        pub dride_amount: f64,
        pub transaction_hash: String,
        pub timestamp: chrono::DateTime<chrono::Utc>,
    }
}
