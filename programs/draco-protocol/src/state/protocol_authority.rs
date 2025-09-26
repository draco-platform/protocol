use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ProtocolAuthority {
    pub authority: Pubkey,
}
