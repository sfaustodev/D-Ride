use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};

declare_id!("DRideToken11111111111111111111111111");

#[program]
pub mod dride_token {
    use super::*;

    /// Initialize the $DRIDE token mint
    /// This is a one-time operation that creates the token with fixed supply
    pub fn initialize_mint(
        ctx: Context<InitializeMint>,
        decimals: u8,
        max_supply: u64,
    ) -> Result<()> {
        let mint = &ctx.accounts.mint;
        let authority = &ctx.accounts.authority;

        // Validate decimals (standard for Solana is 9)
        require!(decimals <= 9, TokenError::InvalidDecimals);

        // Validate max supply (1 billion tokens = 1,000,000,000,000,000,000 with 9 decimals)
        require!(
            max_supply == 1_000_000_000_000_000_000,
            TokenError::InvalidSupply
        );

        // Initialize the mint
        let cpi_accounts = token::InitializeMint {
            mint: mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let signer_seeds = &[
            b"mint_authority",
            &[ctx.bumps.mint_authority],
        ];
        let signer = &[&signer_seeds[..]];

        token::initialize_mint(
            CpiContext::new(cpi_program, cpi_accounts).with_signer(signer),
            decimals,
            &authority.key(),
            None, // No freeze authority
        )?;

        msg!("DRIDE Token initialized with supply: {}", max_supply);

        Ok(())
    }

    /// Mint tokens to a recipient
    /// Only the mint authority can mint tokens
    pub fn mint_tokens(
        ctx: Context<MintTokens>,
        amount: u64,
    ) -> Result<()> {
        let mint = &ctx.accounts.mint;
        let destination = &ctx.accounts.destination;

        // Validate amount
        require!(amount > 0, TokenError::InvalidAmount);

        // Get current supply
        let current_supply = mint.supply;
        let max_supply = 1_000_000_000_000_000_000; // 1 billion tokens with 9 decimals

        // Ensure we don't exceed max supply
        require!(
            current_supply.checked_add(amount).unwrap() <= max_supply,
            TokenError::MaxSupplyExceeded
        );

        // Mint tokens
        let cpi_accounts = MintTo {
            mint: mint.to_account_info(),
            to: destination.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();

        token::mint_to(
            CpiContext::new(cpi_program, cpi_accounts),
            amount,
        )?;

        msg!("Minted {} DRIDE tokens to {}", amount, destination.key());

        Ok(())
    }

    /// Transfer tokens from one account to another
    /// Standard SPL token transfer
    pub fn transfer_tokens(
        ctx: Context<TransferTokens>,
        amount: u64,
    ) -> Result<()> {
        require!(amount > 0, TokenError::InvalidAmount);

        let cpi_accounts = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();

        token::transfer(
            CpiContext::new(cpi_program, cpi_accounts),
            amount,
        )?;

        msg!(
            "Transferred {} DRIDE tokens from {} to {}",
            amount,
            ctx.accounts.from.key(),
            ctx.accounts.to.key()
        );

        Ok(())
    }
}

// ============== Contexts ==============

#[derive(Accounts)]
#[instruction(decimals: u8, max_supply: u64)]
pub struct InitializeMint<'info> {
    /// The token mint account to be initialized
    #[account(
        init,
        payer = payer,
        mint::decimals = decimals,
        mint::authority = mint_authority,
        seeds = [b"dride_mint"],
        bump
    )]
    pub mint: Account<'info, Mint>,

    /// The mint authority (multisig PDA)
    #[account(
        seeds = [b"mint_authority"],
        bump
    )]
    pub mint_authority: SystemAccount<'info>,

    /// The payer for account creation
    #[account(mut)]
    pub payer: Signer<'info>,

    /// System program
    pub system_program: Program<'info, System>,

    /// Token program
    pub token_program: Program<'info, Token>,

    /// Rent sysvar
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    /// The token mint
    #[account(
        mut,
        seeds = [b"dride_mint"],
        bump
    )]
    pub mint: Account<'info, Mint>,

    /// The mint authority (multisig PDA)
    #[account(
        seeds = [b"mint_authority"],
        bump
    )]
    pub mint_authority: Signer<'info>,

    /// The destination token account
    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,

    /// Token program
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    /// The token mint
    #[account(
        seeds = [b"dride_mint"],
        bump
    )]
    pub mint: Account<'info, Mint>,

    /// Source token account
    #[account(
        mut,
        constraint = from.mint == mint.key() @ TokenError::InvalidMint
    )]
    pub from: Account<'info, TokenAccount>,

    /// Destination token account
    #[account(
        mut,
        constraint = to.mint == mint.key() @ TokenError::InvalidMint
    )]
    pub to: Account<'info, TokenAccount>,

    /// Authority of the source token account
    pub authority: Signer<'info>,

    /// Token program
    pub token_program: Program<'info, Token>,
}

// ============== Errors ==============

#[error_code]
pub enum TokenError {
    #[msg("Invalid decimals for token")]
    InvalidDecimals,

    #[msg("Invalid max supply - must be 1 billion tokens")]
    InvalidSupply,

    #[msg("Amount must be greater than 0")]
    InvalidAmount,

    #[msg("Max supply exceeded - cannot mint more tokens")]
    MaxSupplyExceeded,

    #[msg("Invalid mint account")]
    InvalidMint,
}
