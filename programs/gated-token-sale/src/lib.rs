mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;
use state::*;

declare_id!("5TFSj2iJzEXS6z5QTCDwwMs9QnDehHwxEATj5Gx9ShV4");

#[program]
pub mod gated_token_sale {
    use super::*;

    pub fn create_pool(ctx: Context<CreatePool>, price: u64) -> Result<()> {
        create_pool_instruction(ctx, price)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        withdraw_instruction(ctx, amount)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        deposit_instruction(ctx, amount)
    }

    pub fn add_buy_authorization(
        ctx: Context<AddBuyAuthorization>,
        max_quantity: u64,
    ) -> Result<()> {
        add_buy_authorization_instruction(ctx, max_quantity)
    }

    pub fn buy(ctx: Context<Buy>, amount: u64) -> Result<()> {
        buy_instruction(ctx, amount)
    }
}
