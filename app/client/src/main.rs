mod pubkey_cli;
mod create_pool;

use std::path::Path;
use std::rc::Rc;
use anchor_client::{Client, Cluster};
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::signature::Keypair;
use anchor_client::solana_sdk::signer::EncodableKey;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "client")]
#[command(about = "Gated Token Pool CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = "http://localhost:8899")]
    cluster_url: String,

    #[arg(short, long)]
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

}

fn main() {
    let args = Cli::parse();

    let url = args.cluster_url.parse::<Cluster>().unwrap();
    let wallet = Keypair::read_from_file(Path::new(args.keypair_path.as_str())).unwrap();
    let payer = Rc::new(wallet);
    let client = Client::new_with_options(url.clone(), payer.clone(), CommitmentConfig::processed());

    match args.command {
        Commands::CreatePool { quote, token, price } => {
            println!(
                "Creating pool with price={}",
                price
            );


        },
    }
}
