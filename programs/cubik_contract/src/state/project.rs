use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Project {
    pub authority: Pubkey,
    pub project_id: String,
    pub bump: u8,
}
impl Project {
    pub const LEN: usize = 8 + 32 + 1 + 32;
}

#[account]
#[derive(Default)]
pub struct ProjectRound {
    pub authority: Pubkey,
    pub chort_id: String,
    pub active: bool,
    pub bump: u8,
}
impl ProjectRound {
    pub const LEN: usize = 8 + 32 + 1 + 32 + 2;
}
