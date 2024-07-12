use anchor_client::anchor_lang::{system_program, Id, Key};
use anchor_client::solana_client::rpc_config::RpcSendTransactionConfig;
use anchor_client::solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::rent::Rent;
use anchor_client::solana_sdk::signature::{Keypair, Signature};
use anchor_client::solana_sdk::signer::Signer;
use anchor_client::solana_sdk::sysvar::SysvarId;
use anchor_client::Client;
use anchor_spl::associated_token::{get_associated_token_address, AssociatedToken};
use anchor_spl::token::Token;
use std::ops::Deref;
use std::rc::Rc;

pub fn buy<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    signer_wallet: Rc<Keypair>,
    pool: Pubkey,
    amount: u64,
) -> anyhow::Result<Signature> {
    let program = client.program(gated_token_sale::ID)?;

    let pool_account: gated_token_sale::state::GatedTokenPool = program.account(pool)?;

    let gated_token_vault = get_associated_token_address(&pool.key(), &pool_account.token_mint);
    let user_token_vault =
        get_associated_token_address(&signer_wallet.pubkey().key(), &pool_account.token_mint);
    let gated_quote_vault = get_associated_token_address(&pool.key(), &pool_account.quote_mint);
    let user_quote_vault =
        get_associated_token_address(&signer_wallet.pubkey().key(), &pool_account.quote_mint);

    let (authorization, _) = Pubkey::find_program_address(
        &[
            b"authorization".as_ref(),
            pool.as_ref(),
            signer_wallet.pubkey().as_ref(),
        ],
        &gated_token_sale::ID,
    );

    if let Ok(authorization_account) = program.account(authorization) {
        let authorization_account : gated_token_sale::state::Authorization = authorization_account;
        let remaining_amount = authorization_account.allowance_quantity - authorization_account.bought_quantity;
        println!("Previous authorization: {}", remaining_amount);
        println!("Simulated new authorization: {}", remaining_amount as i64 - amount as i64);
    }

    // Build and send a transaction.
    let sig = program
        .request()
        .signer(&signer_wallet)
        .accounts(gated_token_sale::accounts::Buy {
            token_mint: pool_account.token_mint,
            quote_mint: pool_account.quote_mint,
            gated_token_pool: pool.key(),
            gated_token_vault,
            gated_quote_vault,
            user_token_vault,
            user_quote_vault,
            authorization,
            buyer: signer_wallet.pubkey(),
            system_program: system_program::ID,
            token_program: Token::id(),
            associated_token_program: AssociatedToken::id(),
            rent: Rent::id(),
        })
        .args(gated_token_sale::instruction::Buy { amount })
        .send_with_spinner_and_config(RpcSendTransactionConfig {
            skip_preflight: true,
            preflight_commitment: Some(CommitmentLevel::Finalized),
            ..RpcSendTransactionConfig::default()
        })?;

    Ok(sig)
}
