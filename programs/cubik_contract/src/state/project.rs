use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Project {
    pub authority: Pubkey,
    pub project_id: String,
    pub bump: u8,
}
impl Project {
    pub const LEN: usize = 8 + 32 + 1 + 42;
}
