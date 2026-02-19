use anchor_lang::prelude::*;

use crate::domain::{VaultLedger, WhitelistRule};
use crate::faults::VaultFault;

#[derive(Accounts)]
#[instruction(user: Pubkey, amount: u64)]
pub struct AppendWhitelistOp<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault_config"],
        bump = vault_config.config_bump,
        has_one = admin @ VaultFault::UnauthorizedActor,
        realloc = VaultLedger::size_with_entries(vault_config.whitelist.len() + 1),
        realloc::payer = admin,
        realloc::zero = false,
    )]
    pub vault_config: Account<'info, VaultLedger>,

    pub system_program: Program<'info, System>,
}

impl<'info> AppendWhitelistOp<'info> {
    pub fn append(&mut self, user: Pubkey, amount: u64) -> Result<()> {
        require!(
            !self.vault_config.whitelist.iter().any(|entry| entry.address == user),
            VaultFault::DuplicateWhitelistEntry
        );

        self.vault_config.whitelist.push(WhitelistRule { address: user, amount });
        Ok(())
    }
}