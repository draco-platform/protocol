use anchor_lang::prelude::*;

use crate::constants::{INITIAL_REWARD_FULL_MATCH, INITIAL_REWARD_SUIT_MATCH, INITIAL_REWARD_VALUE_MATCH, INITIAL_SUIT_STREAK_BONUSES, INITIAL_VALUE_STREAK_BONUSES, INITIAL_JACKPOT_PERCENTAGE, INITIAL_MAX_BOOST, INITIAL_CURVATURE, INITIAL_LOCK_DIVIDER};
use crate::state::protocol_authority::ProtocolAuthority;
use crate::state::lottery_reward_factors::LotteryRewardFactors;

#[derive(Accounts)]
pub struct InitializeLotteryRewardFactors<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"protocol_authority".as_ref()],
        bump
    )]
    pub protocol_authority: Account<'info, ProtocolAuthority>,

    #[account(
        init, 
        payer = payer, 
        seeds = [b"lottery_reward_factors".as_ref()],
        space = 8 + LotteryRewardFactors::INIT_SPACE,
        bump,
    )]
    pub lottery_reward_factors: Account<'info, LotteryRewardFactors>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateLotteryRewardFactors<'info> {

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"protocol_authority".as_ref()],
        bump
    )]
    pub protocol_authority: Account<'info, ProtocolAuthority>,

    #[account(
        mut,
        seeds = [b"lottery_reward_factors".as_ref()],
        bump
    )]
    pub lottery_reward_factors: Account<'info, LotteryRewardFactors>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_lottery(ctx: Context<InitializeLotteryRewardFactors>) -> Result<()> {
    msg!("Initializing lottery reward factors");
    let lottery_reward_factors = &mut ctx.accounts.lottery_reward_factors;
    lottery_reward_factors.reward_full_match = INITIAL_REWARD_FULL_MATCH;
    lottery_reward_factors.reward_suit_match = INITIAL_REWARD_SUIT_MATCH;
    lottery_reward_factors.reward_value_match = INITIAL_REWARD_VALUE_MATCH;
    lottery_reward_factors.suit_streak_bonuses = INITIAL_SUIT_STREAK_BONUSES;
    lottery_reward_factors.value_streak_bonuses = INITIAL_VALUE_STREAK_BONUSES;
    lottery_reward_factors.jackpot_percentage = INITIAL_JACKPOT_PERCENTAGE;
    lottery_reward_factors.max_boost = INITIAL_MAX_BOOST;
    lottery_reward_factors.curvature = INITIAL_CURVATURE;
    lottery_reward_factors.lock_divider = INITIAL_LOCK_DIVIDER;
    Ok(())
}

pub fn update_lottery(ctx: Context<UpdateLotteryRewardFactors>, 
    reward_full_match: f64, 
    reward_suit_match: f64, 
    reward_value_match: f64, 
    suit_streak_bonuses: [f64; 5], 
    value_streak_bonuses: [f64; 5], 
    jackpot_percentage: f64, 
    max_boost: f64, 
    curvature: f64,
    lock_divider: f64,
) -> Result<()> {
    msg!("Updating lottery reward factors");
    let lottery_reward_factors = &mut ctx.accounts.lottery_reward_factors;
    lottery_reward_factors.reward_full_match = reward_full_match;
    lottery_reward_factors.reward_suit_match = reward_suit_match;
    lottery_reward_factors.reward_value_match = reward_value_match;
    lottery_reward_factors.suit_streak_bonuses = suit_streak_bonuses;
    lottery_reward_factors.value_streak_bonuses = value_streak_bonuses;
    lottery_reward_factors.jackpot_percentage = jackpot_percentage;
    lottery_reward_factors.max_boost = max_boost;
    lottery_reward_factors.curvature = curvature;
    lottery_reward_factors.lock_divider = lock_divider;
    Ok(())
}