use crate::*;

const AIRDROP_AMOUNT: u64 = 1_000_000_000;
const INITIAL_AIRDROP_AMOUNT: u64 = 10 * 1_000_000_000;

#[derive(Accounts)]
#[instruction(params: UserWithdrawParams)] // rpc parameters hint
pub struct UserWithdraw<'info> {
    #[account(
        mut,
        seeds = [
            USER_SEED, 
            house.key().as_ref(), 
            authority.key().as_ref()
        ],
        bump = user.load()?.bump,
        has_one = house,
        has_one = authority,
    )]
    pub user: AccountLoader<'info, UserState>,
    #[account(
        seeds = [HOUSE_SEED, mint.key().as_ref()],
        bump = house.load()?.bump,
        has_one = house_vault,
        has_one = mint,
    )]
    pub house: AccountLoader<'info, HouseState>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = house,
    )]
    pub house_vault: Account<'info, TokenAccount>,
    /// CHECK:
    #[account(
        mut,
    )]
    pub mint: Account<'info, Mint>,
    /// CHECK:
    #[account(mut)]
    pub authority: AccountInfo<'info>,
    /// CHECK:
    #[account(
        mut,
        token::mint = house.load()?.mint,
        token::authority = authority,
    )]
    pub airdrop_token_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UserWithdrawParams {
    pub amount: u64,
}

impl UserWithdraw<'_> {
    pub fn validate(
        &self,
        ctx: &Context<Self>,
        params: &UserWithdrawParams,
    ) -> anchor_lang::Result<()> {
       let user = ctx.accounts.user.load()?;
       if user.deposited < params.amount {
        return Err(error!(VrfFlipError::InvalidBet));

    }
        Ok(())
    }

    pub fn actuate(ctx: &Context<Self>, params: &UserWithdrawParams) -> anchor_lang::Result<()> {
        msg!("user_withdraw");

        let house = ctx.accounts.house.load()?;
        let house_bump = house.bump.clone();

        let mint = ctx.accounts.mint.to_account_info().clone();
        let house_seeds: &[&[&[u8]]] = &[&[&HOUSE_SEED, mint.key.as_ref(), &[house_bump]]];
        drop(house);

        let user = &mut ctx.accounts.user.load_mut()?;
        user.last_airdrop_request_slot = Clock::get()?.slot.clone();
        let house_vault = ctx.accounts.house_vault.to_account_info().clone();

        // deposit to house_vault 

        transfer(
            &ctx.accounts.token_program,
            &ctx.accounts.house_vault,
            &ctx.accounts.airdrop_token_wallet,
            &ctx.accounts.house.to_account_info(),
            house_seeds,
            user.current_round.bet_amount,
        )?;

        user.deposited -= params.amount;

        Ok(())
    }
}
