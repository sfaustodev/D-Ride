use regex::Regex;
use validator::Validate;
use once_cell::sync::Lazy;
use thiserror::Error;

lazy_static! {
    static ref WALLET_ADDRESS_REGEX: Regex =
        Regex::new(r"^[1-9A-HJ-NP-Za-km-z]{32,44}$").unwrap();
    static ref SOL_AMOUNT_REGEX: Regex =
        Regex::new(r"^\d+(\.\d{1,9})?$").unwrap();
    static ref EMAIL_REGEX: Regex =
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    static ref BASE58_REGEX: Regex =
        Regex::new(r"^[1-9A-HJ-NP-Za-km-z]+$").unwrap();
}

#[derive(Debug, Clone, Error)]
pub enum ValidationError {
    #[error("Invalid wallet address format")]
    InvalidWalletAddress,

    #[error("Invalid SOL amount")]
    InvalidAmount,

    #[error("Invalid email format")]
    InvalidEmail,

    #[error("Invalid number format")]
    InvalidNumber,

    #[error("String contains malicious content")]
    MaliciousContent,
}

#[derive(Debug, Clone, Validate)]
pub struct PresalePurchaseRequest {
    #[validate(length(min = 1, max = 100))]
    pub wallet_address: String,

    #[validate(range(min = 0.1, max = 1000.0))]
    pub sol_amount: f64,

    #[validate(email)]
    pub email: Option<String>,
}

pub struct InputValidator;

impl InputValidator {
    /// Validate Solana wallet address
    pub fn validate_wallet_address(address: &str) -> Result<(), ValidationError> {
        // Check length
        if address.len() < 32 || address.len() > 44 {
            return Err(ValidationError::InvalidWalletAddress);
        }

        // Check format with regex
        if !WALLET_ADDRESS_REGEX.is_match(address) {
            return Err(ValidationError::InvalidWalletAddress);
        }

        // Check for malicious patterns
        Self::check_for_malicious_content(address)?;

        Ok(())
    }

    /// Validate SOL amount
    pub fn validate_sol_amount(amount: f64) -> Result<(), ValidationError> {
        // Check range
        if amount < 0.1 || amount > 1000.0 {
            return Err(ValidationError::InvalidAmount);
        }

        // Check for finite number
        if !amount.is_finite() {
            return Err(ValidationError::InvalidAmount);
        }

        // Check for reasonable precision (max 9 decimals for Solana)
        let decimal_places = amount.to_string()
            .split('.')
            .nth(1)
            .map(|s| s.len())
            .unwrap_or(0);

        if decimal_places > 9 {
            return Err(ValidationError::InvalidAmount);
        }

        // Check string representation
        let amount_str = amount.to_string();
        if !SOL_AMOUNT_REGEX.is_match(&amount_str) {
            return Err(ValidationError::InvalidAmount);
        }

        Ok(())
    }

    /// Validate email
    pub fn validate_email(email: &str) -> Result<(), ValidationError> {
        if !EMAIL_REGEX.is_match(email) {
            return Err(ValidationError::InvalidEmail);
        }
        Self::check_for_malicious_content(email)?;
        Ok(())
    }

    /// Sanitize user input (XSS prevention)
    pub fn sanitize_html(input: &str) -> String {
        input
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
            .replace('/', "&#x2F;")
            .replace("\x00", "")
            .replace("\x0D", "")
            .replace("\x1A", "")
    }

    /// Validate numeric input
    pub fn validate_numeric(input: &str) -> Result<f64, ValidationError> {
        let cleaned = input.trim().replace(',', '.');

        cleaned.parse::<f64>()
            .map_err(|_| ValidationError::InvalidNumber)
            .and_then(|n| {
                if n.is_finite() {
                    Ok(n)
                } else {
                    Err(ValidationError::InvalidNumber)
                }
            })
    }

    /// Check for malicious content in strings
    pub fn check_for_malicious_content(input: &str) -> Result<(), ValidationError> {
        let lower = input.to_lowercase();

        // Check for common attack patterns
        let malicious_patterns = [
            "<script",
            "javascript:",
            "onerror=",
            "onload=",
            "onmouseover=",
            "onclick=",
            "eval(",
            "alert(",
            "document.cookie",
            "..\\",
            "../",
            "%3cscript",
            "data:text/html",
            "vbscript:",
            "@import",
            "expression(",
        ];

        for pattern in &malicious_patterns {
            if lower.contains(pattern) {
                tracing::warn!("Malicious content detected in input: {}", input);
                return Err(ValidationError::MaliciousContent);
            }
        }

        Ok(())
    }

    /// Validate URL to prevent SSRF
    pub fn validate_url(url: &str) -> Result<(), ValidationError> {
        let lower = url.to_lowercase();

        // Block private IPs
        let private_ip_patterns = [
            "localhost",
            "127.0.0.1",
            "192.168.",
            "10.",
            "172.16.",
            "172.17.",
            "172.18.",
            "172.19.",
            "172.20.",
            "172.21.",
            "172.22.",
            "172.23.",
            "172.24.",
            "172.25.",
            "172.26.",
            "172.27.",
            "172.28.",
            "172.29.",
            "172.30.",
            "172.31.",
            "0.0.0.0",
            "::1",
        ];

        for pattern in &private_ip_patterns {
            if lower.contains(pattern) {
                return Err(ValidationError::MaliciousContent);
            }
        }

        // Block non-HTTP/HTTPS protocols
        if !lower.starts_with("http://") && !lower.starts_with("https://") {
            return Err(ValidationError::MaliciousContent);
        }

        // Block file:// and other dangerous protocols
        if lower.contains("file://") ||
           lower.contains("ftp://") ||
           lower.contains("data://") ||
           lower.contains("javascript:") {
            return Err(ValidationError::MaliciousContent);
        }

        Ok(())
    }

    /// Validate transaction signature
    pub fn validate_transaction_signature(signature: &str) -> Result<(), ValidationError> {
        // Base58 check
        if !BASE58_REGEX.is_match(signature) {
            return Err(ValidationError::InvalidWalletAddress);
        }

        // Length check (Solana signatures are 64 bytes, ~88 chars in base58)
        if signature.len() < 80 || signature.len() > 95 {
            return Err(ValidationError::InvalidWalletAddress);
        }

        Ok(())
    }

    /// Validate UUID
    pub fn validate_uuid(uuid: &str) -> Result<(), ValidationError> {
        let uuid_regex = Regex::new(
            r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$"
        ).unwrap();

        if !uuid_regex.is_match(uuid) {
            return Err(ValidationError::InvalidNumber);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validates_wallet_address() {
        // Valid addresses
        assert!(InputValidator::validate_wallet_address("Fj4k...x9Rm").is_ok());

        // Invalid addresses
        assert!(InputValidator::validate_wallet_address("invalid").is_err());
        assert!(InputValidator::validate_wallet_address("<script>alert(1)</script>").is_err());
    }

    #[test]
    fn test_validates_sol_amount() {
        assert!(InputValidator::validate_sol_amount(10.0).is_ok());
        assert!(InputValidator::validate_sol_amount(0.1).is_ok());
        assert!(InputValidator::validate_sol_amount(0.0).is_err());
        assert!(InputValidator::validate_sol_amount(1001.0).is_err());
        assert!(InputValidator::validate_sol_amount(f64::NAN).is_err());
        assert!(InputValidator::validate_sol_amount(f64::INFINITY).is_err());
    }

    #[test]
    fn test_sanitizes_html() {
        let input = "<script>alert('xss')</script>";
        let sanitized = InputValidator::sanitize_html(input);

        assert!(!sanitized.contains("<script>"));
        assert!(sanitized.contains("&lt;script&gt;"));
    }

    #[test]
    fn test_detects_malicious_content() {
        let malicious = "<script>alert(1)</script>";
        assert!(InputValidator::check_for_malicious_content(malicious).is_err());

        let sql_injection = "'; DROP TABLE users; --";
        assert!(InputValidator::check_for_malicious_content(sql_injection).is_ok()); // SQL not detected as malicious in general string
    }

    #[test]
    fn test_validates_url() {
        assert!(InputValidator::validate_url("https://api.solana.com").is_ok());
        assert!(InputValidator::validate_url("https://api.devnet.solana.com").is_ok());
        assert!(InputValidator::validate_url("http://localhost:8080").is_err());
        assert!(InputValidator::validate_url("file:///etc/passwd").is_err());
    }

    #[test]
    fn test_validates_transaction_signature() {
        let valid_sig = "5K...verylongsignature...signature";
        assert!(InputValidator::validate_transaction_signature(valid_sig).is_ok());

        assert!(InputValidator::validate_transaction_signature("invalid").is_err());
    }

    #[test]
    fn test_validates_uuid() {
        assert!(InputValidator::validate_uuid("550e8400-e29b-41d4-a716-446655440000").is_ok());
        assert!(InputValidator::validate_uuid("invalid").is_err());
    }
}
