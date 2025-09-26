use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct LotteryRewardFactors {
    pub reward_full_match: f64,
    pub reward_suit_match: f64,
    pub reward_value_match: f64,
    pub suit_streak_bonuses: [f64; 5],
    pub value_streak_bonuses: [f64; 5], 
    pub jackpot_percentage: f64,
    pub max_boost: f64,
    pub curvature: f64,
    pub lock_divider: f64,
}