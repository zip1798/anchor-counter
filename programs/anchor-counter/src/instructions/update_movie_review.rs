use {
    crate::state::MovieAccountState,
    anchor_lang::prelude::*,
    crate::errors::*
};


#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct UpdateMovieReview<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), initializer.key().as_ref()],
        bump,
        realloc = MovieAccountState::INIT_SPACE + 8 + title.len() + description.len(),
        realloc::payer = initializer,
        realloc::zero = true
    )]
    pub movie_review: Account<'info, MovieAccountState>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn update_movie_review(
    ctx: Context<UpdateMovieReview>,
    title: String,
    description: String,
    rating: u8
) -> Result<()> {
    msg!("Movie review account space reallocated");
    msg!("Title {}", title);
    msg!("Desctiption {}", description);
    msg!("Rating {}", rating);

    require!(rating >= 1 && rating <= 5, MovieReviewError::InvalidRating);

    let movie_review = &mut ctx.accounts.movie_review;
    movie_review.rating = rating;
    movie_review.description = description;

    Ok(())
}