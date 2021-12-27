use anchor_lang::prelude::*;

use anchor_spl::token::{
    self, 
    TokenAccount, 
    SetAuthority,
    Mint, 
    Transfer, 
    CloseAccount
}
;
use spl_token::state::AccountState;
use spl_token::instruction::AuthorityType;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod collateral {
    use super::*;
    const ESCROW_ACCOUNT: &[u8] = b"escrow";
    pub fn initialize(ctx: Context<Initialize>,
    _vault_acc: u8,
    val_init: u64,
    taker_val: u64
) -> ProgramResult {
        let mut state = ctx.state();
        ctx.accounts.collateral.init_collateral_key = *ctx.accounts.init.key;
        ctx.accounts.collateral.taker_val = taker_val;
        ctx.accounts.collateral.val_init = val_init;
        ctx.accounts.collateral.deposit_acc_key = *ctx.accounts.deposit_acc.to_account_info().key;
        ctx.accounts.collateral.reciever_acc_key = *ctx.accounts.reciever_acc.to_account_info().key;
        ctx.accounts.set_authority(ctx.origin, ctx.origin, AuthorityType::Origin)?;

        let (vault_auth, _vault_authority_bump) = Pubkey::find_program_address(
            &[ESCROW_ACCOUNT]
            &ctx.program_id);
        token::set_authority( ctx.accounts.into_authority_context_add(),
            ctx.origin,
            vault_auth,
            AuthorityType::AccountOwner)?;
        token::transfer(
            ctx.accounts.into_transfer_context(),
            ctx.origin,
            ctx.accounts.collateral.val_init,
        )?;
        Ok(())
    }

    pub fn exchange(ctx: Context<Trade>) -> ProgramResult {
        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>) -> ProgramResult {
        let (vault_auth, _vault_authority_bump) = Pubkey::find_program_address(
            &[ESCROW_ACCOUNT]
            &ctx.program_id);
            let auth_tok = ctx.accounts.collateral.init_collateral_key;
        Ok(())
    }


}

//main acc
#[account]
pub struct Collateral {
    pub init_collateral_key: Pubkey,
    pub taker_val: u64,
    pub val_init: u64,
    // pub vault_acc: u8,
    pub deposit_acc_key: Pubkey,
    pub reciever_acc_key: Pubkey,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub init: AccountInfo<'info>,
    pub collateral: Box<AccountInfo<'info, Collateral>>,
    pub deposit_acc: Account<'info, TokenAccount>,
    pub reciever_acc: Account<'info, TokenAccount>,
    pub vault_acc: Account<'info, TokenAccount>,
    pub mint : Account<'info, Mint>,
    pub rent : Sysvar<'info, Rent>,
    pub system_program_info :AccountInfo<'info>,
    pub token_program_info :AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Trade<'info> {
    pub init: AccountInfo<'info>,
    pub collateral: Box<AccountInfo<'info, Collateral>>,
    pub deposit_acc: Account<'info, TokenAccount>,
    pub vault_acc: Account<'info, TokenAccount>,
    pub vault_auth :AccountInfo<'info>,
    pub token_program_info :AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Cancel<'info> {
    pub init: AccountInfo<'info>,
    pub collateral: Box<AccountInfo<'info, Collateral>>,
    pub deposit_acc: Account<'info, TokenAccount>,
    pub reciever_acc: Account<'info, TokenAccount>,
    pub taker : Account<'info, TokenAccount>,
    pub deposit_acc_taker: Account<'info, TokenAccount>,
    pub reciever_acc_taker: Account<'info, TokenAccount>,
    pub vault_auth :AccountInfo<'info>,
    pub vault_acc: Account<'info, TokenAccount>,
    pub token_program_info :AccountInfo<'info>,
}

