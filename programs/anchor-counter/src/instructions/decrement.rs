use {
    crate::state::Counter,
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct UpdateDecrement<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,

    pub user: Signer<'info>,
}

pub fn decrement(ctx: Context<UpdateDecrement>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    msg!("Previous Count: {}", counter.count);
    counter.count = counter.count.checked_sub(1).unwrap();
    msg!("Counter decremented. Current Count: {}", counter.count);
    Ok(())
}
