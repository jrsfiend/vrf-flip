#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
// #[allow(unaligned_references)]
pub mod actions;
use std::ops::Deref;

pub use actions::*;

pub mod impls;
pub use impls::*;

pub mod utils;
use solana_program::program_pack::Pack;
pub use utils::*;

pub use solana_program::program_option::COption;

pub use anchor_lang::prelude::Pubkey;
pub use anchor_lang::prelude::*;
pub use anchor_lang::{AnchorDeserialize, AnchorSerialize};
pub use anchor_spl::token::{self, Mint, SetAuthority, Token, TokenAccount, Transfer};
#[derive(Clone)]
pub struct m2(spl_token_2022::state::Mint);


// You don't have to implement the "try_deserialize" function
// from this trait. It delegates to
// "try_deserialize_unchecked" by default which is what we want here
// because non-anchor accounts don't have a discriminator to check
impl anchor_lang::AccountDeserialize for m2 {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self> {
       Ok( spl_token_2022::state::Mint::unpack(buf).map(m2) .unwrap())
    }
}
// AccountSerialize defaults to a no-op which is what we want here
// because it's a foreign program, so our program does not
// have permission to write to the foreign program's accounts anyway
impl anchor_lang::AccountSerialize for m2 {}

impl anchor_lang::Owner for m2 {
    fn owner() -> Pubkey {
        // pub use spl_token::ID is used at the top of the file
        ID
    }
}

// Implement the "std::ops::Deref" trait for better user experience
impl Deref for m2 {
    type Target = spl_token_2022::state::Mint;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
pub struct ta2(spl_token_2022::state::Account);


// You don't have to implement the "try_deserialize" function
// from this trait. It delegates to
// "try_deserialize_unchecked" by default which is what we want here
// because non-anchor accounts don't have a discriminator to check
impl anchor_lang::AccountDeserialize for ta2 {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self> {
       Ok( spl_token_2022::state::Account::unpack(buf).map(ta2) .unwrap())
    }
}
// AccountSerialize defaults to a no-op which is what we want here
// because it's a foreign program, so our program does not
// have permission to write to the foreign program's accounts anyway
impl anchor_lang::AccountSerialize for ta2 {}

impl anchor_lang::Owner for ta2 {
    fn owner() -> Pubkey {
        // pub use spl_token::ID is used at the top of the file
        ID
    }
}

// Implement the "std::ops::Deref" trait for better user experience
impl Deref for ta2 {
    type Target = spl_token_2022::state::Account;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// pub use spl_token::instruction::AuthorityType;

pub use switchboard_v2::{
    AggregatorAccountData, OracleQueueAccountData, PermissionAccountData, SwitchboardDecimal,
    VrfAccountData, SWITCHBOARD_PROGRAM_ID,
};

use bytemuck::{Pod, Zeroable};

use num_derive::*;
// use num_traits::*;

use solana_security_txt::security_txt;

declare_id!("7uXnX9gW2smtc5wvaeU3hBQBViRFpyD6C6D5rTb5J16E");

const HOUSE_SEED: &[u8] = b"HOUSESEED";
const USER_SEED: &[u8] = b"USERSEED";

const MAX_BET_AMOUNT: u64 = 1_000_000_000 * 100;

#[program]
pub mod switchboard_vrf_flip {
    use super::*;

    // house actions
    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn house_init(ctx: Context<HouseInit>, params: HouseInitParams) -> anchor_lang::Result<()> {
        HouseInit::actuate(&ctx, &params)
    }

    // user actions
    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn user_init(ctx: Context<UserInit>, params: UserInitParams) -> anchor_lang::Result<()> {
        UserInit::actuate(&ctx, &params)
    }
    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn user_bet(ctx: Context<UserBet>, params: UserBetParams) -> anchor_lang::Result<()> {
        UserBet::actuate(ctx, &params)
    }
    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn user_settle(
        ctx: Context<UserSettle>,
        params: UserSettleParams,
    ) -> anchor_lang::Result<()> {
        UserSettle::actuate(&ctx, &params)
    }
    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn user_airdrop(
        ctx: Context<UserAirdrop>,
        params: UserAirdropParams,
    ) -> anchor_lang::Result<()> {
        UserAirdrop::actuate(&ctx, &params)
    }
}

#[account(zero_copy)]
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct HouseState {
    pub bump: u8,
    // controls vault and can settle winners
    pub authority: Pubkey,
    // token mint for lottery vault
    pub mint: Pubkey,
    // token vault for future use
    pub house_vault: Pubkey,
    // switchboard queue to request randomness on
    pub switchboard_queue: Pubkey,
    // switchboard mint for vrf requests
    pub switchboard_mint: Pubkey,
    // Buffer for future use
    pub _ebuf: [u8; 1024],
}

#[derive(
    AnchorSerialize, AnchorDeserialize, FromPrimitive, ToPrimitive, Copy, Clone, PartialEq, Eq,
)]
pub enum GameType {
    None,
    CoinFlip,
    SixSidedDiceRoll,
    TwentySidedDiceRoll,
}

#[repr(packed)]
#[zero_copy(unsafe)]
#[derive(PartialEq, Eq, Default)]
pub struct GameConfig {
    // number of VRF requests to complete the game
    pub num_vrf_requests: u8,
    // the min of the result, has to be greater than 0
    pub min: u32,
    // the max of the result
    pub max: u32,
    // payout multiplier
    pub payout_multiplier: u32,
}

#[derive(
    AnchorSerialize, AnchorDeserialize, FromPrimitive, ToPrimitive, Copy, Clone, PartialEq, Eq,
)]
pub enum RoundStatus {
    None,
    Awaiting,
    Settled,
}
impl Default for RoundStatus {
    fn default() -> RoundStatus {
        RoundStatus::None
    }
}

#[repr(packed)]
#[zero_copy(unsafe)]
#[derive(PartialEq, Eq, Default)]
pub struct Round {
    pub round_id: u128,
    pub status: RoundStatus,
    pub bet_amount: u64,
    pub game_type: GameType,
    pub game_config: GameConfig,
    pub guess: u32,
    pub result: u32,
    pub request_slot: u64,
    pub request_timestamp: i64,
    pub settle_slot: u64,
    pub settle_timestamp: i64,
}
unsafe impl Pod for Round {}
unsafe impl Zeroable for Round {}

const MAX_HISTORY: u32 = 48;

#[repr(packed)]
#[zero_copy(unsafe)]
#[derive(PartialEq, Eq)]
pub struct History {
    pub idx: u32,
    pub max: u32,
    pub rounds: [Round; MAX_HISTORY as usize],
}
impl Default for History {
    fn default() -> Self {
        Self {
            idx: 0,
            max: MAX_HISTORY,
            rounds: [Round::default(); MAX_HISTORY as usize],
        }
    }
}
unsafe impl Pod for History {}
unsafe impl Zeroable for History {}

// Each user needs an account with its own VRF to play
#[repr(packed)]
#[account(zero_copy(unsafe))]
pub struct UserState {
    pub bump: u8,
    pub authority: Pubkey,
    pub house: Pubkey,
    pub escrow: Pubkey,
    pub reward_address: Pubkey,
    pub vrf: Pubkey,
    pub switchboard_state_bump: u8,
    pub vrf_permission_bump: u8,
    pub current_round: Round,
    pub last_airdrop_request_slot: u64,
    pub _ebuf: [u8; 1024],
    pub history: History,
}
impl Default for UserState {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[event]
pub struct UserBetPlaced {
    pub round_id: u128,
    pub user: Pubkey,
    pub game_type: GameType,
    pub bet_amount: u64,
    pub guess: u32,
    pub slot: u64,
    pub timestamp: i64,
}

#[event]
pub struct UserBetSettled {
    pub round_id: u128,
    pub user: Pubkey,
    pub user_won: bool,
    pub game_type: GameType,
    pub bet_amount: u64,
    pub escrow_change: u64,
    pub guess: u32,
    pub result: u32,
    pub slot: u64,
    pub timestamp: i64,
}

#[error_code]
#[derive(Eq, PartialEq)]
pub enum VrfFlipError {
    #[msg("VRF Account counter should be 0 for a new lottery")]
    InvalidInitialVrfCounter,
    #[msg("VRF Account authority should be the lottery Pubkey")]
    InvalidVrfAuthority,
    #[msg("Provided account is not owned by the switchboard program")]
    InvalidSwitchboardAccount,
    #[msg("VRF counter does not match the expected round id")]
    IncorrectVrfCounter,
    #[msg("Failed to match the game type")]
    InvalidGameType,
    #[msg("Current round is still active")]
    CurrentRoundStillActive,
    #[msg("Current round has already settled")]
    CurrentRoundAlreadyClosed,
    #[msg("Invalid bet")]
    InvalidBet,
    #[msg("Switchboard queue requires VRF permissions to request randomness")]
    OracleQueueRequiresPermissions,
    #[msg("VRF account belongs to the incorrect oracle queue")]
    OracleQueueMismatch,
    #[msg("User requested an airdrop too soon")]
    AirdropRequestedTooSoon,
    #[msg("User has enough funds and does not require an airdrop")]
    UserTokenBalanceHealthy,
    #[msg("Max bet exceeded")]
    MaxBetAmountExceeded,
    #[msg("Insufficient funds to request randomness")]
    InsufficientFunds,
    #[msg("User can flip once every 10 seconds")]
    FlipRequestedTooSoon,
    #[msg("House has no authority to mint more tokens")]
    UnauthorizedMint,
}

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "Switchboard VRF Flip",
    project_url: "https://switchboard.xyz/",
    contacts: "email:security@switchboard.xyz,link:https://docs.switchboard.xyz/security,discord:switchboardxyz,twitter:switchboardxyz,telegram:switchboardxyz",
    policy: "https://docs.switchboard.xyz/security",
    preferred_languages: "en",
    source_code: "https://github.com/switchboard-xyz/vrf-flip",
    auditors: "None",
    acknowledgements: "
This example program is not elgible for the Switchboard bug bounty program but we appreciate any contributions that lead to safer code for VRF integrators.
 - Switchboard
    "
}
