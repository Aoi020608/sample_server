use std::str::FromStr;

use anchor_client::{anchor_lang::system_program, solana_sdk::signature::Keypair, Client, Cluster};
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use hahatoco::accounts as hahatoco_accounts;
use hahatoco::instruction as hahatoco_instruction;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signer::Signer;
use solana_sdk::sysvar;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    AddMovieReview {
        program_id: String,
        title: String,
        rating: u8,
        description: String,
    },

    UpdateMovieReview {
        program_id: String,
        title: String,
        rating: u8,
        description: String,
    },

    Close {
        program_id: String,
        title: String,
    },

    CreateTokenReward {
        program_id: String,
    },
}
pub fn main() {
    dotenv().ok();
    let cli = Cli::parse();
    let payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("requires a keypair file");

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());

    let initializer = initialize_keypair();
    eprintln!("Initializer Pubkey: {}", initializer.pubkey());

    match &cli.command {
        // Initialize
        Commands::AddMovieReview {
            program_id,
            title,
            rating,
            description,
        } => {
            let program_id = Pubkey::from_str(program_id).expect("parse program_id to Pubkey");
            let program = client.program(program_id).expect("");

            let (pda_account, _bump) = Pubkey::find_program_address(
                &[title.as_bytes().as_ref(), initializer.pubkey().as_ref()],
                &program_id,
            );

            let sig = program
                .request()
                .signer(&initializer)
                .accounts(hahatoco_accounts::AddMovieReview {
                    movie_review: pda_account,
                    initializer: initializer.pubkey(),
                    system_program: system_program::ID,
                })
                .args(hahatoco_instruction::AddMovieReview {
                    title: title.to_string().clone(),
                    description: description.to_string().clone(),
                    rating: *rating,
                })
                .send();

            match sig {
                Ok(transaction_sig) => {
                    println!(
                        "Transaction https://explorer.solana.com/tx/{}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899",
                        transaction_sig
                    );
                }
                Err(e) => println!("Error: {}", e),
            }
        }

        Commands::UpdateMovieReview {
            program_id,
            title,
            rating,
            description,
        } => {
            let program_id = Pubkey::from_str(program_id).expect("parse program_id to Pubkey");
            let program = client.program(program_id).expect("");

            let (pda_account, _bump) = Pubkey::find_program_address(
                &[title.as_bytes().as_ref(), initializer.pubkey().as_ref()],
                &program_id,
            );

            let sig = program
                .request()
                .signer(&initializer)
                .accounts(hahatoco_accounts::UpdateMovieReview {
                    movie_review: pda_account,
                    initializer: initializer.pubkey(),
                    system_program: system_program::ID,
                })
                .args(hahatoco_instruction::UpdateMovieReview {
                    title: title.to_string().clone(),
                    description: description.to_string().clone(),
                    rating: *rating,
                })
                .send();

            match sig {
                Ok(transaction_sig) => {
                    println!(
                        "Transaction https://explorer.solana.com/tx/{}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899",
                        transaction_sig
                    );
                }
                Err(e) => println!("Error: {}", e),
            }
        }

        Commands::Close { program_id, title } => {
            let program_id = Pubkey::from_str(program_id).expect("parse program_id to Pubkey");
            let program = client.program(program_id).expect("");

            let (pda_account, _bump) = Pubkey::find_program_address(
                &[title.as_bytes().as_ref(), initializer.pubkey().as_ref()],
                &program_id,
            );

            let sig = program
                .request()
                .signer(&initializer)
                .accounts(hahatoco_accounts::Close {
                    movie_review: pda_account,
                    reviewer: initializer.pubkey(),
                })
                .args(hahatoco_instruction::Close {})
                .send();

            match sig {
                Ok(transaction_sig) => {
                    println!(
                                 "Transaction https://explorer.solana.com/tx/{}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899",
                                 transaction_sig
                             );
                }
                Err(e) => println!("Error: {}", e),
            }
        }

        Commands::CreateTokenReward { program_id } => {
            let program_id = Pubkey::from_str(program_id).expect("parse program_id to Pubkey");
            let program = client.program(program_id).expect("");

            let (reward_mint_pda, _bump) =
                Pubkey::find_program_address(&["mint".as_bytes()], &program_id);

            let sig = program
                .request()
                .signer(&initializer)
                .accounts(hahatoco_accounts::CreateTokenReward {
                    reward_mint: reward_mint_pda,
                    user: initializer.pubkey(),
                    system_program: system_program::ID,
                    rent: sysvar::rent::ID,
                    token_program: spl_token::ID,
                })
                .args(hahatoco_instruction::CreateRewardMint {})
                .send();

            match sig {
                Ok(transaction_sig) => {
                    println!(
                                 "Transaction https://explorer.solana.com/tx/{}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899",
                                 transaction_sig
                             );
                }
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}

pub fn initialize_keypair() -> Keypair {
    match std::env::var("PRIVATE_KEY") {
        Ok(private_key) => {
            println!("Found a keypair");
            Keypair::from_base58_string(&private_key)
        }
        Err(_) => {
            println!("Generating new keypair...");
            let signer = Keypair::new();
            std::fs::write(".env", format!("PRIVATE_KEY={}", signer.to_base58_string()))
                .expect("write secret key");

            signer
        }
    }
}
