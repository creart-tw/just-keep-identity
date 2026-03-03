# Mission: Unified Auth Flag (-A) Integration

## 1. 背景 (Context)
根據 V28 規格，統一全組件的認證與資料來源旗標為 `-A, --auth <SOURCE>`。這將取代目前分散的 `--force-agent`, `--force-plain` 與 `-I` 旗標，並為 `jki-agent` 提供抽象的 `biometric` 支援。

## 2. 涉及檔案 (Files Involved)
- **`crates/jki-core/src/lib.rs`**: 
    - 實作 `AuthSource` Enum (derive `ValueEnum` for clap)。
    - 重構 `acquire_master_key` 與 `AgentClient` 邏輯以符合新參數。
- **`crates/jki-agent/src/main.rs`**: 
    - 新增 `-A, --auth` 參數。
    - 實作 `AuthSource::Biometric` 的分發邏輯 (暫時連動到目前的認證流程)。
- **`crates/jkim/src/main.rs`** & **`crates/jki/src/main.rs`**: 
    - 統一參數解析，將舊旗標遷移至 `-A`。
    - 確保 `-I` 作為 `-A interactive` 的快捷別名。

## 3. 核心邏輯 (Logic)
- [ ] **jki-core**: 建立單一權威來源枚舉。
- [ ] **Auth Path**: 實作 Fail-fast 邏輯。若指定了 `-A` 卻不可用，不准自動回退到其他路徑。

---
*Status: Delegated by Architect.*
