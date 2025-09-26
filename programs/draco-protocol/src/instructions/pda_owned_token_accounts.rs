use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenInterface, TokenAccount, TransferChecked
};

pub fn contribute<'info>(
    amount: u64,
    sender_token_account: &InterfaceAccount<'info, TokenAccount>,
    pda_owned_token_account: &InterfaceAccount<'info, TokenAccount>,
    payer: &Signer<'info>,
    token_mint: &InterfaceAccount<'info, Mint>,
    token_program: &Interface<'info, TokenInterface>,
) -> Result<()> {
    let cpi_accounts = TransferChecked{
        from: sender_token_account.to_account_info(),
        to: pda_owned_token_account.to_account_info(),
        authority: payer.to_account_info(),
        mint: token_mint.to_account_info(),
    };

    let cpi_program = token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

    transfer_checked(
        cpi_context, 
        amount,
        token_mint.decimals,
    )?;

    Ok(())
}

pub fn withdraw<'info>(
    amount: u64,
    pda_seeds: &[&[u8]],
    pda_bump: u8,
    pda_owned_token_account: &InterfaceAccount<'info, TokenAccount>,
    recipient_token_account: &InterfaceAccount<'info, TokenAccount>,
    token_mint: &InterfaceAccount<'info, Mint>,
    token_program: &Interface<'info, TokenInterface>,
) -> Result<()> {
    let bump_slice = [pda_bump];
    let mut seeds_with_bump = pda_seeds.to_vec();
    seeds_with_bump.push(&bump_slice);
    let signer_seeds: &[&[&[u8]]] = &[&seeds_with_bump];

    let cpi_accounts = TransferChecked{
        mint: token_mint.to_account_info(),
        from: pda_owned_token_account.to_account_info(),
        to: recipient_token_account.to_account_info(),
        authority: pda_owned_token_account.to_account_info(),
    };

    let cpi_program = token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(signer_seeds);

    transfer_checked(
        cpi_context,
        amount,
        token_mint.decimals,
    )?;

    Ok(())
}