# Just Keep Identity (jki)
> **Extreme speed MFA & Identity Session Manager for CLI Power Users.**

`jki` 是一個專為工程師設計的身份授權工具。我們不只是要管理 TOTP，我們是要在不離開終端機的前提下，以「毫秒級」的速度完成身份驗證。

## 核心哲學 (Philosophy)

*   **極速感 (Velocity)**: 查詢器 Cold Start < 3ms。當你需要 OTP 時，它已經在你的剪貼簿裡了。
*   **Session 管理**: 透過背景 Agent 快取解密狀態，規避高昂的 KDF 開銷，實現「一次解鎖，全域極速」。
*   **認證隔離**: 
    *   **jki-agent** 負責整合系統 Keyring，實現背景自動化。
    *   **jki / jkim** 在獨立運作時保持純粹，僅透過檔案或直接輸入進行認證。
*   **人體工學 (Ergonomics)**: 專門優化的 **Micro-Roll** 指令集 (`j-k-i`)，單手即可完成查詢。
*   **可視化控制**: 在 macOS/Windows 提供選單列圖示，一眼看穿金庫狀態，隨手即可 Lock 或 Quit。

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

# 快速同步金庫
jkim sync

# 進入編輯模式 (自動開啟 $EDITOR)
jkim edit
```

---
*Built with ❤️ for those who live in the terminal.*
