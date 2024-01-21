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
        StakingInstruction::InitializeStakeAccount {} => {
            processor::process_initialize_stake_account(program_id, accounts)
        }
        StakingInstruction::Stake {} => processor::process_stake(program_id, accounts),
        StakingInstruction::Redeem {} => processor::process_redeem(program_id, accounts),
        StakingInstruction::Unstake {} => processor::process_stake(program_id, accounts),
    }
}
