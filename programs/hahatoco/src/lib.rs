use anchor_lang::prelude::*;

declare_id!("9jDRA8bpXP4h1r3DPRJYzJpWJzhCfqrWhf86gL3mN2Nj");

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
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Counter {
    pub count: u64,
}
