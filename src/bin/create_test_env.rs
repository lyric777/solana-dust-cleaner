use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{read_keypair_file, Keypair, Signer, write_keypair_file},
    transaction::Transaction,
    system_instruction,
    program_pack::Pack,
    pubkey::Pubkey,
    native_token::LAMPORTS_PER_SOL,
};
use spl_token::{
    instruction as token_instruction,
    state::Mint,
};
use anyhow::Result;
use std::path::Path;

// use Devnet 
const RPC_URL: &str = "https://api.devnet.solana.com";
const KEYPAIR_PATH: &str = "id.json";

fn main() -> Result<()> {
    let client = RpcClient::new_with_commitment(RPC_URL, CommitmentConfig::confirmed());
    println!("🧪 init env (Devnet)...");

    // 1. create or get wallet
    let my_keypair = if Path::new(KEYPAIR_PATH).exists() {
        read_keypair_file(KEYPAIR_PATH).unwrap()
    } else {
        let kp = Keypair::new();
        write_keypair_file(&kp, KEYPAIR_PATH).unwrap();
        println!("🆕 create new wallet id.json");
        kp
    };
    let my_pubkey = my_keypair.pubkey();
    println!("📍 test wallet: {}", my_pubkey);

    let balance = client.get_balance(&my_pubkey)?;
    if balance < LAMPORTS_PER_SOL / 2 {
        println!("💧 Balance insufficient, applying for airdrop...");
        match client.request_airdrop(&my_pubkey, LAMPORTS_PER_SOL) {
            Ok(sig) => {
                client.confirm_transaction(&sig)?;
                println!("✅ Airdrop succeed！");
            },
            Err(_) => println!("⚠️ Airdrop failed (possible rate limiting), if subsequent attempts fail, please manually claim airdrop"),
        }
    }

    println!("😈 making SCAM token...");
    let mint_keypair = Keypair::new();
    let mint_pubkey = mint_keypair.pubkey();
    let mint_rent = client.get_minimum_balance_for_rent_exemption(Mint::LEN)?;

    let create_mint_ix = system_instruction::create_account(
        &my_pubkey, &mint_pubkey, mint_rent, Mint::LEN as u64, &spl_token::id(),
    );
    let init_mint_ix = token_instruction::initialize_mint(
        &spl_token::id(), &mint_pubkey, &my_pubkey, None, 2,
    )?;

    // create account
    let token_account_keypair = Keypair::new();
    let token_account_pubkey = token_account_keypair.pubkey();
    let acc_rent = client.get_minimum_balance_for_rent_exemption(spl_token::state::Account::LEN)?;

    let create_acc_ix = system_instruction::create_account(
        &my_pubkey, &token_account_pubkey, acc_rent, spl_token::state::Account::LEN as u64, &spl_token::id(),
    );
    let init_acc_ix = token_instruction::initialize_account(
        &spl_token::id(), &token_account_pubkey, &mint_pubkey, &my_pubkey,
    )?;
    //  666 
    let mint_to_ix = token_instruction::mint_to(
        &spl_token::id(), &mint_pubkey, &token_account_pubkey, &my_pubkey, &[], 66600,
    )?;

    println!("🗑️  create Empty Account...");
    let empty_acc_kp = Keypair::new();
    let create_empty_ix = system_instruction::create_account(
        &my_pubkey, &empty_acc_kp.pubkey(), acc_rent, spl_token::state::Account::LEN as u64, &spl_token::id(),
    );
    let init_empty_ix = token_instruction::initialize_account(
        &spl_token::id(), &empty_acc_kp.pubkey(), &mint_pubkey, &my_pubkey,
    )?;

    // send all trans
    let tx = Transaction::new_signed_with_payer(
        &[create_mint_ix, init_mint_ix, create_acc_ix, init_acc_ix, mint_to_ix, create_empty_ix, init_empty_ix],
        Some(&my_pubkey),
        &[&my_keypair, &mint_keypair, &token_account_keypair, &empty_acc_kp],
        client.get_latest_blockhash()?,
    );

    client.send_and_confirm_transaction(&tx)?;
    println!("✅ test env created!");
    println!("   - one spam (balance 666 SCAM)");
    println!("   - 1 empty account (balance 0)");
    println!("   now lets run main script to burn them！");

    Ok(())
}