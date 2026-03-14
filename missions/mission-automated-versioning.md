# Mission: 導入 `cargo-release` 自動化版本管理

## 1. 背景與動機 (Context)

目前的 JKI 專案版本號（如 `v0.1.0-alpha`）採手動管理，這在多人協作或頻繁發布時容易導致 `Cargo.toml` 與 Git Tag 不一致，且手動打包 Homebrew 容易出錯。

為了提升發布的嚴謹度，我們計畫導入 Rust 生態系成熟的 `cargo-release` 工具。

---

## 2. 核心目標 (Objectives)

1.  **一鍵發布 (One-command Release)**：透過 `cargo release` 自動完成版號提升、Tag 建立與推送。
2.  **Workspace 同步**：確保根目錄與所有子 Crate (`jki`, `jkim`, `jki-agent`) 的版本號保持一致。
3.  **自動化 Hooks**：
    *   發布前自動執行 `make test-all` 與 `make cov`。
    *   發布後自動更新 Homebrew Formula (未來擴充)。

---

## 3. 實施計畫 (Implementation Plan)

### 第一階段：工具安裝與驗證
*   確認本地已安裝 `cargo-release`。
*   在 `Cargo.toml` 中配置 `[package.metadata.release]`。

### 第二階段：配置規範 (Configuration)
*   **Version Format**: 使用 `v` 前綴的 SemVer。
*   **Shared Version**: 強制所有 Workspace 成員共享同一個版本號。
*   **Tag Message**: 自動生成包含變更摘要的 Tag 訊息。

### 第三階段：流程整合
*   將 `cargo release` 指令整合進 `Makefile` (例如 `make release-bump`)。
*   建立正式的 `CHANGELOG.md` 更新流程。

---

## 4. 預期效益 (Benefits)

1.  **工程化**：消除手動修改 `Cargo.toml` 的低級錯誤。
2.  **透明度**：Git 歷史將清晰呈現每一次版本跳轉的節點。
3.  **擴充性**：為未來的 CI/CD (GitHub Actions) 自動發布打下基礎。

---

**狀態：已建檔，等待執行。**
