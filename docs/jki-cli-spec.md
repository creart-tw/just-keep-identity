# **Just Keep Identity (jki)：CLI 指令手冊 (Command-Line Interface Spec)**

這份文件詳細記載 `jki` 與 `jkim` 的所有指令、旗標與預期行為，作為實作與測試的依準。

---

## **1. 全域旗標 (Global Flags)**

適用於所有 `jki` 與 `jkim` 指令：
*   `-I, --interactive`: 強制互動模式。忽略系統 Keychain、磁碟上的 `master.key` 與 Agent 快取，直接開啟密碼輸入。
*   `-q, --quiet`: 安靜模式。抑制 stderr 的提示、進度訊息與非關鍵警告。
*   `-d, --default`: 自動模式。對於所有具備建議偏好的詢問（如狀態轉換、匯入衝突），自動套用系統推薦行為，不再進行互動詢問。

---

## **1.1 認證優先序 (Authentication Priority)**

當指令需要解鎖加密金庫（`.age`）時，依序嘗試以下來源：
1.  **Interactive Override**: 若開啟 `-I` 旗標，跳過所有儲存點，直接要求輸入。
2.  **System Keychain**: 嘗試從 macOS Keychain 或 Windows Credential Manager 獲取 `jki:master_key`。
3.  **Master Key File**: 檢查 `$JKI_HOME/master.key` (需具備 0600 權限)。
4.  **Agent IPC**: 請求背景 `jki-agent` 協助解密或生成 OTP。
5.  **Interactive Prompt**: 若上述皆不可用，開啟密碼輸入介面。

---

## **2. 執行器：jki**

### **2.1 搜尋與 OTP 生成 (預設行為)**
`jki [PATTERNS]... [INDEX]`

#### **搜尋優先序 (Search Priority)**
為達成零延遲查詢，`jki` 依序嘗試以下路徑，一旦成功即回傳結果：
1.  **Plaintext Path**: 讀取 `vault.secrets.json` (不需任何認證，極速)。
2.  **Agent Path**: 透過 IPC 請求 `jki-agent` (需 Socket 連接，不需本地解密)。
3.  **Static Key Path**: 使用 `master.key` 解鎖 `.age` (自動解密)。
4.  **Interactive Path**: 提示輸入 Master Key 解鎖。

#### **搜尋邏輯 (Matching Logic)**
*   **PATTERNS**: 多關鍵字篩選。
    *   **欄位隔離**：每個關鍵字必須在 `Issuer` 或 `Account Name` 其中之一完整匹配。
    *   **多維交集**：多個關鍵字之間為 AND 邏輯。
*   **INDEX**: 當多個結果時，可指定數字選取。

#### **旗標**
*   `-s, --stdout`: 直接在 stdout 印出 OTP（預設為複製到剪貼簿）。
*   `-`: 等同於 `--stdout`。
*   `-l, --list`: 強制顯示匹配清單，即使只有一個結果。
*   `-o, --otp`: 在清單模式下顯示 OTP。
*   `--force-agent`: 強制僅透過 Agent 查詢，若 Agent 未啟動則失敗（防止意外的本地解密開銷）。

---

## **3. 管理中心：jkim**

### **3.1 狀態檢查 (status)**
`jkim status`
*   檢查環境健康狀況，回報以下資訊：
    *   **Master Key File**: 檢查檔案是否存在且具備 0600 安全權限。
    *   **System Keychain**: 檢查系統 Keychain 中是否存有 `jki:master_key`。
    *   **jki-agent**: 檢查背景 Agent 是否運行及其 PID。
    *   **Git Repository**: 檢查 Git 分支、工作目錄清潔度與遠端設定。

### **3.2 環境初始化 (init)**
`jkim init [--force]`
*   **預設行為 (Transparent Init)**：檢查環境並回報狀態。若有衝突則提示警告。
*   **旗標**:
    *   `-f, --force`: **重置模式**。刪除現有的金庫資料，重新建立乾淨環境。

### **3.2 金鑰管理 (master-key)**
`jkim master-key <SUBCOMMAND>`
*   `set [--force] [--keychain]`:
    *   將 Master Key 寫入 `master.key` (0600)。
    *   `--keychain` (預設為 true)：同時將金鑰存入系統 Keychain (`jki:master_key`)。
*   `remove [--force] [--keychain]`:
    *   刪除磁碟上的 `master.key`。
    *   `--keychain`：同時從系統 Keychain 移除金鑰。
*   `change [--commit]`:
    *   重新加密金庫並變更金鑰。同時自動更新系統 Keychain。

### **3.3 資料管理 (Vault Management)**
*   **decrypt**: 將金庫解密為明文 (`vault.secrets.json`)。
    *   支援 `-d, --default` 套用推薦路徑：**刪除 .age，保留 master.key**。
*   **encrypt**: 將明文金庫重新封裝為加密檔 (`.age`) 並物理刪除明文。
*   **import-winauth <FILE>**:
    *   **狀態感知**：若存在加密金庫且具備 `master.key`，匯入後應自動壓回加密檔。
    *   支援 `-y, --yes` (或 `-d`) 跳過衝突確認。
*   **export [--output <FILE>]**:
    *   將所有帳號匯出為密碼保護的 ZIP 檔案。
    *   包含 `accounts.txt` (OTPAuth URI 格式)，採用 AES-256 加密。

### **3.4 資料編輯 (edit)**
`jkim edit`
*   採用 `$EDITOR` 流程。儲存後執行 JSON Schema 驗證。
*   **狀態優先**：若存在明文金庫，優先編輯明文；否則編輯加密金庫。

### **3.5 同步 (sync)**
`jkim sync`
*   執行 Git 原子化備份：`add` -> `commit` -> `pull --rebase` -> `push`。
*   **安全隔離**：明文金庫 (`.json`) 必須被 `.gitignore` 排除，絕不參與同步。

---

## **4. 輸出規範 (Output Standards)**

### **4.1 訊息流向**
*   **stderr**: 用於提示、警告、互動詢問與密碼輸入。
*   **stdout**: 僅用於純淨的資料輸出（如 OTP、JSON）。

### **4.2 衝突處理規範 (Conflict Handling)**
*   當發生「狀態衝突」（如資料同步衝突或雙模金庫資料不一）時，強制使用者確認。
*   支援 `-d, --default` 或 `-y, --yes` 套用系統推薦的安全路徑。
