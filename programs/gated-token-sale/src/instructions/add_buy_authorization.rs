use crate::state::{Authorization, GatedTokenPool};
use anchor_lang::prelude::*;
use anchor_lang::Accounts;

pub fn add_buy_authorization_instruction(
    ctx: Context<AddBuyAuthorization>,
    max_amount: u64,
) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.gated_token_pool.admin.key(),
        ctx.accounts.admin.key.key()
    );

    ctx.accounts.authorization.allowance_quantity = max_amount;
    ctx.accounts.authorization.bought_quantity = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct AddBuyAuthorization<'info> {
    #[account()]
    pub gated_token_pool: Account<'info, GatedTokenPool>,

    #[account(mut)]
    admin: Signer<'info>,

    /// CHECK: only used for whitelisting user
    #[account()]
    buyer: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = admin,
        space = 8 + 8 * 2,
        seeds = [b"authorization", gated_token_pool.key().as_ref(), buyer.key().as_ref()],
        bump
    )]
    pub authorization: Account<'info, Authorization>,

    pub system_program: Program<'info, System>,
}
