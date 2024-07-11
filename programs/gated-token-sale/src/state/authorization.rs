use anchor_lang::account;
use anchor_lang::prelude::*;

#[account]
pub struct Authorization {
    pub allowance_quantity: u64,
    pub bought_quantity: u64,
}
