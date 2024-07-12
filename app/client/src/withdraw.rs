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

pub fn withdraw<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    signer_wallet: Rc<Keypair>,
    pool: Pubkey,
    is_quote: bool,
    amount: u64,
) -> anyhow::Result<Signature> {
    let program = client.program(gated_token_sale::ID)?;

    let pool_account: gated_token_sale::state::GatedTokenPool = program.account(pool)?;

    let mint = if is_quote {
        pool_account.quote_mint
    } else {
        pool_account.token_mint
    };
    let gated_vault = get_associated_token_address(&pool.key(), &mint);
    let wallet_account = get_associated_token_address(&signer_wallet.pubkey().key(), &mint);

    // Build and send a transaction.
    let sig = program
        .request()
        .signer(&signer_wallet)
        .accounts(gated_token_sale::accounts::Withdraw {
            admin: signer_wallet.pubkey(),
            gated_token_pool: pool.key(),
            mint,
            vault: gated_vault.key(),
            user_account: wallet_account.key(),
            token_program: Token::id(),
        })
        .args(gated_token_sale::instruction::Withdraw { amount })
        .send_with_spinner_and_config(RpcSendTransactionConfig {
            skip_preflight: true,
            preflight_commitment: Some(CommitmentLevel::Finalized),
            ..RpcSendTransactionConfig::default()
        })?;

    Ok(sig)
}
