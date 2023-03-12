use crate::state::{Round};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{self, system_program, sysvar::rent::Rent};

#[derive(Accounts)]
#[instruction(_round_id:String)]
pub struct CreateRoundContext<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init,
        payer = authority,
        space = Round::LEN,
        seeds = [b"round".as_ref(),_round_id.as_ref()],
        bump 
    )]
    pub round_account: Box<Account<'info, Round>>,

    // Misc Accounts
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    #[account(address = solana_program::sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}
pub fn create_round_handler(ctx: Context<CreateRoundContext>, _round_id: String,matching_pool:u64,project_size:u64) -> Result<()> {
    let round_account = &mut ctx.accounts.round_account;
    round_account.authority = ctx.accounts.authority.key();
    round_account.pool_size = matching_pool;
    round_account.round_id = _round_id;
    round_account.project_size = project_size;
    round_account.bump = *ctx.bumps.get("round_account").unwrap();

    Ok(())
}