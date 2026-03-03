# Mission Report: Unified Auth Flag (-A) Integration

## 1. 任務概述 (Mission Overview)
根據 V28 規格與 `mission-unified-auth-flag.md` 之要求，本任務已成功統一 `jki`  suite 全組件的認證與資料來源旗標為 `-A, --auth <SOURCE>`。此變更取代了原先分散的 `--force-agent`, `--force-plain` (jki-agent 內部邏輯) 與 `-I` 旗標，並在 `jki-core` 中建立了單一權威的 `AuthSource` 枚舉。

## 2. 實作細節 (Implementation Details)

### 2.1 jki-core (核心邏輯)
- **`AuthSource` Enum**: 於 `crates/jki-core/src/lib.rs` 實作，支援 `Auto`, `Agent`, `Interactive`, `Keyfile`, `Keychain`, `Plaintext`, `Biometric`。
- **`acquire_master_key` 重構**: 
    - 參數由 `force_interactive: bool` 改為 `source: AuthSource`。
    - 導入 **Fail-fast** 邏輯：若指定特定來源（如 `Agent`, `Keyfile`, `Keychain`）但該來源不可用，將直接報錯，不再自動回退到其他路徑。
    - `Auto` 模式保持原有的優先序回退邏輯。
- **Clap 整合**: `jki-core` 新增 `clap` 依賴（具備 `derive` feature），使 `AuthSource` 能直接作為 `ValueEnum` 供各 CLI 工具使用。

### 2.2 jki-agent (背景服務)
- **參數遷移**: 移除 `--force-age`，統一使用 `-A, --auth`。
- **加載邏輯**:
    - `AuthSource::Plaintext`: 強制僅允許讀取 `vault.secrets.json`。
    - 其他非 `Auto` 來源: 若加密金庫 (`.age`) 不存在，則拒絕啟動/解鎖。
- **Biometric 預留**: 新增 `Biometric` 旗標分發邏輯預留位。

### 2.3 jki & jkim (CLI 工具)
- **統一參數**: 
    - 移除 `jki` 的 `--force-agent`。
    - 統一導入 `-A, --auth <SOURCE>`。
    - 保留 `-I` 作為 `--auth interactive` 的別名，維持 UX 慣性。
- **邏輯對齊**: 
    - `jki` 根據 `AuthSource` 決定搜尋金鑰與金庫資料的順序。
    - `jkim` 在 `Export`, `Decrypt`, `Encrypt`, `Import` 等所有涉及主金鑰取得的點，皆已套用新邏輯。

## 3. 驗證與測試 (Verification & Testing)

### 3.1 自動化測試
- 全組件 41 個測試案例全部通過 (`cargo test`)。
- **新增/修正測試**:
    - `test_auth_agent_refusal_plaintext`: 驗證 `-A agent` 模式下拒絕明文金庫的安全性。
    - `test_run_auth_agent_skips_plaintext`: 驗證 `jki` 在指定來源時能正確跳過明文路徑。
    - `test_acquire_master_key_priority`: 驗證 `jki-core` 的 Fail-fast 分發邏輯。

### 3.2 物理驗證
- **編譯檢查**: `cargo build` 通過，確認無型別或 Macro 錯誤。
- **CLI 測試**: 驗證 `-I` 確實能正確觸發 `Interactive` 模式，且 `-A` 參數解析正常。

## 4. 結語 (Conclusion)
本任務透過 `AuthSource` 的抽象化，顯著提升了系統認證邏輯的一致性與安全性。Fail-fast 邏輯的導入確保了在特定安全需求下，程式行為的可預測性。

---
*Status: Completed & Verified.*
*Reported by: Gemini CLI*
