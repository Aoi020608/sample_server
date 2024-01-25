use anchor_lang::prelude::*;

declare_id!("9jDRA8bpXP4h1r3DPRJYzJpWJzhCfqrWhf86gL3mN2Nj");

#[program]
pub mod hahatoco {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
