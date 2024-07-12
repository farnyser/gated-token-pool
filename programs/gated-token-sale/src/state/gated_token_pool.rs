use anchor_lang::prelude::*;

#[account()]
pub struct GatedTokenPool {
    pub bump: u8,
    pub price: u64,
    pub token_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub admin: Pubkey,
}
