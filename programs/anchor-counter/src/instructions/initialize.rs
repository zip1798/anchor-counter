use {
    crate::state::{
        // change_request::{ChangeRequest, ChangeRequestTarget},
        Counter,
    },
    anchor_lang::prelude::*,
};

const ANCHOR_DISCRIMINATOR: usize = 8;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer=user, 
        space=ANCHOR_DISCRIMINATOR+Counter::INIT_SPACE
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    counter.count = 0;

    msg!("Counter Account Created");
    msg!("Counter Count: {}", counter.count);
    Ok(())
}

