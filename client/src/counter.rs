use std::str::FromStr;

use solana_program::{
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    system_program,
};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{signer::Signer, transaction::Transaction};

use crate::initialize_keypair;

pub async fn initialize(rpc_client: &RpcClient, program_id: &str) {
    let signer = initialize_keypair();

    let program_id = Pubkey::from_str(program_id).expect("parse to Pubkey");

    let (pda_counter, _bump) =
        Pubkey::find_program_address(&[&signer.pubkey().to_bytes()], &program_id);

    let accounts = vec![
        AccountMeta::new(pda_counter, false),
        AccountMeta::new(signer.pubkey(), true),
        AccountMeta::new(system_program::id(), false),
    ];

    let instruction = Instruction::new_with_bytes(program_id, &[0], accounts);
    let message = Message::new(&[instruction], Some(&signer.pubkey()));
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .await
        .expect("get latest block hash");
    let transaction = Transaction::new(&[&signer], message, recent_blockhash);
    let transaction_sig = rpc_client
        .send_and_confirm_transaction(&transaction)
        .await
        .expect("send and confirm transaction");
    println!(
        "Transaction https://explorer.solana.com/tx/{}?cluster=devnet",
        transaction_sig
    );
}
