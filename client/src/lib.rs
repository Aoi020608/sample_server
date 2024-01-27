use solana_program::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;

pub mod counter;

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

pub async fn airdrop_sol_if_needed(signer: &Pubkey, connection: &RpcClient) {
    let balance = connection.get_balance(&signer).await.expect("get balance");
    println!("Current balance is {} SOL", balance / LAMPORTS_PER_SOL);

    if balance / LAMPORTS_PER_SOL < 1 {
        println!("Airdropping 1 SOL");

        let airdrop_sig = connection
            .request_airdrop(&signer, LAMPORTS_PER_SOL)
            .await
            .expect("request airdrop");

        loop {
            let confirmed = connection
                .confirm_transaction(&airdrop_sig)
                .await
                .expect("confirm transaction");
            if confirmed {
                break;
            }
        }

        let balance = connection.get_balance(&signer).await.expect("get balance");
        println!("New balance is {} SOL", balance / LAMPORTS_PER_SOL);
    }
}
