# Week 1 Transfer Hook Vault - Architecture

## Overview

Week 1 Transfer Hook Vault implements a Token-2022 transfer-hook extension that validates transfers based on whitelist membership. Every token transfer is intercepted and verified before execution.

![Architecture Diagram](./assets/week2transferhookvault.png)

## Accounts

| Account | Purpose |
|---------|----------|
| **Vault Config PDA** | Stores mint reference and validation rules |
| **Mint** | Token-2022 with transfer-hook extension |
| **Vault Token Account** | Holds tokens; owned by Vault Config |
| **Extra Account Meta List** | TLV-encoded validation metadata |
| **Mint Signer PDA** | Provides mint authority |

## User Stories

### Initialize Transfer-Hook Protected Mint
As a token issuer, I need a Token-2022 mint with built-in transfer validation.
- Mint created with transfer-hook extension
- Vault Config PDA linked to mint
- Vault token account initialized
- Transfer restrictions configured

### Manage Whitelist and Limits
As a vault operator, I need to control who can transfer and in what amounts.
- Add/remove senders from whitelist
- Configure transfer amount limits
- Changes apply to next transfer

### Validate Transfers Automatically
As the system, I need to verify each transfer before allowing it.
- Transfer-hook invoked by Token-2022 automatically
- Sender whitelist checked
- Transfer amount verified against limits
- Transfer rejected if constraints fail
- No extra transaction required

### Deposit and Withdraw from Vault
As a user, I need to deposit tokens to vault and withdraw later.
- Whitelisted users can deposit
- Vault owner can withdraw anytime
- Deposits subject to transfer validation
- Vault tracks all balances

## Core Instructions

| Instruction | Action |
|------------|--------|
| **Bootstrap Vault** | Create mint, vault config, Token-2022 extensions |
| **Configure Meta** | Set up TLV metadata for validation |
| **Enforce Policy** | Called by Token-2022 during transfers (auto-invoked) |

## Technology Stack

- **Blockchain**: Solana
- **Framework**: Anchor 0.32.1
- **Token Standard**: Token-2022
- **Hook Interface**: spl-transfer-hook-interface
