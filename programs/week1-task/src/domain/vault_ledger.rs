use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct WhitelistRule {
    pub address: Pubkey,
    pub amount: u64,
}

impl WhitelistRule {
    pub const SIZE: usize = 32 + 8;
}

#[account]
pub struct VaultLedger {
    pub admin: Pubkey,
    pub mint: Pubkey,
    pub vault_bump: u8,
    pub config_bump: u8,
    pub whitelist: Vec<WhitelistRule>,
}

impl VaultLedger {
    pub const BASE_SIZE: usize = 8 + 32 + 32 + 1 + 1 + 4;

    pub fn size_with_entries(entries: usize) -> usize {
        Self::BASE_SIZE + entries * WhitelistRule::SIZE
    }
}