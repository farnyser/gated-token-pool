use crate::state::GatedTokenPool;
use anchor_lang::prelude::*;
use anchor_lang::Accounts;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub fn create_pool_instruction(ctx: Context<CreatePool>, price: u64) -> Result<()> {
    require_gt!(price, 0);
    msg!(
        "Creating pool for {} at price {}",
        ctx.accounts.token_mint.key(),
        price
    );

    ctx.accounts.gated_token_pool.price = price;
    ctx.accounts.gated_token_pool.token_mint = ctx.accounts.token_mint.key();
    ctx.accounts.gated_token_pool.quote_mint = ctx.accounts.quote_mint.key();
    Ok(())
}

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account()]
    token_mint: Account<'info, Mint>,

    #[account()]
    quote_mint: Account<'info, Mint>,

    #[account(mut)]
    admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + 8 + 32 * 3,
        seeds = [b"pool", token_mint.key().as_ref(), quote_mint.key().as_ref(), admin.key().as_ref()],
        bump
    )]
    pub gated_token_pool: Account<'info, GatedTokenPool>,

    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = token_mint,
        associated_token::authority = gated_token_pool
    )]
    pub gated_token_vault: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = quote_mint,
        associated_token::authority = gated_token_pool
    )]
    pub gated_base_vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}
