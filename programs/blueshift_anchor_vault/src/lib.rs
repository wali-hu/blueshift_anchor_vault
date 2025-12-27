use anchor_lang::prelude::*;

// declare_id!("5hf3H8py8w5VjTrCeDMZw7yr6PEkiwhCB67wHdZ2cfp");
declare_id!("22222222222222222222222222222222222222222222");

#[program]
pub mod blueshift_anchor_vault {
    use super::*;

    pub fn deposit(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn withdraw(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct VaultAction<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut,
    seeds = [b"vault_account", signer.key().as_ref()],
    bump,
    )]
    pub vault_account: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum VaultError {
    #[msg("Vault already exists.")]
    VaultAlreadyExists,

    #[msg("Invalid amount.")]
    InvalidAmount,
}
