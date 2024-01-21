use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    clock::UnixTimestamp,
    program_pack::{IsInitialized, Sealed},
    pubkey::Pubkey,
};

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct UserStakeInfo {
    pub is_initialized: bool,
    pub token_account: Pubkey,
    pub stake_start_time: UnixTimestamp,
    pub last_stake_redeem: UnixTimestamp,
    pub user_pubkey: Pubkey,
    pub stake_state: StakeState,
}

impl UserStakeInfo {
    pub const SIZE: usize = 1 + 32 + 64 + 64 + 32 + 1;
}

impl Sealed for UserStakeInfo {}

impl IsInitialized for UserStakeInfo {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
pub enum StakeState {
    Staked,
    Unstaked,
}
