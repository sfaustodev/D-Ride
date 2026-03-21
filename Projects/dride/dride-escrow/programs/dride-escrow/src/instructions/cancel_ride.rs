use anchor_lang::prelude::*;

use crate::errors::DRideError;
use crate::state::{EscrowStatus, RideEscrow};

/// Refund full escrow amount back to passenger
pub fn handler(ctx: Context<CancelRide>) -> Result<()> {
    let status = ctx.accounts.escrow.status;
    let amount = ctx.accounts.escrow.amount;

    require!(
        status == EscrowStatus::Created || status == EscrowStatus::Active,
        DRideError::InvalidStatus
    );

    // Transfer full amount back to passenger
    let escrow_info = ctx.accounts.escrow.to_account_info();
    **escrow_info.try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.passenger.to_account_info().try_borrow_mut_lamports()? += amount;

    ctx.accounts.escrow.status = EscrowStatus::Cancelled;

    msg!("Ride cancelled: refunded {} lamports to passenger", amount);
    Ok(())
}

#[derive(Accounts)]
pub struct CancelRide<'info> {
    /// Authority: either the passenger themselves or the backend authority
    pub authority: Signer<'info>,

    /// Passenger receiving the refund
    /// CHECK: Validated against escrow.passenger
    #[account(
        mut,
        constraint = passenger.key() == escrow.passenger @ DRideError::NotPassenger,
    )]
    pub passenger: UncheckedAccount<'info>,

    /// Escrow PDA for this ride
    #[account(
        mut,
        seeds = [b"ride", escrow.ride_id.as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, RideEscrow>,
}
