# Homebrew 純淨隔離測試指南 (Zero-Trust Method)

本指南提供一種「完全不依賴系統現有 Homebrew」的測試方法。我們將在臨時目錄中下載一個全新的 Homebrew 實體，從零開始執行安裝測試。

## 1. 建立完全隔離的測試空間

我們將在專案根目錄下建立一個 `brew-test` 資料夾（已在 `.gitignore` 中排除）。

```bash
# 1. 進入專案根目錄並建立測試目錄
cd /Users/lichih/code/just-keep-identity
export JKI_PURITY_DIR="$(pwd)/brew-test"
rm -rf "$JKI_PURITY_DIR" # 確保絕對乾淨
mkdir -p "$JKI_PURITY_DIR"

# 2. 下載全新的 Homebrew 核心
curl -L https://github.com/Homebrew/brew/tarball/master | tar xz --strip 1 -C "$JKI_PURITY_DIR"
```

## 2. 配置臨時環境變數

我們需要暫時遮蔽系統的 brew，並讓新的 brew 知道它的家在哪裡。

```bash
# 封閉環境設定
export PATH="$JKI_PURITY_DIR/bin:/usr/bin:/bin:/usr/sbin:/sbin" # 只留系統基礎路徑與新的 brew
export HOMEBREW_PREFIX="$JKI_PURITY_DIR"
export HOMEBREW_CELLAR="$JKI_PURITY_DIR/Cellar"
export HOMEBREW_REPOSITORY="$JKI_PURITY_DIR"
export HOMEBREW_CACHE="$JKI_PURITY_DIR/cache"

# 驗證目前使用的 brew 位置 (應該要指向 /tmp/...)
which brew
brew --version
```

## 3. 執行安裝測試

現在你在一個完全「空無一物」的 Homebrew 環境中了。

### A. 測試二進位下載安裝 (Bottle)
```bash
# 使用專案中的 Formula 檔案
brew install --verbose --debug ./docs/homebrew-jki.rb
```

### B. 測試源碼編譯 (Source Build)
```bash
# 移除剛才的安裝
brew uninstall jki

# 測試源碼編譯流程
brew install --build-from-source --verbose --debug /Users/lichih/code/just-keep-identity/docs/homebrew-jki.rb
```

## 4. 為什麼這個方案更可信？

1.  **無污染**：它沒有使用你系統中 `/opt/homebrew` 的任何檔案。
2.  **可複製性**：這就是 Homebrew 在 CI (GitHub Actions) 上的運作方式。
3.  **物理隔離**：所有的二進位檔、快取、Metadata 全都在 `$JKI_PURITY_DIR` 下。

## 5. 清理

測試結束後，直接關閉終端機或 `unset` 變數，並刪除目錄即可：
```bash
rm -rf "$JKI_PURITY_DIR"
```
