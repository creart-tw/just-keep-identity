use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use secrecy::SecretString;
use std::io::{self, Read, Write};

pub mod import;
pub mod paths;

#[derive(Archive, Deserialize, Serialize, SerdeDeserialize, SerdeSerialize, Debug, Clone)]
#[archive(check_bytes)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub issuer: Option<String>,
    pub account_type: AccountType,
    
    // 這些欄位僅在記憶體整合後存在，不應出現在 metadata.json 中
    #[serde(skip_serializing, default)]
    pub secret: String,
    #[serde(skip_serializing, default = "default_digits")]
    pub digits: u32,
    #[serde(skip_serializing, default = "default_algorithm")]
    pub algorithm: String,
}

fn default_digits() -> u32 { 6 }
fn default_algorithm() -> String { "SHA1".to_string() }

#[derive(Archive, Deserialize, Serialize, SerdeDeserialize, SerdeSerialize, Debug, Clone, PartialEq)]
#[archive(check_bytes)]
pub enum AccountType {
    Standard,
    Steam,
    Blizzard,
}

#[derive(Archive, Deserialize, Serialize, SerdeDeserialize, SerdeSerialize, Debug, Clone)]
#[archive(check_bytes)]
pub struct AccountSecret {
    pub secret: String,
    pub digits: u32,
    pub algorithm: String,
}

// --- 加解密核心 ---

pub fn encrypt_with_master_key(data: &[u8], master_key: &SecretString) -> Result<Vec<u8>, String> {
    let encryptor = age::Encryptor::with_user_passphrase(master_key.clone());
    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted).map_err(|e| e.to_string())?;
    writer.write_all(data).map_err(|e| e.to_string())?;
    writer.finish().map_err(|e| e.to_string())?;
    Ok(encrypted)
}

pub fn decrypt_with_master_key(encrypted_data: &[u8], master_key: &SecretString) -> Result<Vec<u8>, String> {
    let decryptor = match age::Decryptor::new(encrypted_data).map_err(|e| e.to_string())? {
        age::Decryptor::Passphrase(d) => d,
        _ => return Err("Expected passphrase-encrypted data".to_string()),
    };
    let mut reader = decryptor.decrypt(master_key, None).map_err(|e| e.to_string())?;
    let mut decrypted = vec![];
    reader.read_to_end(&mut decrypted).map_err(|e| e.to_string())?;
    Ok(decrypted)
}

// --- 搜尋邏輯 ---

pub fn fuzzy_match(pattern: &str, target: &str) -> bool {
    let pattern = pattern.to_lowercase();
    let target = target.to_lowercase();
    let mut target_chars = target.chars();
    for p in pattern.chars() {
        if !target_chars.any(|t| t == p) { return false; }
    }
    true
}

pub fn search_accounts(accounts: &[Account], patterns: &[String]) -> Vec<Account> {
    accounts.iter()
        .filter(|acc| {
            let target_str = format!("{} {}", acc.issuer.as_deref().unwrap_or_default(), acc.name);
            patterns.iter().all(|p| fuzzy_match(p, &target_str))
        })
        .cloned()
        .collect()
}

// --- 金鑰獲取 ---

pub fn acquire_master_key() -> Result<SecretString, String> {
    use crate::paths::JkiPath;
    use crossterm::{
        event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
        terminal::{disable_raw_mode, enable_raw_mode},
        execute, cursor, style::Print,
    };

    let key_path = JkiPath::master_key_path();
    if key_path.exists() {
        if JkiPath::check_secure_permissions(&key_path).is_ok() {
            let content = std::fs::read_to_string(key_path).map_err(|e| e.to_string())?;
            return Ok(SecretString::from(content.trim().to_string()));
        }
    }

    if !atty::is(atty::Stream::Stdin) {
        let mut line = String::new();
        io::stdin().read_line(&mut line).map_err(|e| e.to_string())?;
        return Ok(SecretString::from(line.trim().to_string()));
    }

    enable_raw_mode().map_err(|e| e.to_string())?;
    let mut stdout = io::stderr();
    execute!(stdout, Print("Enter Master Key: [ "), cursor::SavePosition, Print("_ ]"), cursor::RestorePosition).ok();
    stdout.flush().ok();

    let mut password = String::new();
    let mut toggle = false;

    let result = loop {
        if let Ok(Event::Key(KeyEvent { code, modifiers, .. })) = event::read() {
            match code {
                KeyCode::Enter => {
                    execute!(stdout, cursor::RestorePosition, cursor::MoveRight(2), Print("\n")).ok();
                    break Ok(SecretString::from(password));
                }
                KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                    execute!(stdout, cursor::RestorePosition, cursor::MoveRight(2), Print("\nCancelled\n")).ok();
                    break Err("Interrupted".to_string());
                }
                KeyCode::Char(c) => { password.push(c); toggle = !toggle; }
                KeyCode::Backspace => { if !password.is_empty() { password.pop(); toggle = !toggle; } }
                _ => continue,
            }
            let symbol = if password.is_empty() { "_" } else if toggle { "*" } else { "x" };
            execute!(stdout, cursor::RestorePosition, Print(symbol), cursor::RestorePosition).ok();
            stdout.flush().ok();
        }
    };
    disable_raw_mode().ok();
    result
}

pub mod git {
    use std::process::Command;
    use std::path::Path;
    pub struct GitRepoStatus { pub branch: String, pub is_clean: bool, pub has_remote: bool }
    pub fn check_status(repo_path: &Path) -> Option<GitRepoStatus> {
        if !repo_path.join(".git").exists() { return None; }
        let b = Command::new("git").args(["-C", repo_path.to_str()?, "rev-parse", "--abbrev-ref", "HEAD"]).output().ok()?;
        let s = Command::new("git").args(["-C", repo_path.to_str()?, "status", "--porcelain"]).output().ok()?;
        let r = Command::new("git").args(["-C", repo_path.to_str()?, "remote"]).output().ok()?;
        Some(GitRepoStatus {
            branch: String::from_utf8_lossy(&b.stdout).trim().to_string(),
            is_clean: s.stdout.is_empty(),
            has_remote: !r.stdout.is_empty(),
        })
    }
}
