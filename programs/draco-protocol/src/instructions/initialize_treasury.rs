use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{ Mint, TokenInterface, TokenAccount}};

use crate::state::protocol_authority::ProtocolAuthority;
use crate::state::six_month_cliff::SixMonthCliff;

pub use crate::instructions;
pub use crate::constants::{TREASURY_INITIAL_AMOUNT, SIX_MONTH_CLIFF_AMOUNT};
pub use crate::state::protocol_error::ErrorCode;
#[derive(Accounts)]
pub struct InitializeTreasury<'info> {
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
        init,
        seeds = [b"treasury".as_ref()],
        bump,
        payer = payer,
        token::mint = token_mint,
        token::authority = treasury_token_account, // Owned by the PDA from the start
    )]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        seeds = [b"six_month_cliff_treasury".as_ref()],
        bump,
        payer = payer,
        token::mint = token_mint,
        token::authority = six_month_cliff_token_account,
    )]
    pub six_month_cliff_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        space = 8 + SixMonthCliff::INIT_SPACE,
        seeds = [b"six_month_cliff_state".as_ref()],
        bump
    )]
    pub six_month_cliff_state: Account<'info, SixMonthCliff>,

    #[account(mut)]
    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [b"protocol_authority".as_ref()],
        bump
    )]
    pub protocol_authority: Account<'info, ProtocolAuthority>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn contribute_treasury_account(ctx: &Context<InitializeTreasury>) -> Result<()> {
    msg!("Contributing {} DRACO to the treasury", TREASURY_INITIAL_AMOUNT);

    let scaled_amount = instructions::utils::get_scaled_amount(
        TREASURY_INITIAL_AMOUNT, 
        ctx.accounts.token_mint.decimals
    )?;

    let _ = instructions::pda_owned_token_accounts::contribute(
        scaled_amount,
        &ctx.accounts.payer_token_account,
        &ctx.accounts.treasury_token_account,
        &ctx.accounts.payer,
        &ctx.accounts.token_mint,
        &ctx.accounts.token_program,
    );
    msg!("Treasury account contributed successfully");
    Ok(())
}

pub fn contribute_six_month_cliff_account(ctx: Context<InitializeTreasury>) -> Result<()> {
    msg!("Contributing {} DRACO to the six month cliff", SIX_MONTH_CLIFF_AMOUNT * 3 );

    let scaled_amount = instructions::utils::get_scaled_amount(
        SIX_MONTH_CLIFF_AMOUNT * 3,
        ctx.accounts.token_mint.decimals
    )?;

    let _ = instructions::pda_owned_token_accounts::contribute(
        scaled_amount,
        &ctx.accounts.payer_token_account,
        &ctx.accounts.six_month_cliff_token_account,
        &ctx.accounts.payer,
        &ctx.accounts.token_mint,
        &ctx.accounts.token_program,
    );
    ctx.accounts.six_month_cliff_state.last_transfer_out_datetime = Clock::get()?.unix_timestamp;
    ctx.accounts.six_month_cliff_state.transfers_performed = 0;
    msg!("Six month cliff account contributed successfully");

    Ok(())
}