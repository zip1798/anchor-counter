use {
    crate::state::Counter,
    anchor_lang::prelude::*,
};


#[derive(Accounts)]
pub struct UpdateIncrement<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,

    pub user: Signer<'info>,
}

pub fn increment(ctx: Context<UpdateIncrement>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    msg!("Previous Count: {}", counter.count);
    counter.count = counter.count.checked_add(1).unwrap();
    msg!("Counter incremented. Current Count: {}", counter.count);
    Ok(())
}
