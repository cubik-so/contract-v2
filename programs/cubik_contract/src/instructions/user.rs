use crate::state::{User};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{self, system_program, sysvar::rent::Rent};

#[derive(Accounts)]
#[instruction(_user_id:String)]
pub struct CreateUserContext<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init,
        payer = authority,
        space = User::LEN,
        seeds = [b"user".as_ref(),_user_id.as_ref()],
        bump 
    )]
    pub user_account: Account<'info, User>,

    // Misc Accounts
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    #[account(address = solana_program::sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}
pub fn create_user_handler(ctx: Context<CreateUserContext>, _user_id: String) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;
    user_account.user_id = _user_id;
    user_account.bump = *ctx.bumps.get("user_account").unwrap();

    Ok(())
}