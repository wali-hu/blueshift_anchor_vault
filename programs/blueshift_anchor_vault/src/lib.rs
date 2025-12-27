use anchor_lang::prelude::*;

declare_id!("5hf3H8py8w5VjTrCeDMZw7yr6PEkiwhCB67wHdZ2cfp");

#[program]
pub mod blueshift_anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
