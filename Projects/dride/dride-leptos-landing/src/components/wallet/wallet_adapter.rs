use leptos::*;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum WalletType {
    Phantom,
    Solflare,
    Backpack,
    Glow,
    Brave,
    Trust,
    Coinbase,
}

impl WalletType {
    pub fn name(&self) -> &'static str {
        match self {
            WalletType::Phantom => "Phantom",
            WalletType::Solflare => "Solflare",
            WalletType::Backpack => "Backpack",
            WalletType::Glow => "Glow",
            WalletType::Brave => "Brave Wallet",
            WalletType::Trust => "Trust Wallet",
            WalletType::Coinbase => "Coinbase Wallet",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            WalletType::Phantom => "👻",
            WalletType::Solflare => "🔥",
            WalletType::Backpack => "🎒",
            WalletType::Glow => "✨",
            WalletType::Brave => "🦁",
            WalletType::Trust => "🔒",
            WalletType::Coinbase => "🔵",
        }
    }

    pub fn window_object(&self) -> &'static str {
        match self {
            WalletType::Phantom => "phantom",
            WalletType::Solflare => "solflare",
            WalletType::Backpack => "backpack",
            WalletType::Glow => "glowSolana",
            WalletType::Brave => "braveWalletSolana",
            WalletType::Trust => "trustwallet",
            WalletType::Coinbase => "coinbaseWalletExtension",
        }
    }

    pub fn recommended(&self) -> bool {
        matches!(self, WalletType::Phantom)
    }
}

#[derive(Debug, Clone)]
pub struct WalletAdapter {
    pub wallet_type: WalletType,
    pub is_available: bool,
    pub is_connected: bool,
    pub public_key: Option<Pubkey>,
    pub balance: Option<f64>,
}

impl WalletAdapter {
    pub fn new(wallet_type: WalletType) -> Self {
        Self {
            wallet_type,
            is_available: Self::check_availability(&wallet_type),
            is_connected: false,
            public_key: None,
            balance: None,
        }
    }

    fn check_availability(wallet_type: &WalletType) -> bool {
        // In WASM, check if wallet is available in window object
        let window = web_sys::window();
        let wallet_name = wallet_type.window_object();

        match js_sys::Reflect::get(&window, &wallet_name.into()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub async fn connect(&mut self) -> Result<(), WalletError> {
        if !self.is_available {
            return Err(WalletError::WalletNotAvailable);
        }

        // Connect to wallet using browser APIs
        // This would use @solana/wallet-adapter in production
        // For now, we'll simulate the connection
        self.is_connected = true;
        self.public_key = Some(Pubkey::new_unique());

        Ok(())
    }

    pub async fn disconnect(&mut self) {
        self.is_connected = false;
        self.public_key = None;
        self.balance = None;
    }

    pub async fn sign_transaction(
        &self,
        transaction: &str,
    ) -> Result<String, WalletError> {
        if !self.is_connected {
            return Err(WalletError::NotConnected);
        }

        // Sign transaction using wallet adapter
        // In production, this would use the wallet's signTransaction method
        // For now, return a placeholder signature
        Ok("placeholder_signature_64_chars_long".to_string())
    }

    pub async fn get_balance(&mut self) -> Result<f64, WalletError> {
        if !self.is_connected || self.public_key.is_none() {
            return Err(WalletError::NotConnected);
        }

        // Get balance from Solana RPC
        // In production, this would query the chain
        // For now, return a placeholder
        self.balance = Some(12.5);
        Ok(12.5)
    }
}

#[derive(Debug, Clone)]
pub struct WalletManager {
    available_wallets: HashMap<WalletType, WalletAdapter>,
    active_wallet: Signal<Option<WalletAdapter>>,
}

impl WalletManager {
    pub fn new() -> Self {
        let mut available_wallets = HashMap::new();

        // Check availability of all wallets
        for wallet_type in [
            WalletType::Phantom,
            WalletType::Solflare,
            WalletType::Backpack,
            WalletType::Glow,
            WalletType::Brave,
            WalletType::Trust,
            WalletType::Coinbase,
        ] {
            let adapter = WalletAdapter::new(wallet_type.clone());
            if adapter.is_available {
                available_wallets.insert(wallet_type, adapter);
            }
        }

        Self {
            available_wallets,
            active_wallet: create_signal(None),
        }
    }

    pub fn get_available_wallets(&self) -> Vec<WalletType> {
        self.available_wallets
            .keys()
            .cloned()
            .collect()
    }

    pub async fn connect_wallet(&self, wallet_type: WalletType) -> Result<(), WalletError> {
        if let Some(adapter) = self.available_wallets.get(&wallet_type) {
            let mut adapter = adapter.clone();
            adapter.connect().await?;
            self.active_wallet.set(Some(adapter));
            Ok(())
        } else {
            Err(WalletError::WalletNotAvailable)
        }
    }

    pub fn disconnect(&self) {
        self.active_wallet.set(None);
    }

    pub fn is_connected(&self) -> bool {
        self.active_wallet.get().is_some()
    }

    pub fn get_public_key(&self) -> Option<Pubkey> {
        self.active_wallet.get()?.public_key.clone()
    }

    pub fn get_wallet_type(&self) -> Option<WalletType> {
        self.active_wallet.get().map(|w| w.wallet_type.clone())
    }

    pub fn get_balance(&self) -> Option<f64> {
        self.active_wallet.get()?.balance
    }

    pub fn get_wallet_address(&self) -> Option<String> {
        self.active_wallet.get()?.public_key.as_ref().map(|pk| pk.to_string())
    }

    pub async fn refresh_balance(&self) -> Result<(), WalletError> {
        if let Some(adapter) = self.active_wallet.get() {
            let mut active_wallets = &mut self.available_wallets.clone();
            if let Some(ref mut adapter) = active_wallets.get_mut(&adapter.wallet_type) {
                let balance = adapter.get_balance().await?;
                adapter.balance = Some(balance);
                self.active_wallet.set(Some(adapter.clone()));
            }
            Ok(())
        } else {
            Err(WalletError::NotConnected)
        }
    }

    pub fn has_available_wallets(&self) -> bool {
        !self.available_wallets.is_empty()
    }

    pub fn get_recommended_wallet(&self) -> Option<WalletType> {
        self.available_wallets
            .keys()
            .find(|w| w.recommended())
            .cloned()
    }
}

impl Default for WalletManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum WalletError {
    #[error("Wallet not available")]
    WalletNotAvailable,

    #[error("Wallet not connected")]
    NotConnected,

    #[error("Transaction rejected by user")]
    TransactionRejected,

    #[error("Network error")]
    NetworkError,

    #[error("Invalid transaction")]
    InvalidTransaction,
}
