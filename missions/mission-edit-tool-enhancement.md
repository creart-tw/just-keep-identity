# Mission: Opencode `edit` 工具安全性與診斷能力增強需求

## 1. 執行背景 (Context)

在 `just-keep-identity` 專案開發過程中，發生了一次嚴重的「靜默抹除」事件。Agent 在試圖修改 `.gitignore` 時，因對檔案末尾換行符號與不可見字元的認知偏差，導致 `edit` 工具在執行替換時，無意中抹除了後方關鍵的安全排除清單（`private/`, `data/`）。

雖然目前的 `expectedOldLineCount` 強制校驗已上線，但為了進一步提升 Agent 的開發效率與安全性，我們提議對 `edit` 工具進行原生層級的功能增強。

---

## 2. 核心功能需求 (Functional Requirements)

### 2.1 精確的行數不符報錯 (Line-count Mismatch Diagnostics)
*   **需求**：當 `expectedOldLineCount` 校驗失敗時，工具應回傳實際數到的行數。
*   **效益**：縮短 Agent 的「猜測」時間，強制其根據物理事實重新校正認知。

### 2.2 物理座標標註 (Start/End Line Mapping)
*   **需求**：無論成功或失敗，工具應回傳 `oldString` 匹配區塊在原始檔案中的 **起始行號** 與 **結束行號**。
*   **效益**：提供物理座標參考，避免 Agent 在大型設定檔中定位失準。

### 2.3 格式與不可見字元提示 (Format & Whitespace Hints)
*   **需求**：針對因換行符號（LF/CRLF）或末尾空白導致的匹配差異，提供語意化提示。
*   **效益**：解決 AI 最常遇到的「換行符號認知錯誤」問題，減少無意義的重複嘗試。

---

## 3. 安全防禦建議 (Safety Hooks)

### 3.1 敏感區塊保護 (Sensitive Pattern Detection)
*   **提議**：若 `edit` 操作會移除或大幅修改包含關鍵註釋（如 `# <SECURE>`, `# Private`）的行，工具應拋出高級別警告。

### 3.2 匹配唯一性 (Uniqueness Enforcement)
*   **提議**：持續強化「唯一匹配」原則。若 `oldString` 出現多次，工具應回傳所有出現的行號，由 Agent 重新指定更精確的範圍。

---

## 4. 預期 API 交互範例

**Agent 調用：**
```json
{
  "filePath": ".gitignore",
  "oldString": "# Private\nprivate/",
  "expectedOldLineCount": 2
}
```

**工具回傳 (診斷輸出)：**
```json
{
  "status": "failed",
  "reason": "line_count_mismatch",
  "details": {
    "expected": 2,
    "actual": 3,
    "line_range": [40, 42],
    "hint": "The matching block ends with a newline character (LF). Consider if this was omitted in your count."
  }
}
```

---

## 5. 結論

將「安全性」從 Agent 的注意力轉移到工具的「硬性契約」上，是提升 Agent 穩定性的必經之路。透過引入行數校驗與診斷回饋，`edit` 工具將從單純的字串處理器進化為具備 **「原子化誠信 (Atomic Integrity)」** 的專業工程工具。

**建議 Opencode 團隊：評估此類診斷資訊回傳對提升 Agentic Workflow 穩定性的巨大價值。**
