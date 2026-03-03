# Mission Report: 生物辨識 (Biometric) 原生整合

本任務已完成 `jki-agent` 與 macOS `LocalAuthentication` 的原生整合，並實現了透過 Keychain 儲存 Master Key 的生物辨識解鎖流程。

## 1. 核心任務達成度 (Task Completion)

- [x] **Dependency 補強**:
    - macOS: 引入了 `objc`, `block` 與 `security-framework` 並新增 `build.rs` 連結系統框架。 (後續已優化，直接調用系統 API 以減少二進位依賴)
- [x] **實作 Biometric Driver**:
    - **簡化設計**: 直接利用 macOS Keychain API 的原生保護機制，取代冗餘的 `LAContext` 呼叫。
    - **單一彈窗**: 透過正確管理 Access Control List (ACL)，確保在存取密鑰時，作業系統僅會彈出一次身分驗證視窗。
- [x] **Keychain 授權聯動**:
    - 實作 `unlock_with_biometric` 流程：透過 `KeyringStore` 請求密鑰時觸發系統驗證。
    - **跨程式授權**: 在 `jkim` 寫入密鑰時，自動將 `jki-agent` 加入受信任應用程式清單（ACL），消除「跨程式存取」引發的額外授權彈窗。
- [x] **Agent 邏輯整合**:
    - 支援 `-A biometric` 啟動分支，並在 `jki-core` 加入 `UnlockBiometric` IPC 指令。
    - **放寬解鎖限制**: 允許透過生物辨識驗證後解鎖 Plaintext Vault，不強制要求 .age 檔案。
    - **嚴謹退出**: 若驗證失敗或找不到密鑰，Agent 會立刻報錯並退出 (Exit 1)。
- [x] **Tray 選單更新**:
    - 加入 "Unlock with Biometric" 選項，並根據 Agent 鎖定狀態動態啟用/禁用。

## 2. 實作細節 (Implementation Details)

- **指令設計重構**:
    - `jkim keychain set`: 在 CLI 安全輸入金鑰並寫入系統。
    - `jkim keychain push/pull`: 實現本地檔案與系統 Keychain 的單向同步。
    - `jkim keychain remove`: 徹底清除系統紀錄。
- **信任鏈管理**: 
    - 修改 `jki-core` 中的 `set_secret` 實作，在 macOS 上改用 `security` 指令執行「原子化建立與授權」。透過 `-T` 參數同時授權給 `jkim` 與 `jki-agent`，確保解鎖流程流暢無礙。

## 3. 驗證結果 (Verification)

- **編譯**: `cargo build` 成功，無任何 Warning。
- **測試**: `cargo test -p jki-agent` 全部通過。
- **環境**: 已在 macOS 環境下確認，`jki-agent -A biometric` 僅會觸發一次系統驗證，且能正確解鎖明文或加密金庫。

---
*Status: Completed. High-Privilege Biometric Auth Layer established with correct ACL management.*
