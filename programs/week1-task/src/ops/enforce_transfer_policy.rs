use std::cell::RefMut;

use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::spl_token_2022::{
        extension::{
            transfer_hook::TransferHookAccount,
            BaseStateWithExtensionsMut,
            PodStateWithExtensionsMut,
        },
        pod::PodAccount,
    },
    token_interface::{Mint, TokenAccount},
};

use crate::domain::VaultLedger;
use crate::faults::VaultFault;

#[derive(Accounts)]
pub struct EnforceTransferPolicyOp<'info> {
    #[account(
        token::mint = mint,
        token::authority = owner,
    )]
    pub source_token: InterfaceAccount<'info, TokenAccount>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        token::mint = mint,
    )]
    pub destination_token: InterfaceAccount<'info, TokenAccount>,

    pub owner: UncheckedAccount<'info>,

    #[account(
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,

    #[account(
        seeds = [b"vault_config"],
        bump = vault_config.config_bump,
    )]
    pub vault_config: Account<'info, VaultLedger>,
}

impl<'info> EnforceTransferPolicyOp<'info> {
    pub fn enforce(&mut self, amount: u64) -> Result<()> {
        self.assert_transfer_in_progress()?;

        if self.owner.key() == self.vault_config.key() {
            return Ok(());
        }

        let sender = self.owner.key();
        let rule = self
            .vault_config
            .whitelist
            .iter()
            .find(|entry| entry.address == sender)
            .ok_or(VaultFault::UserNotWhitelisted)?;

        require!(amount <= rule.amount, VaultFault::TransferLimitExceeded);
        Ok(())
    }

    fn assert_transfer_in_progress(&mut self) -> Result<()> {
        let source_info = self.source_token.to_account_info();
        let mut account_data: RefMut<&mut [u8]> = source_info.try_borrow_mut_data()?;

        let mut unpacked = PodStateWithExtensionsMut::<PodAccount>::unpack(*account_data)?;
        let transfer_extension = unpacked.get_extension_mut::<TransferHookAccount>()?;

        require!(bool::from(transfer_extension.transferring), VaultFault::TransferHookNotActive);
        Ok(())
    }
}