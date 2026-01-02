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

// 强制使用 Devnet 进行测试
const RPC_URL: &str = "https://api.devnet.solana.com";
const KEYPAIR_PATH: &str = "id.json";

fn main() -> Result<()> {
    let client = RpcClient::new_with_commitment(RPC_URL, CommitmentConfig::confirmed());
    println!("🧪 正在初始化测试环境 (Devnet)...");

    // 1. 获取或创建钱包
    let my_keypair = if Path::new(KEYPAIR_PATH).exists() {
        read_keypair_file(KEYPAIR_PATH).unwrap()
    } else {
        let kp = Keypair::new();
        write_keypair_file(&kp, KEYPAIR_PATH).unwrap();
        println!("🆕 创建了新钱包 id.json");
        kp
    };
    let my_pubkey = my_keypair.pubkey();
    println!("📍 测试钱包: {}", my_pubkey);

    // 2. 检查余额 & 领水
    let balance = client.get_balance(&my_pubkey)?;
    if balance < LAMPORTS_PER_SOL / 2 {
        println!("💧 余额不足，正在申请空投...");
        match client.request_airdrop(&my_pubkey, LAMPORTS_PER_SOL) {
            Ok(sig) => {
                client.confirm_transaction(&sig)?;
                println!("✅ 空投成功！");
            },
            Err(_) => println!("⚠️ 空投失败 (可能是由于限流)，如果后续失败请手动领水。"),
        }
    }

    // 3. 制造一个“诈骗代币” (Scam Token)
    println!("😈 正在制造 'SCAM' 代币...");
    let mint_keypair = Keypair::new();
    let mint_pubkey = mint_keypair.pubkey();
    let mint_rent = client.get_minimum_balance_for_rent_exemption(Mint::LEN)?;

    let create_mint_ix = system_instruction::create_account(
        &my_pubkey, &mint_pubkey, mint_rent, Mint::LEN as u64, &spl_token::id(),
    );
    let init_mint_ix = token_instruction::initialize_mint(
        &spl_token::id(), &mint_pubkey, &my_pubkey, None, 2,
    )?;

    // 创建接收账户
    let token_account_keypair = Keypair::new();
    let token_account_pubkey = token_account_keypair.pubkey();
    let acc_rent = client.get_minimum_balance_for_rent_exemption(spl_token::state::Account::LEN)?;

    let create_acc_ix = system_instruction::create_account(
        &my_pubkey, &token_account_pubkey, acc_rent, spl_token::state::Account::LEN as u64, &spl_token::id(),
    );
    let init_acc_ix = token_instruction::initialize_account(
        &spl_token::id(), &token_account_pubkey, &mint_pubkey, &my_pubkey,
    )?;
    // 发 666 个币
    let mint_to_ix = token_instruction::mint_to(
        &spl_token::id(), &mint_pubkey, &token_account_pubkey, &my_pubkey, &[], 66600,
    )?;

    // 4. 再制造一个纯空账户 (Empty Account)
    println!("🗑️  正在制造纯空账户...");
    let empty_acc_kp = Keypair::new();
    let create_empty_ix = system_instruction::create_account(
        &my_pubkey, &empty_acc_kp.pubkey(), acc_rent, spl_token::state::Account::LEN as u64, &spl_token::id(),
    );
    let init_empty_ix = token_instruction::initialize_account(
        &spl_token::id(), &empty_acc_kp.pubkey(), &mint_pubkey, &my_pubkey,
    )?;

    // 发送所有交易
    let tx = Transaction::new_signed_with_payer(
        &[create_mint_ix, init_mint_ix, create_acc_ix, init_acc_ix, mint_to_ix, create_empty_ix, init_empty_ix],
        Some(&my_pubkey),
        &[&my_keypair, &mint_keypair, &token_account_keypair, &empty_acc_kp],
        client.get_latest_blockhash()?,
    );

    client.send_and_confirm_transaction(&tx)?;
    println!("✅ 测试环境构建完成！");
    println!("   - 1个诈骗账户 (余额 666 SCAM)");
    println!("   - 1个空账户 (余额 0)");
    println!("   现在运行主程序来清理它们吧！");

    Ok(())
}