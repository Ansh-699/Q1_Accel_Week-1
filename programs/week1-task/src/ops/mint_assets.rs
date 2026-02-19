use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, MintTo, TokenAccount, TokenInterface};

use crate::domain::VaultLedger;
use crate::faults::VaultFault;

#[derive(Accounts)]
pub struct MintAssetsOp<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        seeds = [b"vault_config"],
        bump = vault_config.config_bump,
        has_one = admin @ VaultFault::UnauthorizedActor,
        has_one = mint,
    )]
    pub vault_config: Account<'info, VaultLedger>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::token_program = token_program,
    )]
    pub destination: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> MintAssetsOp<'info> {
    pub fn mint_assets(&mut self, amount: u64) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[b"vault_config", &[self.vault_config.config_bump]]];

        token_interface::mint_to(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                MintTo {
                    mint: self.mint.to_account_info(),
                    to: self.destination.to_account_info(),
                    authority: self.vault_config.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
        )?;

        Ok(())
    }
}