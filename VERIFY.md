# JKI 人工冒煙測試清單 (Smoke Test Checklist)

在發布 `v0.1.0-alpha` 前，建議開發者親自執行以下場景，驗證 CLI 的「手感」與「流程邏輯」是否符合預期。

## 🧪 場景 1：初始化與基礎操作 (The First Mile)
*   [ ] **全新初始化**：
    *   `rm -rf ~/.config/jki` (請先備份現有資料)
    *   執行 `jkim init` -> 檢查是否正確建立 Git Repo、YAML 與 Master Key。
*   [ ] **互動式新增 (Live Handshake)**：
    *   執行 `jkim add Google test@gmail.com --secret JBSWY3DPEHPK3PXP`
    *   驗證：是否出現 `(Copied!)` 且剪貼簿有每 30 秒更新一次的 OTP？按下 Enter 是否正確寫入？
*   [ ] **搜尋與索引**：
    *   執行 `jki google` -> 驗證是否自動選中唯一項並產碼。
    *   執行 `jki g 1` -> 驗證索引選取功能是否直覺。

## 🕵️‍♂️ 場景 2：Agent 智慧化行為 (Optional - macOS/Desktop Optimized)
*   [ ] **冷啟動 (Cold Start)**：
    *   確保 `jki-agent` 沒在跑。執行 `jki google` -> 驗證是否出現 `[Tip]` 建議啟動 Agent。
*   [ ] **明文自動解鎖**：
    *   執行 `jkim decrypt` (將金庫轉為明文)。
    *   執行 `jkim agent restart`。
    *   執行 `jki google -A agent` -> 驗證 Agent 是否能「秒出」OTP (代表它啟動時已自動加載明文)。
*   [ ] **加密解鎖死鎖測試**：
    *   執行 `jkim master-key set --force` (加密金庫)。
    *   重啟 Agent (此時處於 Locked 狀態)。
    *   執行 `jki -A agent google` -> 驗證是否會提示解鎖 -> 輸入密碼後是否正確獲取結果？(確保無死鎖)。

## 🧹 場景 3：資料整合與修復 (Data Integrity)
*   [ ] **YAML 手動編輯與同步**：
    *   執行 `jkim edit` -> 在 YAML 改名或換 ID -> 存檔。
    *   執行 `jki <新名字>` -> 驗證是否能立即搜到 (代表 Agent 已被通知 Reload)。
*   [ ] **健康檢查與去重**：
    *   人為在 YAML 弄出重複的 ID 或 Secret。
    *   執行 `jkim config check` -> 驗證是否能抓出錯誤。
    *   執行 `jkim dedupe` -> 驗證 Index 選取保留項的邏輯是否順手。

## 📄 場景 4：文檔與引導 (Onboarding)
*   [ ] **說明手冊**：
    *   執行 `jkim man` -> 閱讀渲染後的說明，判斷排版在你的終端機是否舒適。
    *   執行 `jkim status` -> 檢查顯示的磁碟權限與 Keychain 狀態是否準確。

---

> **開發者筆記**：
> 如果在測試過程中覺得「這裡還要多打一次 Enter 很煩」或「錯誤提示看不懂」，請記錄下來，這就是 v0.1.x 改版的首要目標。
