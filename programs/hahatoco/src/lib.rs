use anchor_lang::prelude::*;

declare_id!("CiQgCHtk9t7V5C7m1UMKCD7aoz7Egzida1BhVQQ1UnQ4");

#[program]
pub mod hahatoco {
    use anchor_lang::{context::Context, Result};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, authority: Pubkey) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        msg!("Previous Count: {}", counter.count);

        counter.authority = authority;
        counter.count = 0;

        msg!("Counter Incremented");
        msg!("Counter count: {}", counter.count);

        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        msg!("Previous Count: {}", counter.count);

        counter.count += 1;

        msg!("Counter Incremented");
        msg!("Current count: {}", counter.count);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 40)]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, Counter>,

    pub authority: Signer<'info>,
}

#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
}
