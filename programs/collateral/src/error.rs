use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Clone, Error, Copy, Clone)]
pub enum ErrorCollateral {
    #[error("Instruction error: Not a valid instruction")]
    InvalidInstruction,
    #[error("Error: Not enough tokens")]
    NotEnoughTokens,
    #[error("Error: Overflow")]
    AmountOverflow,
    #[error("Error: Not enough lamports")]
    NotEnoughLamports,
    #[error("Error: Incorrect amount")]
    ExpectedAmountMismatch,
    #[error("Error: Lamport amount is less than the minimum rent exemption")]
    NotRentExempt,
    #[error("Error: Not a valid mint")]
    InvalidMint,
}

impl From<ErrorCollateral> for ProgramError {
    pub fn from(err: ErrorCollateral) -> Self {
        ProgramError::Custom(err as u32)
    }
}