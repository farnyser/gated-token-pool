use crate::state::GatedTokenPool;
use anchor_lang::prelude::*;
use anchor_lang::Accounts;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

pub fn deposit_instruction(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    require_gt!(amount, 0);
    msg!(
        "Depositing {} of {} into pool vault",
        amount,
        ctx.accounts.mint.key()
    );

    let cpi_accounts = Transfer {
        from: ctx.accounts.user_account.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.admin.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    anchor_spl::token::transfer(cpi_ctx, amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct Deposit<'info> {
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
