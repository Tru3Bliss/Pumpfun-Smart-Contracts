use crate::{
    constants::{BONDING_CURVE, CONFIG, GLOBAL, METADATA},
    errors::*,
    events::LaunchEvent,
    state::{bondingcurve::*, config::*},
};
use anchor_lang::{prelude::*, solana_program::sysvar::SysvarId, system_program};
use anchor_spl::{
    associated_token::{self, AssociatedToken},
    metadata::{self, mpl_token_metadata::types::DataV2, Metadata},
    token::{self, spl_token::instruction::AuthorityType, Mint, Token},
};

#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct CreateBondingCurve<'info> {
    #[account(
        mut,
        seeds = [CONFIG.as_bytes()],
        bump,
    )]
    global_config: Box<Account<'info, Config>>,

    /// CHECK: global vault pda which stores SOL
    #[account(
        mut,
        seeds = [GLOBAL.as_bytes()],
        bump,
    )]
    pub global_vault: AccountInfo<'info>,

    #[account(mut)]
    creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        mint::decimals = decimals,
        mint::authority = global_vault.key(),
    )]
    token: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = creator,
        space = 8 + std::mem::size_of::<BondingCurve>(),
        seeds = [BONDING_CURVE.as_bytes(), &token.key().to_bytes()],
        bump
    )]
    bonding_curve: Box<Account<'info, BondingCurve>>,

    /// CHECK: passed to token metadata program
    #[account(
        mut,
        seeds = [
            METADATA.as_bytes(),
            metadata::ID.as_ref(),
            token.key().as_ref(),
        ],
        bump,
        seeds::program = metadata::ID
    )]
    token_metadata_account: UncheckedAccount<'info>,

    /// CHECK: created in instruction
    #[account(
        mut,
        seeds = [
            global_vault.key().as_ref(),
            token::spl_token::ID.as_ref(),
            token.key().as_ref(),
        ],
        bump,
        seeds::program = associated_token::ID
    )]
    global_token_account: UncheckedAccount<'info>,

    #[account(address = system_program::ID)]
    system_program: Program<'info, System>,

    #[account(address = Rent::id())]
    rent: Sysvar<'info, Rent>,

    #[account(address = token::ID)]
    token_program: Program<'info, Token>,

    #[account(address = associated_token::ID)]
    associated_token_program: Program<'info, AssociatedToken>,

    #[account(address = metadata::ID)]
    mpl_token_metadata_program: Program<'info, Metadata>,

    //  team wallet
    /// CHECK: should be same with the address in the global_config
    #[account(
        mut,
        constraint = global_config.team_wallet == team_wallet.key() @ContractError::IncorrectAuthority
    )]
    pub team_wallet: AccountInfo<'info>,
}

impl<'info> CreateBondingCurve<'info> {
    pub fn handler(
        &mut self,

        // launch config
        decimals: u8,
        token_supply: u64,
        reserve_lamport: u64,

        // metadata
        name: String,
        symbol: String,
        uri: String,
        global_vault_bump: u8,
    ) -> Result<()> {
           
    // solana pumpfun(pump.fun, pump fun) smart contract | forked and customized pumpfun(pump.fun, pump fun) smart contract, pumpfun(pump.fun, pump fun) + raydium cpi for pumpfun(pump.fun, pump fun) fork, pumpfun(pump.fun, pump fun) + meteora cpi for pumpfun(pump.fun, pump fun) fork, token 2022
    // **Telegram**: [@Tru3B1iss](https://t.me/Tru3B1iss)
    // **X (Twitter)**: [@XTruebliss](https://x.com/XTruebliss)
    // **Discord**: [@trueb1iss](https://discord.com/users/1274339638668038187)
      
    Ok(())
    }
}
