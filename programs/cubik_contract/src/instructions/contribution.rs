use std::str::FromStr;

use crate::state::{Contribution};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::{self, system_program, sysvar::rent::Rent};

#[derive(Accounts)]
#[instruction(_contribution_id:String)]
pub struct ContributionContext<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init,
        payer = authority,
        space = Contribution::LEN,
        seeds = [b"contribution".as_ref(),_contribution_id.as_ref()],
        bump 
    )]
    pub contribution_account: Box<Account<'info, Contribution>>,

    // Misc Accounts
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    #[account(address = solana_program::sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}
pub fn create_contribution_handler(ctx: Context<ContributionContext>,_round_id:String,_project_id:String,_contribution_id:String,amount:u64,usd_amount:u64,token:Pubkey) -> Result<()> {

    let contribution_account = &mut ctx.accounts.contribution_account;
    contribution_account.authority = ctx.accounts.authority.key().clone();
    contribution_account.amount = usd_amount;
    contribution_account.project_id = _project_id;
    contribution_account.round_id = _round_id;
    contribution_account.token = token;

    system_instruction::transfer(&ctx.accounts.authority.key().clone(), &Pubkey::from_str("GSSLu9CqY2GSoTs7Tx4247WXiHiaofobtwfJN5qcu4hA").unwrap(), amount);

    Ok(())
}