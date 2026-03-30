use dride_leptos_landing::security::input_validation::{InputValidator, ValidationError};

#[test]
fn test_validates_wallet_address() {
    // Valid Base58 addresses
    assert!(InputValidator::validate_wallet_address("Fj4k...x9Rm").is_ok());
    assert!(InputValidator::validate_wallet_address("9WzDX6Btest5vUsAiCSD").is_ok());

    // Invalid addresses
    assert!(InputValidator::validate_wallet_address("invalid").is_err());
    assert!(InputValidator::validate_wallet_address("<script>alert(1)</script>").is_err());
    assert!(InputValidator::validate_wallet_address("12345").is_err());
}

#[test]
fn test_validates_sol_amount() {
    // Valid amounts
    assert!(InputValidator::validate_sol_amount(10.0).is_ok());
    assert!(InputValidator::validate_sol_amount(0.1).is_ok());
    assert!(InputValidator::validate_sol_amount(100.5).is_ok());

    // Invalid amounts
    assert!(InputValidator::validate_sol_amount(0.0).is_err());
    assert!(InputValidator::validate_sol_amount(0.0).is_err());
    assert!(InputValidator::validate_sol_amount(0.000001).is_err());
    assert!(InputValidator::validate_sol_amount(1001.0).is_err());
    assert!(InputValidator::validate_sol_amount(f64::NAN).is_err());
    assert!(InputValidator::validate_sol_amount(f64::INFINITY).is_err());
}

#[test]
fn test_sanitizes_html() {
    let input = "<script>alert('xss')</script>";
    let sanitized = InputValidator::sanitize_html(input);

    assert!(!sanitized.contains("<script>"));
    assert!(!sanitized.contains("</script>"));
    assert!(sanitized.contains("&lt;script&gt;"));
}

#[test]
fn test_validates_email() {
    // Valid emails
    assert!(InputValidator::validate_email("test@example.com").is_ok());
    assert!(InputValidator::validate_email("user@domain.co.uk").is_ok());

    // Invalid emails
    assert!(InputValidator::validate_email("invalid").is_err());
    assert!(InputValidator::validate_email("@example.com").is_err());
    assert!(InputValidator::validate_email("test@").is_err());
}

#[test]
fn test_detects_malicious_content() {
    // XSS attempts
    assert!(InputValidator::check_for_malicious_content("<script>alert(1)</script>").is_err());
    assert!(InputValidator::check_for_malicious_content("javascript:alert(1)").is_err());
    assert!(InputValidator::check_for_malicious_content("onload=\"alert(1)\"").is_err());
    assert!(InputValidator::check_for_malicious_content("document.cookie='xss'").is_err());

    // Safe content
    assert!(InputValidator::check_for_malicious_content("hello world").is_ok());
    assert!(InputValidator::check_for_malicious_content("test string with < and > symbols").is_ok());
}

#[test]
fn test_validates_url() {
    // Valid URLs
    assert!(InputValidator::validate_url("https://api.solana.com").is_ok());
    assert!(InputValidator::validate_url("https://api.devnet.solana.com").is_ok());

    // Invalid URLs
    assert!(InputValidator::validate_url("http://localhost:8080").is_err());
    assert!(InputValidator::validate_url("file:///etc/passwd").is_err());
    assert!(InputValidator::validate_url("127.0.0.1").is_err());
    assert!(InputValidator::validate_url("ftp://example.com").is_err());
}

#[test]
fn test_validates_transaction_signature() {
    // Valid signature (88 chars is base58)
    let valid_sig = "5K...verylongsignature...signature88chars";
    assert!(InputValidator::validate_transaction_signature(valid_sig).is_ok());

    // Invalid signatures
    assert!(InputValidator::validate_transaction_signature("invalid").is_err());
    assert!(InputValidator::validate_transaction_signature("toolshort").is_err());
}

#[test]
fn test_validates_uuid() {
    // Valid UUIDs
    assert!(InputValidator::validate_uuid("550e8400-e29b-41d4-a716-446655440000").is_ok());
    assert!(InputValidator::validate_uuid("6ba7b810-9dad-11d1-80b4-00dab6696d6").is_ok());

    // Invalid UUIDs
    assert!(InputValidator::validate_uuid("not-a-uuid").is_err());
    assert!(InputValidator::validate_uuid("550e8400-e29b").is_err());
}
