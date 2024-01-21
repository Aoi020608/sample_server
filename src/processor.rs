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

use crate::{error::ReviewError, state::StakingAccountState};

/// This creates a new account where we're going to store state information about the staking
/// process for each user/nft combination. The seeds for this PDA should be the user's public key
/// and the nft's token account.
pub fn initialize_stake_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    token: Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), token.to_bytes().as_ref()],
        program_id,
    );
    if pda != *pda_account.key {
        msg!("Invaid seeds for PDA");
        return Err(ProgramError::InvalidArgument);
    }

    let total_len = 32 + 8;

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(total_len);

    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            total_len as u64,
            program_id,
        ),
        &[
            initializer.clone(),
            pda_account.clone(),
            system_program.clone(),
        ],
        &[&[
            initializer.key.as_ref(),
            token.to_bytes().as_ref(),
            &[bump_seed],
        ]],
    )?;

    msg!("PDA created: {}", pda);

    Ok(())
}

/// This instruction typically is where the actual staking occurs. However, we're not going to do
/// any real staking at this point. We're just going to update the "state" account to reflect that
/// the token is staked, the time at which it's staked, etc.
pub fn stake(program_id: &Pubkey, accounts: &[AccountInfo], token: Pubkey) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;

    if pda_account.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), token.to_bytes().as_ref()],
        program_id,
    );
    if pda != *pda_account.key {
        msg!("Invaid seeds for PDA");
        return Err(ProgramError::InvalidArgument);
    }

    msg!("unpacking state account");
    let mut account_data =
        try_from_slice_unchecked::<StakingAccountState>(&pda_account.data.borrow()).unwrap();

    if !account_data.is_initialized() {
        msg!("Account is not initialized");
        return Err(ReviewError::UninitializedAccount.into());
    }

    let clock = Clock::get()?;

    account_data.is_initialized = true;
    account_data.token = token;
    account_data.insert_date = clock.unix_timestamp;

    msg!("Serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("state account serialized");

    msg!("Staking...");
    msg!("Token: {}", &account_data.token);
    msg!("Insert Date: {}", &account_data.insert_date);

    Ok(())
}

/// This is where you would send user's their reward tokens based on how long they've been staking.
/// But for now just log how many tokens they should get (you can just assume 1 token per unit of
/// time for now) and update the state to reflect when they last redeemed tokens.
pub fn redeem(program_id: &Pubkey, accounts: &[AccountInfo], token: Pubkey) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;

    if pda_account.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), token.to_bytes().as_ref()],
        program_id,
    );
    if pda != *pda_account.key {
        msg!("Invaid seeds for PDA");
        return Err(ProgramError::InvalidArgument);
    }

    msg!("unpacking state account");
    let mut account_data =
        try_from_slice_unchecked::<StakingAccountState>(&pda_account.data.borrow()).unwrap();

    if !account_data.is_initialized() {
        msg!("Account is not initialized");
        return Err(ReviewError::UninitializedAccount.into());
    }

    let clock = Clock::get()?;

    let reward = clock.unix_timestamp - account_data.insert_date;
    msg!("Redeemed: {}", reward);

    account_data.is_initialized = true;
    account_data.token = token;
    account_data.insert_date = clock.unix_timestamp;

    msg!("Serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("state account serialized");

    msg!("Redeemed...");
    msg!("Token: {}", &account_data.token);
    msg!("Insert Date: {}", &account_data.insert_date);
    Ok(())
}

/// This is where you redeem any additional tokens and then unstake the NFT. For now, that just
/// means updating state to reflect that the NFT isn't staked and logging how many reward tokens
/// they should get.
pub fn unstake(program_id: &Pubkey, accounts: &[AccountInfo], token: Pubkey) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;

    if pda_account.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), token.to_bytes().as_ref()],
        program_id,
    );
    if pda != *pda_account.key {
        msg!("Invaid seeds for PDA");
        return Err(ProgramError::InvalidArgument);
    }

    msg!("unpacking state account");
    let mut account_data =
        try_from_slice_unchecked::<StakingAccountState>(&pda_account.data.borrow()).unwrap();

    if !account_data.is_initialized() {
        msg!("Account is not initialized");
        return Err(ReviewError::UninitializedAccount.into());
    }

    let clock = Clock::get()?;

    let reward = clock.unix_timestamp - account_data.insert_date;
    msg!("Redeemed: {}", reward);

    **pda_account.lamports.borrow_mut() = 0;

    // stack exchange https://solana.stackexchange.com/questions/8293/how-can-i-check-if-an-anchor-account-was-closed/8295#8295
    pda_account.assign(&system_program::id());
    pda_account.realloc(0, false)?;

    msg!("Unstaking...");
    Ok(())
}
