use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Ticket {
    pub lottery_id: u64,
    #[max_len(8)]
    pub combination: String,
    pub participant: Pubkey,
    pub amount: u64,
    pub is_claimed: bool,
    pub is_initialized: bool,
}