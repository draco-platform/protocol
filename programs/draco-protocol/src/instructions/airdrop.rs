use anchor_lang::prelude::*;

pub use crate::state::airdrop::Airdrop;
pub use crate::state::airdrop_claimed::AirdropClaimed;
use crate::{instructions, state::protocol_authority::ProtocolAuthority};
use crate::state::protocol_error::ErrorCode;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{ Mint, TokenInterface, TokenAccount}};
use crate::instructions::pda_owned_token_accounts;

#[derive(Accounts)]
#[instruction(airdrop_id: u64)]
pub struct CreateAirdrop<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [
            b"airdrop".as_ref(), 
            airdrop_id.to_le_bytes().as_ref()
        ],
        bump,
        space = 8 + Airdrop::INIT_SPACE,
    )]
    pub airdrop: Account<'info, Airdrop>,

    #[account(
        mut,
        seeds = [b"protocol_authority".as_ref()],
        bump
    )]
    pub protocol_authority: Account<'info, ProtocolAuthority>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(airdrop_id: u64)]
pub struct ClaimAirdrop<'info> {
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
        seeds = [b"treasury".as_ref()],
        bump
    )]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            b"airdrop".as_ref(), 
            airdrop_id.to_le_bytes().as_ref()
        ],
        bump,
    )]
    pub airdrop: Account<'info, Airdrop>,

    #[account(
        init,
        payer = payer,
        seeds = [
            b"airdrop_claimed".as_ref(),
            airdrop_id.to_le_bytes().as_ref(), 
            payer.key().to_bytes().as_ref(),
        ],
        bump,
        space = 8 + AirdropClaimed::INIT_SPACE,
    )]
    pub airdrop_claimed: Account<'info, AirdropClaimed>,

    #[account(mut)]
    pub token_mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create(
    ctx: Context<CreateAirdrop>,
    airdrop_id: u64,
    name: String,
    supply: u64,
    amount_per_claim: u64,
    start_datetime: i64,
    end_datetime: i64,
) -> Result<()> {
    require!(
        start_datetime < end_datetime,
        ErrorCode::InvalidAirdropStartEndDatetime
    );
    require!(
        supply > 0,
        ErrorCode::InvalidAirdropSupply
    );
    require!(
        amount_per_claim > 0,
        ErrorCode::InvalidAmount
    );
    require!(
        supply >= amount_per_claim,
        ErrorCode::InvalidAirdropSupply
    );

    let airdrop = &mut ctx.accounts.airdrop;
    airdrop.airdrop_id = airdrop_id;
    airdrop.name = name;
    airdrop.supply = supply;
    airdrop.supplied = 0;
    airdrop.amount_per_claim = amount_per_claim;
    airdrop.start_datetime = start_datetime;
    airdrop.end_datetime = end_datetime;
    Ok(())
}

pub fn claim(
    ctx: Context<ClaimAirdrop>,
    airdrop_id: u64,
) -> Result<()> {
    let airdrop = &mut ctx.accounts.airdrop;
    let current_timestamp = Clock::get()?.unix_timestamp;
    
    require!(
        current_timestamp >= airdrop.start_datetime,
        ErrorCode::AirdropNotStarted
    );
    require!(
        current_timestamp <= airdrop.end_datetime,
        ErrorCode::AirdropEnded
    );
    
    require!(
        airdrop.supplied.checked_add(airdrop.amount_per_claim).unwrap_or(u64::MAX) <= airdrop.supply,
        ErrorCode::AirdropSupplyExhausted
    );
    
    let scaled_amount = instructions::utils::get_scaled_amount(
        airdrop.amount_per_claim,
        ctx.accounts.token_mint.decimals
    )?;
    
    let _ = pda_owned_token_accounts::withdraw(
        scaled_amount,
        &[b"treasury"],
        ctx.bumps.treasury_token_account,
        &ctx.accounts.treasury_token_account,
        &ctx.accounts.payer_token_account,
        &ctx.accounts.token_mint,
        &ctx.accounts.token_program,
    );
    
    airdrop.supplied = airdrop.supplied.checked_add(airdrop.amount_per_claim)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    
    let airdrop_claimed = &mut ctx.accounts.airdrop_claimed;
    airdrop_claimed.airdrop_id = airdrop_id;
    airdrop_claimed.claimer = ctx.accounts.payer.key();

    Ok(())
}