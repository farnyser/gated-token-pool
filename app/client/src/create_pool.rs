use std::ops::Deref;
use std::rc::Rc;
use anchor_client::anchor_lang::{Id, system_program};
use anchor_client::Client;
use anchor_client::solana_client::rpc_config::RpcSendTransactionConfig;
use anchor_client::solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::rent::Rent;
use anchor_client::solana_sdk::signature::{Keypair, Signature};
use anchor_client::solana_sdk::signer::Signer;
use anchor_client::solana_sdk::sysvar::SysvarId;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;

pub fn create_pool<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    signer_wallet: Rc<Keypair>,
    quote: Pubkey,
    token: Pubkey,
    price: u64
) -> anyhow::Result<Signature> {
    let program = client.program(gated_token_sale::ID)?;

    // Build and send a transaction.
    let sig = program
        .request()
        .signer(&signer_wallet)
        .accounts(gated_token_sale::accounts::CreatePool {
            token_mint: token,
            quote_mint: quote,
            admin: signer_wallet.pubkey(),
            gated_token_pool: Default::default(),
            gated_token_vault: Default::default(),
            gated_base_vault: Default::default(),
            system_program: system_program::ID,
            token_program: Token::id(),
            associated_token_program: AssociatedToken::id(),
            rent: Rent::id(),
        })
        .args(gated_token_sale::instruction::CreatePool {
            price: price
        })
        .send_with_spinner_and_config(RpcSendTransactionConfig{
            skip_preflight: true,
            preflight_commitment: Some(CommitmentLevel::Finalized),
            ..RpcSendTransactionConfig::default()
        })?;

    Ok(sig)
}