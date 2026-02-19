use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Burn, Mint, MintTo, TokenAccount, TokenInterface};

use crate::domain::VaultLedger;
use crate::faults::VaultFault;

#[derive(Accounts)]
#[instruction(amount: u64, user: Pubkey)]
pub struct WithdrawAssetsOp<'info> {
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
        seeds = [b"vault", vault_config.key().as_ref()],
        bump = vault_config.vault_bump,
        token::mint = mint,
        token::authority = vault_config,
        token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint,
        token::token_program = token_program,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> WithdrawAssetsOp<'info> {
    pub fn withdraw_assets(&mut self, amount: u64) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[b"vault_config", &[self.vault_config.config_bump]]];

        token_interface::burn(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                Burn {
                    mint: self.mint.to_account_info(),
                    from: self.vault.to_account_info(),
                    authority: self.vault_config.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
        )?;

        token_interface::mint_to(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                MintTo {
                    mint: self.mint.to_account_info(),
                    to: self.user_token_account.to_account_info(),
                    authority: self.vault_config.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
        )?;

        Ok(())
    }
}