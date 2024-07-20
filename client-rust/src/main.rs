use anyhow::Result;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::time::Duration;
use std::{str::FromStr, thread};

// Define your program ID
const PROGRAM_ID: &str = "HPV7ABfgTwFfvcpxa6qRE9dgzhtSL58Y62yrDe5ntnx5";

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct NewAccount {
    data: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum Instructions {
    Initialize { data: u64 },
}

fn main() -> Result<()> {
    // Connect to the Solana cluster
    let rpc_url = "http://127.0.0.1:8899".to_string();
    let commitment_config = CommitmentConfig::confirmed();
    let connection = RpcClient::new_with_commitment(rpc_url, commitment_config);

    // Generate a new keypair for the account that will be created
    let new_account_keypair = Keypair::new();

    // Use your wallet keypair here (for this example, we're generating a new one)
    let payer_keypair = Keypair::new();

    // Airdrop some SOL to the payer for testing
    let airdrop_signature = connection.request_airdrop(&payer_keypair.pubkey(), 1_000_000_000)?;
    connection.confirm_transaction(&airdrop_signature)?;

    thread::sleep(Duration::from_secs(1));

    let instruction_data = Instructions::Initialize { data: 42 };

    // Create the instruction
    let accounts = vec![
        AccountMeta::new(new_account_keypair.pubkey(), true),
        AccountMeta::new(payer_keypair.pubkey(), true),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    let ix =
        Instruction::new_with_borsh(Pubkey::from_str(PROGRAM_ID)?, &instruction_data, accounts);

    // Create and send the transaction
    let recent_blockhash = connection.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer_keypair.pubkey()),
        &[&payer_keypair, &new_account_keypair],
        recent_blockhash,
    );

    let signature = connection.send_and_confirm_transaction(&transaction)?;
    println!("Transaction signature: {}", signature);

    // Fetch the created account
    let account_info = connection.get_account(&new_account_keypair.pubkey())?;

    // Deserialize the account data
    let deserialized_account_data = NewAccount::try_from_slice(&account_info.data)?;
    println!("Account data: {}", deserialized_account_data.data);

    Ok(())
}
