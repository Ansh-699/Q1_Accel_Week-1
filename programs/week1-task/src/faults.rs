use anchor_lang::prelude::*;

#[error_code]
pub enum VaultFault {
    #[msg("User is not whitelisted")]
    UserNotWhitelisted,
    #[msg("Transfer amount exceeds whitelist limit")]
    TransferLimitExceeded,
    #[msg("User is already whitelisted")]
    DuplicateWhitelistEntry,
    #[msg("User not found in whitelist")]
    WhitelistEntryMissing,
    #[msg("Unauthorized: only admin can perform this action")]
    UnauthorizedActor,
    #[msg("The transfer hook was not invoked during a transfer")]
    TransferHookNotActive,
    #[msg("Failed to initialize transfer hook metadata")]
    MetaInitializationFailed,
}