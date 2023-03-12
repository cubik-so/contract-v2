use std::str::FromStr;

use crate::state::{RoundJoin};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{self, system_program, sysvar::rent::Rent};

#[derive(Accounts)]
#[instruction(_round_id:String,project_id:String)]
pub struct UpdateVerficationContext<'info> {
    #[account(mut,
        constraint = authority.key() == Pubkey::from_str("GSSLu9CqY2GSoTs7Tx4247WXiHiaofobtwfJN5qcu4hA").unwrap() 
    )]
    pub authority: Signer<'info>,

    #[account(mut,
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
pub fn update_round_verfication_handler(ctx: Context<UpdateVerficationContext>,_round_id:String,project_id:String) -> Result<()> {

    let round_verfication_account = &mut ctx.accounts.round_verfication_account;

    round_verfication_account.approve = true;

    Ok(())
}