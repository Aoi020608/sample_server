use anchor_lang::prelude::*;

declare_id!("95N6ocYECUTFYczRpqCXznCii1iHAF8pAYbQucuxoYTN");

#[program]
pub mod hahatoco {
    use anchor_lang::{context::Context, Result};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        msg!("Previous Count: {}", counter.count);

        counter.count = counter.count.checked_add(1).unwrap_or(0);

        msg!("Counter Incremented");
        msg!("Counter count: {}", counter.count);

        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        msg!("Previous Count: {}", counter.count);

        counter.count = counter.count.checked_add(1).unwrap_or(0);

        msg!("Counter Incremented");
        msg!("Current count: {}", counter.count);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(init, payer = user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,

    pub user: Signer<'info>,
}

#[account]
pub struct Counter {
    pub count: u64,
}
