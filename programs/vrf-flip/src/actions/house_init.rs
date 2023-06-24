use std::ops::Deref;
use spl_token_2022::extension::transfer_fee::instruction::set_transfer_fee;
use spl_token_2022::extension::transfer_fee::instruction::TransferFeeInstruction::SetTransferFee;
use crate::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, MintTo, Mint, Token, TokenAccount},
};
use anchor_spl::token_interface::Token2022;

#[derive(Accounts, anchor_lang::AnchorSerialize, anchor_lang::AnchorDeserialize)]

#[instruction(params: HouseInitParams)] // rpc parameters hint
pub struct HouseInit<'info> {
    #[account(
        init,
        space = 8 + std::mem::size_of::<HouseState>(),
        payer = payer,
        seeds = [HOUSE_SEED],
        bump
    )]
    pub house: AccountLoader<'info, HouseState>,
    /// CHECK:
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,

    pub switchboard_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        constraint = 
            switchboard_queue.load()?.unpermissioned_vrf_enabled == true @ VrfFlipError::OracleQueueRequiresPermissions
    )]
    pub switchboard_queue: AccountLoader<'info, OracleQueueAccountData>,

    #[account(
        mut
    )]
    pub mint: Account<'info, m2>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = house,
    )]
    pub house_vault: Account<'info, ta2>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // SYSTEM ACCOUNTS
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
pub token_program_2022: Program<'info, Token2022>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK:
    #[account(address = solana_program::sysvar::rent::ID)]
    pub rent: AccountInfo<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct HouseInitParams {}

impl HouseInit<'_> {
    pub fn validate(
        &self,
        _ctx: &Context<Self>,
        _params: &HouseInitParams,
    ) -> anchor_lang::Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: &Context<Self>, _params: &HouseInitParams) -> anchor_lang::Result<()> {
        msg!("house_init");

        let house_bump = ctx.bumps.get("house").unwrap().clone();

        let house_seeds: &[&[&[u8]]] = &[&[&HOUSE_SEED, &[house_bump]]];
        if ctx.accounts.mint.mint_authority.is_some()
            && ctx.accounts.mint.mint_authority.unwrap() == ctx.accounts.house.key()
        {
            msg!("minting 100_000_000 tokens to house vault");
            anchor_spl::token_2022::mint_to(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program_2022.to_account_info().clone(),
                    anchor_spl::token_2022::MintTo {
                        mint: ctx.accounts.mint.to_account_info().clone(),
                        authority: ctx.accounts.house.to_account_info().clone(),
                        to: ctx.accounts.house_vault.to_account_info().clone(),
                    },
                    house_seeds,
                ),
                100_000_000_000_000_000,
            )?;
        }


        let cpi_context = SetTransferFee {
            transfer_fee_basis_points: 200,
            maximum_fee: 100_000_000_000_000_000_000_000_000
        };

        let ix = set_transfer_fee(
            &ctx.accounts.token_program_2022.key(),
            &ctx.accounts.mint.key(), &ctx.accounts.house.key(), &[], 65535, 200)?;
        
        let house = &mut ctx.accounts.house.load_init()?;
        house.bump = house_bump;
        house.authority = ctx.accounts.authority.key().clone();
        house.switchboard_mint = ctx.accounts.switchboard_mint.key().clone();
        house.mint = ctx.accounts.mint.key().clone();
        house.switchboard_queue = ctx.accounts.switchboard_queue.key().clone();
        house.house_vault = ctx.accounts.house_vault.key().clone();
        drop(house);

        Ok(())
    }
}
