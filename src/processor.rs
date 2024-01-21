use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh0_10::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::{error::ReviewError, state::MovieAccountState};

const ACCOUNT_LEN: usize = 1000;

pub fn add_movie_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    description: String,
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
        &[initializer.key.as_ref(), title.as_bytes().as_ref()],
        program_id,
    );
    if pda != *pda_account.key {
        msg!("Invaid seeds for PDA");
        return Err(ProgramError::InvalidArgument);
    }

    if rating > 5 || rating < 1 {
        msg!("Rating can not be higher than 5");
        return Err(ReviewError::InvalidRating.into());
    }

    let total_len = 1 + 1 + (4 + title.len()) + (4 + description.len());
    if total_len > ACCOUNT_LEN {
        msg!("Data length is larger than 1000 bytes");
        return Err(ReviewError::InvalidDataLength.into());
    }

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(ACCOUNT_LEN);

    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            ACCOUNT_LEN as u64,
            program_id,
        ),
        &[
            initializer.clone(),
            pda_account.clone(),
            system_program.clone(),
        ],
        &[&[
            initializer.key.as_ref(),
            title.as_bytes().as_ref(),
            &[bump_seed],
        ]],
    )?;

    msg!("PDA created: {}", pda);

    msg!("unpacking state account");
    let mut account_data =
        try_from_slice_unchecked::<MovieAccountState>(&pda_account.data.borrow()).unwrap();

    account_data.title = title;
    account_data.rating = rating;
    account_data.description = description;
    account_data.is_initialized = true;

    msg!("Serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("state account serialized");

    msg!("Adding movie review...");
    msg!("Title: {}", &account_data.title);
    msg!("Rating: {}", rating);
    msg!("Description: {}", &account_data.description);

    Ok(())
}

pub fn update_movie_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    description: String,
) -> ProgramResult {
    msg!("Updating movie review...");

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

    let (pda, _bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), title.as_bytes().as_ref()],
        program_id,
    );
    if pda != *pda_account.key {
        msg!("Invaid seeds for PDA");
        return Err(ProgramError::InvalidArgument);
    }

    if rating > 5 || rating < 1 {
        msg!("Rating can not be higher than 5");
        return Err(ReviewError::InvalidRating.into());
    }

    let total_len = 1 + 1 + (4 + title.len()) + (4 + description.len());
    if total_len > ACCOUNT_LEN {
        msg!("Data length is larger than 1000 bytes");
        return Err(ReviewError::InvalidDataLength.into());
    }

    msg!("unpacking state account");
    let mut account_data =
        try_from_slice_unchecked::<MovieAccountState>(&pda_account.data.borrow()).unwrap();

    if !account_data.is_initialized() {
        msg!("Account is not initialized");
        return Err(ReviewError::UninitializedAccount.into());
    }

    account_data.title = title;
    account_data.rating = rating;
    account_data.description = description;

    msg!("Serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("state account serialized");

    msg!("Updating movie review...");
    msg!("Title: {}", &account_data.title);
    msg!("Rating: {}", rating);
    msg!("Description: {}", &account_data.description);

    Ok(())
}
