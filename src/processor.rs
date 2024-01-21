use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh0_10::try_from_slice_unchecked,
    clock::Clock,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction, system_program,
    sysvar::Sysvar,
};

use crate::{
    error::StakeError,
    state::{StakeState, UserStakeInfo},
};

/// This creates a new account where we're going to store state information about the staking
/// process for each user/nft combination. The seeds for this PDA should be the user's public key
/// and the nft's token account.
pub fn process_initialize_stake_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let nft_token_account = next_account_info(account_info_iter)?;
    let stake_state = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (stake_state_pda, bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), nft_token_account.key.as_ref()],
        program_id,
    );
    if stake_state_pda != *stake_state.key {
        msg!("Invaid seeds for PDA");
        return Err(ProgramError::InvalidArgument);
    }

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(UserStakeInfo::SIZE);

    msg!("Creating state account at {:?}", stake_state_pda);
    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            stake_state.key,
            rent_lamports,
            UserStakeInfo::SIZE as u64,
            program_id,
        ),
        &[
            initializer.clone(),
            stake_state.clone(),
            system_program.clone(),
        ],
        &[&[
            initializer.key.as_ref(),
            nft_token_account.key.as_ref(),
            &[bump_seed],
        ]],
    )?;

    let mut account_data =
        try_from_slice_unchecked::<UserStakeInfo>(&stake_state.data.borrow()).unwrap();

    if account_data.is_initialized() {
        msg!("Account already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    account_data.token_account = *nft_token_account.key;
    account_data.user_pubkey = *initializer.key;
    account_data.stake_state = StakeState::Unstaked;
    account_data.is_initialized = true;

    account_data.serialize(&mut &mut stake_state.data.borrow_mut()[..])?;

    Ok(())
}

/// This instruction typically is where the actual staking occurs. However, we're not going to do
/// any real staking at this point. We're just going to update the "state" account to reflect that
/// the token is staked, the time at which it's staked, etc.
pub fn process_stake(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let nft_token_account = next_account_info(account_info_iter)?;
    let stake_state = next_account_info(account_info_iter)?;

    let (stake_state_pda, _bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), nft_token_account.key.as_ref()],
        program_id,
    );
    if stake_state_pda != *stake_state.key {
        msg!("Invaid seeds for PDA");
        return Err(ProgramError::InvalidArgument);
    }

    msg!("unpacking state account");
    let mut account_data =
        try_from_slice_unchecked::<UserStakeInfo>(&stake_state.data.borrow()).unwrap();

    if !account_data.is_initialized() {
        msg!("Account is not initialized");
        return Err(StakeError::UninitializedAccount.into());
    }

    let clock = Clock::get()?;

    account_data.token_account = *nft_token_account.key;
    account_data.user_pubkey = *initializer.key;
    account_data.stake_state = StakeState::Staked;
    account_data.stake_start_time = clock.unix_timestamp;
    account_data.last_stake_redeem = clock.unix_timestamp;
    account_data.is_initialized = true;

    account_data.serialize(&mut &mut stake_state.data.borrow_mut()[..])?;

    Ok(())
}

/// This is where you would send user's their reward tokens based on how long they've been staking.
/// But for now just log how many tokens they should get (you can just assume 1 token per unit of
/// time for now) and update the state to reflect when they last redeemed tokens.
pub fn process_redeem(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let nft_token_account = next_account_info(account_info_iter)?;
    let stake_state = next_account_info(account_info_iter)?;

    let (stake_state_pda, _bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), nft_token_account.key.as_ref()],
        program_id,
    );
    if stake_state_pda != *stake_state.key {
        msg!("Invaid seeds for PDA");
        return Err(ProgramError::InvalidArgument);
    }

    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut account_data =
        try_from_slice_unchecked::<UserStakeInfo>(&stake_state.data.borrow()).unwrap();

    if !account_data.is_initialized() {
        msg!("Account is not initialized");
        return Err(StakeError::UninitializedAccount.into());
    }

    if account_data.stake_state != StakeState::Staked {
        msg!("Stake account is not staking anything");
        return Err(ProgramError::InvalidArgument);
    }

    if *initializer.key != account_data.user_pubkey {
        msg!("Incorrect stake account for user");
        return Err(StakeError::InvalidStakeAccount.into());
    }

    if *nft_token_account.key != account_data.token_account {
        msg!("NFT Token account do not match");
        return Err(StakeError::InvalidTokenAccount.into());
    }

    let clock = Clock::get()?;

    let unix_time = clock.unix_timestamp - account_data.last_stake_redeem;
    let reward = unix_time;
    msg!("Redeeming {} tokens", reward);

    account_data.last_stake_redeem = clock.unix_timestamp;

    account_data.serialize(&mut &mut stake_state.data.borrow_mut()[..])?;
    msg!("state account serialized");

    Ok(())
}

/// This is where you redeem any additional tokens and then unstake the NFT. For now, that just
/// means updating state to reflect that the NFT isn't staked and logging how many reward tokens
/// they should get.
pub fn process_unstake(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let nft_token_account = next_account_info(account_info_iter)?;
    let stake_state = next_account_info(account_info_iter)?;

    let (stake_state_pda, _bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), nft_token_account.key.as_ref()],
        program_id,
    );
    if stake_state_pda != *stake_state.key {
        msg!("Invaid seeds for PDA");
        return Err(ProgramError::InvalidArgument);
    }

    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut account_data =
        try_from_slice_unchecked::<UserStakeInfo>(&stake_state.data.borrow()).unwrap();

    if !account_data.is_initialized() {
        msg!("Account is not initialized");
        return Err(StakeError::UninitializedAccount.into());
    }

    if account_data.stake_state != StakeState::Staked {
        msg!("Stake account is not staking anything");
        return Err(ProgramError::InvalidArgument);
    }
    let clock = Clock::get()?;

    let unix_time = clock.unix_timestamp - account_data.last_stake_redeem;
    let reward = unix_time;
    msg!("Redeeming {} tokens", reward);

    msg!("Setting stake state to unstaked");
    account_data.stake_state = StakeState::Unstaked;
    account_data.serialize(&mut &mut stake_state.data.borrow_mut()[..])?;

    **stake_state.lamports.borrow_mut() = 0;

    // stack exchange https://solana.stackexchange.com/questions/8293/how-can-i-check-if-an-anchor-account-was-closed/8295#8295
    stake_state.assign(&system_program::id());
    stake_state.realloc(0, false)?;

    msg!("Unstaking...");
    Ok(())
}
