use anchor_lang::prelude::*;

declare_id!("KoRiProtocol11111111111111111111111111111111");

#[program]
pub mod kori_protocol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.locked = true;
        msg!("Kori Protocol Vault Initialized");
        Ok(())
    }

    pub fn release_funds(ctx: Context<ReleaseFunds>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.locked = false;
        msg!("Funds Released to Guide");
        Ok(())
    }
}

#[account]
pub struct Vault {
    pub locked: bool,
    pub authority: Pubkey,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 40)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReleaseFunds<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
}
