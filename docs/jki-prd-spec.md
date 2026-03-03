# **Just Keep Identity (jki)：JK Suite 極速 MFA 數位金庫**

## **產品需求 (PRD) 與技術規格 (Spec) 文件 - V28 (統一 Biometric 認證版)**

### **第一章：品牌與核心原則 (Principles)**

#### **1.4 金庫狀態與認證邏輯 (Vault States & Auth Logic)**
*   **無感解鎖 (Lazy Unlock)**：`jki-agent` 啟動時預設為 Locked。當 `jki` 執行查詢時，若發現 Agent 已鎖定，應主動調用認證並將解密結果傳遞給 Agent。
*   **認證職責隔離 (Auth Separation)**：
    *   **Agent (High-Privilege)**：負責調用 **Biometric (OS 原生生物辨識)** 或互動認證。
    *   **CLI (Lightweight)**：獨立運作時不調用系統級安全框架，僅限檔案或直接互動。

### ---

**第二章：架構定義 (Architecture)**

#### **2.2 代理服務與可視化管理 (jki-agent)**
*   **定位**：唯一的 Session 管理器與高權限認證門戶。
*   **跨平台形態**：macOS/Windows 提供選單列圖示 (Menu Bar)，Linux CLI 為純背景 Daemon。
*   **IPC 協議**：支援 `Ping`, `Unlock`, `GetOTP`, `GetMasterKey`, `Reload`, `Lock`, `Quit`。

#### **2.3 權威來源旗標 (-A, --auth)**
為顯式指定認證來源並實現 Fail-fast 策略，所有組件支援 `-A, --auth <SOURCE>` 參數。

| 參數值 (`-A`) | 行為 (Behavior) | 適用組件 |
| :--- | :--- | :--- |
| **`biometric`** | 強制調用 **OS 原生生物辨識** (macOS TouchID / Windows Hello)。 | `agent` |
| **`agent`** | 強制僅向 `jki-agent` 請求 (Session 快取)。 | `jki`, `jkim` |
| **`plain`** | 強制僅讀取 `vault.json` (零延遲明文)。 | 全組件 |
| **`mkey`** | 強制僅讀取物理 `master.key` 檔案 (0600)。 | 全組件 |
| **`interactive`** | 強制 Stdin 互動輸入 (Ask Pwd)。別名 `-I`。 | 全組件 |

**認證優先序路徑 (Default Priority Path):**
*   **CLI Path**: `Agent` > `Plain` > `MasterKey` > `Interactive`.
*   **Agent Path**: `Biometric` > `MasterKey` > `Interactive`.

### ---

**第三章：安全硬化標準 (Security Hardening)**

#### **3.1 代理通訊安全**
*   Local Socket 必須強制執行 **0600 (Owner Only)** 權限。
*   Master Key 在 Agent 記憶體中應使用 `SecretString` 保護。

---
*Status: Architecture Baselined (V28 - Unified Biometric).*
