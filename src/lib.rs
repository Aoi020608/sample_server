use instruction::StakingInstruction;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = StakingInstruction::unpack(instruction_data)?;

    match instruction {
        StakingInstruction::InitializeStakeAccount { token } => {
            processor::initialize_stake_account(program_id, accounts, token)
        }
        StakingInstruction::Stake { token } => processor::stake(program_id, accounts, token),
        StakingInstruction::Redeem { token } => processor::redeem(program_id, accounts, token),
        StakingInstruction::Unstake { token } => processor::unstake(program_id, accounts, token),
    }
}
