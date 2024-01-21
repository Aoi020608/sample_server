use solana_program::program_error::ProgramError;

pub enum StakingInstruction {
    InitializeStakeAccount,
    Stake,
    Redeem,
    Unstake,
}

impl StakingInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&varint, _rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match varint {
            0 => Self::InitializeStakeAccount {},
            1 => Self::Stake {},
            2 => Self::Redeem {},
            3 => Self::Unstake {},
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
