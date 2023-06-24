use anchor_spl::{token_2022, token_interface::TransferChecked};

use crate::*;

pub fn transfer<'a>(
    token_program: &AccountInfo<'a>,
    from: &Account<'a, ta2>,
    to: &Account<'a, ta2>,
    authority: &AccountInfo<'a>,
    auth_seed: &[&[&[u8]]],
    amount: u64,
    mint: &Account<'a, m2>,
    decimals : u8,
) -> anchor_lang::Result<()> {
    let cpi_program = token_program.clone();
    let cpi_accounts = TransferChecked {
        from: from.to_account_info(),
        mint: mint.to_account_info(),
        to: to.to_account_info(),
        authority: authority.clone(),
    };
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, auth_seed);
    anchor_spl::token_2022::transfer_checked(cpi_ctx, amount, decimals)?;
     
    Ok(())
}
