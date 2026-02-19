use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::domain::VaultLedger;

#[derive(Accounts)]
pub struct BootstrapVaultOp<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = VaultLedger::size_with_entries(0),
        seeds = [b"vault_config"],
        bump,
    )]
    pub vault_config: Account<'info, VaultLedger>,

    #[account(
        init,
        payer = admin,
        mint::decimals = 9,
        mint::authority = vault_config,
        mint::token_program = token_program,
        extensions::transfer_hook::authority = vault_config,
        extensions::transfer_hook::program_id = crate::ID,
        extensions::metadata_pointer::authority = vault_config,
        extensions::metadata_pointer::metadata_address = mint,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = admin,
        seeds = [b"vault", vault_config.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = vault_config,
        token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> BootstrapVaultOp<'info> {
    pub fn bootstrap(&mut self, bumps: &BootstrapVaultOpBumps) -> Result<()> {
        self.vault_config.set_inner(VaultLedger {
            admin: self.admin.key(),
            mint: self.mint.key(),
            vault_bump: bumps.vault,
            config_bump: bumps.vault_config,
            whitelist: Vec::new(),
        });

        Ok(())
    }
}