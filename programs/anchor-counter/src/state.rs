use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64,
}

#[account]
#[derive(InitSpace)]
pub struct MovieAccountState {
    pub rating: u8,
    
    #[max_len(50)]
    pub title: String,
    
    #[max_len(100)]
    pub description: String,

    pub reviewer: Pubkey,
}