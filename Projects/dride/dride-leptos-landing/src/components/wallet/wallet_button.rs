use leptos::*;
use super::wallet_adapter::{WalletManager, WalletType, WalletError};

#[component]
pub fn WalletButton(
    wallet_manager: Signal<WalletManager>,
) -> impl IntoView {
    let is_connected = create_memo(move |_| {
        wallet_manager.get().is_connected()
    });

    let wallet_address = create_memo(move |_| {
        wallet_manager.get().get_wallet_address()
    });

    let wallet_type = create_memo(move |_| {
        wallet_manager.get().get_wallet_type()
    });

    let show_modal = create_signal(false);

    let on_connect = move |_| {
        show_modal.set(true);
    };

    let on_disconnect = move |_| {
        wallet_manager.get().disconnect();
    };

    view! {
        <div class="relative">
            {if *is_connected {
                Some(view! {
                    <div class="flex items-center gap-2">
                        <div class="flex items-center gap-2 px-4 py-2 bg-bg-secondary rounded-lg border border-border">
                            {if let Some(wt) = *wallet_type {
                                Some(view! {
                                    <span class="text-xl">{wt.icon()}</span>
                                    <span class="font-medium text-text-primary">{wt.name()}</span>
                                })
                            }}
                            {if let Some(addr) = wallet_address.clone() {
                                Some(view! {
                                    <span class="text-sm text-text-tertiary font-mono">
                                        {format!("{}...{}", &addr[..8], &addr[addr.len()-8..])}
                                    </span>
                                })
                            }}
                        </div>
                        <button
                            class="px-3 py-2 text-sm font-medium text-text-secondary hover:text-text-primary transition-colors"
                            on:click=on_disconnect
                        >
                            "Disconnect"
                        </button>
                    </div>
                })
            } else {
                Some(view! {
                    <button
                        class="flex items-center gap-2 px-4 py-2 bg-brand-purple hover:bg-brand-purple-dark text-white font-medium rounded-lg transition-all duration-200 shadow-lg shadow-brand-purple/25"
                        on:click=on_connect
                    >
                        <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 9V7a2 2 0 002-2H5a2 2 0 00-2 2v2m3 4a2 2 0 012 2v6a2 2 0 01-2 2H7a2 2 0 01-2-2v-6m3-4h2M5 12H9a2 2 0 00-2-2V7a2 2 0 012-2h4v2" />
                        </svg>
                        <span>"Connect Wallet"</span>
                    </button>
                })
            }}

            // Wallet Selection Modal
            <WalletModal
                is_open=show_modal
                wallet_manager=wallet_manager
            />
        </div>
    }
}
