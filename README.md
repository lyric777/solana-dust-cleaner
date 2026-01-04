# 🧹 Solana Dust Cleaner (Rust CLI)

**English** | [中文](README_CN.md)

> A high-performance, safe, and open-source CLI tool to reclaim rent (SOL) from empty token accounts and burn scam tokens on Solana.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Language](https://img.shields.io/badge/language-Rust-orange.svg)
![Platform](https://img.shields.io/badge/platform-Solana-purple.svg)

## 🚀 Why use this?

On Solana, every token account requires a small amount of SOL (rent) to be stored on-chain (approx. ~0.002 SOL per account). Over time, your wallet gets cluttered with:
1.  **Empty Accounts:** Old tokens you sold/transferred but the accounts remain open.
2.  **Scam Tokens:** "Dust" or spam tokens airdropped to you that you can't easily remove.

**Solana Dust Cleaner** helps you:
* 🔥 **Burn** unwanted/scam tokens.
* 💰 **Close** token accounts to reclaim your SOL rent.
* 🛡️ **Safe:** Atomic transactions ensure you either succeed fully or change nothing. No partial failures.
* ⚡ **Fast:** Written in Rust, scans and cleans in seconds.

---

## 📦 Installation

### Option 1: Download Binary (Recommended)
Go to the [Releases](https://github.com/lyric777/solana-dust-cleaner/releases) page and download the executable for your OS (Windows/Mac/Linux).

### Option 2: Build from Source
Ensure you have Rust installed.

```bash
git clone https://github.com/lyric777/solana-dust-cleaner.git
cd solana-dust-cleaner
cargo build --release
```
The binary will be in target/release/solana-dust-cleaner.

## 🛠 Usage
By default, the tool connects to Solana Mainnet and looks for id.json in the current directory.

### 1. Dry Run (Scan Only)
Check how much SOL you can reclaim without executing any transactions.
```bash
./solana-dust-cleaner
```
### 2. Execute Cleanup
Actually burn tokens and close accounts. Requires confirmation.
```bash
./solana-dust-cleaner --clean
```
### 3. Custom RPC & Keypair
If you are using a custom RPC (recommended for speed) or a different keyfile:
```bash
./solana-dust-cleaner --keypair /path/to/mainnet.json --rpc [https://your-helius-rpc.com] --clean
```
### 4. Skip Confirmation (For Scripts)
⚠️ Dangerous! Use only if you know what you are doing.
```bash
./solana-dust-cleaner --clean --yes
```
## 🧪 Testing on Devnet
Want to try it safely first?
Generate Test Data: We provide a script to create a fresh wallet, airdrop SOL, and create spam tokens on Devnet.
```bash
cargo run --bin create-spam
```
Clean it up:
```bash
cargo run --bin solana-dust-cleaner -- --rpc https://api.devnet.solana.com --clean
```
## ⚠️ Disclaimer
This software is provided "as is", without warranty of any kind.

This tool BURNS tokens. Once burnt, they are gone forever.

Always run a Dry Run (without --clean) first to see what will be removed.

Use at your own risk.

## ☕ Support
If this tool helped you, star this repo on GitHub! ⭐

(or feel free to buy me a coffee! SOL Address: 7yc6YrCWR7p5sTUZiiwmkE9RVuhACdSgr9GqNqZCk6pY)
