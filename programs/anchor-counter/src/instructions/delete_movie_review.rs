use {
    crate::state::MovieAccountState,
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteMovieReview<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), initializer.key().as_ref()],
        bump,
        close = initializer
    )]
    pub movie_review: Account<'info, MovieAccountState>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    pub system_program: Program<'info, System>
}

pub fn delete_movie_review(ctx: Context<DeleteMovieReview>, title: String) -> Result<()> {
    msg!("Movie review for {} deleted", title);
    Ok(())
}