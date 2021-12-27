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

        let (vault_auth, _vault_acc_cx) = Pubkey::find_program_address(
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
        let (_vault_auth, vault_acc_cx) = Pubkey::find_program_address(
            &[ESCROW_ACCOUNT]
            &ctx.program_id);
        let seeds = &[&ESCROW_ACCOUNT[..],&[vault_acc_cx]]
        token::transfer(
            ctx.accounts.into_transfer_init_context(),
            ctx.accounts.collateral.taker_val,
        )?;
        token::transfer(
            ctx.accounts.into_taker_transfer_context().with_signer(&[&seeds[..]]),
            ctx.origin,
            ctx.accounts.collateral.val_init,
        )?; 
        token::close_account(
            ctx.accounts.into_close_context().with_signer(&[&seeds[..]]),
        )?;
        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>) -> ProgramResult {
        let (_vault_auth, vault_acc_cx) = Pubkey::find_program_address(
            &[ESCROW_ACCOUNT]
            &ctx.program_id);
        let seeds = &[&ESCROW_ACCOUNT[..],&[vault_acc_cx]]
            &ctx.program_id;
        token::transfer(
            ctx.accounts.into_transfer_init_context().with_signer(&[&seeds[..]]),
            ctx.origin,
            ctx.accounts.collateral.val_init,
        )?;
        token::close_account(
            ctx.accounts.into_close_context().with_signer(&[&seeds[..]]),
        )?;
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
#[instruction(vault_acc_cx: u8, val_init: u64)]
pub struct Initialize<'info> {
    pub init: AccountInfo<'info>,
    #[account(zero)]
    pub collateral: Box<AccountInfo<'info, Collateral>>,
    #[account(mut, constraint = val_init <= deposit_acc.amount)]
    pub deposit_acc: Account<'info, TokenAccount>,
    pub reciever_acc: Account<'info, TokenAccount>,
    #[account(init, payer = init, seeds = [b"token-seed".as_ref()], bump = vault_acc_cx,, token::authority = initializer, token::mint = mint,)]
    pub vault_acc: Account<'info, TokenAccount>,
    pub mint : Account<'info, Mint>,
    pub rent : Sysvar<'info, Rent>,
    pub system_program_info :AccountInfo<'info>,
    pub token_program_info :AccountInfo<'info>,
}

impl<'info> Initialize<'info> {

    pub fn into_transfer_context(&self) -> CpiContext<'_,'_,'_,'info, Transfer<'info>> {
        let acc_cpi =Transfer {
            from: self.deposit_acc.to_account_info().clone(),
            to: self.vault_acc.to_account_info().clone(),
            authority: self.init.clone(),
        };
        CpiContext::new( self.token_program_info.clone() ,acc_cpi)
    }
    pub fn into_authority_context_add(&self) -> CpiContext<'_,'_,'_,'info, SetAuthority<'info>> {
        let acc_cpi = SetAuthority {
            current_authority: self.init.clone(),
            account_or_mint: self.vault_acc.to_account_info().clone(),
        };
        CpiContext::new( self.token_program_info.clone() ,acc_cpi)
    }


}

#[derive(Accounts)]
pub struct Trade<'info> {
    #[account(mut, signer)]
    pub init: AccountInfo<'info>,
    pub collateral: Box<AccountInfo<'info, Collateral>>,
    #[account(mut)]
    pub deposit_acc: Account<'info, TokenAccount>,
    #[account(mut)]
    pub reciever_acc: Account<'info, TokenAccount>,
    #[account(signer)]
    pub taker : Account<'info, TokenAccount>,
    #[account(mut)]
    pub deposit_acc_taker: Account<'info, TokenAccount>,
    #[account(mut)]
    pub reciever_acc_taker: Account<'info, TokenAccount>,
    pub vault_auth :AccountInfo<'info>,
    #[account(mut)]
    pub vault_acc: Account<'info, TokenAccount>,
    pub token_program_info :AccountInfo<'info>
}

impl<'info> Trade<'info> {
    pub fn into_transfer_init_context(&self) -> CpiContext<'_,'_,'_,'info, Transfer<'info>> {
        let acc_cpi =Transfer {
            from: self.deposit_acc_taker.to_account_info().clone(),
            to: self.reciever_acc.to_account_info().clone(),
            authority: self.taker.clone(),
        };
        CpiContext::new( self.token_program_info.clone() ,acc_cpi)
    }
    pub fn into_close_context(&self) -> CpiContext<'_,'_,'_,'info, CloseAccount<'info>> {
        let acc_cpi =CloseAccount {
            account: self.vault_acc.to_account_info().clone(),
            destination: self.init.clone(),
            authority: self.vault_auth.clone(),
        };
        CpiContext::new( self.token_program_info.clone() ,acc_cpi)
    }
    pub fn into_taker_transfer_context(&self) -> CpiContext<'_,'_,'_,'info, Transfer<'info>> {
        let acc_cpi =Transfer {
            from: self.vault_acc.to_account_info().clone(),
            to: self.reciever_acc_taker.to_account_info().clone(),
            authority: self.vault_auth.clone(),
        };
        CpiContext::new( self.token_program_info.clone() ,acc_cpi)
    }


}

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut, signer)]
    pub init: AccountInfo<'info>,
    pub collateral: Box<AccountInfo<'info, Collateral>>,
    #[account(mut)]
    pub deposit_acc: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault_acc: Account<'info, TokenAccount>,
    pub vault_auth :AccountInfo<'info>,
    pub token_program_info :AccountInfo<'info>
}


impl<'info> Cancel<'info> {
    pub fn into_transfer_init_context(&self) -> CpiContext<'_,'_,'_,'info, Transfer<'info>> {
        let acc_cpi =Transfer {
            from: self.vault_acc.to_account_info().clone(),
            to: self.deposit_acc.to_account_info().clone(),
            authority: self.vault_auth.clone(),
        };
        CpiContext::new( self.token_program_info.clone() ,acc_cpi)
    }
    pub fn into_close_context(&self) -> CpiContext<'_,'_,'_,'info, CloseAccount<'info>> {
        let acc_cpi =CloseAccount {
            account: self.vault_acc.to_account_info().clone(),
            destination: self.init.clone(),
            authority: self.vault_auth.clone(),
        };
        CpiContext::new( self.token_program_info.clone() ,acc_cpi)
    }
}

