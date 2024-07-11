use std::ops::Deref;
use anchor_client::anchor_lang::system_program;
use anchor_client::Client;
use anchor_client::solana_client::rpc_config::RpcSendTransactionConfig;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::{Keypair, Signature};
use anchor_client::solana_sdk::signer::Signer;

pub fn create_pool<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    signer_wallet: &Keypair,
    fund: Pubkey,
    price: u64
) -> anyhow::Result<Signature> {
    let program = client.program(gated_token_sale::ID)?;

    // Build and send a transaction.
    let sig = program
        .request()
        .signer(&signer_wallet)
        .accounts(gated_token_sale::accounts::CreatePool {
            token_mint: Default::default(),
            quote_mint: Default::default(),
            admin: signer_wallet.pubkey(),
            gated_token_pool: Default::default(),
            gated_token_vault: Default::default(),
            gated_base_vault: Default::default(),
            system_program: system_program::ID,
            token_program: Default::default(),
            associated_token_program: Default::default(),
            rent: Default::default(),
        })
        .args(gated_token_sale::instruction::CreatePool {
            price: price
        })
        .send_with_spinner_and_config(RpcSendTransactionConfig{
            skip_preflight: false,
            ..RpcSendTransactionConfig::default()
        })?;

    Ok(sig)
}