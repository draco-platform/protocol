use anchor_lang::prelude::*;

pub use crate::state::protocol_authority::ProtocolAuthority;

#[derive(Accounts)]
pub struct InitializeAuthority<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + ProtocolAuthority::INIT_SPACE,
        seeds = [b"protocol_authority".as_ref()],
        bump
    )]
    pub protocol_authority: Account<'info, ProtocolAuthority>,
    pub system_program: Program<'info, System>,
}

pub fn set_protocol_authority(ctx: Context<InitializeAuthority>) -> Result<()> {
    msg!("Initializing DRACO protocol and defining its authority {:?}", ctx.program_id);
    ctx.accounts.protocol_authority.authority = ctx.accounts.payer.key();
    msg!("ProtocolAuthority initialized successfully");
    Ok(())
}

