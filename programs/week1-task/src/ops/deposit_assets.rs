use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Burn, Mint, MintTo, TokenAccount, TokenInterface};

use crate::domain::VaultLedger;
use crate::faults::VaultFault;

#[derive(Accounts)]
pub struct DepositAssetsOp<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        seeds = [b"vault_config"],
        bump = vault_config.config_bump,
        has_one = mint,
    )]
    pub vault_config: Account<'info, VaultLedger>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = depositor,
        token::token_program = token_program,
    )]
    pub depositor_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vault", vault_config.key().as_ref()],
        bump = vault_config.vault_bump,
        token::mint = mint,
        token::authority = vault_config,
        token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> DepositAssetsOp<'info> {
    pub fn deposit_assets(&mut self, amount: u64) -> Result<()> {
        let depositor_address = self.depositor.key();
        let rule = self
            .vault_config
            .whitelist
            .iter()
            .find(|entry| entry.address == depositor_address)
            .ok_or(VaultFault::UserNotWhitelisted)?;

        require!(amount <= rule.amount, VaultFault::TransferLimitExceeded);

        token_interface::burn(
            CpiContext::new(
                self.token_program.to_account_info(),
                Burn {
                    mint: self.mint.to_account_info(),
                    from: self.depositor_token_account.to_account_info(),
                    authority: self.depositor.to_account_info(),
                },
            ),
            amount,
        )?;

        let signer_seeds: &[&[&[u8]]] = &[&[b"vault_config", &[self.vault_config.config_bump]]];

        token_interface::mint_to(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                MintTo {
                    mint: self.mint.to_account_info(),
                    to: self.vault.to_account_info(),
                    authority: self.vault_config.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
        )?;

        Ok(())
    }
}