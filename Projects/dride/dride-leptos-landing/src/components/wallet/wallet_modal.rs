use leptos::*;
use super::wallet_adapter::{WalletManager, WalletType, WalletError};

#[component]
pub fn WalletModal(
    is_open: Signal<bool>,
    wallet_manager: Signal<WalletManager>,
) -> impl IntoView {
    let available_wallets = create_memo(move |_| {
        wallet_manager.get().get_available_wallets()
    });

    let recommended_wallet = create_memo(move |_| {
        wallet_manager.get().get_recommended_wallet()
    });

    let on_wallet_select = move |wallet_type: WalletType| {
        spawn_local(async move {
            match wallet_manager.get().connect_wallet(wallet_type).await {
                Ok(_) => {
                    is_open.set(false);
                }
                Err(e) => {
                    tracing::error!("Failed to connect wallet: {:?}", e);
                    // Show error to user
                }
            }
        });
    };

    view! {
        <Show when=move || is_open.get()>
            <div class="fixed inset-0 z-50 flex items-center justify-center p-4">
                {/* Backdrop */}
                <div
                    class="absolute inset-0 bg-black/50 backdrop-blur-sm transition-opacity duration-200"
                    on:click=move |_| is_open.set(false)
                ></div>

                {/* Modal */}
                <div class="relative z-10 w-full max-w-md bg-bg-secondary rounded-2xl border border-border shadow-2xl p-6">
                    <div class="flex items-center justify-between mb-6">
                        <h2 class="text-2xl font-bold text-text-primary">Select Wallet</h2>
                        <button
                            class="p-2 hover:bg-bg-tertiary rounded-lg transition-colors"
                            on:click=move |_| is_open.set(false)
                        >
                            <svg class="w-6 h-6 text-text-secondary" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </button>
                    </div>

                    <p class="text-text-secondary mb-6">
                        "Connect your Solana wallet to purchase $DRIDE tokens. All transactions will be signed by your wallet."
                    </p>

                    <div class="space-y-3">
                        <For
                            each=available_wallets
                            key=|wallet_type| format!("{:?}", wallet_type)
                            children=move |wallet_type| {
                                let is_recommended = *recommended_wallet == Some(wallet_type.clone());

                                view! {
                                    <button
                                        class=format!(
                                            "w-full flex items-center gap-3 px-4 py-3 rounded-lg border transition-all duration-200 {}",
                                            if is_recommended {
                                                "border-brand-purple bg-brand-purple/10 hover:bg-brand-purple/20"
                                            } else {
                                                "border-border hover:border-brand-purple/30 hover:bg-bg-tertiary"
                                            }
                                        )
                                        on:click=move |_| on_wallet_select(wallet_type.clone())
                                    >
                                        <span class="text-2xl">{wallet_type.icon()}</span>
                                        <span class="flex-1 text-left">
                                            <div class="font-semibold text-text-primary">{wallet_type.name()}</div>
                                            {if is_recommended {
                                                Some(view! {
                                                    <div class="text-xs text-brand-purple font-medium">Recommended</div>
                                                })
                                            }}
                                        </span>
                                    </button>
                                }
                            }
                        />
                    </div>

                    <div class="mt-6 p-4 bg-bg-tertiary/50 rounded-lg">
                        <div class="flex items-start gap-2">
                            <svg class="w-5 h-5 text-accent-amber flex-shrink-0 mt-0.5" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 0h.01m-6.938 4.13-1.732 3.592l.79-1.612a1 1 0 01.248-1.269l-.004-4.858a1 1 0 00-.002-1.813L8.732 2.558a1 1 0 00-1.457-1.65l-6.026-6.726c-.613-.22-1.638-.34-2.372-.436-.736.16-1.259.244-1.702.252-3.648a1 1 0 00-1.057-.026l-1.246-.488a1 1 0 00-1.472-.585l-.008-.016a1 1 0 00-2.026-.006l-1.628.912a1 1 0 00-1.298-.493l-.008-.016a1 1 0 00-1.065-.277l-.009-.016a1 1 0 00-1.05-.278l-.009-.016a1 1 0 00-.564-.479l.002-.003a1 1 0 00-.933.719l.002-.003a1 1 0 00-.726.278l.003.003a1 1 0 00-.23.473l.002-.003a1 1 0 00-.473.063l.002-.002a1 1 0 00-.083.241l.002.003a1 1 0 00-.242.473l-.004-.003a1 1 0 00-.472.082l.003-.003a1 1 0 00-.335.063l.002-.003a1 1 0 00-.335.242l.002-.003a1 1 0 00.564-.478l.001-.002a1 1 0 00.727.722l.001.002a1 1 0 00.933.72l.001-.002a1 1 0 00.474.065l-.003.003a1 1 0 00.472.082l-.004.003a1 1 0 00.564-.479l.004-.003a1 1 0 00.726.279l.003.002a1 1 0 00.934.718l.002.003a1 1 0 001.298.493l-.009.016a1 1 0 001.056.277l-.008.016a1 1 0 001.058.277l-.009.016a1 1 0 00.243.473l-.002.003a1 1 0 00.473.063l-.003.002a1 1 0 00.083.242l-.002.003a1 1 0 00.242.474l-.003.003a1 1 0 00.336.063l.002-.003a1 1 0 00.335.241l-.001-.002a1 1 0 00.727.722l-.002-.003a1 1 0 00.933.719l.003.003a1 1 0 001.472.585l.008.016a1 1 0 001.056.026l1.627.912a1 1 0 001.298.493l.008.016a1 1 0 002.026.006l6.027 6.726c.736.216 1.702.341 2.372.435 1.259.244l4.858.002a1 1 0 001.812.026l.004.4.858a1 1 0 00.002.001l.008.016a1 1 0 001.056.277l.008-.016a1 1 0 001.065-.277l.009.016a1 1 0 00.563.479l.003.003a1 1 0 00.336.242l-.003-.002a1 1 0 00.335.241l-.002-.003a1 1 0 00.473.082l-.003-.003a1 1 0 00.472.063l-.002-.003a1 1 0 00.083.242l-.003.002a1 1 0 00.242.473l-.002-.003a1 1 0 00.472.082l-.004-.003a1 1 0 00.564.479l-.004-.003a1 1 0 00.726.278l-.003.003a1 1 0 00.934.72l-.002.003a1 1 0 001.472.585l.009.016a1 1 0 001.057.026l1.246.488a1 1 0 001.457.65l.79 1.612a1 1 0 00.002-1.813l.004-4.858a1 1 0 00.002-1.065l.79 1.612a1 1 0 001.248-1.269l.79-1.612a1 1 0 01.65 2.413z" />
                            </svg>
                            <div class="text-sm text-text-secondary">
                                <p class="mb-1">"Make sure you're using a secure wallet and verify all transactions before signing."</p>
                                <p class="text-text-tertiary text-xs">"Never share your seed phrase or private key with anyone."</p>
                            </div>
                        </div>
                    </div>

                    <button
                        class="w-full mt-6 px-4 py-3 bg-bg-tertiary hover:bg-bg-secondary text-text-secondary hover:text-text-primary font-medium rounded-lg transition-all duration-200 border border-border"
                        on:click=move |_| is_open.set(false)
                    >
                        "Cancel"
                    </button>
                </div>
            </div>
        </Show>
    }
}
