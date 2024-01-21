use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_pack::{IsInitialized, Sealed},
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct StakingAccountState {
    pub is_initialized: bool,
    pub token: Pubkey,
    pub insert_date: i64
}

impl Sealed for StakingAccountState {}

impl IsInitialized for StakingAccountState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
