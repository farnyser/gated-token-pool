use crate::state::GatedTokenPool;
use anchor_lang::prelude::*;
use anchor_lang::Accounts;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

pub fn withdraw_instruction(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    require_gt!(amount, 0);
    msg!(
        "Withdrawing {} of {} from pool vault",
        amount,
        ctx.accounts.mint.key()
    );

    let admin = ctx.accounts.admin.key();
    let bump_seed = ctx.accounts.gated_token_pool.bump;
    let seeds = &[
        b"pool",
        ctx.accounts.gated_token_pool.token_mint.as_ref(),
        ctx.accounts.gated_token_pool.quote_mint.as_ref(),
        admin.as_ref(),
        &[bump_seed],
    ];
    let signer = &[&seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.user_account.to_account_info(),
        authority: ctx.accounts.gated_token_pool.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

    anchor_spl::token::transfer(cpi_ctx, amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account()]
    pub gated_token_pool: Account<'info, GatedTokenPool>,

    #[account()]
    mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = gated_token_pool
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = admin
    )]
    pub user_account: Account<'info, TokenAccount>,

    #[account()]
    admin: Signer<'info>,

    pub token_program: Program<'info, Token>,
}
