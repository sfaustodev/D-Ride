use anchor_lang::prelude::*;

#[error_code]
pub enum DRideError {
    #[msg("Invalid amount: must be greater than zero")]
    InvalidAmount,

    #[msg("Protocol fee too high: maximum 50%")]
    FeeTooHigh,

    #[msg("Invalid escrow status for this operation")]
    InvalidStatus,

    #[msg("Unauthorized: signer is not the expected authority")]
    Unauthorized,

    #[msg("Unauthorized: signer is not the passenger")]
    NotPassenger,

    #[msg("Driver already assigned to this escrow")]
    DriverAlreadyAssigned,

    #[msg("No driver assigned to this escrow")]
    NoDriverAssigned,

    #[msg("Arithmetic overflow")]
    Overflow,
}
