use anchor_lang::prelude::*;

#[error_code]
pub enum MovieReviewError {
    #[msg("Rating must be between 1 and 5")]
    InvalidRating
}