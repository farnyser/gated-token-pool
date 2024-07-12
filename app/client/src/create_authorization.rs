use std::ops::Deref;
use std::rc::Rc;

use anchor_client::anchor_lang::{system_program, Key};
use anchor_client::solana_client::rpc_config::RpcSendTransactionConfig;
use anchor_client::solana_sdk::commitment_config::CommitmentLevel;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::{Keypair, Signature};
use anchor_client::solana_sdk::signer::Signer;
use anchor_client::Client;

pub fn create_authorization<C: Deref<Target = impl Signer> + Clone>(
    client: &Client<C>,
    signer_wallet: Rc<Keypair>,
    pool: Pubkey,
    user: Pubkey,
    max_quantity: u64,
) -> anyhow::Result<Signature> {
    let program = client.program(gated_token_sale::ID)?;

    let (authorization, _) = Pubkey::find_program_address(
        &[b"authorization".as_ref(), pool.as_ref(), user.as_ref()],
        &gated_token_sale::ID,
    );

    // Build and send a transaction.
    let sig = program
        .request()
        .signer(&signer_wallet)
        .accounts(gated_token_sale::accounts::AddBuyAuthorization {
            admin: signer_wallet.pubkey(),
            gated_token_pool: pool.key(),
            authorization: authorization.key(),
            buyer: user.key(),
            system_program: system_program::ID,
        })
        .args(gated_token_sale::instruction::AddBuyAuthorization { max_quantity })
        .send_with_spinner_and_config(RpcSendTransactionConfig {
            skip_preflight: true,
            preflight_commitment: Some(CommitmentLevel::Finalized),
            ..RpcSendTransactionConfig::default()
        })?;

    Ok(sig)
}
