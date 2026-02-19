use anchor_lang::prelude::*;

use crate::domain::VaultLedger;
use crate::faults::VaultFault;

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct PruneWhitelistOp<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault_config"],
        bump = vault_config.config_bump,
        has_one = admin @ VaultFault::UnauthorizedActor,
        constraint = !vault_config.whitelist.is_empty() @ VaultFault::WhitelistEntryMissing,
        realloc = VaultLedger::size_with_entries(vault_config.whitelist.len() - 1),
        realloc::payer = admin,
        realloc::zero = false,
    )]
    pub vault_config: Account<'info, VaultLedger>,

    pub system_program: Program<'info, System>,
}

impl<'info> PruneWhitelistOp<'info> {
    pub fn prune(&mut self, user: Pubkey) -> Result<()> {
        let entry_index = self
            .vault_config
            .whitelist
            .iter()
            .position(|entry| entry.address == user)
            .ok_or(VaultFault::WhitelistEntryMissing)?;

        self.vault_config.whitelist.remove(entry_index);
        Ok(())
    }
}