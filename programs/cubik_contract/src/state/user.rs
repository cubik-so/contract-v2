use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct User {
    pub authority: Pubkey,
    pub user_id: String,
    pub bump: u8,
}
impl User {
    pub const LEN: usize = 8 + 32 + 42 + 1;
}
