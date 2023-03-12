use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod cubik_contract {
    use super::*;

    pub fn create_user(ctx: Context<CreateUserContext>, _user_id: String) -> Result<()> {
        create_user_handler(ctx, _user_id)
    }
    pub fn create_project(ctx: Context<CreateProjectContext>, _project_id: String) -> Result<()> {
        create_project_handler(ctx, _project_id)
    }

    pub fn create_round(
        ctx: Context<CreateRoundContext>,
        _round_id: String,
        matching_pool: u64,
        project_size: u64,
    ) -> Result<()> {
        create_round_handler(ctx, _round_id, matching_pool, project_size)
    }
    pub fn create_contribution(
        ctx: Context<ContributionContext>,
        _round_id: String,
        _project_id: String,
        _contribution_id: String,
        amount: u64,
        usd_amount: u64,
        token: Pubkey,
    ) -> Result<()> {
        create_contribution_handler(
            ctx,
            _round_id,
            _project_id,
            _contribution_id,
            amount,
            usd_amount,
            token,
        )
    }
}
