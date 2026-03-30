use dride_leptos_landing::services::presale_service::{PresaleService, PresaleError, PurchaseResult};

#[tokio::test]
async fn test_get_presale_info() {
    let service = PresaleService::new();

    let info = service.get_presale_info().await.unwrap();

    assert!(info.total_raised > 0.0);
    assert!(info.hard_cap > 0.0);
    assert!(info.progress_percent >= 0.0 && info.progress_percent <= 100.0);
    assert!(info.current_price > 0.0);
    assert!(info.tokens_per_sol > 0.0);
}

#[tokio::test]
async fn test_presale_purchase_valid() {
    let service = PresaleService::new();

    let result = service.purchase_tokens(
        "Fj4k...x9Rm".to_string(),
        10.0,
    ).await;

    assert!(result.is_ok());

    let purchase = result.unwrap();

    assert_eq!(purchase.dride_amount, 2000.0); // 10 SOL * 200 DRIDE
    assert!(!purchase.transaction_hash.is_empty());
}

#[tokio::test]
async fn test_presale_purchase_invalid_wallet() {
    let service = PresaleService::new();

    let result = service.purchase_tokens(
        "invalid_wallet".to_string(),
        10.0,
    ).await;

    assert!(matches!(result, Err(PresaleError::InvalidWallet(_))));
}

#[tokio::test]
async fn test_presale_purchase_invalid_amount() {
    let service = PresaleService::new();

    // Too small
    let result = service.purchase_tokens(
        "Fj4k...x9Rm".to_string(),
        0.0,
    ).await;

    assert!(matches!(result, Err(PresaleError::InvalidAmount(_))));

    // Too large
    let result = service.purchase_tokens(
        "Fj4k...x9Rm".to_string(),
        1001.0,
    ).await;

    assert!(matches!(result, Err(PresaleError::InvalidAmount(_))));
}

#[tokio::test]
async fn test_presale_settings() {
    let service = PresaleService::new();

    let settings = service.get_presale_settings();

    assert_eq!(settings.price, 0.005);
    assert_eq!(settings.next_price, 0.008);
    assert_eq!(settings.tokens_per_sol, 200.0);
    assert_eq!(settings.min_purchase, 0.1);
    assert_eq!(settings.max_purchase, 1000.0);
    assert_eq!(settings.hard_cap, 1_000_000.0);
    assert_eq!(settings.max_per_wallet, 1000.0);
}
