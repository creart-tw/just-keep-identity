# **Just Keep Identity (jki)：JK Suite 極速 MFA 數位金庫**

## **產品需求 (PRD) 與技術規格 (Spec) 文件 - V27 (精確認證隔離版)**

### **第一章：品牌與核心原則 (Principles)**

#### **1.4 金庫狀態與認證邏輯 (Vault States & Auth Logic)**
*   **無感解鎖 (Lazy Unlock)**：`jki-agent` 啟動時預設為 Locked。當 `jki` 執行查詢時，若發現 Agent 已鎖定，應主動調用認證（如 Ask Pwd）並將解密結果傳遞給 Agent，實現 Session 快取。
*   **認證職責隔離 (Auth Separation)**：
    *   **Agent (High-Privilege)**：負責調用 OS Keyring (macOS Keychain / Windows Hello) 或互動認證。
    *   **CLI (Lightweight)**：獨立運作時支援「Plaintext > master.key > Ask Pwd」，**嚴禁**主動調用 OS Keyring。

### ---

**第二章：架構定義 (Architecture)**

#### **2.2 代理服務與可視化管理 (jki-agent)**
*   **定位**：唯一的 Session 管理器與高權限認證門戶。負責在記憶體中快取解密後的 Secrets 與 Master Key。
*   **跨平台形態**：macOS/Windows 提供選單列圖示 (Menu Bar)，Linux CLI 為純背景 Daemon。
*   **IPC 協議**：支援 `Ping`, `Unlock`, `GetOTP`, `GetMasterKey`, `Reload`, `Lock`, `Quit`。

#### **2.3 認證優先順序 (Authentication Priority)**

**A. 當 jki / jkim 執行時 (CLI Path):**
1.  **Agent IPC**：向背景 Agent 請求結果或 Master Key (最優先)。
2.  **Plaintext Vault**：讀取 `vault.secrets.json`。
3.  **Static Key File**：讀取 `$JKI_HOME/master.key` (0600)。
4.  **Interactive Prompt**：彈出 `Ask Pwd` 輸入。

**B. 當 jki-agent 啟動/解鎖時 (Agent Path):**
1.  **System Keyring**：嘗試獲取 `jki:master_key` (Silent Auth)。
2.  **Interactive Prompt**：彈出 `Ask Pwd`。
3.  **Static Key/Plain Path**：支援檔案系統路徑 (Dev/Test 模式)。

### ---

**第三章：安全硬化標準 (Security Hardening)**

#### **3.1 代理通訊安全**
*   Local Socket 必須強制執行 **0600 (Owner Only)** 權限，防止其他使用者截獲 Master Key 或 OTP。
*   Master Key 在 Agent 記憶體中應使用 `SecretString` (secrecy crate) 保護，防止 Swap 落地或 Memory Dump。

---
*Status: Architecture Baselined.*
