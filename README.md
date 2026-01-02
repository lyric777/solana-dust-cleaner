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
git clone [https://github.com/lyric777/solana-dust-cleaner.git](https://github.com/lyric777/solana-dust-cleaner.git)
cd solana-dust-cleaner
cargo build --release