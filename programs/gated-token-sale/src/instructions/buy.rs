use anchor_lang::prelude::*;

use crate::state::Authorization;
use crate::GatedTokenPool;
use anchor_lang::context::Context;
use anchor_lang::prelude::{Account, Signer};
use anchor_lang::Accounts;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

pub fn buy_instruction(ctx: Context<Buy>, token_amount: u64) -> anchor_lang::Result<()> {
    require_gt!(token_amount, 0);
    require_gt!(
        ctx.accounts.authorization.allowance_quantity,
        ctx.accounts.authorization.bought_quantity + token_amount
    );
    ctx.accounts.authorization.bought_quantity += token_amount;

    let quote_amount = token_amount * ctx.accounts.gated_token_pool.price;

    msg!(
        "Buying {} {} against {} {}",
        token_amount,
        ctx.accounts.token_mint.key(),
        quote_amount,
        ctx.accounts.quote_mint.key(),
    );

    // Quote
    {
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_quote_vault.to_account_info(),
            to: ctx.accounts.gated_quote_vault.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        anchor_spl::token::transfer(cpi_ctx, quote_amount)?;
    }

    // Token
    {
        let seeds = &[
            b"pool",
            ctx.accounts.gated_token_pool.token_mint.as_ref(),
            ctx.accounts.gated_token_pool.quote_mint.as_ref(),
            ctx.accounts.gated_token_pool.admin.as_ref(),
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.gated_token_vault.to_account_info(),
            to: ctx.accounts.user_token_vault.to_account_info(),
            authority: ctx.accounts.gated_token_pool.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        anchor_spl::token::transfer(cpi_ctx, token_amount)?;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account()]
    token_mint: Account<'info, Mint>,

    #[account()]
    quote_mint: Account<'info, Mint>,

    #[account()]
    pub gated_token_pool: Account<'info, GatedTokenPool>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = gated_token_pool
    )]
    pub gated_token_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = quote_mint,
        associated_token::authority = gated_token_pool
    )]
    pub gated_quote_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = buyer
    )]
    pub user_token_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = quote_mint,
        associated_token::authority = buyer
    )]
    pub user_quote_vault: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"authorization", gated_token_pool.key().as_ref(), buyer.key().as_ref()],
        bump
    )]
    pub authorization: Account<'info, Authorization>,

    #[account()]
    buyer: Signer<'info>,

    pub token_program: Program<'info, Token>,
}
