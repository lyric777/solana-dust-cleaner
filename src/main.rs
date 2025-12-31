use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer, read_keypair_file, write_keypair_file},
    native_token::LAMPORTS_PER_SOL,
    system_instruction,
    transaction::Transaction,
    pubkey::Pubkey,
    program_pack::Pack,
};
use spl_token::state::Account as TokenAccount;
use anyhow::Result;
use std::path::Path;
use std::str::FromStr; // <--- 新增：用于解析字符串地址

const RPC_URL: &str = "https://api.devnet.solana.com";
const KEYPAIR_PATH: &str = "id.json";

fn main() -> Result<()> {
    let client = RpcClient::new_with_commitment(RPC_URL, CommitmentConfig::confirmed());
    println!("📡 连接 Devnet 成功");

    // 读取钱包
    let my_keypair = if Path::new(KEYPAIR_PATH).exists() {
        read_keypair_file(KEYPAIR_PATH).map_err(|_| anyhow::anyhow!("无法读取 id.json"))?
    } else {
        let kp = Keypair::new();
        write_keypair_file(&kp, KEYPAIR_PATH).map_err(|_| anyhow::anyhow!("无法写入"))?;
        kp
    };
    let my_pubkey = my_keypair.pubkey();

    // 检查余额
    let start_balance = client.get_balance(&my_pubkey)?;
    println!("💰 当前余额: {:.4} SOL", start_balance as f64 / LAMPORTS_PER_SOL as f64);

    if start_balance < LAMPORTS_PER_SOL / 10 {
        println!("❌ 余额不足，请去领水！");
        return Ok(());
    }

    // --- 核心逻辑：制造垃圾 ---
    println!("---------------------------------------------------");
    println!("🗑️  准备制造 3 个闲置空账户...");
    
    let token_program_id = spl_token::id();
    
    // 🔥 [修正点]：使用真实的 Wrapped SOL Mint 地址
    // 这个地址在 Devnet 和 Mainnet 都是一样的，永远有效
    let valid_mint = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();

    let space = TokenAccount::LEN;
    let rent_lamports = client.get_minimum_balance_for_rent_exemption(space)?;
    println!("ℹ️  单账户租金成本: {:.5} SOL", rent_lamports as f64 / LAMPORTS_PER_SOL as f64);

    for i in 1..=3 {
        let new_token_account = Keypair::new();
        
        let create_ix = system_instruction::create_account(
            &my_pubkey,
            &new_token_account.pubkey(),
            rent_lamports,
            space as u64,
            &token_program_id,
        );

        let init_ix = spl_token::instruction::initialize_account(
            &token_program_id,
            &new_token_account.pubkey(),
            &valid_mint, // <--- 这里换成了真实的 Mint
            &my_pubkey,
        )?;

        let mut tx = Transaction::new_with_payer(
            &[create_ix, init_ix],
            Some(&my_pubkey),
        );
        
        let recent_blockhash = client.get_latest_blockhash()?;
        tx.sign(&[&my_keypair, &new_token_account], recent_blockhash);

        print!("   [#{}] 创建中... ", i);
        match client.send_and_confirm_transaction(&tx) {
            Ok(_) => println!("✅ 成功! 地址: {:?}", new_token_account.pubkey()),
            Err(e) => println!("❌ 失败: {}", e),
        }
    }

    let end_balance = client.get_balance(&my_pubkey)?;
    let lost = start_balance - end_balance;

    println!("---------------------------------------------------");
    println!("📉 制造垃圾完毕！");
    println!("💰 最新余额: {:.4} SOL", end_balance as f64 / LAMPORTS_PER_SOL as f64);
    println!("💸 为了这 3 个垃圾账号，你一共被锁定了: {:.5} SOL", lost as f64 / LAMPORTS_PER_SOL as f64);
    println!("---------------------------------------------------");

    Ok(())
}