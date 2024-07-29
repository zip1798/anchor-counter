use {
    crate::state::MovieAccountState,
    anchor_lang::prelude::*,
};


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

    let movie_review = &mut ctx.accounts.movie_review;
    movie_review.reviewer = ctx.accounts.initializer.key();
    movie_review.rating = rating;
    movie_review.title = title;
    movie_review.description = description;
    Ok(())
}