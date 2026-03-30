use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};
use anchor_spl::associated_token::AssociatedToken;

declare_id!("DRidePresale11111111111111111111111111");

#[program]
pub mod dride_presale {
    use super::*;

    /// Initialize the presale vault
    /// One-time setup that creates the vault to hold SOL during presale
    pub fn initialize_presale(
        ctx: Context<InitializePresale>,
        hard_cap_sol: u64,
        tokens_per_sol: u64,
        start_timestamp: i64,
        end_timestamp: i64,
    ) -> Result<()> {
        let presale_state = &mut ctx.accounts.presale_state;

        // Validate presale parameters
        require!(hard_cap_sol > 0, PresaleError::InvalidParameters);
        require!(tokens_per_sol == 200, PresaleError::InvalidParameters); // 1 SOL = 200 DRIDE
        require!(start_timestamp > 0, PresaleError::InvalidParameters);
        require!(end_timestamp > start_timestamp, PresaleError::InvalidParameters);

        // Validate duration (presale must be between 1 day and 1 year)
        let duration_secs = end_timestamp - start_timestamp;
        require!(duration_secs >= 86400, PresaleError::InvalidParameters); // 1 day minimum
        require!(duration_secs <= 31536000, PresaleError::InvalidParameters); // 1 year maximum

        // Validate hard cap (1M SOL = 1,000,000,000,000 lamports)
        require!(
            hard_cap_sol <= 1_000_000_000_000,
            PresaleError::InvalidHardCap
        );

        // Initialize presale state
        presale_state.authority = ctx.accounts.authority.key();
        presale_state.vault = ctx.accounts.vault.key();
        presale_state.token_mint = ctx.accounts.token_mint.key();
        presale_state.hard_cap_sol = hard_cap_sol;
        presale_state.total_raised_sol = 0;
        presale_state.tokens_per_sol = tokens_per_sol;
        presale_state.start_timestamp = start_timestamp;
        presale_state.end_timestamp = end_timestamp;
        presale_state.is_paused = false;
        presale_state.bump = ctx.bumps.presale_state;

        msg!(
            "Presale initialized: Hard cap {} SOL, {} tokens/SOL",
            hard_cap_sol / 1_000_000_000,
            tokens_per_sol
        );

        Ok(())
    }

    /// Purchase $DRIDE tokens with SOL during presale
    pub fn purchase_tokens(
        ctx: Context<PurchaseTokens>,
        sol_amount: u64,
    ) -> Result<()> {
        let presale_state = &mut ctx.accounts.presale_state;
        let current_time = Clock::get()?.unix_timestamp;

        // Check if presale is active
        require!(!presale_state.is_paused, PresaleError::PresalePaused);
        require!(
            current_time >= presale_state.start_timestamp,
            PresaleError::PresaleNotStarted
        );
        require!(
            current_time <= presale_state.end_timestamp,
            PresaleError::PresaleEnded
        );

        // Validate purchase amount
        require!(sol_amount >= 100_000_000, PresaleError::InvalidAmount); // Minimum 0.1 SOL
        require!(
            sol_amount <= 100_000_000_000,
            PresaleError::InvalidAmount
        ); // Maximum 100 SOL per transaction

        // Check hard cap
        let new_total = presale_state.total_raised_sol.checked_add(sol_amount).unwrap();
        require!(
            new_total <= presale_state.hard_cap_sol,
            PresaleError::HardCapReached
        );

        // Check per-wallet limit (1000 SOL per wallet)
        let user_purchased = ctx.accounts.user_state.amount_purchased;
        let new_user_total = user_purchased.checked_add(sol_amount).unwrap();
        require!(
            new_user_total <= 1_000_000_000_000,
            PresaleError::WalletLimitExceeded
        ); // 1000 SOL limit

        // Calculate tokens to receive
        let tokens_to_receive = sol_amount
            .checked_mul(presale_state.tokens_per_sol)
            .unwrap();

        // Transfer SOL from user to vault
        let cpi_accounts = Transfer {
            from: ctx.accounts.user.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.system_program.to_account_info();
        token::transfer(CpiContext::new(cpi_program, cpi_accounts), sol_amount)?;

        // Update presale totals
        presale_state.total_raised_sol = new_total;

        // Update user state
        let user_state = &mut ctx.accounts.user_state;
        user_state.authority = ctx.accounts.user.key();
        user_state.amount_purchased = new_user_total;
        user_state.tokens_received = user_state
            .tokens_received
            .checked_add(tokens_to_receive)
            .unwrap();
        user_state.purchase_count += 1;
        user_state.last_purchase_at = current_time;

        msg!(
            "Purchase: {} SOL -> {} DRIDE tokens (Total: {} SOL)",
            sol_amount / 1_000_000_000,
            tokens_to_receive,
            new_total / 1_000_000_000
        );

        Ok(())
    }

    /// Pause the presale (authority only)
    pub fn pause_presale(ctx: Context<PresaleAuthority>) -> Result<()> {
        let presale_state = &mut ctx.accounts.presale_state;
        require!(!presale_state.is_paused, PresaleError::AlreadyPaused);
        presale_state.is_paused = true;
        msg!("Presale paused by authority");
        Ok(())
    }

    /// Unpause the presale (authority only)
    pub fn unpause_presale(ctx: Context<PresaleAuthority>) -> Result<()> {
        let presale_state = &mut ctx.accounts.presale_state;
        require!(presale_state.is_paused, PresaleError::NotPaused);
        presale_state.is_paused = false;
        msg!("Presale unpaused by authority");
        Ok(())
    }

    /// Withdraw SOL from vault (authority only, after presale ends)
    pub fn withdraw_sol(
        ctx: Context<WithdrawSol>,
        amount: u64,
    ) -> Result<()> {
        let presale_state = &ctx.accounts.presale_state;
        let current_time = Clock::get()?.unix_timestamp;

        // Only allow withdrawal after presale ends
        require!(
            current_time > presale_state.end_timestamp,
            PresaleError::PresaleStillActive
        );

        // Cannot withdraw more than raised amount
        require!(
            amount <= presale_state.total_raised_sol,
            PresaleError::InsufficientFunds
        );

        // Transfer SOL from vault to authority
        let vault_lamports = ctx.accounts.vault.get_lamports();
        let authority_lamports = ctx.accounts.authority.get_lamports();

        **ctx.accounts.vault.lamports.borrow_mut() -= amount;
        **ctx.accounts.authority.lamports.borrow_mut() += amount;

        msg!("Withdrew {} SOL from presale vault", amount / 1_000_000_000);

        Ok(())
    }

    /// Update presale parameters (authority only)
    pub fn update_presale(
        ctx: Context<PresaleAuthority>,
        new_end_timestamp: Option<i64>,
    ) -> Result<()> {
        let presale_state = &mut ctx.accounts.presale_state;
        let current_time = Clock::get()?.unix_timestamp;

        if let Some(new_end) = new_end_timestamp {
            // Can only extend presale, not shorten
            require!(
                new_end > presale_state.end_timestamp,
                PresaleError::InvalidParameters
            );
            presale_state.end_timestamp = new_end;
            msg!("Presale extended to {}", new_end);
        }

        Ok(())
    }
}

// ============== Contexts ==============

#[derive(Accounts)]
#[instruction(
    hard_cap_sol: u64,
    tokens_per_sol: u64,
    start_timestamp: i64,
    end_timestamp: i64
)]
pub struct InitializePresale<'info> {
    /// Presale state PDA
    #[account(
        init,
        payer = payer,
        space = 8 + PresaleState::INIT_SPACE,
        seeds = [b"presale_state"],
        bump
    )]
    pub presale_state: Account<'info, PresaleState>,

    /// Presale authority (multisig)
    pub authority: Signer<'info>,

    /// SOL vault (PDA)
    #[account(
        init,
        payer = payer,
        space = 0,
        seeds = [b"presale_vault"],
        bump
    )]
    pub vault: SystemAccount<'info>,

    /// DRIDE token mint
    pub token_mint: Account<'info, Mint>,

    /// Payer for account creation
    #[account(mut)]
    pub payer: Signer<'info>,

    /// System program
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PurchaseTokens<'info> {
    /// Presale state
    #[account(
        mut,
        seeds = [b"presale_state"],
        bump
    )]
    pub presale_state: Account<'info, PresaleState>,

    /// User state PDA (tracks purchases per wallet)
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + UserState::INIT_SPACE,
        seeds = [b"user_state", user.key().as_ref()],
        bump
    )]
    pub user_state: Account<'info, UserState>,

    /// User's SOL account (the signer)
    #[account(mut)]
    pub user: Signer<'info>,

    /// Presale vault (SOL)
    #[account(
        mut,
        seeds = [b"presale_vault"],
        bump
    )]
    pub vault: SystemAccount<'info>,

    /// DRIDE token mint
    pub token_mint: Account<'info, Mint>,

    /// User's DRIDE token account
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_mint,
        associated_token::authority = user,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    /// System program
    pub system_program: Program<'info, System>,

    /// Token program
    pub token_program: Program<'info, Token>,

    /// Associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct PresaleAuthority<'info> {
    /// Presale state
    #[account(
        mut,
        seeds = [b"presale_state"],
        bump,
        constraint = presale_state.authority == authority.key() @ PresaleError::Unauthorized
    )]
    pub presale_state: Account<'info, PresaleState>,

    /// Presale authority (multisig)
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct WithdrawSol<'info> {
    /// Presale state
    #[account(
        mut,
        seeds = [b"presale_state"],
        bump,
        constraint = presale_state.authority == authority.key() @ PresaleError::Unauthorized
    )]
    pub presale_state: Account<'info, PresaleState>,

    /// Presale vault (SOL)
    #[account(
        mut,
        seeds = [b"presale_vault"],
        bump
    )]
    pub vault: SystemAccount<'info>,

    /// Presale authority
    #[account(mut)]
    pub authority: Signer<'info>,

    /// System program
    pub system_program: Program<'info, System>,
}

// ============== State ==============

#[account]
#[derive(InitSpace)]
pub struct PresaleState {
    pub authority: Pubkey,
    pub vault: Pubkey,
    pub token_mint: Pubkey,
    pub hard_cap_sol: u64,
    pub total_raised_sol: u64,
    pub tokens_per_sol: u64,
    pub start_timestamp: i64,
    pub end_timestamp: i64,
    pub is_paused: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct UserState {
    pub authority: Pubkey,
    pub amount_purchased: u64,
    pub tokens_received: u64,
    pub purchase_count: u32,
    pub last_purchase_at: i64,
    pub bump: u8,
}

// ============== Errors ==============

#[error_code]
pub enum PresaleError {
    #[msg("Invalid presale parameters")]
    InvalidParameters,

    #[msg("Invalid hard cap - must be <= 1M SOL")]
    InvalidHardCap,

    #[msg("Presale has not started yet")]
    PresaleNotStarted,

    #[msg("Presale has ended")]
    PresaleEnded,

    #[msg("Presale is paused")]
    PresalePaused,

    #[msg("Presale is already paused")]
    AlreadyPaused,

    #[msg("Presale is not paused")]
    NotPaused,

    #[msg("Presale is still active")]
    PresaleStillActive,

    #[msg("Hard cap has been reached")]
    HardCapReached,

    #[msg("Invalid purchase amount (min 0.1 SOL, max 100 SOL)")]
    InvalidAmount,

    #[msg("Wallet purchase limit exceeded (1000 SOL)")]
    WalletLimitExceeded,

    #[msg("Insufficient funds in vault")]
    InsufficientFunds,

    #[msg("Unauthorized - only presale authority can perform this action")]
    Unauthorized,
}
