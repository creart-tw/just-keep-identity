# **Just Keep Identity (jki)：CLI 指令手冊 (Command-Line Interface Spec)**

這份文件詳細記載 `jki` 與 `jkim` 的所有指令、旗標與預期行為，作為實作與測試的依準。

---

## **1. 全域旗標 (Global Flags)**

適用於所有 `jki` 與 `jkim` 指令：
*   `-I, --interactive`: 強制互動模式。忽略磁碟上的 `master.key` 與 Agent 快取，直接開啟密碼輸入。
*   `-q, --quiet`: 安靜模式。抑制 stderr 的提示、進度訊息與非關鍵警告。
*   `-d, --default`: 自動模式。對於所有具備建議偏好的詢問，自動套用系統推薦行為，不再進行互動詢問。

---

## **2. 執行器：jki**

### **2.1 搜尋與 OTP 生成 (預設行為)**
`jki [PATTERNS]... [INDEX]`
*   **PATTERNS**: 多關鍵字篩選。
    *   **邏輯**：每個關鍵字必須在 `Issuer` 或 `Account Name` 其中之一找到匹配。
    *   **範例**：
        *   `jki g li` -> 匹配 `[G]oogle` 的 `[li]chihwu` 帳號。
        *   `jki gh` -> 匹配 `[G][i][t][h][u][b]`，但不會誤中 `[G]oogle-lic[h]ih`。
*   **INDEX**: 當多個結果時，可指定數字選取。
*   **旗標**:
    *   `-s, --stdout`: 直接在 stdout 印出 OTP（預設為複製到剪貼簿）。
    *   `-`: 等同於 `--stdout`。
    *   `-l, --list`: 強制顯示匹配清單，即使只有一個結果。
    *   `-o, --otp`: 在清單模式下顯示 OTP。

### **2.2 Agent 互動**
`jki agent <SUBCOMMAND>`
*   `ping`: 檢查 Agent 是否存活。
*   `get <ID>`: 透過 Agent 獲獲取指定 ID 的 OTP（不經由本地解密）。

---

## **3. 管理中心：jkim**

### **3.1 環境初始化 (init)**
`jkim init [--force]`
*   **預設行為 (Transparent Init)**：檢查環境並回報狀態。若有衝突則提示警告。
*   **旗標**:
    *   `-f, --force`: **重置模式**。刪除現有的 `vault.metadata.json` 與 `vault.secrets.bin.age`，重新建立乾淨的金庫環境。

### **3.2 金鑰管理 (master-key)**
`jkim master-key <SUBCOMMAND>`
*   `set [--force]`: 將 Master Key 寫入 `master.key` (0600)。
*   `remove [--force]`: 刪除磁碟上的 `master.key`。
*   `change [--commit]`: 重新加密金庫並變更金鑰。

### **3.3 資料管理 (Vault Management)**
*   `decrypt [-k, --keep] [--remove-key]`: 將金庫解密為明文 (`vault.secrets.json`)。
    *   **無 Flag 互動行為**：
        1.  `Delete encrypted source (.age)? [Y/n]` (預設刪除，維持狀態純粹)。
        2.  `Delete master key file? [y/N]` (預設保留，便於後續自動封裝)。
    *   **旗標效果**：
        *   `-d, --default`: 自動執行上述推薦路徑（刪除 .age，保留 key）。
        *   `-k, --keep`: 保留來源加密檔，跳過第一項詢問。
        *   `--remove-key`: 移除 master.key 檔案，跳過第二項詢問。
*   `encrypt`: 將明文金庫重新封裝為加密檔 (`.age`) 並物理刪除明文。
*   `import-winauth <FILE> [--overwrite] [--force-new-vault]`: 
    *   **狀態感知**：若存在明文金庫且具備 `master.key`，匯入後應自動壓回加密檔。
    *   **預設行為**：所有狀態變更應在 Stderr 提示，並支援 `-d, --default` 套用推薦路徑。

### **3.4 資料編輯 (edit)**
`jkim edit`
*   採用 `crontab -e` 流程。開啟 `$EDITOR` 並於儲存後執行 JSON 驗證。
*   **優先序**：若存在明文金庫，優先編輯明文；否則編輯 Metadata 並從加密金庫讀取（需解鎖）。

### **3.5 同步 (sync)**
`jkim sync`
*   執行 Git 原子化備份：`add` -> `commit` -> `pull --rebase` -> `push`。
*   **注意**：明文金庫應始終被 `.gitignore` 排除，不參與同步。

---

## **4. 輸出規範 (Output Standards)**

### **4.1 訊息流向**
*   **stderr**: 用於提示、警告、以及密碼輸入。
*   **stdout**: 僅用於純淨的資料輸出。

### **4.2 衝突處理規範 (Conflict Handling)**
*   **原則**：發生「狀態衝突」（如兩份金庫皆存在且資料不一）時，強制使用者確認預設行為。
*   **自動化支援**：支援 `-d, --default` 套用系統推薦路徑，不進行互動式詢問。
