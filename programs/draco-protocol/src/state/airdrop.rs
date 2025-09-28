use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Airdrop {
    pub airdrop_id: u64,
    #[max_len(32)]
    pub name: String,
    pub supply: u64,
    pub supplied: u64,
    pub amount_per_claim: u64,
    pub start_datetime: i64,
    pub end_datetime: i64,
}