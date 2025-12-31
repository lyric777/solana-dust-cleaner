use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use anyhow::Result;

const RPC_URL: &str = "https://api.devnet.solana.com";

fn main() -> Result<()> {
    println!("🚀 正在连接 Solana Devnet...");

    // 1. 创建一个 RPC 客户端
    // CommitmentConfig::confirmed() 意思是我们要确认交易至少被确认过
    let client = RpcClient::new_with_commitment(
        RPC_URL,
        CommitmentConfig::confirmed()
    );

    // 2. 获取当前区块链的版本
    let version = client.get_version()?;

    println!("✅ 连接成功！");
    println!("Solana 版本: {}", version.solana_core);

    // 3. 顺便查一下这上面现在的 Gas 费（Slot 高度）
    let block_height = client.get_block_height()?;
    println!("当前区块高度: {}", block_height);

    Ok(())
}