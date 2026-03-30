use dride_leptos_landing::components::wallet::{
    WalletManager,
    WalletAdapter,
    WalletType,
    WalletError,
};

#[tokio::test]
async fn test_wallet_manager_creation() {
    let manager = WalletManager::new();

    // Should have at least Phantom available (in real WASM environment)
    assert!(manager.has_available_wallets());

    // Phantom should be recommended
    assert_eq!(
        manager.get_recommended_wallet(),
        Some(WalletType::Phantom)
    );
}

#[tokio::test]
async fn test_wallet_connection() {
    let manager = WalletManager::new();

    // In production, this would connect to real wallet
    // For now, we test the logic
    let result = manager.connect_wallet(WalletType::Phantom).await;

    // In simulation, connection succeeds
    assert!(result.is_ok());

    // Wallet should be connected
    assert!(manager.is_connected());
}

#[tokio::test]
async fn test_wallet_disconnection() {
    let manager = WalletManager::new();

    // Connect first
    manager.connect_wallet(WalletType::Phantom).await.unwrap();

    // Should be connected
    assert!(manager.is_connected());

    // Disconnect
    manager.disconnect();

    // Should be disconnected
    assert!(!manager.is_connected());
    assert!(manager.get_public_key().is_none());
}

#[tokio::test]
async fn test_wallet_type_properties() {
    // Test Phantom
    assert_eq!(WalletType::Phantom.name(), "Phantom");
    assert_eq!(WalletType::Phantom.icon(), "👻");
    assert_eq!(WalletType::Phantom.window_object(), "phantom");
    assert!(WalletType::Phantom.recommended(), true);

    // Test Solflare
    assert_eq!(WalletType::Solflare.name(), "Solflare");
    assert_eq!(WalletType::Solflare.icon(), "🔥");
    assert!(WalletType::Solflare.window_object(), "solflare");
    assert!(!WalletType::Solflare.recommended());

    // Test Backpack
    assert_eq!(WalletType::Backpack.name(), "Backpack");
    assert_eq!(WalletType::Backpack.icon(), "🎒");
    assert_eq!(WalletType::Backpack.window_object(), "backpack");
}

#[tokio::test]
async fn test_multiple_wallets_available() {
    let manager = WalletManager::new();

    let available = manager.get_available_wallets();

    // Should have multiple wallets available
    assert!(available.len() >= 1);

    // Each wallet should be a valid WalletType
    for wallet_type in available {
        match wallet_type {
            WalletType::Phantom
            | WalletType::Solflare
            | WalletType::Backpack
            | WalletType::Glow
            | WalletType::Brave
            | WalletType::Trust
            | WalletType::Coinbase => {}
        }
    }
}
