use anchor_lang::prelude::*;

/// PDA seeds: ["ride", ride_uuid_bytes]
#[account]
pub struct RideEscrow {
    /// UUID bytes of the ride
    pub ride_id: [u8; 16],
    /// Passenger who deposited SOL
    pub passenger: Pubkey,
    /// Assigned driver (Pubkey::default() if none yet)
    pub driver: Pubkey,
    /// Total fare in lamports
    pub amount: u64,
    /// Protocol fee in basis points (1000 = 10%)
    pub protocol_fee_bps: u16,
    /// Treasury wallet that receives protocol fee
    pub protocol_wallet: Pubkey,
    /// Current escrow status
    pub status: EscrowStatus,
    /// Unix timestamp when created
    pub created_at: i64,
    /// PDA bump seed
    pub bump: u8,
}

impl RideEscrow {
    /// Account size: 8 (discriminator) + 16 + 32 + 32 + 8 + 2 + 32 + 1 + 8 + 1 = 140
    pub const LEN: usize = 8 + 16 + 32 + 32 + 8 + 2 + 32 + 1 + 8 + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum EscrowStatus {
    /// SOL deposited, waiting for driver
    Created,
    /// Driver accepted, ride in progress
    Active,
    /// Funds released to driver + protocol
    Completed,
    /// Funds returned to passenger
    Cancelled,
    /// Frozen, pending resolution
    Disputed,
}
