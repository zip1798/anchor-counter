use anchor_lang::prelude::*;

// const ANCHOR_DISCRIMINATOR: usize = 8;

pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("EX6RpRe8PhX8DzcWzAm481E7nfF3iDsqhnteL1Rqxtd6");

#[program]
pub mod anchor_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn increment(ctx: Context<UpdateIncrement>) -> Result<()> {
        instructions::increment(ctx)
    }

    pub fn decrement(ctx: Context<UpdateDecrement>) -> Result<()> {
        instructions::decrement(ctx)
    }

}




