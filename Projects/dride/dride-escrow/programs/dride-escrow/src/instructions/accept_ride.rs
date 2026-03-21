use anchor_lang::prelude::*;

use crate::errors::DRideError;
use crate::state::{EscrowStatus, RideEscrow};

/// Backend authority assigns a driver to the escrow
pub fn handler(ctx: Context<AcceptRide>) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;

    require!(escrow.status == EscrowStatus::Created, DRideError::InvalidStatus);
    require!(escrow.driver == Pubkey::default(), DRideError::DriverAlreadyAssigned);

    escrow.driver = ctx.accounts.driver.key();
    escrow.status = EscrowStatus::Active;

    msg!("Ride accepted: driver={}", ctx.accounts.driver.key());
    Ok(())
}

#[derive(Accounts)]
pub struct AcceptRide<'info> {
    /// Backend authority that orchestrates ride matching
    pub authority: Signer<'info>,

    /// Driver being assigned
    /// CHECK: Driver pubkey validated by the backend
    pub driver: UncheckedAccount<'info>,

    /// Escrow PDA for this ride
    #[account(
        mut,
        seeds = [b"ride", escrow.ride_id.as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, RideEscrow>,
}
