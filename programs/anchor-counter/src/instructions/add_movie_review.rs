use {
    crate::state::MovieAccountState,
    anchor_lang::prelude::*,
    crate::errors::*
};
use anchor_spl::token::{mint_to, MintTo,Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;


#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct AddMovieReview<'info> {
    #[account(
        init,
        seeds = [title.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = MovieAccountState::INIT_SPACE + 8 + title.len() + description.len()
    )]
    pub movie_review: Account<'info, MovieAccountState>,

    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
    
    // Account for mint token
    pub token_program: Program<'info, Token>,

    #[account(
        seeds = ["mint".as_bytes()],
        bump,
        mut
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = initializer,
        associated_token::mint = mint,
        associated_token::authority = initializer
    )]
    pub token_account: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>
}

pub fn add_movie_review(
    ctx: Context<AddMovieReview>,
    title: String,
    description: String,
    rating: u8
) -> Result<()> {
    msg!("Movie Review Account Created");
    msg!("Title {}", title);
    msg!("Description {}", description);
    msg!("Rating {}", rating);

    require!(rating >= 1 && rating <= 5, MovieReviewError::InvalidRating);

    let movie_review = &mut ctx.accounts.movie_review;
    movie_review.reviewer = ctx.accounts.initializer.key();
    movie_review.rating = rating;
    movie_review.title = title;
    movie_review.description = description;

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                authority: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            },
            &[&[
                "mint".as_bytes(),
                &[ctx.bumps.mint],
            ]],
        ),
        10*10^6
    )?;

    msg!("Minted tokens");

    Ok(())
}