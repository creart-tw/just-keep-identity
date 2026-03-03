# Mission: Agent Session Logic Integration (Phase 1)

## 1. 背景 (Context)
補完 `jki-agent` 作為 Session 管理者的核心邏輯。實現組件間的通訊連動，達成「一次認證，全域生效」與「資料變更自動刷新」。

## 2. 涉及檔案 (Files Involved)
- **`crates/jki-core/src/lib.rs`**: 
    - 擴展 `agent::Request` (GetMasterKey, Reload) 與 `Response` (MasterKey)。
    - 實作 `AgentClient` 封裝通訊細節。
- **`crates/jki-agent/src/main.rs`**: 
    - `State` 結構強化 (持駐 master_key)。
    - 處理 `GetMasterKey` 與 `Reload` 請求。
- **`crates/jkim/src/main.rs`**: 
    - 更新 `acquire_master_key` (優先 IPC)。
    - 在資料異動操作 (edit, import, change) 後觸發 `Reload`。
- **`crates/jki/src/main.rs`**: 
    - 實作 **Lazy Unlock** (認證成功後自動同步給 Agent)。

## 3. 核心邏輯 (Logic)
- [ ] **jki-core**: 建立 `AgentClient` 供全域組件使用。
- [ ] **jki-agent**: 實現「記憶金鑰」與「通知後重新載入」能力。
- [ ] **jkim**: 實現「Session 感知」與「主動重新整理」。
- [ ] **jki**: 實現「無感啟動快取」。

---
*Status: Strategy Finalized by Architect. Ready for implementation.*
