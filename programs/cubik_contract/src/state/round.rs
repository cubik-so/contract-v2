use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Round {
    pub authority: Pubkey,
    pub round_id: String,
    pub bump: u8,
    pub pool_size: u64,
    pub project_size: u64,
}
impl Round {
    pub const LEN: usize = 8 + 1 + 32 + 42 + 16;
}

#[account]
#[derive(Default)]
pub struct RoundJoin {
    pub authority: Pubkey,
    pub round_id: String,
    pub approve: bool,
    pub bump: u8,
    pub project_id: String,
}
impl RoundJoin {
    pub const LEN: usize = 8 + 1 + 32 + 42 + 42 + 1;
}
