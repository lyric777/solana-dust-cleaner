# 🧹 Solana Dust Cleaner (Rust CLI)

**中文文档** | [English](README.md)

> 一个高性能、安全且开源的 Rust 命令行工具，用于销毁 Solana 上的垃圾代币并回收闲置账户的租金 (SOL)。

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Language](https://img.shields.io/badge/language-Rust-orange.svg)
![Platform](https://img.shields.io/badge/platform-Solana-purple.svg)

## 🚀 为什么要用这个工具？

在 Solana 网络上，每一个 Token 账户都需要占用链上存储空间，因此需要抵押少量的 SOL 作为“租金”（Rent），每个账户大约占用 **~0.002 SOL**。

随着时间的推移，活跃用户的钱包里往往会堆积大量无用的账户：
1.  **闲置空账户：** 你已经卖出或转走了代币，但剩下的空账户依然在占用你的 SOL。
2.  **诈骗/垃圾代币：** 莫名其妙空投到你钱包里的垃圾币 (Dust/Spam)，无法轻易删除。

**Solana Dust Cleaner** 可以帮你：
* 🔥 **销毁 (Burn)** 那些看着心烦的垃圾代币。
* 💰 **关闭 (Close)** Token 账户，把抵押的 SOL 租金退回你的钱包。
* 🛡️ **安全原子性：** 采用原子交易，要么全部成功，要么什么都不发生，不会出现“币毁了但钱没退”的中间状态。
* ⚡ **极速：** 基于 Rust 编写，扫描和清理在几秒内完成。

---

## 📦 安装

### 方法 1: 下载可执行文件 (推荐普通用户)
前往 [Releases](../../releases) 页面，下载对应你系统 (Windows/Mac/Linux) 的文件即可直接运行。

### 方法 2: 源码编译 (推荐开发者)
确保你已经安装了 [Rust](https://www.rust-lang.org/) 环境。

```bash
git clone [https://github.com/lyric777/solana-dust-cleaner.git](https://github.com/lyric777/solana-dust-cleaner.git)
cd solana-dust-cleaner
cargo build --release