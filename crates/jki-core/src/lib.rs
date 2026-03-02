use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

pub mod import;

#[derive(Archive, Deserialize, Serialize, SerdeDeserialize, SerdeSerialize, Debug, Clone)]
#[archive(check_bytes)]
pub struct Account {
    pub id: String, // Unique ID for IPC and reference
    pub name: String,
    pub issuer: Option<String>,
    pub secret: String,
    pub digits: u32,
    pub algorithm: String,
    pub account_type: AccountType,
}

#[derive(Archive, Deserialize, Serialize, SerdeDeserialize, SerdeSerialize, Debug, Clone, PartialEq)]
#[archive(check_bytes)]
pub enum AccountType {
    Standard,
    Steam,
    Blizzard,
}

#[derive(Archive, Deserialize, Serialize, SerdeDeserialize, SerdeSerialize, Debug, Clone)]
#[archive(check_bytes)]
pub struct Vault {
    pub accounts: Vec<Account>,
    pub version: u32,
}

impl Vault {
    pub fn new() -> Self {
        Self {
            accounts: Vec::new(),
            version: 1,
        }
    }
}

pub fn fuzzy_match(pattern: &str, target: &str) -> bool {
    let pattern = pattern.to_lowercase();
    let target = target.to_lowercase();
    let mut target_chars = target.chars();
    for p in pattern.chars() {
        if !target_chars.any(|t| t == p) {
            return false;
        }
    }
    true
}

pub fn search_accounts(accounts: &[Account], patterns: &[String]) -> Vec<Account> {
    accounts
        .iter()
        .filter(|acc| {
            let target_str = format!("{} {}", acc.issuer.as_deref().unwrap_or_default(), acc.name);
            patterns.iter().all(|p| fuzzy_match(p, &target_str))
        })
        .cloned()
        .collect()
}
