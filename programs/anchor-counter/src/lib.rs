use anchor_lang::prelude::*;

// const ANCHOR_DISCRIMINATOR: usize = 8;

pub mod instructions;
pub mod state;
pub mod errors;

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

    pub fn addMovieReview(ctx: Context<AddMovieReview>, title: String, description: String, rating: u8) -> Result<()> {
        instructions::add_movie_review(ctx, title, description, rating)
    }

    pub fn updateMovieReview(ctx: Context<UpdateMovieReview>, title: String, description: String, rating: u8) -> Result<()> {
        instructions::update_movie_review(ctx, title, description, rating)
    }

    pub fn deleteMovieReview(ctx: Context<DeleteMovieReview>, title: String) -> Result<()> {
        instructions::delete_movie_review(ctx, title)
    }

    pub fn initializeTokenMint(ctx: Context<InitializeMint>) -> Result<()> {
        instructions::initialize_token_mint(ctx)
    }

}




