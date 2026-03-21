use anchor_lang::prelude::*;

use crate::errors::DRideError;
use crate::state::{EscrowStatus, RideEscrow};

/// Backend authority releases escrow: 90% to driver, 10% to protocol
pub fn handler(ctx: Context<CompleteRide>) -> Result<()> {
    // Read values before any borrows
    let amount = ctx.accounts.escrow.amount;
    let fee_bps = ctx.accounts.escrow.protocol_fee_bps;
    let status = ctx.accounts.escrow.status;
    let driver = ctx.accounts.escrow.driver;

    require!(status == EscrowStatus::Active, DRideError::InvalidStatus);
    require!(driver != Pubkey::default(), DRideError::NoDriverAssigned);

    let fee = amount
        .checked_mul(fee_bps as u64)
        .ok_or(DRideError::Overflow)?
        .checked_div(10_000)
        .ok_or(DRideError::Overflow)?;
    let driver_amount = amount.checked_sub(fee).ok_or(DRideError::Overflow)?;

    // Transfer to driver
    let escrow_info = ctx.accounts.escrow.to_account_info();
    **escrow_info.try_borrow_mut_lamports()? -= driver_amount;
    **ctx.accounts.driver.to_account_info().try_borrow_mut_lamports()? += driver_amount;

    // Transfer fee to protocol wallet
    **escrow_info.try_borrow_mut_lamports()? -= fee;
    **ctx.accounts.protocol_wallet.to_account_info().try_borrow_mut_lamports()? += fee;

    // Update status
    ctx.accounts.escrow.status = EscrowStatus::Completed;

    msg!(
        "Ride completed: driver_amount={}, protocol_fee={}",
        driver_amount,
        fee
    );
    Ok(())
}

#[derive(Accounts)]
pub struct CompleteRide<'info> {
    /// Backend authority that confirms ride completion
    pub authority: Signer<'info>,

    /// Driver receiving payment
    /// CHECK: Validated against escrow.driver
    #[account(
        mut,
        constraint = driver.key() == escrow.driver @ DRideError::Unauthorized,
    )]
    pub driver: UncheckedAccount<'info>,

    /// Protocol treasury receiving fee
    /// CHECK: Validated against escrow.protocol_wallet
    #[account(
        mut,
        constraint = protocol_wallet.key() == escrow.protocol_wallet @ DRideError::Unauthorized,
    )]
    pub protocol_wallet: UncheckedAccount<'info>,

    /// Escrow PDA for this ride
    #[account(
        mut,
        seeds = [b"ride", escrow.ride_id.as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, RideEscrow>,
}
