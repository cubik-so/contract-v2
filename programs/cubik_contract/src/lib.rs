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
}
