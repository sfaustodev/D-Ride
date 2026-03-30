use ed25519_dalek::{Keypair, Signature, Signer, Verifier};
use sha2::{Sha256, Digest};
use hmac::{Hmac, Mac};
use rand::rngs::OsRng;
use base64::{Engine as _, engine::general_purpose};
use thiserror::Error;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone, Error)]
pub enum CryptoError {
    #[error("Invalid key length")]
    InvalidKeyLength,
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Hashing error")]
    HashingError,
}

/// Cryptographic operations for security
pub struct CryptoService {
    presale_authority: Option<Keypair>,
}

impl CryptoService {
    pub fn new() -> Self {
        Self {
            presale_authority: None,
        }
    }

    /// Initialize with presale authority keypair
    pub fn with_authority(keypair: Keypair) -> Self {
        Self {
            presale_authority: Some(keypair),
        }
    }

    /// Sign data with presale authority
    pub fn sign_transaction(&self, data: &[u8]) -> Option<Signature> {
        self.presale_authority.as_ref().map(|key| key.sign(data))
    }

    /// Verify transaction signature
    pub fn verify_signature(
        &self,
        data: &[u8],
        signature: &Signature,
        public_key: &ed25519_dalek::PublicKey,
    ) -> Result<(), CryptoError> {
        public_key
            .verify(data, signature)
            .map_err(|_| CryptoError::InvalidSignature)
    }

    /// Hash transaction data
    pub fn hash_transaction(data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    /// Generate secure random token
    pub fn generate_token(&self) -> String {
        let token: [u8; 32] = rand::random();
        general_purpose::URL_SAFE_NO_PAD.encode(token)
    }

    /// HMAC for API requests
    pub fn hmac_sign(&self, key: &[u8], data: &[u8]) -> Result<String, CryptoError> {
        let mut mac = HmacSha256::new_from_slice(key)
            .map_err(|_| CryptoError::InvalidKeyLength)?;
        mac.update(data);
        Ok(hex::encode(mac.finalize().into_bytes()))
    }

    /// Verify HMAC
    pub fn hmac_verify(&self, key: &[u8], data: &[u8], signature: &str) -> bool {
        match Self::hmac_sign(self, key, data) {
            Ok(expected_sig) => expected_sig == signature,
            Err(_) => false,
        }
    }

    /// Generate keypair
    pub fn generate_keypair() -> Keypair {
        Keypair::generate(&mut OsRng)
    }

    /// Get public key from keypair
    pub fn public_key_from_keypair(keypair: &Keypair) -> String {
        keypair.public.to_bytes().to_vec()
    }

    /// Derive PDA (Program Derived Address)
    pub fn derive_pda(
        seeds: &[&[u8]],
        program_id: &solana_sdk::pubkey::Pubkey,
    ) -> (solana_sdk::pubkey::Pubkey, u8) {
        solana_sdk::pubkey::Pubkey::find_program_address(seeds, program_id)
    }
}

impl Default for CryptoService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_verify_signature() {
        let keypair = CryptoService::generate_keypair();
        let data = b"test data";
        let signature = keypair.sign(data);

        let verification = CryptoService::verify_signature(
            &CryptoService::new(),
            data,
            &signature,
            &keypair.public,
        );

        assert!(verification.is_ok());
    }

    #[test]
    fn test_hash_transaction() {
        let data = b"test transaction";
        let hash = CryptoService::hash_transaction(data);

        assert_eq!(hash.len(), 32); // SHA256 produces 32 bytes
    }

    #[test]
    fn test_generate_token() {
        let crypto = CryptoService::new();
        let token1 = crypto.generate_token();
        let token2 = crypto.generate_token();

        // Tokens should be unique
        assert_ne!(token1, token2);

        // Tokens should be valid base64url
        assert!(token1.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_'));
    }

    #[test]
    fn test_hmac_sign_and_verify() {
        let crypto = CryptoService::new();
        let key = b"test_key_32_bytes_long_12345678";
        let data = b"test data";

        let signature = crypto.hmac_sign(key, data).unwrap();
        let verified = crypto.hmac_verify(key, data, &signature);

        assert!(verified);
    }

    #[test]
    fn test_pda_derivation() {
        let program_id = solana_sdk::pubkey::new_unique();
        let seeds = &[b"test", b"seeds"];

        let (pda, bump) = CryptoService::derive_pda(seeds, &program_id);

        // Bump should be between 0 and 255
        assert!(bump <= 255);

        // PDA should be different from the program ID
        assert_ne!(pda, program_id);
    }
}
