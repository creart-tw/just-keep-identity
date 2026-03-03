# Mission Report: Agent Session Logic Integration (Phase 1)

## 1. 任務目標 (Objective)
補完 `jki-agent` 作為 Session 管理者的核心邏輯。實現組件間的通訊連動，達成「一次認證，全域生效」與「資料變更自動刷新」。

## 2. 實作變更 (Changes)

### `crates/jki-core` (核心通訊與抽象)
- **`Cargo.toml`**: 新增 `interprocess` 依賴。
- **`src/lib.rs`**:
    - **`agent` 模組擴展**: 新增 `Request::GetMasterKey`, `Request::Reload` 與 `Response::MasterKey`, `Response::Success`。
    - **`AgentClient` 實作**: 封裝 IPC 通訊細節，提供 `ping`, `unlock`, `get_otp`, `get_master_key`, `reload` 等高階 API。
    - **`ensure_agent_running`**: 從 `jki` 移至 `jki-core` 以供全域使用。
    - **`acquire_master_key`**: 更新優先序，首位嘗試透過 `AgentClient` 獲取 Session 內的金鑰。

### `crates/jki-agent` (Session 管理)
- **`State` 結構強化**: 新增 `master_key: Option<secrecy::SecretString>` 持久化。
- **透明重新載入**: `get_otp` 若偵測到 cache 已清除但 session 仍在，會自動利用 `master_key` 重新加載。
- **IPC 處理器**: 實作 `GetMasterKey` (回傳持駐金鑰) 與 `Reload` (清除 `secrets` 快取) 邏輯。

### `crates/jkim` (Session 感知與主動更新)
- **`handle_edit` / `handle_import_winauth` / `handle_sync`**: 資料異動後主動通知 Agent 執行 `Reload`。
- **`MasterKey::Change`**: 密鑰更換後同步執行 `AgentClient::unlock` 更新 Agent Session。
- **`handle_status`**: 更新為真實檢測 Agent 運行狀態 (Running/Locked/Unlocked)。

### `crates/jki` (Lazy Unlock)
- **Lazy Unlock 實作**: 在 fallback 至本地解密並取得金鑰後，主動發送 `Unlock` 給 Agent 建立 Session。
- **架構簡化**: 移除重複的 IPC 邏輯，全面改用 `AgentClient`。

## 3. 驗證結果 (Validation)
- **單元測試**: 全數通過 (41 tests passed)。
    - 新增 `test_handle_client_reload` 驗證快取清除邏輯。
    - 新增 `test_handle_client_get_master_key` 驗證 Session 獲取。
- **功能驗證**: 
    - 驗證 `jki` 本地認證後 `jkim status` 顯示 `Running (Unlocked)`。
    - 驗證 `jkim edit` 後 Agent 能正確感知資料變更。

---
**Status: COMPLETED**
**Date: 2026-03-03**
