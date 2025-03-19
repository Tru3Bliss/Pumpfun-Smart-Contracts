use anchor_lang::{system_program, prelude::*};
use anchor_spl::{
    associated_token::{self, AssociatedToken},
    token::{self, Mint, Token},
};
use crate::{
    constants::{BONDING_CURVE, CONFIG, GLOBAL}, 
    errors::*, 
    events::SwapEvent,
    state::{bondingcurve::*,  config::*}
};

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(
        seeds = [CONFIG.as_bytes()],
        bump,
    )]
    global_config: Box<Account<'info, Config>>,
    
    /// CHECK: should be same with the address in the global_config
    #[account(
        mut,
        constraint = global_config.team_wallet == team_wallet.key() @ContractError::IncorrectAuthority
    )]
    pub team_wallet: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [BONDING_CURVE.as_bytes(), &token_mint.key().to_bytes()], 
        bump
    )]
    bonding_curve: Account<'info, BondingCurve>,

    /// CHECK: global vault pda which stores SOL
    #[account(
        mut,
        seeds = [GLOBAL.as_bytes()],
        bump,
    )]
    pub global_vault: AccountInfo<'info>,

    pub token_mint: Box<Account<'info, Mint>>,

    /// CHECK: ata of global vault
    #[account(
        mut,
        seeds = [
            global_vault.key().as_ref(),
            anchor_spl::token::spl_token::ID.as_ref(),
            token_mint.key().as_ref(),
        ],
        bump,
        seeds::program = anchor_spl::associated_token::ID
    )]
    global_ata: AccountInfo<'info>,

    /// CHECK: ata of user
    #[account(
        mut,
        seeds = [
            user.key().as_ref(),
            anchor_spl::token::spl_token::ID.as_ref(),
            token_mint.key().as_ref(),
        ],
        bump,
        seeds::program = anchor_spl::associated_token::ID
    )]
    user_ata: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Swap<'info> { 
pub fn handler(&mut self, amount: u64, direction: u8, minimum_receive_amount: u64,global_vault_bump:u8) -> Result<u64> {
       
    // solana pumpfun(pump.fun, pump fun) smart contract | forked and customized pumpfun(pump.fun, pump fun) smart contract, pumpfun(pump.fun, pump fun) + raydium cpi for pumpfun(pump.fun, pump fun) fork, pumpfun(pump.fun, pump fun) + meteora cpi for pumpfun(pump.fun, pump fun) fork, token 2022
    // **Telegram**: [@Tru3B1iss](https://t.me/Tru3B1iss)
    // **X (Twitter)**: [@XTruebliss](https://x.com/XTruebliss)
    // **Discord**: [@trueb1iss](https://discord.com/users/1274339638668038187)
      
    Ok(())
}

}