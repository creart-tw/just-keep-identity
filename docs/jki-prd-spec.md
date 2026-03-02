# **Just Keep Identity (jki)：JK Suite 極速 MFA 數位金庫**

## **產品需求 (PRD) 與技術規格 (Spec) 文件 - V21 (安全架構與交互細節同步版)**

### **第一章：品牌與體系定義 (Brand & System)**

#### **1.1 品牌命名決策**
*   **正式名稱**：**Just Keep Identity (jki)**。
*   **人體工學**：主打 **右手單手操作 (Micro-Roll)**，支援本地 (Desktop) 與遠端 (SSH/Headless) 環境。

#### **1.2 平台支援矩陣 (Support Matrix)**
*   **Tier 1 (Desktop)**：macOS, Windows。支援生體辨識 (TouchID / Hello ESS)。
*   **Tier 2 (Headless / SSH)**：支援 0600 金鑰檔案、環境變數與互動式密碼輸入。

### ---

**第二章：交互邏輯與 Unix 工具鏈規範 (CLI Standards)**

#### **2.1 查詢行為與一致性 (Consistency)**
*   **單一結果**：複製 OTP / 輸出 stdout。
*   **清單模式 (預設)**：僅顯示 Metadata，**不計算 OTP** 以確保極速與安全。
*   **不一致處理 (Missing Secrets)**：
    *   **預設行為**：偵測到 Metadata 有帳號但加密庫無 Secret 時，印出 Warning 並列出受影響項目。
    *   **安靜模式 (-q)**：自動過濾遺失密鑰的帳號，不顯示 Warning，僅從搜尋池中移除。

#### **2.2 參數規範**
*   `-q`: 安靜模式 (抑制 stderr 提示與一致性警告)。
*   `-`: stdout 模式 (純淨輸出 OTP)。
*   `--list`: 強制顯示匹配清單。
*   `-o / --otp`: 在清單模式下強制計算並顯示 OTP。
*   `--`: 終止選項解析。

### ---

**第三章：數據層與安全性 (Technical Spec)**

#### **3.1 認證體系 (Hardened Auth)**
*   **交互式輸入 (Indicator)**：實現「切換式狀態指示器」以防範長度洩漏並提供焦點回饋。
    *   空值：`[ _ ]`
    *   輸入/刪除動作：在 `[ * ]` 與 `[ x ]` 之間循環切換。
*   **優先順序 (Precedence)**：
    1.  **0600 靜態金鑰檔案** (預設 `~/.config/jki/master.key`)。
    2.  **Agent Session** (記憶體快取)。
    3.  **互動式詢問** (Stdin)。

#### **3.2 環境變數優先級**
*   `JKI_HOME`：全域根目錄覆寫。
*   `JKI_METADATA_PATH`：索引檔路徑覆寫。
*   `JKI_SECRETS_PATH`：加密秘密檔路徑覆寫。
*   `JKI_MASTER_KEY_PATH`：金鑰檔案路徑覆寫。

#### **3.3 資料拆分保護**
*   **`vault.metadata.json`**：僅含搜尋 Metadata，不含加密欄位。
*   **`vault.secrets.json.age`**：整包加密之秘密資料庫 (Passphrase-based age encryption)。

### ---

**第四章：實作路徑 (Roadmap)**

1.  **Phase 1: Foundation**：Workspace 建立、Python MVP 驗證 (Done)。
2.  **Phase 2: Core Executor (jki)**：Rust 實作、系統整合、資料拆分加密 (Done)。
3.  **Phase 3: Management (jkim)**：Git 整合 (init/sync/remote)、TUI 編輯器 (In Progress)。
4.  **Phase 4: Agent & IPC (jki-agent)**：跨平台 Socket、記憶體 Session 快取。
5.  **Phase 5: Refinement**：二進位優化 (rkyv)、WSL 橋接、安裝腳本。

### ---

**第五章：安全性硬化 (Security Hardening)**

#### **5.1 認證隔離**
*   `jki` 於 Standalone 模式下執行「啟動即解鎖、全量整合」，確保記憶體內資料完整性。
*   敏感資料欄位 (`secret`, `digits`, `algorithm`) 嚴禁出現於 `metadata.json` 中。

#### **5.2 記憶體防護**
*   使用 `SecretString` (secrecy) 保護 Master Key。
*   (待實作) `mlock` 防止交換與 `Zeroize` 主動抹除。
