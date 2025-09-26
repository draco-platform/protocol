use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct SixMonthCliff {
    pub last_transfer_out_datetime: i64,
    pub transfers_performed: u64
}