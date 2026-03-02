# **Just Keep Identity (jki)：JK Suite 極速 MFA 數位金庫**

## **產品需求 (PRD) 與技術規格 (Spec) 文件 - V20 (全平台適應與降級機制版)**

### **第一章：品牌與體系定義 (Brand & System)**

#### **1.1 品牌命名決策**
*   **正式名稱**：**Just Keep Identity (jki)**。
*   **人體工學**：主打 **右手單手操作**，支援本地與遠端開發環境。

#### **1.2 平台支援矩陣 (Support Matrix)**
*   **Tier 1 (Desktop)**：macOS, Windows。支援生體辨識 (TouchID / Hello ESS)。
*   **Tier 2 (Linux Local)**：支援 Secret Service (libsecret) 與 TPM 整合。
*   **Tier 3 (Headless / SSH)**：支援環境變數、手動密碼輸入與 Socket Forwarding。

### ---

**第二章：交互邏輯與 Unix 工具鏈規範 (CLI Standards)**

#### **2.1 查詢行為精煉**
*   **單一結果**：複製 OTP / 輸出 stdout。
*   **清單模式 (預設)**：僅顯示 Metadata，**不計算 OTP** 以確保極速與安全。
*   **清單模式 (手動)**：`-o` / `--otp` 強制計算並顯示所有 OTP。

#### **2.2 參數規範**
*   `-q`: 安靜模式。
*   `-`: stdout 模式。
*   `--list`: 強制顯示清單。
*   `--`: 終止解析。

### ---

**第三章：數據層與安全性 (Technical Spec)**

#### **3.1 認證雙軌制 (Dual Path Auth)**
*   **Path A: Static Key (0600 File)**：適用於自動化與遠端環境。
*   **Path B: Agent Session (Biometric)**：適用於桌面環境，Master Key 僅留存於記憶體。

#### **3.2 jkim 管理功能擴充**
*   **Key Setup**：建立靜態金鑰檔案並自動設置 0600 權限。
*   **Git Integration**：
    *   `jkim init`：初始化 JKI 倉庫、生成 .gitignore 與 .gitattributes。
    *   `jkim sync`：一鍵執行 Add/Commit/Pull/Push 同步流。
    *   `jkim remote`：轉接至原生 git remote 指令。
*   **Auth Status**：`jkim status` 回報目前的認證路徑、Git 同步狀態與 Agent 存活狀態。
*   **Session Logout**：`jkim logout` 通知 Agent 立即清除記憶體快取。

### ---

**第四章：實作路徑 (Roadmap)**

1.  **Phase 1: Foundation**：Workspace 建立、Python MVP 驗證 (Done)。
2.  **Phase 2: Core Executor (jki)**：Rust 實作、系統整合、參數解析 (In Progress)。
3.  **Phase 3: Agent & IPC (jki-agent)**：跨平台 Socket、認證降級路徑、ID-based 協議。
4.  **Phase 4: Management (jkim)**：TUI 管理、數據優化與加密轉檔。
5.  **Phase 5: Remote & Linux**：SSH Socket Forwarding 驗證、Linux 系統整合。

### ---

**第五章：安全性硬化 (Security Hardening)**

#### **5.1 認證隔離**
*   `jki` (Client) 僅處理搜尋與 Metadata。
*   `jki-agent` (Server) 負責保管 Master Key 與計算 OTP。
*   **ID 通訊**：Client 傳遞 UUID，Server 回傳單一 OTP。

#### **5.2 記憶體防護**
*   敏感數據使用 `Zeroize` 抹除。
*   支援 `mlock` 防止交換至磁碟。
