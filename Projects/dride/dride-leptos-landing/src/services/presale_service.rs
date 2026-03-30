use chrono::{Utc, DateTime};
use solana_sdk::{
    pubkey::Pubkey,
    signature::Signature,
    transaction::Transaction,
    commitment_config::CommitmentConfig,
};
use crate::security::input_validation::{InputValidator, PresalePurchaseRequest};
use crate::security::monitoring::{SecurityMonitor, SecurityEvent, SecurityEventType, EventSeverity, EventOutcome};
use crate::security::rate_limiter::{RateLimiter, RateLimitError};
use crate::services::presale_service::{PresaleError, PurchaseResult};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct PresaleService {
    rpc_url: String,
    presale_wallet: Pubkey,
    token_mint: Pubkey,
    program_id: Pubkey,
    rate_limiter: RateLimiter,
    security_monitor: SecurityMonitor,
}

impl PresaleService {
    pub fn new() -> Self {
        let rpc_url = std::env::var("SOLANA_RPC_URL")
            .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());

        let presale_wallet = std::env::var("PRESALE_WALLET")
            .unwrap_or_else(|_| "YourPresaleWalletPubkey".to_string())
            .parse()
            .expect("Invalid presale wallet address");

        let token_mint = std::env::var("TOKEN_MINT")
            .unwrap_or_else(|_| "DRideToken11111111111111111111111111".to_string())
            .parse()
            .expect("Invalid token mint address");

        let program_id = std::env::var("PROGRAM_ID")
            .unwrap_or_else(|_| "DRidePresale11111111111111111111111111".to_string())
            .parse()
            .expect("Invalid program ID");

        Self {
            rpc_url,
            presale_wallet,
            token_mint,
            program_id,
            rate_limiter: RateLimiter::new(),
            security_monitor: SecurityMonitor,
        }
    }

    /// Get presale information
    pub async fn get_presale_info(&self) -> Result<PresaleInfo, PresaleError> {
        // In production, this would query the blockchain or backend
        // For now, return mock data
        Ok(PresaleInfo {
            total_raised: 680_000.0,
            total_tokens: 136_000_000.0,
            hard_cap: 1_000_000.0,
            progress_percent: 68.0,
            current_price: 0.005,
            tokens_per_sol: 200.0,
            end_date: Utc::now() + chrono::Duration::days(90),
            is_active: true,
            is_paused: false,
        })
    }

    /// Purchase $DRIDE tokens during presale
    pub async fn purchase_tokens(
        &self,
        wallet_address: String,
        sol_amount: f64,
    ) -> Result<PurchaseResult, PresaleError> {
        // Validate input
        let request = PresalePurchaseRequest {
            wallet_address: wallet_address.clone(),
            sol_amount,
            email: None,
        };

        if let Err(e) = request.validate() {
            SecurityMonitor::log_invalid_input(
                Some(&wallet_address),
                "purchase_request",
                &format!("{:?}", request),
                &e.to_string(),
            );
            return Err(PresaleError::InvalidInput(e.to_string()));
        }

        // Additional wallet validation
        if let Err(e) = InputValidator::validate_wallet_address(&wallet_address) {
            SecurityMonitor::log_invalid_input(
                Some(&wallet_address),
                "wallet_address",
                &wallet_address,
                &e.to_string(),
            );
            return Err(PresaleError::InvalidWallet(e.to_string()));
        }

        // Additional amount validation
        if let Err(e) = InputValidator::validate_sol_amount(sol_amount) {
            SecurityMonitor::log_invalid_input(
                Some(&wallet_address),
                "sol_amount",
                &sol_amount.to_string(),
                &e.to_string(),
            );
            return Err(PresaleError::InvalidAmount(e.to_string()));
        }

        // Check rate limits
        if let Err(e) = self.rate_limiter.check_purchase_rate(&wallet_address) {
            SecurityMonitor::log_rate_limit(&wallet_address, "purchase");
            return Err(PresaleError::RateLimitExceeded(e.to_string()));
        }

        // Calculate tokens to receive
        let dride_amount = sol_amount * 200.0; // 1 SOL = 200 DRIDE

        // Log purchase attempt
        SecurityMonitor::log_presale_attempt(&wallet_address, sol_amount, false, None);

        // Create Solana transaction
        let transaction = match self.build_purchase_transaction(&wallet_address, sol_amount) {
            Ok(tx) => tx,
            Err(e) => {
                SecurityMonitor::log_presale_attempt(&wallet_address, sol_amount, false, Some(e.to_string()));
                return Err(e);
            }
        };

        // In production, send transaction to wallet for signing
        // For now, simulate successful transaction
        let signature = Signature::new_unique();

        // Calculate transaction hash
        let transaction_hash = bs58::encode(signature.as_ref());

        // Simulate sending to blockchain
        // In production, use solana_client::send_transaction()
        let lamports_amount = (sol_amount * 1_000_000_000.0) as u64;

        // Log successful purchase
        SecurityMonitor::log_presale_attempt(&wallet_address, sol_amount, true, None);

        Ok(PurchaseResult {
            dride_amount,
            transaction_hash,
            timestamp: Utc::now(),
        })
    }

    /// Build purchase transaction
    fn build_purchase_transaction(
        &self,
        wallet_address: &str,
        sol_amount: f64,
    ) -> Result<Transaction, PresaleError> {
        let user_wallet = wallet_address
            .parse()
            .map_err(|_| PresaleError::InvalidWallet("Invalid wallet address".to_string()))?;

        let lamports_amount = (sol_amount * 1_000_000_000.0) as u64;

        // Create transfer instruction
        let transfer_instruction = solana_sdk::system_instruction::transfer(
            &user_wallet,
            &self.presale_wallet,
            lamports_amount,
        );

        // Create transaction
        let mut transaction = Transaction::new_with_payer(
            &[transfer_instruction],
            Some(&user_wallet),
        );

        Ok(transaction)
    }

    /// Verify presale is active
    pub async fn verify_presale_active(&self) -> Result<bool, PresaleError> {
        let presale_info = self.get_presale_info().await?;

        if presale_info.is_paused {
            return Err(PresaleError::PresalePaused);
        }

        if !presale_info.is_active {
            return Err(PresaleError::PresaleInactive);
        }

        Ok(true)
    }

    /// Get presale settings
    pub fn get_presale_settings(&self) -> PresaleSettings {
        PresaleSettings {
            price: 0.005,
            next_price: 0.008,
            tokens_per_sol: 200,
            min_purchase: 0.1,
            max_purchase: 1000.0,
            hard_cap: 1_000_000.0,
            max_per_wallet: 1000.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PresaleInfo {
    pub total_raised: f64,
    pub total_tokens: f64,
    pub hard_cap: f64,
    pub progress_percent: f64,
    pub current_price: f64,
    pub tokens_per_sol: f64,
    pub end_date: DateTime<Utc>,
    pub is_active: bool,
    pub is_paused: bool,
}

#[derive(Debug, Clone)]
pub struct PresaleSettings {
    pub price: f64,
    pub next_price: f64,
    pub tokens_per_sol: f64,
    pub min_purchase: f64,
    pub max_purchase: f64,
    pub hard_cap: f64,
    pub max_per_wallet: f64,
}
