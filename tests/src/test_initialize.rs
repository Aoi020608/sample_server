use std::str::FromStr;

use anchor_client::{
    solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Keypair},
    Client, Cluster,
};
// use solana_program_test::*;

#[test]
fn test_initialize() {
    let program_id = "DjLzptXB2VJH9GTXacQHPApSBfUg72rvrGJaj4wnPjkA";
    let keypair = Keypair::new();
    let client =
        Client::new_with_options(Cluster::Localnet, &keypair, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).expect("parse program_id to Pubkey");
    let program = client.program(program_id).expect("");

    let tx = program
        .request()
        .accounts(hahatoco::accounts::Initialize {})
        .args(hahatoco::instruction::Initialize {})
        .send()
        .expect("");

    println!("Your transaction signature {}", tx);
}
