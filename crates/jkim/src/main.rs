use clap::{Parser, Subcommand};
use jki_core::{
    paths::JkiPath, 
    git, 
    Account, 
    AccountSecret,
    acquire_master_key, 
    encrypt_with_master_key,
    decrypt_with_master_key,
    import::parse_otpauth_uri
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    layout::{Layout, Constraint, Direction},
    style::{Style, Modifier, Color},
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

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
    /// Initialize the JKI home directory and Git repository
    Init,
    /// Sync changes to Git (add, commit, pull --rebase, push)
    Sync,
    /// Edit accounts in a TUI
    Edit,
    /// Import accounts from a WinAuth decrypted text file
    ImportWinauth {
        /// Path to the decrypted WinAuth .txt file
        file: PathBuf,
        /// Overwrite existing accounts if name+issuer matches
        #[arg(short, long)]
        overwrite: bool,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct MetadataFile {
    accounts: Vec<Account>,
    version: u32,
}

fn handle_status() {
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
    let config_dir = JkiPath::home_dir();
    if let Some(repo_status) = git::check_status(&config_dir) {
        println!("  - Git Repository  : OK ({:?})", config_dir);
        println!("  - Current Branch  : {}", repo_status.branch);
        println!("  - Working Tree    : {}", if repo_status.is_clean { "Clean" } else { "Modified" });
        println!("  - Remote          : {}", if repo_status.has_remote { "Configured" } else { "None" });
    } else {
        println!("  - Git Repository  : Not initialized");
    }

    println!("\n[Paths]");
    println!("  - Metadata Path   : {:?}", JkiPath::metadata_path());
    println!("  - Secrets Path    : {:?}", JkiPath::secrets_path());
}

fn handle_sync() {
    let config_dir = JkiPath::home_dir();
    println!("Syncing JKI Home at {:?}...", config_dir);

    let status = match git::check_status(&config_dir) {
        Some(s) => s,
        None => {
            eprintln!("Error: Not a git repository. Run 'jkim init' first.");
            return;
        }
    };

    println!("  - Stage changes...");
    git::add_all(&config_dir).expect("Failed to add files");

    println!("  - Commit...");
    let now = chrono::Local::now();
    let msg = format!("jki backup: {}", now.format("%Y-%m-%d %H:%M:%S"));
    match git::commit(&config_dir, &msg) {
        Ok(true) => println!("  - Committed: {}", msg),
        Ok(false) => println!("  - Nothing to commit, working tree clean."),
        Err(e) => eprintln!("  - Commit failed: {}", e),
    }

    if status.has_remote {
        println!("  - Pull --rebase...");
        if let Err(e) = git::pull_rebase(&config_dir) {
            eprintln!("  - Pull failed: {}. Resolve conflicts manually.", e);
            return;
        }

        println!("  - Push...");
        if let Err(e) = git::push(&config_dir) {
            eprintln!("  - Push failed: {}.", e);
            return;
        }
        println!("Sync completed successfully!");
    } else {
        println!("No remote configured. Local backup complete.");
    }
}

fn handle_init() {
    let config_dir = JkiPath::home_dir();
    println!("Initializing JKI Home at {:?}...", config_dir);
    if !config_dir.exists() {
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            fs::DirBuilder::new().mode(0o700).recursive(true).create(&config_dir).expect("Failed to create config directory");
        }
        #[cfg(windows)]
        {
            fs::create_dir_all(&config_dir).expect("Failed to create config directory");
        }
        println!("  - Directory created.");
    }
    if !config_dir.join(".git").exists() {
        let status = Command::new("git").args(["init", "-b", "main"]).current_dir(&config_dir).status().expect("Failed to git init");
        if status.success() { println!("  - Git initialized."); }
    }
    let gitignore_path = config_dir.join(".gitignore");
    fs::write(gitignore_path, "# JKI\nmaster.key\nvault.json\n*.txt\n*.bin\n").ok();
    let gitattrs_path = config_dir.join(".gitattributes");
    fs::write(gitattrs_path, "vault.secrets.bin.age binary\nvault.metadata.json filter=age\n").ok();
    println!("\nInitialization complete!");
}

fn handle_import_winauth(file: &PathBuf, overwrite: bool) {
    if !file.exists() { eprintln!("Error: File not found."); return; }

    let meta_path = JkiPath::metadata_path();
    let sec_path = JkiPath::secrets_path();

    // 1. Acquire Master Key EARLIER (We need it to load existing secrets)
    println!("Please unlock your vault to perform import.");
    let master_key = acquire_master_key().unwrap_or_else(|e| {
        eprintln!("Authentication failed: {}", e);
        std::process::exit(1);
    });

    // 2. Load existing Metadata
    let mut metadata = if meta_path.exists() {
        let content = fs::read_to_string(&meta_path).unwrap();
        serde_json::from_str::<MetadataFile>(&content).unwrap()
    } else {
        MetadataFile { accounts: vec![], version: 1 }
    };

    // 3. Load and Decrypt existing Secrets (Merge-aware)
    let mut secrets_map: HashMap<String, AccountSecret> = if sec_path.exists() {
        let encrypted = fs::read(&sec_path).expect("Failed to read secrets");
        let decrypted = decrypt_with_master_key(&encrypted, &master_key).expect("Failed to decrypt existing secrets. Is the master key correct?");
        serde_json::from_slice(&decrypted).expect("Failed to parse secrets JSON")
    } else {
        HashMap::new()
    };

    // 4. Process Import
    let content = fs::read_to_string(file).expect("Failed to read file");
    let mut new_count = 0;
    let mut updated_count = 0;
    let mut skip_count = 0;

    for line in content.lines() {
        if let Some(mut acc) = parse_otpauth_uri(line) {
            let existing_pos = metadata.accounts.iter().position(|m| m.name == acc.name && m.issuer == acc.issuer);
            
            if let Some(pos) = existing_pos {
                let id = metadata.accounts[pos].id.clone();
                
                if !overwrite {
                    // 即使跳過 Metadata 更新，如果 secrets_map 裡已有，也必須保留
                    skip_count += 1;
                    continue;
                }
                
                // Update case
                let entry = AccountSecret { secret: acc.secret.clone(), digits: acc.digits, algorithm: acc.algorithm.clone() };
                acc.id = id.clone();
                acc.secret = "".to_string();
                metadata.accounts[pos] = acc;
                secrets_map.insert(id, entry);
                updated_count += 1;
            } else {
                // Insert case
                let id = acc.id.clone();
                let entry = AccountSecret { secret: acc.secret.clone(), digits: acc.digits, algorithm: acc.algorithm.clone() };
                acc.secret = "".to_string();
                metadata.accounts.push(acc);
                secrets_map.insert(id, entry);
                new_count += 1;
            }
        }
    }

    // 5. Encrypt and Save
    let secrets_json = serde_json::to_vec(&secrets_map).unwrap();
    let encrypted_data = encrypt_with_master_key(&secrets_json, &master_key).expect("Encryption failed");

    fs::write(&meta_path, serde_json::to_string_pretty(&metadata).unwrap()).unwrap();
    fs::write(&sec_path, encrypted_data).unwrap();

    println!("\nImport completed successfully!");
    println!("  - New: {}, Updated: {}, Skipped: {}", new_count, updated_count, skip_count);
}

fn filter_accounts<'a>(accounts: &'a [Account], query: &str) -> Vec<&'a Account> {
    accounts.iter()
        .filter(|acc| {
            let target = format!("{} {}", acc.issuer.as_deref().unwrap_or_default(), acc.name).to_lowercase();
            target.contains(&query.to_lowercase())
        })
        .collect()
}

fn handle_edit() {
    let meta_path = JkiPath::metadata_path();
    if !meta_path.exists() {
        eprintln!("Error: Metadata not found. Run 'jkim init' or import accounts first.");
        return;
    }

    let content = fs::read_to_string(&meta_path).expect("Failed to read metadata");
    let metadata: MetadataFile = serde_json::from_str(&content).expect("Failed to parse metadata");

    // TUI Setup
    enable_raw_mode().unwrap();
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut search_query = String::new();
    let mut list_state = ListState::default();
    list_state.select(Some(0));

    loop {
        let filtered_accounts = filter_accounts(&metadata.accounts, &search_query);

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)])
                .split(f.size());

            let search_box = Paragraph::new(search_query.as_str())
                .block(Block::default().borders(Borders::ALL).title(" Search (Type to filter, ESC to exit) "));
            f.render_widget(search_box, chunks[0]);

            let items: Vec<ListItem> = filtered_accounts.iter()
                .map(|acc| {
                    let label = format!("{} - {}", acc.issuer.as_deref().unwrap_or("No Issuer"), acc.name);
                    ListItem::new(label)
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(" Accounts "))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow))
                .highlight_symbol("> ");
            f.render_stateful_widget(list, chunks[1], &mut list_state);
        }).unwrap();

        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Char(c) => {
                        search_query.push(c);
                        list_state.select(Some(0));
                    }
                    KeyCode::Backspace => {
                        search_query.pop();
                        list_state.select(Some(0));
                    }
                    KeyCode::Up => {
                        let i = match list_state.selected() {
                            Some(i) => if i == 0 { filtered_accounts.len().saturating_sub(1) } else { i - 1 },
                            None => 0,
                        };
                        list_state.select(Some(i));
                    }
                    KeyCode::Down => {
                        let i = match list_state.selected() {
                            Some(i) => if i >= filtered_accounts.len().saturating_sub(1) { 0 } else { i + 1 },
                            None => 0,
                        };
                        list_state.select(Some(i));
                    }
                    _ => {}
                }
            }
        }
    }

    // TUI Cleanup
    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture).unwrap();
    terminal.show_cursor().unwrap();
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Status => handle_status(),
        Commands::Init => handle_init(),
        Commands::Sync => handle_sync(),
        Commands::Edit => handle_edit(),
        Commands::ImportWinauth { file, overwrite } => handle_import_winauth(file, *overwrite),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use tempfile::tempdir;
    use std::env;

    #[test]
    #[serial]
    fn test_handle_init() {
        let temp = tempdir().unwrap();
        let home = temp.path().join("jki_home");
        env::set_var("JKI_HOME", &home);

        handle_init();

        assert!(home.exists());
        assert!(home.join(".git").exists());
        assert!(home.join(".gitignore").exists());
        assert!(home.join(".gitattributes").exists());
    }

    #[test]
    #[serial]
    fn test_handle_status() {
        let temp = tempdir().unwrap();
        let home = temp.path().join("jki_home_status");
        env::set_var("JKI_HOME", &home);
        
        // Before init
        handle_status();
        
        handle_init();
        
        // After init
        handle_status();
    }

    #[test]
    #[serial]
    #[cfg(unix)]
    fn test_handle_import_winauth() {
        use std::os::unix::fs::PermissionsExt;
        let temp = tempdir().unwrap();
        let home = temp.path().join("jki_home_import");
        fs::create_dir_all(&home).unwrap();
        env::set_var("JKI_HOME", &home);

        // 1. Create master.key
        let key_path = home.join("master.key");
        fs::write(&key_path, "testpass").unwrap();
        fs::set_permissions(&key_path, fs::Permissions::from_mode(0o600)).unwrap();

        // 2. Create WinAuth export file
        let import_file = temp.path().join("winauth.txt");
        fs::write(&import_file, "otpauth://totp/Google:test@gmail.com?secret=JBSWY3DPEHPK3PXP&issuer=Google\n").unwrap();

        // 3. Run import
        handle_import_winauth(&import_file, false);

        // 4. Verify files
        let meta_path = home.join("vault.metadata.json");
        let sec_path = home.join("vault.secrets.bin.age");
        assert!(meta_path.exists());
        assert!(sec_path.exists());

        let meta_content = fs::read_to_string(meta_path).unwrap();
        let metadata: MetadataFile = serde_json::from_str(&meta_content).unwrap();
        assert_eq!(metadata.accounts.len(), 1);
        assert_eq!(metadata.accounts[0].name, "test@gmail.com");
        assert_eq!(metadata.accounts[0].issuer, Some("Google".to_string()));
    }

    #[test]
    #[serial]
    fn test_handle_sync() {
        let temp = tempdir().unwrap();
        let home = temp.path().join("jki_home_sync");
        env::set_var("JKI_HOME", &home);

        // 1. Init
        handle_init();
        
        // 2. Add some file
        fs::write(home.join("test.txt"), "content").unwrap();
        
        // 3. Sync (should commit)
        handle_sync();
        
        // 4. Verify commit
        let output = Command::new("git")
            .args(["-C", home.to_str().unwrap(), "log", "-n", "1"])
            .output()
            .unwrap();
        let log = String::from_utf8_lossy(&output.stdout);
        assert!(log.contains("jki backup:"));
    }

    #[test]
    fn test_filter_accounts() {
        use jki_core::AccountType;
        let accounts = vec![
            Account { id: "1".to_string(), name: "John".to_string(), issuer: Some("Google".to_string()), account_type: AccountType::Standard, secret: "".to_string(), digits: 6, algorithm: "".to_string() },
            Account { id: "2".to_string(), name: "Jane".to_string(), issuer: Some("Facebook".to_string()), account_type: AccountType::Standard, secret: "".to_string(), digits: 6, algorithm: "".to_string() },
        ];

        let filtered = filter_accounts(&accounts, "goog");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "1");

        let filtered = filter_accounts(&accounts, "John");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "1");

        let filtered = filter_accounts(&accounts, "xyz");
        assert_eq!(filtered.len(), 0);
    }
}
