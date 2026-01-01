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
use solana_account_decoder::UiAccountData; // 用于识别数据格式
use anyhow::Result;
use std::str::FromStr;

const RPC_URL: &str = "https://api.devnet.solana.com";
const KEYPAIR_PATH: &str = "id.json";

fn main() -> Result<()> {
    // 1. 初始化
    let client = RpcClient::new_with_commitment(RPC_URL, CommitmentConfig::confirmed());
    println!("📡 连接 Devnet 成功");

    let my_keypair = read_keypair_file(KEYPAIR_PATH)
        .map_err(|_| anyhow::anyhow!("找不到 id.json"))?;
    let my_pubkey = my_keypair.pubkey();

    let start_balance = client.get_balance(&my_pubkey)?;
    println!("💰 当前余额: {:.5} SOL", start_balance as f64 / LAMPORTS_PER_SOL as f64);
    println!("---------------------------------------------------");
    println!("🔍 正在全网扫描你的 Token 账户...");

    // 2. 获取所有 Token 账户 (使用标准方法)
    let all_accounts = client.get_token_accounts_by_owner(
        &my_pubkey,
        TokenAccountsFilter::ProgramId(spl_token::id()),
    )?;

    println!("📊 扫描完毕，发现一共有 {} 个账户", all_accounts.len());

    // 3. 筛选出可以回收的账户
    let mut accounts_to_close = vec![];

    for keyed_account in all_accounts {
        let account_pubkey = Pubkey::from_str(&keyed_account.pubkey)?;
        
        // --- 核心修复：智能判断数据格式 ---
        // Solana 有时候返回二进制，有时候返回 JSON，我们两个都处理
        let is_empty_account = match keyed_account.account.data {
            // 情况 A: 返回的是 JSON 格式 (Parsed)
            UiAccountData::Json(parsed_account) => {
                // 我们深入 JSON 结构去找 "amount" 字段
                // 结构通常是: parsed_account.parsed["info"]["tokenAmount"]["amount"]
                let amount_str = parsed_account.parsed
                    .get("info")
                    .and_then(|info| info.get("tokenAmount"))
                    .and_then(|amt| amt.get("amount"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("1"); // 如果找不到，就默认当成 1 (不处理)，防止误删
                
                amount_str == "0"
            },
            // 情况 B: 返回的是二进制格式 (LegacyBinary / Binary)
            UiAccountData::Binary(ref data, _) | UiAccountData::LegacyBinary(ref data) => {
                 // 解码 Base64/Base58 字符串为字节数组
                 if let Some(bytes) = keyed_account.account.data.decode() {
                     if let Ok(token_account) = TokenAccount::unpack(&bytes) {
                         token_account.amount == 0
                     } else { false }
                 } else { false }
            },
        };

        if is_empty_account {
            accounts_to_close.push(account_pubkey);
            println!("   [✅ 发现猎物] 地址: {}... | 余额: 0 (待回收)", &account_pubkey.to_string()[0..8]);
        }
    }

    if accounts_to_close.is_empty() {
        println!("✅ 没有发现闲置账户。");
        return Ok(());
    }

    println!("---------------------------------------------------");
    println!("🔥 准备回收 {} 个账户的租金...", accounts_to_close.len());

    // 4. 构建批量回收指令
    let mut instructions = vec![];
    
    for account_pubkey in &accounts_to_close {
        let close_ix = spl_token::instruction::close_account(
            &spl_token::id(),
            account_pubkey,
            &my_pubkey, // 钱退给你
            &my_pubkey, // 你签名
            &[],
        )?;
        instructions.push(close_ix);
    }

    // 5. 发送交易
    let mut tx = Transaction::new_with_payer(
        &instructions,
        Some(&my_pubkey),
    );
    
    let recent_blockhash = client.get_latest_blockhash()?;
    tx.sign(&[&my_keypair], recent_blockhash);

    println!("🚀 发送交易中...");
    match client.send_and_confirm_transaction(&tx) {
        Ok(sig) => println!("✅ 回收成功! 交易哈希: {}", sig),
        Err(e) => println!("❌ 交易失败: {}", e),
    }

    // 6. 最终算账
    let final_balance = client.get_balance(&my_pubkey)?;
    let profit = final_balance - start_balance;

    println!("---------------------------------------------------");
    println!("💰 回收后余额: {:.5} SOL", final_balance as f64 / LAMPORTS_PER_SOL as f64);
    println!("🎉 恭喜！你刚刚赚回了: {:.5} SOL", profit as f64 / LAMPORTS_PER_SOL as f64);
    println!("---------------------------------------------------");

    Ok(())
}