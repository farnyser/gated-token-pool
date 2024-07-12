mod buy;
mod create_authorization;
mod create_pool;
mod deposit;
mod pubkey_cli;
mod withdraw;

use crate::buy::buy;
use crate::create_authorization::create_authorization;
use crate::create_pool::create_pool;
use crate::deposit::deposit;
use crate::withdraw::withdraw;
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::signature::Keypair;
use anchor_client::solana_sdk::signer::EncodableKey;
use anchor_client::{Client, Cluster};
use clap::{ArgAction, Parser, Subcommand};
use std::path::Path;
use std::rc::Rc;

#[derive(Debug, Parser)]
#[command(name = "client")]
#[command(about = "Gated Token Pool CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = "http://localhost:8899")]
    cluster_url: String,

    #[arg(short('k'), long)]
    keypair_path: String,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    CreatePool {
        /// token mint
        #[arg(long, value_parser = clap::value_parser!(pubkey_cli::PubkeyCli))]
        token: pubkey_cli::PubkeyCli,

        /// quote mint
        #[arg(long, value_parser = clap::value_parser!(pubkey_cli::PubkeyCli))]
        quote: pubkey_cli::PubkeyCli,

        /// fixed price for 1 native token (in native quote unit)
        #[arg(short, long)]
        price: u64,
    },
    CreateAuthorization {
        /// Pool
        #[arg(long, value_parser = clap::value_parser!(pubkey_cli::PubkeyCli))]
        pool: pubkey_cli::PubkeyCli,

        /// User for new authorization
        #[arg(long, value_parser = clap::value_parser!(pubkey_cli::PubkeyCli))]
        user: pubkey_cli::PubkeyCli,

        /// max amount buy-able for this user (in token native)
        #[arg(short, long)]
        amount: u64,
    },
    Deposit {
        /// pool
        #[arg(long, value_parser = clap::value_parser!(pubkey_cli::PubkeyCli))]
        pool: pubkey_cli::PubkeyCli,

        /// amount of token to deposit into program vault
        #[arg(short, long)]
        amount: u64,
    },
    Withdraw {
        /// pool
        #[arg(long, value_parser = clap::value_parser!(pubkey_cli::PubkeyCli))]
        pool: pubkey_cli::PubkeyCli,

        /// withdraw token
        #[arg(long, num_args(0), action(ArgAction::SetTrue))]
        token: Option<bool>,

        /// withdraw quote
        #[arg(long, num_args(0), action(ArgAction::SetTrue))]
        quote: Option<bool>,

        /// amount to withdraw from program vault
        #[arg(short, long)]
        amount: u64,
    },
    Buy {
        /// pool
        #[arg(long, value_parser = clap::value_parser!(pubkey_cli::PubkeyCli))]
        pool: pubkey_cli::PubkeyCli,

        /// amount of token to buy from program vault
        #[arg(short, long)]
        amount: u64,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let url = args.cluster_url.parse::<Cluster>().unwrap();
    let wallet = Keypair::read_from_file(Path::new(args.keypair_path.as_str())).unwrap();
    let payer = Rc::new(wallet);
    let client =
        Client::new_with_options(url.clone(), payer.clone(), CommitmentConfig::confirmed());

    match args.command {
        Commands::CreatePool {
            quote,
            token,
            price,
        } => {
            println!("Creating pool with price={}", price);

            create_pool(&client, payer.clone(), quote.0, token.0, price)?
        }
        Commands::Deposit { pool, amount } => {
            println!("Depositing {} token into pool's vault {}", amount, pool.0);

            deposit(&client, payer.clone(), pool.0, amount)?
        }
        Commands::CreateAuthorization { pool, user, amount } => {
            println!(
                "Creating buy authorization for user {} taking max {} of pool {}",
                user.0, amount, pool.0
            );

            create_authorization(&client, payer.clone(), pool.0, user.0, amount)?
        }
        Commands::Withdraw {
            pool,
            token,
            quote,
            amount,
        } => {
            let token = token.unwrap_or(false);
            let quote = quote.unwrap_or(false);
            if token == false && quote == false {
                anyhow::bail!("Must select token or quote");
            }
            if token && quote {
                anyhow::bail!("Must select one of token or quote only");
            }

            println!("Withdrawing {} from pool's vault {}", amount, pool.0);

            withdraw(&client, payer.clone(), pool.0, quote, amount)?
        }
        Commands::Buy { pool, amount } => {
            println!("Buying {} token from pool's vault {}", amount, pool.0);

            buy(&client, payer.clone(), pool.0, amount)?
        }
    };

    Ok(())
}
