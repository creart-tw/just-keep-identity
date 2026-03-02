# **Just Keep Identity (jki)：CLI 指令手冊 (Command-Line Interface Spec)**

這份文件詳細記載 `jki` 與 `jkim` 的所有指令、旗標與預期行為，作為實作與測試的依準。

---

## **1. 全域旗標 (Global Flags)**

適用於所有 `jki` 與 `jkim` 指令：
*   `-I, --interactive`: 強制互動模式。忽略磁碟上的 `master.key` 與 Agent 快取，直接開啟密碼輸入。
*   `-q, --quiet`: 安靜模式。抑制 stderr 的提示、進度訊息與非關鍵警告。

---

## **2. 執行器：jki**

### **2.1 搜尋與 OTP 生成 (預設行為)**
`jki [PATTERNS]... [INDEX]`
*   **PATTERNS**: 關鍵字篩選（不分大小寫，模糊匹配）。
*   **INDEX**: 當多個結果時，可指定數字選取。
*   **旗標**:
    *   `-s, --stdout`: 直接在 stdout 印出 OTP（預設為複製到剪貼簿）。
    *   `-`: 等同於 `--stdout`。
    *   `-l, --list`: 強制顯示匹配清單，即使只有一個結果。
    *   `-o, --otp`: 在清單模式下顯示 OTP。

### **2.2 Agent 互動**
`jki agent <SUBCOMMAND>`
*   `ping`: 檢查 Agent 是否存活。
*   `get <ID>`: 透過 Agent 獲取指定 ID 的 OTP（不經由本地解密）。

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

### **3.3 資料匯入 (import-winauth)**
`jkim import-winauth <FILE> [--overwrite] [--force-new-vault]`
*   **預設行為**：嘗試使用 Master Key 解密現有金庫以進行「增量合併 (Merge)」。
*   **旗標**:
    *   `--overwrite`: 若匯入項目與現有帳號重複，則強制覆寫 Metadata 與 Secret。
    *   `--force-new-vault`: **強制作廢模式**。若無法解密現有金庫（如密碼錯誤或金庫毀損），則直接放棄舊資料，以本次輸入的密碼建立全新的金庫並完成匯入。

### **3.4 資料編輯 (edit)**
`jkim edit`
*   採用 `crontab -e` 流程。開啟 `$EDITOR` 並於儲存後執行 JSON 驗證。

### **3.5 同步 (sync)**
`jkim sync`
*   執行 Git 原子化備份：`add` -> `commit` -> `pull --rebase` -> `push`。

---

## **4. 輸出規範 (Output Standards)**

### **4.1 訊息流向**
*   **stderr**: 用於提示、警告、以及密碼輸入。
*   **stdout**: 僅用於純淨的資料輸出。

### **4.2 錯誤處理與解決方案引導**
*   **禁止 Panic**: 嚴禁在正常業務流程中發生 Thread Panic。
*   **解決方案導引**: 當發生「密碼錯誤」或「資料衝突」時，系統必須在錯誤訊息後條列式提供可能的解決方案（如建議使用的旗標或手動排除步驟）。
