#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

mod domain;
mod faults;
mod ops;

use ops::*;

use spl_discriminator::SplDiscriminate;
use spl_transfer_hook_interface::instruction::ExecuteInstruction;
use spl_tlv_account_resolution::state::ExtraAccountMetaList;

declare_id!("75rznRBCfaY7do322oxyeEpcDf73xskqx8D7rTkYE66c");

#[program]
pub mod week1_transfer_hook_vault {
    use super::*;

    pub fn initialize(ctx: Context<BootstrapVaultOp>) -> Result<()> {
        ctx.accounts.bootstrap(&ctx.bumps)
    }

    pub fn add_to_whitelist(ctx: Context<AppendWhitelistOp>, user: Pubkey, amount: u64) -> Result<()> {
        ctx.accounts.append(user, amount)
    }

    pub fn remove_from_whitelist(ctx: Context<PruneWhitelistOp>, user: Pubkey) -> Result<()> {
        ctx.accounts.prune(user)
    }

    pub fn mint_tokens(ctx: Context<MintAssetsOp>, amount: u64) -> Result<()> {
        ctx.accounts.mint_assets(amount)
    }

    pub fn deposit(ctx: Context<DepositAssetsOp>, amount: u64) -> Result<()> {
        ctx.accounts.deposit_assets(amount)
    }

    pub fn withdraw(ctx: Context<WithdrawAssetsOp>, amount: u64, _user: Pubkey) -> Result<()> {
        ctx.accounts.withdraw_assets(amount)
    }

    pub fn initialize_extra_account_metas(ctx: Context<ConfigureTransferHookMetaOp>) -> Result<()> {
        let extra_account_metas = ConfigureTransferHookMetaOp::build_meta_layout()?;

        ExtraAccountMetaList::init::<ExecuteInstruction>(
            &mut ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?,
            &extra_account_metas,
        )
        .map_err(|_| error!(crate::faults::VaultFault::MetaInitializationFailed))?;

        Ok(())
    }

    #[instruction(discriminator = ExecuteInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn transfer_hook(ctx: Context<EnforceTransferPolicyOp>, amount: u64) -> Result<()> {
        ctx.accounts.enforce(amount)
    }
}
