use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Contribution {
    pub authority: Pubkey,
    pub project_id: String,
    pub round_id: String,
    pub token: Pubkey,
    pub amount: u64,
    pub bump: u8,
}
impl Contribution {
    pub const LEN: usize = 8 + 1 + 32 + 32 + 40 + 40 + 8;
}
