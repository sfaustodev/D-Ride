use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::errors::DRideError;
use crate::state::{EscrowStatus, RideEscrow};

/// Passenger deposits SOL into escrow PDA
pub fn handler(
    ctx: Context<CreateRide>,
    ride_id: [u8; 16],
    amount: u64,
    protocol_fee_bps: u16,
) -> Result<()> {
    require!(amount > 0, DRideError::InvalidAmount);
    require!(protocol_fee_bps <= 5000, DRideError::FeeTooHigh);

    // Transfer SOL from passenger to escrow PDA
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.passenger.to_account_info(),
                to: ctx.accounts.escrow.to_account_info(),
            },
        ),
        amount,
    )?;

    let escrow = &mut ctx.accounts.escrow;
    escrow.ride_id = ride_id;
    escrow.passenger = ctx.accounts.passenger.key();
    escrow.driver = Pubkey::default();
    escrow.amount = amount;
    escrow.protocol_fee_bps = protocol_fee_bps;
    escrow.protocol_wallet = ctx.accounts.protocol_wallet.key();
    escrow.status = EscrowStatus::Created;
    escrow.created_at = Clock::get()?.unix_timestamp;
    escrow.bump = ctx.bumps.escrow;

    msg!("Escrow created: amount={} lamports", amount);
    Ok(())
}

#[derive(Accounts)]
#[instruction(ride_id: [u8; 16])]
pub struct CreateRide<'info> {
    /// Passenger who pays for the ride
    #[account(mut)]
    pub passenger: Signer<'info>,

    /// Escrow PDA: seeds = ["ride", ride_uuid_bytes]
    #[account(
        init,
        payer = passenger,
        space = RideEscrow::LEN,
        seeds = [b"ride", ride_id.as_ref()],
        bump,
    )]
    pub escrow: Account<'info, RideEscrow>,

    /// Protocol treasury wallet (just needs to be a valid pubkey)
    /// CHECK: This is the protocol fee recipient, validated by the backend
    pub protocol_wallet: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}
