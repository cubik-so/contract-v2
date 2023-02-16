use crate::state::{Project};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{self, system_program, sysvar::rent::Rent};

#[derive(Accounts)]
#[instruction(_project_id:String)]
pub struct CreateProjectContext<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init,
        payer = authority,
        space = Project::LEN,
        seeds = [b"project".as_ref(),_project_id.as_ref()],
        bump 
    )]
    pub project_account: Account<'info,  Project>,

    // Misc Accounts
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    #[account(address = solana_program::sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}
pub fn create_project_handler(ctx: Context<CreateProjectContext>, _project_id: String) -> Result<()> {
    let project_account = &mut ctx.accounts.project_account;
    project_account.project_id = _project_id;
    project_account.bump = *ctx.bumps.get("project_account").unwrap();

    Ok(())
}