use std::path::PathBuf;

pub struct JkiPath;

impl JkiPath {
    /// 獲取 JKI 的設定目錄
    /// 優先順序：
    /// 1. JKI_HOME 環境變數
    /// 2. ~/.config/jki (Unix 風格)
    /// 3. %LOCALAPPDATA%\jki (Windows)
    pub fn config_dir() -> PathBuf {
        if let Ok(home) = std::env::var("JKI_HOME") {
            return PathBuf::from(home);
        }

        #[cfg(not(windows))]
        {
            if let Some(home) = dirs::home_dir() {
                return home.join(".config").join("jki");
            }
        }

        dirs::config_dir()
            .map(|p| p.join("jki"))
            .unwrap_or_else(|| {
                dirs::home_dir()
                    .map(|h| h.join(".jki"))
                    .expect("Could not find home directory")
            })
    }

    pub fn metadata_path() -> PathBuf {
        std::env::var("JKI_METADATA_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| Self::config_dir().join("vault.metadata.json"))
    }

    pub fn secrets_path() -> PathBuf {
        std::env::var("JKI_SECRETS_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| Self::config_dir().join("vault.secrets.json"))
    }

    pub fn master_key_path() -> PathBuf {
        std::env::var("JKI_MASTER_KEY_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| Self::config_dir().join("master.key"))
    }

    /// 檢查檔案權限是否足夠安全 (Unix: 0600)
    pub fn check_secure_permissions(path: &PathBuf) -> Result<(), String> {
        if !path.exists() {
            return Err("File does not exist".to_string());
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = std::fs::metadata(path).map_err(|e| e.to_string())?;
            let mode = metadata.permissions().mode() & 0o777;
            if mode != 0o600 {
                return Err(format!(
                    "Insecure file permissions: {:o}. Expected 0600 (read/write by owner only).",
                    mode
                ));
            }
        }

        #[cfg(windows)]
        {
            // TODO: 在 Windows 環境實作 ACL 檢查
            // 確保檔案擁有者是目前使用者且無其他存取權
        }

        Ok(())
    }
}
