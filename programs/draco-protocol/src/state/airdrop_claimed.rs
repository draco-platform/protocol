use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct AirdropClaimed {
    pub airdrop_id: u64,
    pub claimer: Pubkey,
}