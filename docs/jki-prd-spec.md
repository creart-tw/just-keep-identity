# **Just Keep Identity (jki)：JK Suite 極速 MFA 數位金庫**

## **產品需求 (PRD) 與技術規格 (Spec) 文件 - V25 (產品化與可靠性版)**

### **第一章：品牌與核心原則 (Principles)**

#### **1.1 品牌定位**
*   **正式名稱**：**Just Keep Identity (jki)**。
*   **設計語義**：主打 **右手單手操作 (Micro-Roll)**，在 CLI 環境下追求極致的流暢感。

#### **1.2 三大底線 (The Three Bottom Lines)**
1.  **安全性 (Security)**：敏感資料嚴禁明文落地，密碼輸入防範長度洩漏。
2.  **透明度 (Transparency)**：任何具備副作用的初始化動作必須清晰回報狀態（Skipped/Created/Updated），嚴禁靜默成功。
3.  **簡潔性 (Simplicity)**：遵循 Unix 哲學，Fail-fast 設計，不實作過度複雜的重試機制。

#### **1.3 搜尋哲學：欄位隔離 (Field Isolation)**
*   **物理邊界 (Physical Boundary)**：單一關鍵字 (Pattern) 必須完全落在「單一欄位」內（Issuer 或 Account Name）。嚴禁單一關鍵字跨越欄位邊界匹配（例如 `gh` 不能一半匹配 `[G]oogle` 一半匹配 `lic[h]ih`），以消除高密度資料下的雜訊。
*   **多維交集 (Multi-pattern AND Logic)**：空格分隔的多個關鍵字採交集 (AND) 邏輯。例如 `jki g li` 表示「(關鍵字 g 存在於任一欄位) AND (關鍵字 li 存在於任一欄位)」。這能讓使用者透過極短的字元組合精準定位身份。

#### **1.4 金庫狀態與極速路徑 (Vault States & Speed Path)**
*   **狀態靈活性**：JKI 支援「透明脫殼模式」。使用者可選擇將金庫解密為明文存放在磁碟上，以換取極致的查詢效能。
*   **狀態定義**：
    *   **Encrypted (標準)**：磁碟僅存 `.age` 加密檔，需密碼、Keychain 或 Agent。
    *   **Plaintext (極速)**：磁碟存在 `vault.secrets.json` 明文檔。`jki` 應自動偵測並優先讀取此檔，繞過所有身份驗證與 IPC 流程，達成真正的零延遲。
*   **狀態轉換 (Transition)**：轉換應具備「偏好感知 (Preference-Aware)」。`jkim decrypt` 預設路徑為「刪除加密來源」但「保留 Master Key」，以在確保資料權威性之餘，保留自動化封裝的能力。系統應透過具備預設值的詢問引導使用者，並支援全局預設 Flag (`-d, --default`)。

### ---

**第二章：架構定義 (Architecture)**

#### **2.1 資料拆分保護模型**
*   **索引層 (`vault.metadata.json`)**：僅含搜尋用的 Metadata（Name, Issuer, ID），完全不含加密欄位。
*   **機密層 (`vault.secrets.bin.age`)**：採用 `age` 整包加密之二進位秘密資料庫。
*   **關聯鍵 (`ID`)**：兩層資料透過隨機生成的 ID 進行關聯整合。

#### **2.2 認證層級與優先順序**
詳細指令與參數請參閱：[CLI 指令手冊 (jki-cli-spec.md)](jki-cli-spec.md)
1.  **強制互動模式** (`-I`)。
2.  **系統金鑰鏈** (macOS Keychain / Windows Credential Manager)。
3.  **0600 靜態金鑰檔案** (`master.key`)。
4.  **Agent Session** (記憶體快取，需啟動 `jki-agent`)。
5.  **自動回退互動式詢問** (Stdin)。

### ---

**第三章：安全硬化標準 (Security Hardening)**

#### **3.1 檔案保護**
*   磁碟上的金鑰檔 (`master.key`) 強制執行 **0600 (Owner Only)** 權限檢查。
*   初始化時自動產生 `.gitignore` 排除所有敏感檔案。

#### **3.2 代理服務隔離 (jki-agent)**
*   採用跨平台 Local Sockets 進行通訊。
*   解鎖後的 Secrets 僅存在於 Agent 記憶體中，CLI 透過 IPC 向其請求 OTP 生成（提升密鑰安全性）。

### ---

**第四章：實作路徑 (Roadmap)**

1.  **Phase 1: Foundation**：WORKSPACE 建立 (Done)。
2.  **Phase 2: Core Executor (jki)**：Rust 實作、資料拆分加密 (Done)。
3.  **Phase 3: Management (jkim)**：Git 同步、編輯器模式、Master Key 工具集 (Done)。
4.  **Phase 4: Agency & Key Caching (jki-agent)**：實作 Agent 快取機制與搜尋優先序重構 (Done)。
5.  **Phase 5: Productization & Reliability**：系統 Keychain 整合、匯出工具、安裝腳本與單元測試強化 (Done)。

---
*詳細操作範例與輸出格式請參考 `jki-cli-spec.md`。*
