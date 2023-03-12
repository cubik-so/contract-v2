use crate::state::{RoundJoin};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{self, system_program, sysvar::rent::Rent};

#[derive(Accounts)]
#[instruction(_round_id:String,project_id:String)]
pub struct RoundVerficationContext<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init,
        payer = authority,
        space =RoundJoin::LEN,
        seeds = [b"roundjoin".as_ref(),_round_id.as_ref(),project_id.as_ref()],
        bump 
    )]
    pub round_verfication_account: Box<Account<'info, RoundJoin>>,

    // Misc Accounts
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    #[account(address = solana_program::sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}
pub fn create_round_verfication_handler(ctx: Context<RoundVerficationContext>,_round_id:String,project_id:String) -> Result<()> {

    let round_verfication_account = &mut ctx.accounts.round_verfication_account;

    round_verfication_account.authority = ctx.accounts.authority.key();
    round_verfication_account.approve = false;
    round_verfication_account.project_id = project_id;
    round_verfication_account.round_id = _round_id;
    round_verfication_account.bump = *ctx.bumps.get("round_verfication").unwrap();

    Ok(())
}