# Homebrew 本地隔離測試指南

本指南說明如何建立一個臨時的隔離環境，來測試 `jki` 的 Homebrew Formula 是否能正確安裝（包含二進位包與源碼編譯）。

## 1. 建立隔離環境

為了不影響系統，我們建立一個臨時目錄作為 Homebrew 的 `CELLAR` 與 `PREFIX`。

```bash
# 建立臨時測試目錄
export JKI_TEST_DIR="/tmp/jki-brew-test"
mkdir -p $JKI_TEST_DIR

# 設定 Homebrew 環境變數
# 注意：這會讓 brew 嘗試在該目錄下運作
export HOMEBREW_PREFIX="$JKI_TEST_DIR"
export HOMEBREW_CELLAR="$JKI_TEST_DIR/Cellar"
export HOMEBREW_REPOSITORY="$JKI_TEST_DIR/homebrew"
```

## 2. 測試二進位安裝 (Binary/Bottle)

這是測試從 GitHub Release 下載 `.tar.gz` 並解壓安裝的流程。

```bash
# 使用本地 Formula 檔案進行安裝
# --debug 可以看到詳細過程，--verbose 可以看到下載路徑
brew install --debug --verbose ./docs/homebrew-jki.rb

# 驗證安裝路徑
ls -l $JKI_TEST_DIR/bin/jki

# 執行測試
$JKI_TEST_DIR/bin/jki --version
```

## 3. 測試源碼編譯安裝 (Source Build)

這是模擬在非 macOS ARM64 平台（或強制要求編譯）時的行為。

```bash
# 強制從源碼編譯 (--build-from-source)
brew install --build-from-source --debug --verbose ./docs/homebrew-jki.rb

# 驗證是否正確呼叫了 cargo install
# 驗證二進位檔是否正確產出
$JKI_TEST_DIR/bin/jkim --version
```

## 4. 常見問題排除

### SHA256 不符
如果發生 `SHA256 mismatch`，請重新執行 `make brew-package` 獲取正確的 Hash 並更新 `docs/homebrew-jki.rb`。

### 權限問題
如果在 `$JKI_TEST_DIR` 下發生權限錯誤，請確保該目錄對當前使用者有完整寫入權限（通常在 `/tmp` 下沒問題）。

## 5. 清理測試環境

測試完成後，直接刪除臨時目錄即可：

```bash
rm -rf "$JKI_TEST_DIR"
```
