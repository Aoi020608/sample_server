use clap::{Parser, Subcommand};
use client::counter;
use dotenv::dotenv;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize account
    Initialize { program_id: String },

    /// Stake
    Increment { program_id: String },
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Cli::parse();
    let url = "https://api.devnet.solana.com";
    let rpc_client = RpcClient::new(url.to_string());

    match &cli.command {
        Commands::Initialize { program_id } => {
            counter::initialize(&rpc_client, program_id).await;
        }

        Commands::Increment { program_id } => {}
    }

    println!("Hello, world!");
}
