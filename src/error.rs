use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StakeError {
    #[error("Account not initialized yet")]
    UninitializedAccount,

    #[error("PDA derived does not equal PDA passed in")]
    InvalidPDA,

    #[error("Invalid token account")]
    InvalidTokenAccount,

    #[error("Invalid stake account")]
    InvalidStakeAccount,
}

impl From<StakeError> for ProgramError {
    fn from(value: StakeError) -> Self {
        ProgramError::Custom(value as u32)
    }
}
