use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

pub use instructions::accept_ride::*;
pub use instructions::cancel_ride::*;
pub use instructions::complete_ride::*;
pub use instructions::create_ride::*;

declare_id!("2fhZ4fGn3NoU64UCbPxKEYEakPXXBTHJsHCw73QV7APx");

#[program]
pub mod dride_escrow {
    use super::*;

    /// Passenger deposits SOL into escrow PDA
    pub fn create_ride(
        ctx: Context<CreateRide>,
        ride_id: [u8; 16],
        amount: u64,
        protocol_fee_bps: u16,
    ) -> Result<()> {
        instructions::create_ride::handler(ctx, ride_id, amount, protocol_fee_bps)
    }

    /// Backend authority assigns a driver to the ride
    pub fn accept_ride(ctx: Context<AcceptRide>) -> Result<()> {
        instructions::accept_ride::handler(ctx)
    }

    /// Backend authority releases escrow: 90% driver, 10% protocol
    pub fn complete_ride(ctx: Context<CompleteRide>) -> Result<()> {
        instructions::complete_ride::handler(ctx)
    }

    /// Cancel ride and refund passenger
    pub fn cancel_ride(ctx: Context<CancelRide>) -> Result<()> {
        instructions::cancel_ride::handler(ctx)
    }
}
