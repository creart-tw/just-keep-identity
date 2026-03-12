# Mission: CLI Configuration Persistence (Minimalist & Personalization)

## 1. 背景與目標 (Background & Objective)
`jki` 追求極致速度與人體工學，但不同使用者對於「智慧程度」與「預設行為」有不同的偏好。為維持系統的極簡性並避免過度自動化帶來的誤導，我們需要一套「讀寫分離」的配置管理系統。

本任務目標是：
- **讀寫分離架構**：`jki` 僅負責讀取配置並合併參數；`jkim` 負責所有配置的修改與持久化。
- **配置持久化**：使用人類可讀的 TOML 格式存儲於 `JKI_HOME/config.toml`。
- **明確逃生口**：實作 `--ignore-config` (或 `-X`) 旗標，確保在任何時候都能回歸原始確定的行為。
- **AI 友善設計**：避免引入複雜的反轉旗標（如 `-v`），降低代碼維護難度與 AI 誤導風險。

## 2. 策略與階段 (Strategy & Phases)

### Phase 1: 唯讀配置基礎設施 (Read-Only Infra in `jki`)
1. **定義 `Config` 結構**：
   - 包含 `quiet`, `fuzzy_mode`, `auto_select_threshold`, `clipboard_notify` 等項。
2. **實作層級合併 (Layered Merge)**：
   - 優先序：`CLI Flags` > `Env Vars (JKI_*)` > `config.toml` > `Hardcoded Defaults`。
3. **實作 `--ignore-config` (`-X`)**：
   - 當此旗標存在時，跳過層級合併，僅保留 `CLI Flags` 與 `Hardcoded Defaults`。

### Phase 2: 配置管理權威 (Write Authority in `jkim`)
1. **實作 `jkim config-jki save <FLAGS...>`**：
   - 解析傳入的旗標（不執行動作），僅將對應的非預設值寫入 `config.toml`。
   - 範例：`jkim config-jki save -q` 執行後，`jki` 預設即為 `-q`。
2. **實作 `jkim config set/get/reset`**：
   - 提供細粒度的鍵值對管理。
3. **實作 `jkim config status` (UX 關鍵)**：
   - 模擬並輸出 `jki` 目前的生效配置路徑，標註每個值的來源（預設/環境變數/設定檔）。

### Phase 3: 安全與硬化 (Security & Hardening)
1. **TOML 序列化與權限**：
   - 確保寫入的 `config.toml` 維持 0600 權限。
   - 支援 TOML 註解的讀取與保留（可選，增加可讀性）。
2. **錯誤處理**：
   - 設定檔損壞時，`jki` 應發出警告但自動退回 (Fallback) 到預設行為，絕不中斷核心查詢。

## 3. 指令範例 (Command Examples)

### 3.1. 修改預設值
```bash
# 透過 jkim 將 -q 設為 jki 的持久化預設值
jkim config-jki save -q
```

### 3.2. 確認生效行為
```bash
# 預覽 jki 的當前生效配置
jkim config status
```

### 3.3. 強制原始行為 (Escape Hatch)
```bash
# 忽略所有設定檔與環境變數，執行最原始的 jki
jki github -X
```

## 4. 完成定義 (Definition of Done)
- [ ] `jki` 成功實作層級配置合併與 `-X` 逃生口。
- [ ] `jkim` 具備完整的配置修改能力（`save`, `set`, `status`）。
- [ ] 通過「環境變數覆蓋設定檔」與「命令列覆蓋設定檔」的測試。
- [ ] 規格文件 `docs/jki-cli-spec.md` 已同步更新。
