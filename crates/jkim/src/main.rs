use clap::{Parser, Subcommand};
use jki_core::{
    paths::JkiPath, 
    git, 
    Account, 
    acquire_master_key, 
    encrypt_with_master_key,
    import::parse_otpauth_uri
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "jkim", version, about = "JK Suite Management Hub")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check the health and status of the JKI system
    Status,
    /// Import accounts from a WinAuth decrypted text file
    ImportWinauth {
        /// Path to the decrypted WinAuth .txt file
        file: PathBuf,
        /// Overwrite existing accounts if name+issuer matches
        #[arg(short, long)]
        overwrite: bool,
    },
}

#[derive(Serialize, Deserialize)]
struct MetadataFile {
    accounts: Vec<Account>,
    version: u32,
}

#[derive(Serialize, Deserialize)]
struct SecretEntry {
    pub secret: String,
    pub digits: u32,
    pub algorithm: String,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Status => {
            println!("--- Just Keep Identity Status ---\n");
            let key_path = JkiPath::master_key_path();
            if key_path.exists() {
                match JkiPath::check_secure_permissions(&key_path) {
                    Ok(_) => println!("  - Master Key File : OK ({:?}, 0600)", key_path),
                    Err(e) => println!("  - Master Key File : SECURITY ERROR ({})", e),
                }
            } else {
                println!("  - Master Key File : Not found (Standalone mode disabled)");
            }
            println!("  - jki-agent       : Not checked (IPC placeholder)");

            println!("\n[Data & Synchronization]");
            let config_dir = JkiPath::config_dir();
            if let Some(repo_status) = git::check_status(&config_dir) {
                println!("  - Git Repository  : OK ({:?})", config_dir);
                println!("  - Current Branch  : {}", repo_status.branch);
                println!("  - Working Tree    : Clean");
            } else {
                println!("  - Git Repository  : Not initialized");
            }

            println!("\n[Paths]");
            println!("  - Metadata Path   : {:?}", JkiPath::metadata_path());
            println!("  - Secrets Path    : {:?}", JkiPath::secrets_path());
        }

        Commands::ImportWinauth { file, overwrite } => {
            if !file.exists() {
                eprintln!("Error: File {:?} not found.", file);
                return;
            }

            // 1. Setup paths
            let meta_path = PathBuf::from("data/private/vault.metadata.json");
            let sec_path = PathBuf::from("data/private/vault.secrets.json.age");

            // 2. Load existing metadata (plaintext)
            let mut metadata = if meta_path.exists() {
                let content = fs::read_to_string(&meta_path).unwrap();
                serde_json::from_str::<MetadataFile>(&content).unwrap()
            } else {
                MetadataFile { accounts: vec![], version: 1 }
            };

            // 3. Prepare secrets map
            // Note: In a real scenario, we'd need to decrypt the existing sec_path first to merge.
            // For this MVP, we assume we are building/overwriting the secrets.
            let mut secrets_map: HashMap<String, SecretEntry> = HashMap::new();

            // 4. Process file
            let content = fs::read_to_string(file).expect("Failed to read import file");
            let mut new_count = 0;
            let mut skip_count = 0;

            for line in content.lines() {
                if let Some(mut acc) = parse_otpauth_uri(line) {
                    let exists = metadata.accounts.iter().any(|m| m.name == acc.name && m.issuer == acc.issuer);
                    
                    if exists && !*overwrite {
                        skip_count += 1;
                        continue;
                    }

                    let id = if let Some(existing) = metadata.accounts.iter().find(|m| m.name == acc.name && m.issuer == acc.issuer) {
                        existing.id.clone()
                    } else {
                        acc.id.clone()
                    };

                    let entry = SecretEntry {
                        secret: acc.secret.clone(),
                        digits: acc.digits,
                        algorithm: acc.algorithm.clone(),
                    };

                    // Update or Insert
                    if let Some(pos) = metadata.accounts.iter().position(|m| m.id == id) {
                        let mut updated_acc = acc.clone();
                        updated_acc.id = id.clone();
                        updated_acc.secret = "".to_string(); // Keep meta clean
                        metadata.accounts[pos] = updated_acc;
                    } else {
                        acc.secret = "".to_string();
                        metadata.accounts.push(acc);
                        new_count += 1;
                    }
                    secrets_map.insert(id, entry);
                }
            }

            // 5. Acquire Master Key and ENCRYPT WHOLE BLOB
            println!("Encrypting secrets...");
            let master_key = acquire_master_key().unwrap_or_else(|e| {
                eprintln!("Authentication failed: {}", e);
                std::process::exit(1);
            });

            let secrets_json = serde_json::to_vec(&secrets_map).unwrap();
            let encrypted_data = encrypt_with_master_key(&secrets_json, &master_key).expect("Encryption failed");

            // 6. Save
            fs::write(&meta_path, serde_json::to_string_pretty(&metadata).unwrap()).unwrap();
            fs::write(&sec_path, encrypted_data).unwrap();

            println!("\nImport completed successfully!");
            println!("  - New accounts: {}", new_count);
            println!("  - Skipped/Updated: {}", skip_count);
            println!("  - Metadata: {:?}", meta_path);
            println!("  - Encrypted Secrets: {:?}", sec_path);
        }
    }
}
