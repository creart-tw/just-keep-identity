# Just Keep Identity (jki)
> **Extreme speed MFA & Identity Session Manager for CLI Power Users.**

![JKI Demo](docs/assets/demo.gif)

[English](README.md) / [繁體中文](README.zh-TW.md)

`jki` 是一個專為工程師設計的身份授權工具。我們不只是要管理 TOTP，我們是要在不離開終端機的前提下，以「毫秒級」的速度完成身份驗證。

## 核心哲學 (Philosophy)

*   **極速感 (Velocity)**: 查詢器 Cold Start < 3ms。
*   **Fuzzy Intelligence**: 支援模糊搜尋與匹配字元高亮顯示，即使記不清全名也能瞬間定位。
*   **Smart Agent**: 智慧背景代理，支援明文金庫自動解鎖與磁碟資料主動同步 (Active Reload)。
*   **物理隔離與安全**: 基於 `age` 加密，所有秘密僅存於本地或你的私有 Git，絕對拒絕雲端。
*   **人體工學 (Ergonomics)**: 專門優化的 Micro-Roll 指令集，單手即可完成產碼。

## 技術架構 (Technical DNA)

`jki` 採用 Rust 構建，追求極致的穩定性與安全性：

*   **智慧型代理 (Intelligent Agent)**: `jki-agent` 持有解密後的記憶體快取。它是系統中唯一與 OS Keyring 互動的門戶。
*   **雙模金庫 (Dual-Mode Vault)**: 
    *   `Plaintext Mode`: 追求極速，讀取本地加密環境下的明文快取。
    *   `Encrypted Mode`: 採用 `age` 加密，適合 Git 同步與長期儲存。
*   **Unix-Friendly**: 完美的管道支援 (`stdout -`)，輕鬆與 `ssh`, `git`, `kubectl` 等工具整合。

## 快速開始 (Quick Start)

```bash
# 查詢並複製 OTP (優先向 Agent 要，若無 Agent 則支援 master.key 或直接問密碼)
jki github

# 智慧過濾：搜尋 google 並直接選擇第 2 個結果
jki google 2

# 驗證過濾結果：列出搜尋結果而不執行
jki google 2 -l

# 快速同步金庫 (Git commit/pull/push)
jkim git sync
```

### 智慧過濾與選擇 (Smart Filtering & Selection)

`jki` 遵循「過濾 (Filter) -> 動作 (Action)」的邏輯鏈，讓你在複雜的帳號清單中如魚得水：

1.  **多重過濾**: `jki [PATTERNS]... [INDEX]`
    *   `jki u`：列出所有符合 `u` 的帳號 (如 Uber, Uplay)。
    *   `jki u 2`：直接獲取 `u` 搜尋結果中第 2 項的 OTP。
2.  **清單模式 (`-l, --list`)**: 
    *   任何時候加上 `-l` 都會將 `jki` 切換為「只列出、不執行」模式。
    *   這對於在大量結果中確認索引號 (`INDEX`) 非常有用。
3.  **無感報錯**: 搜尋結果不唯一時不再視為錯誤，而是優雅地列出候選清單並提示你如何精確選擇。

---

## 📦 安裝方式 (macOS)

```bash
# 複製並安裝
git clone https://github.com/creart-tw/just-keep-identity.git
cd just-keep-identity
make install
```

---

*Built with ❤️ for those who live in the terminal.*
