use anchor_lang::prelude::*;

use crate::state::protocol_error::ErrorCode;

pub fn enforce_protocol_authority<'info>(
    authority: Pubkey,
    payer_pub_key: Pubkey
) -> Result<()> {
    require!(authority == payer_pub_key, ErrorCode::InvalidAuthority);
    Ok(())
}