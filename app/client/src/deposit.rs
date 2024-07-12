use std::ops::Deref;
use std::rc::Rc;

use anchor_client::anchor_lang::{Id, Key};
use anchor_client::solana_client::rpc_config::RpcSendTransactionConfig;
use anchor_client::solana_sdk::commitment_config::CommitmentLevel;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::{Keypair, Signature};
use anchor_client::solana_sdk::signer::Signer;
use anchor_client::Client;
use anchor_spl::associated_token::get_associated_token_address;
use anchor_spl::token::Token;

pub fn deposit<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    signer_wallet: Rc<Keypair>,
    pool: Pubkey,
    amount: u64,
) -> anyhow::Result<Signature> {
    let program = client.program(gated_token_sale::ID)?;

    let pool_account: gated_token_sale::state::GatedTokenPool = program.account(pool)?;

    let gated_token_vault = get_associated_token_address(&pool.key(), &pool_account.token_mint);
    let wallet_token_account =
        get_associated_token_address(&signer_wallet.pubkey().key(), &pool_account.token_mint);

    // Build and send a transaction.
    let sig = program
        .request()
        .signer(&signer_wallet)
        .accounts(gated_token_sale::accounts::Deposit {
            admin: signer_wallet.pubkey(),
            gated_token_pool: pool.key(),
            mint: pool_account.token_mint,
            vault: gated_token_vault.key(),
            user_account: wallet_token_account.key(),
            token_program: Token::id(),
        })
        .args(gated_token_sale::instruction::Deposit { amount })
        .send_with_spinner_and_config(RpcSendTransactionConfig {
            skip_preflight: true,
            preflight_commitment: Some(CommitmentLevel::Finalized),
            ..RpcSendTransactionConfig::default()
        })?;

    Ok(sig)
}
