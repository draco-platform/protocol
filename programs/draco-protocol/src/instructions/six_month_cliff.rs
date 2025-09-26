use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{ Mint, TokenInterface, TokenAccount}};

use crate::state::protocol_authority::ProtocolAuthority;
use crate::state::six_month_cliff::SixMonthCliff;

pub use crate::instructions;
pub use crate::state::protocol_error::ErrorCode;
pub use crate::constants::{SIX_MONTHS, SIX_MONTHS_TRANSFERS_PER_PERIOD, SIX_MONTH_CLIFF_AMOUNT};

#[derive(Accounts)]
pub struct TransferOutFromSixMonthCliff<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = token_mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program
    )]
    pub payer_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"protocol_authority".as_ref()],
        bump
    )]
    pub protocol_authority: Account<'info, ProtocolAuthority>,
    
    #[account(
        mut,
        seeds = [b"six_month_cliff_state".as_ref()],
        bump
    )]
    pub six_month_cliff_state: Account<'info, SixMonthCliff>,

    #[account(
        mut,
        seeds = [b"six_month_cliff_treasury".as_ref()],
        bump
    )]
    pub six_month_cliff_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub token_mint: InterfaceAccount<'info, Mint>,

    
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn transfer_out(ctx: Context<TransferOutFromSixMonthCliff>) -> Result<()> {
    msg!("Transferring out from six month cliff");

    msg!("Checking if it has passed six months or more since the last transfer");
    let current_time = Clock::get()?.unix_timestamp;
    let last_transfer_out_datetime = ctx.accounts.six_month_cliff_state.last_transfer_out_datetime;
    let time_passed = current_time - last_transfer_out_datetime;

    require!(
        time_passed >= SIX_MONTHS, 
        ErrorCode::NotEnoughTimePassed
    );

    require!(
        ctx.accounts.six_month_cliff_state.transfers_performed <= SIX_MONTHS_TRANSFERS_PER_PERIOD, 
        ErrorCode::MaxTransfersPerformed
    );

    ctx.accounts.six_month_cliff_state.last_transfer_out_datetime = current_time;
    ctx.accounts.six_month_cliff_state.transfers_performed += 1;

    msg!("Withdrawing {} DRACO from the six month cliff", SIX_MONTH_CLIFF_AMOUNT);

    let scaled_amount = instructions::utils::get_scaled_amount(
        SIX_MONTH_CLIFF_AMOUNT, 
        ctx.accounts.token_mint.decimals
    )?;

    let _ = instructions::pda_owned_token_accounts::withdraw(
        scaled_amount,
        &[b"six_month_cliff_treasury"],
        ctx.bumps.six_month_cliff_token_account,
        &ctx.accounts.six_month_cliff_token_account,
        &ctx.accounts.payer_token_account,
        &ctx.accounts.token_mint,
        &ctx.accounts.token_program,
    );
    msg!("Six month cliff transfer out successful");
    Ok(())
}