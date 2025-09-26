use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct LotteryState {
    pub lottery_id: u64,
    #[max_len(32)]
    pub lottery_name: String,
    #[max_len(1024)]
    pub lottery_description: String,
    pub lottery_type: u8,
    pub lottery_start_datetime: i64,
    pub lottery_end_datetime: i64,
    pub is_closed: bool,
    pub initial_prize_pool: u64,
    pub accumulated_prize_pool: u64,
    pub participants_count: u64,
    // If lottery type is PAY_LOTTERY_TYPE, this is ticket price
    // If lottery type is LOCK_LOTTERY_TYPE, this is the minimum amount of tokens to lock the ticker
    pub min_tokens_per_participant: u64,
    #[max_len(8)]
    pub winning_combination: Option<String>,
    pub randomness_account: Pubkey,
}
    