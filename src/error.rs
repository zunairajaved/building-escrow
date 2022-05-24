// inside error.rs
use thiserror::Error;

use solana_program::program_error::ProgramError;

// use crate::instruction::EscrowInstruction;
use crate::{instruction::EscrowInstruction, error::EscrowError as OtherEscrowError};

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
     /// Not Rent Exempt
     #[error("Not Rent Exempt")]
     NotRentExempt,
     /// Expected Amount Mismatch
     #[error("Expected Amount Mismatch")]
     ExpectedAmountMismatch,
     /// Amount Overflow
     #[error("Amount Overflow")]
     AmountOverflow,
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}