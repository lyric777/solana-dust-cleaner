use clap::Parser; 
use solana_client::{
    rpc_client::RpcClient,
    rpc_request::TokenAccountsFilter,
};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{read_keypair_file, Signer},
    native_token::LAMPORTS_PER_SOL,
    transaction::Transaction,
    pubkey::Pubkey,
    program_pack::Pack,
};
use spl_token::state::Account as TokenAccount;
use solana_account_decoder::UiAccountData;
use anyhow::Result;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to your wallet keypair file
    #[arg(short, long, default_value = "id.json")]
    keypair: String,

    /// RPC URL (Defaults to Public RPC)
    #[arg(short, long, default_value = "https://api.mainnet-beta.solana.com")]
    rpc: String,

    /// EXECUTE the cleanup (Burn tokens & Close accounts). 
    /// If not provided, runs in "Dry Run" mode (scan only).
    #[arg(long, default_value_t = false)]
    clean: bool,

    /// Skip the confirmation prompt (dangerous!)
    #[arg(long, default_value_t = false)]
    yes: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let client = RpcClient::new_with_commitment(&args.rpc, CommitmentConfig::confirmed());
    
    let my_keypair = read_keypair_file(&args.keypair)
        .map_err(|e| anyhow::anyhow!("Failed to read keypair file '{}': {}", args.keypair, e))?;
    let my_pubkey = my_keypair.pubkey();

    println!("=======================================================");
    println!("🧹 SOLANA DUST CLEANER v0.1.0");
    println!("=======================================================");
    println!("📡 RPC URL: {}", args.rpc);
    println!("mw Wallet : {}...", &my_pubkey.to_string()[0..8]);
    
    let start_balance = client.get_balance(&my_pubkey)?;
    println!("💰 Balance: {:.5} SOL", start_balance as f64 / LAMPORTS_PER_SOL as f64);
    println!("-------------------------------------------------------");

    if !args.clean {
        println!("ℹ️  MODE: DRY RUN (Scanning only, no actions taken)");
        println!("   Use '--clean' to execute the reclamation.");
    } else {
        println!("⚠️  MODE: EXECUTE (Burning tokens and closing accounts)");
    }
    println!("-------------------------------------------------------");
    println!("🔍 Scanning for token accounts...");

    let all_accounts = client.get_token_accounts_by_owner(
        &my_pubkey,
        TokenAccountsFilter::ProgramId(spl_token::id()),
    )?;

    let mut instructions = vec![];
    let mut burn_count = 0;
    let mut close_count = 0;
    let mut expected_reclaim = 0.0;

    for keyed_account in all_accounts {
        let account_pubkey = Pubkey::from_str(&keyed_account.pubkey)?;
        
        let rent = client.get_account(&account_pubkey)?.lamports;

        let (amount, mint) = match keyed_account.account.data {
            UiAccountData::Binary(ref _data, _) | UiAccountData::LegacyBinary(ref _data) => {
                 if let Some(bytes) = keyed_account.account.data.decode() {
                     if let Ok(acc) = TokenAccount::unpack(&bytes) {
                         (acc.amount, acc.mint)
                     } else { (0, Pubkey::default()) }
                 } else { (0, Pubkey::default()) }
            },
            UiAccountData::Json(parsed) => {
                let info = parsed.parsed.get("info");
                let amt = info
                    .and_then(|i| i.get("tokenAmount"))
                    .and_then(|t| t.get("amount"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("0")
                    .parse::<u64>()
                    .unwrap_or(0);
                let mint_str = info
                    .and_then(|i| i.get("mint"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                (amt, Pubkey::from_str(mint_str).unwrap_or_default())
            }
        };

        if amount > 0 {
            println!("   [Found Dust] {} | Balance: {} | Action: Burn & Close", &account_pubkey.to_string()[0..8], amount);
            if args.clean {
                let burn_ix = spl_token::instruction::burn(
                    &spl_token::id(),
                    &account_pubkey,
                    &mint,
                    &my_pubkey,
                    &[],
                    amount,
                )?;
                instructions.push(burn_ix);
            }
            burn_count += 1;
        } else {
            println!("   [Found Idle] {} | Balance: 0 | Action: Close", &account_pubkey.to_string()[0..8]);
        }

        if args.clean {
            let close_ix = spl_token::instruction::close_account(
                &spl_token::id(),
                &account_pubkey,
                &my_pubkey,
                &my_pubkey,
                &[],
            )?;
            instructions.push(close_ix);
        }
        close_count += 1;
        expected_reclaim += rent as f64 / LAMPORTS_PER_SOL as f64;
    }

    if close_count == 0 {
        println!("✅ No dust accounts found. Your wallet is clean!");
        return Ok(());
    }

    println!("-------------------------------------------------------");
    println!("📊 Summary:");
    println!("   Accounts to close: {}", close_count);
    println!("   Tokens to burn   : {}", burn_count);
    println!("   Est. Reclaimable : ~{:.5} SOL", expected_reclaim);

    if !args.clean {
        println!("-------------------------------------------------------");
        println!("💡 To perform the cleanup, run:");
        println!("   cargo run -- --clean");
        return Ok(());
    }

    if !args.yes {
        println!("-------------------------------------------------------");
        println!("⚠️  WARNING: YOU ARE ABOUT TO BURN TOKENS ON: {}", args.rpc);
        println!("   This action is IRREVERSIBLE.");
        println!("   Are you sure you want to continue? (Type 'yes' to confirm)");
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        
        if input.trim() != "yes" {
            println!("❌ Operation cancelled by user.");
            return Ok(());
        }
    }

    println!("-------------------------------------------------------");
    println!("🚀 Executing cleanup transaction...");

    let tx = Transaction::new_signed_with_payer(
        &instructions,
        Some(&my_pubkey),
        &[&my_keypair],
        client.get_latest_blockhash()?,
    );

    match client.send_and_confirm_transaction(&tx) {
        Ok(sig) => {
            println!("✅ Success! Transaction Signature:");
            println!("   {}", sig);
            
            let final_balance = client.get_balance(&my_pubkey)?;
            let profit = final_balance as i64 - start_balance as i64;
            
            println!("-------------------------------------------------------");
            println!("🎉 Final Balance: {:.5} SOL", final_balance as f64 / LAMPORTS_PER_SOL as f64);
            println!("💰 Net Change   : {:.5} SOL", profit as f64 / LAMPORTS_PER_SOL as f64);
        },
        Err(e) => {
            println!("❌ Transaction Failed: {}", e);
        }
    }

    Ok(())
}