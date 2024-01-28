use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

declare_id!("5gJBdt8qtYxCDQ4LnXgYPXgq9FmtDGNaweKZojWkem5H");

#[program]
pub mod hahatoco {
    use anchor_spl::token;
    use mpl_token_metadata::instructions::CreateV1Builder;
    use solana_program::program::invoke_signed;

    use super::*;

    pub fn add_movie_review(
        ctx: Context<AddMovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        msg!("Movie Review Account Created");
        msg!("Title: {}", title);
        msg!("Description: {}", description);
        msg!("Rating: {}", rating);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.reviewer = ctx.accounts.initializer.key();
        movie_review.rating = rating;
        movie_review.title = title;
        movie_review.description = description;

        Ok(())
    }

    pub fn update_movie_review(
        ctx: Context<UpdateMovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        msg!("Update Review Account Created");
        msg!("Title: {}", title);
        msg!("Description: {}", description);
        msg!("Rating: {}", rating);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.rating = rating;
        movie_review.description = description;

        Ok(())
    }

    pub fn close(_ctx: Context<Close>) -> Result<()> {
        Ok(())
    }

    pub fn create_reward_mint(
        ctx: Context<CreateTokenReward>,
        uri: String,
        name: String,
        symbol: String,
    ) -> Result<()> {
        msg!("Create Reward Token");

        let seeds = &["mint".as_bytes(), &[ctx.bumps.reward_mint]];

        let signer = [&seeds[..]];
        let cpi_program = ctx.accounts.token_program;
        let cpi_accounts = token::InitializeMint {
            mint: ctx.accounts.reward_mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program.to_account_info(), cpi_accounts);
        token::initialize_mint(cpi_ctx, 6, &ctx.accounts.user.key(), None);

        let account_info = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.reward_mint.to_account_info(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        let create_ix = CreateV1Builder::new()
            .metadata(ctx.accounts.metadata.key())
            .mint(ctx.accounts.reward_mint.key(), true)
            .authority(ctx.accounts.user.key())
            .payer(ctx.accounts.user.key())
            .update_authority(ctx.accounts.user.key(), false)
            .name(name)
            .symbol(symbol)
            .uri(uri)
            .instruction();

        invoke_signed(&create_ix, &account_info, &signer)?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String, description: String, rating: u8)]
pub struct AddMovieReview<'info> {
    #[account(
        init,
        seeds = [title.as_bytes().as_ref(), initializer.key.as_ref()],
        bump,
        payer = initializer,
        space = 8 + 32 + 1 + 4 + title.len() + 4 + description.len()
    )]
    pub movie_review: Account<'info, MovieAccountState>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String, description: String, rating: u8)]
pub struct UpdateMovieReview<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes().as_ref(), initializer.key.as_ref()],
        bump,
        realloc = 8 + 32 + 1 + 4 + title.len() + 4 + description.len(),
        realloc::payer = initializer,
        realloc::zero = true,
    )]
    pub movie_review: Account<'info, MovieAccountState>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut, close = reviewer, has_one = reviewer)]
    movie_review: Account<'info, MovieAccountState>,

    #[account(mut)]
    reviewer: Signer<'info>,
}

#[derive(Accounts)]
pub struct CreateTokenReward<'info> {
    #[account(
        init,
        seeds = ["mint".as_bytes()],
        bump,
        payer = user,
        space = Mint::LEN
    )]
    pub reward_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,

    pub token_program: Program<'info, Token>,
}

#[account]
pub struct MovieAccountState {
    pub reviewer: Pubkey,
    pub rating: u8,
    pub title: String,
    pub description: String,
}
