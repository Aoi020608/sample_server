use borsh::BorshDeserialize;
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

pub enum StakingInstruction {
    InitializeStakeAccount { token: Pubkey },
    Stake { token: Pubkey },
    Redeem { token: Pubkey },
    Unstake { token: Pubkey },
}

impl StakingInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&varint, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        let payload = StakingPayload::try_from_slice(rest).unwrap();

        Ok(match varint {
            0 => Self::InitializeStakeAccount {
                token: payload.token,
            },
            1 => Self::Stake {
                token: payload.token,
            },
            2 => Self::Redeem {
                token: payload.token,
            },
            3 => Self::Unstake {
                token: payload.token,
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}

#[derive(BorshDeserialize)]
struct StakingPayload {
    token: Pubkey,
}
