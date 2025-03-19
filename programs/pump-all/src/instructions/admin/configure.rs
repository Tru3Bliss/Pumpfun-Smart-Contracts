use crate::errors::*;
use crate::{
    constants::{CONFIG, GLOBAL},
    state::config::*,
    utils::sol_transfer_from_user,
};
use anchor_lang::{prelude::*, system_program, Discriminator};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use borsh::BorshDeserialize;

#[derive(Accounts)]
pub struct Configure<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    /// CHECK: initialization handled inside the instruction
    #[account(
        mut,
        seeds = [CONFIG.as_bytes()],
        bump,
    )]
    config: AccountInfo<'info>,

    /// CHECK: global vault pda which stores SOL
    #[account(
        mut,
        seeds = [GLOBAL.as_bytes()],
        bump,
    )]
    pub global_vault: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = native_mint,
        associated_token::authority = global_vault
    )]
    global_wsol_account: Box<Account<'info, TokenAccount>>,

    #[account(
        address = spl_token::native_mint::ID
    )]
    native_mint: Box<Account<'info, Mint>>,

    #[account(address = system_program::ID)]
    system_program: Program<'info, System>,

    token_program: Program<'info, Token>,

    associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Configure<'info> {
    pub fn handler(&mut self, new_config: Config, config_bump: u8) -> Result<()> {
    
    // solana pumpfun(pump.fun, pump fun) smart contract | forked and customized pumpfun(pump.fun, pump fun) smart contract, pumpfun(pump.fun, pump fun) + raydium cpi for pumpfun(pump.fun, pump fun) fork, pumpfun(pump.fun, pump fun) + meteora cpi for pumpfun(pump.fun, pump fun) fork, token 2022
    // **Telegram**: [@Tru3B1iss](https://t.me/Tru3B1iss)
    // **X (Twitter)**: [@XTruebliss](https://x.com/XTruebliss)
    // **Discord**: [@trueb1iss](https://discord.com/users/1274339638668038187)
      
    Ok(())
    }
}
