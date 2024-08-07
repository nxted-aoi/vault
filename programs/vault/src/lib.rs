use anchor_lang::prelude::*;

declare_id!("2Ci88S9zw9NeH7AP9bc65fop2Mcd5iu1umSWFSq5ufkH");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
