use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::{invoke, invoke_signed};
use anchor_lang::solana_program::{self, system_program, sysvar::rent::Rent};

// spl token
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Approve, Mint, MintTo, Token, TokenAccount, Transfer};
// metaplex
use mpl_token_metadata::instruction::{
    create_master_edition_v3, create_metadata_accounts_v3, freeze_delegated_account,
    update_metadata_accounts_v2,
};
use mpl_token_metadata::state::Creator;

#[derive(Accounts)]
#[instruction(_user_id : String)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        mint::decimals = 0,
        mint::authority = authority,
        mint::freeze_authority = authority,
    )]
    pub mint: Account<'info, Mint>,

    #[account(mut,seeds=[b"user".as_ref(),_user_id.as_ref()],bump = user_account.bump)]
    pub user_account: Box<Account<'info, User>>,

    #[account(init, payer = authority, associated_token::mint = mint, associated_token::authority = authority)]
    pub badge_ata: Box<Account<'info, TokenAccount>>,

    /// CHECK
    #[account(address = mpl_token_metadata::id())]
    pub mpl_program: AccountInfo<'info>,
    /// CHECK
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    /// CHECK
    #[account(mut)]
    pub master_edition: AccountInfo<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    #[account(address = solana_program::sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<MintNFT>,
    name: String,
    symbol: String,
    uri: String,
    _user_id: String,
) -> Result<()> {
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info().clone(),
        to: ctx.accounts.badge_ata.to_account_info().clone(),
        authority: ctx.accounts.authority.to_account_info().clone(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::mint_to(cpi_ctx, 1)?;
    let creators = vec![Creator {
        address: *ctx.accounts.authority.key,
        verified: false,
        share: 100,
    }];
    let seller_fee_basis_points: u16 = 10000;
    let metadata_ix = create_metadata_accounts_v3(
        ctx.accounts.mpl_program.key(),
        ctx.accounts.metadata.key(),
        ctx.accounts.mint.key(),
        ctx.accounts.authority.key(),
        ctx.accounts.authority.key(),
        ctx.accounts.authority.key(),
        name,
        symbol,
        uri,
        Some(creators),
        seller_fee_basis_points,
        true,
        true,
        None,
        None,
        None,
    );

    invoke(
        &metadata_ix,
        &[
            ctx.accounts.metadata.clone(),
            ctx.accounts.mint.to_account_info().clone(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.mpl_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
    )?;

    let ix = create_master_edition_v3(
        *ctx.accounts.mpl_program.key,
        *ctx.accounts.master_edition.key,
        ctx.accounts.mint.key(),
        ctx.accounts.authority.key(),
        ctx.accounts.authority.key(),
        ctx.accounts.metadata.key(),
        ctx.accounts.authority.key(),
        Some(0),
    );

    invoke(
        &ix,
        &[
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mpl_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
    )?;
    let update_ix = update_metadata_accounts_v2(
        ctx.accounts.mpl_program.key(),
        ctx.accounts.metadata.key(),
        ctx.accounts.authority.key(),
        Some(ctx.accounts.user_account.key()),
        None,
        Some(true),
        Some(true),
    );
    invoke(
        &update_ix,
        &[
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.authority.to_account_info(),
        ],
    )?;
    let cpi_accounts = Approve {
        to: ctx.accounts.badge_ata.to_account_info(),
        delegate: ctx.accounts.user_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    token::approve(cpi_context, 1)?;

    let user_seeds = &[
        "user".as_bytes(),
        _user_id.as_ref(),
        &[ctx.accounts.user_account.bump],
    ];
    invoke_signed(
        &freeze_delegated_account(
            *ctx.accounts.mpl_program.key,
            ctx.accounts.user_account.key(),
            ctx.accounts.badge_ata.key(),
            *ctx.accounts.master_edition.key,
            ctx.accounts.mint.key(),
        ),
        &[
            ctx.accounts.user_account.to_account_info(),
            ctx.accounts.badge_ata.to_account_info(),
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
        ],
        &[user_seeds],
    )?;
    Ok(())
}
