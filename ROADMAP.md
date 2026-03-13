# Just Keep Identity (JKI) - Roadmap

本專案目前處於 **Alpha 階段 (v0.1.x)**，核心專注於「物理安全性」與「資料完整性」，CLI 的操作邏輯與 UX 仍可能根據社群回饋進行重大調整。

## 🎯 Phase 1: UX Tuning & User Feedback (v0.1.x) - [當前重點]
*   **元件分離策略**：
    *   **Core (jki/jkim)**：作為全平台通用核心，確保在 macOS, Linux, Windows 具備一致的 CLI 行為與資料安全性。
    *   **Agent (jki-agent)**：作為可選 (Optional) 的加值元件，初期以 macOS 優化為主，提供背景解密與系統匣交互。
*   **平台策略**：以 **macOS (Darwin)** 為首發平台進行完整套件 (Core + Agent) 的深度優化。
    *   收集搜尋模式 (Pattern) 與索引 (Index) 決策邏輯的回饋。
    *   優化 `jki` 在多重結果下的引導訊息與提示。
*   **Agent 穩定性 Harden**：
    *   處理不同平台的 IPC 權限與連線穩定度。
    *   補強 Agent 在 Locked 狀態下的「被動解鎖」流程回饋。
*   **文檔與引導**：
    *   建立繁體中文與英文雙語手冊。
    *   提供各類 MFA 服務匯入的 YAML 模板。

## 🧪 Phase 2: Integration & Coverage (v0.5.x)
*   **端到端測試 (E2E Integration Tests)**：建立自動化腳本模擬 `init` -> `add` -> `search` -> `sync` 的完整路徑。
*   **跨平台原生發布**：
    *   提供 macOS Homebrew Formula 與 Linux Binary 下載。
    *   Windows 系統 Tray 圖示優化與自啟動支持。
*   **性能審查**：確保在 500+ 分錄下，搜尋結果的加權計分與高亮顯示仍能保持 < 50ms 的感官延遲。

## 🌟 Phase 3: Feature Expansion (v1.0.0+)
*   **系統 Keychain 深度整合**：原生支援 macOS TouchID 與 Windows Hello 解鎖 Agent。
*   **第三方工具匯入插件**：直接導入 Bitwarden, Authy, Google Authenticator 的加密匯出包。
*   **多金庫並行 (Multi-Vault)**：支援切換不同的 JKI_HOME 以隔離工作與私人帳號。

---

## 📈 版本躍升量化矩陣 (v0.1.0 → v1.0.0)

為了確保專案的穩定性與使用者體驗，版本的晉升需符合以下量化指標與驗收標準：

| 版本號碼 | 階段定義 | 驗收標準 (量化指標) | 核心目標 (規格分級實作) |
| :--- | :--- | :--- | :--- |
| **v0.1.x** | **Alpha (UX 修正期)** | 1. 週期: 2-4 週<br>2. 反饋量: > 20 則操作建議<br>3. 平台: **macOS 穩定運行** | **MH (100%)**: 修正所有 CLI 語義與手感問題。<br>**MH**: macOS Agent 啟動與 Tray 交互無誤。 |
| **v0.3.x** | **Beta (穩定性試驗)** | 1. 活躍測試者: 20+ 位<br>2. MH 反饋納入率: > 80%<br>3. 平台: **跨平台 (WSL/Linux/Win)** | **MH (100%)**: 完成跨平台 IPC 壓力測試。<br>**MH**: 完善跨平台 Agent 自啟動機制。 |
| **v0.5.x** | **Feature Complete** | 1. 開發時長: 累計 3-4 個月<br>2. 反饋總量: > 50 則有效 Issue<br>3. 測試覆蓋: Happy Path E2E 100% | **MH (100%)**: 完成 Phase 2 規格。<br>**MH**: 建立完整的自動化集成測試。 |
| **v0.8.x** | **RC (發布候選)** | 1. 穩定度: 連續 2 週無新增 MH Bug<br>2. 文檔精準度: Help 資訊 100% 準確<br>3. NH 實作率: > 60% | **MH (100%)**: 安全性審查與記憶體清理優化。<br>**MH**: 凍結 CLI 語義 (No Breaking Changes)。 |
| **v1.0.0** | **General Availability** | 1. 穩定用戶: 100+ 位<br>2. 資料安全: 0 宗資料損壞案例 | **1.0 規格全數達成**：包含系統級生體辨識整合。 |

### 🛠 規格與驗收定義 (Quantification Rules)

1.  **規格分級 (Spec Grading)**:
    *   **Must-have (MH)**: 涉及「資料安全性」、「物理完整性」、「核心搜尋/解密流程」。若未 100% 達成，版本不允許躍遷。
    *   **Nice-to-have (NH)**: 涉及「UI 裝飾」、「第三方插件」、「便利性捷徑」。作為版本增益，不阻礙版本躍遷。
2.  **反饋納入率 (Feedback Incorporation)**:
    *   **公式**: `(已實作的 MH 反饋 / 收集到的有效 MH 反饋總量) %`。
    *   在 v0.5.x 前，納入率必須維持在 **80% 以上**，確保產品是根據真實需求演進。
3.  **穩定性對帳**:
    *   進入 RC (v0.8.x) 的門檻是 **Roadmap MH 任務 100% 達成**。所有 AI 生成的測試案例必須通過，且經過 `VERIFY.md` 的人工冒煙測試驗證。

---

## 🔒 安全宣言
*   **No-Network First**：除了顯式的 Git 同步指令，禁止任何背景網絡通訊。
*   **Physical First**：所有變動必須在本地磁碟有對應檔案，確保「看得見的透明」。
