use {
    crate::state::MovieAccountState,
    anchor_lang::prelude::*,
};
use anchor_spl::token::{mint_to, Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        seeds = ["mint".as_bytes()],
        bump,
        payer = user,
        mint::decimals = 6,
        mint::authority = mint    
    )]
    pub mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    
    pub rent: Sysvar<'info, Rent>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_token_mint(_ctx: Context<InitializeMint>) -> Result<()> { 
    msg!("Token mint initialized");

    Ok(()) 
}