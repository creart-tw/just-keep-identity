use clap::Parser;
use jki_core::{Account, search_accounts};
use std::fs;
use std::path::Path;
use std::process;

/// Just Keep Identity (jki) - 極速執行器
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// 搜尋關鍵字 (不分大小寫，交集模糊匹配)
    patterns: Vec<String>,

    /// 強制顯示列表，不執行複製或純文字輸出
    #[arg(short, long)]
    list: bool,

    /// 在列表模式下同時計算並顯示 OTP (向 Agent 請求)
    #[arg(short, long)]
    otp: bool,

    /// 安靜模式，不輸出任何 stderr 提示訊息
    #[arg(short, long)]
    quiet: bool,

    /// 輸出到 stdout 而非剪貼簿 (Unix 風格 '-')
    #[arg(short = 's', long = "stdout")]
    stdout: bool,
}

// TODO: Phase 3 將此處改為真正的 IPC 通訊 (Unix Socket / Named Pipe)
fn request_otp_placeholder(account: &Account) -> String {
    use totp_rs::{Algorithm, TOTP, Secret};
    
    // totp-rs 的 Secret::Encoded 會自動處理 base32 解碼
    let secret = Secret::Encoded(account.secret.clone()).to_bytes().expect("Failed to decode base32 secret");

    let totp = TOTP::new(
        Algorithm::SHA1,
        account.digits as usize,
        1,
        30,
        secret,
    ).expect("Failed to initialize TOTP");
    
    totp.generate_current().unwrap()
}

fn main() {
    // 1. 解析參數
    let mut args = Args::parse();
    
    // 處理特殊的 '-' 作為 stdout 標籤的模擬
    if args.patterns.contains(&"-".to_string()) {
        args.stdout = true;
        args.patterns.retain(|x| x != "-");
    }

    // 2. 讀取資料
    let vault_path = "data/private/vault.json";
    if !Path::new(vault_path).exists() {
        eprintln!("Error: vault.json not found. Run import script first.");
        process::exit(100);
    }
    
    let content = fs::read_to_string(vault_path).expect("Failed to read vault");
    let vault: serde_json::Value = serde_json::from_str(&content).expect("Failed to parse JSON");
    let accounts: Vec<Account> = serde_json::from_value(vault["accounts"].clone()).unwrap();

    // 3. 處理無參數情況
    if args.patterns.is_empty() {
        if !args.quiet { eprintln!("All Accounts:"); }
        for (i, acc) in accounts.iter().enumerate() {
            let otp_str = if args.otp {
                format!("{} - ", request_otp_placeholder(acc))
            } else {
                "".to_string()
            };
            println!("{:2}) {}{}{}", i + 1, otp_str, acc.issuer.as_deref().map(|s| format!("[{}] ", s)).unwrap_or_default(), acc.name);
        }
        return;
    }

    // 4. 交集模糊搜尋 (使用 jki_core 提供的邏輯)
    let mut search_terms = args.patterns.clone();
    let mut index_selection: Option<usize> = None;
    
    if search_terms.len() > 1 && search_terms.last().unwrap().chars().all(|c| c.is_ascii_digit()) {
        index_selection = search_terms.pop().and_then(|s| s.parse().ok());
    }

    let results = search_accounts(&accounts, &search_terms);

    if results.is_empty() {
        if !args.quiet { eprintln!("No matches found for patterns: {:?}", search_terms); }
        process::exit(1);
    }

    // 5. 選取邏輯
    let target = if results.len() == 1 && !args.list {
        Some(&results[0])
    } else if let Some(idx) = index_selection {
        if idx >= 1 && idx <= results.len() {
            Some(&results[idx - 1])
        } else {
            if !args.quiet { eprintln!("Error: Index {} out of range (1-{}).", idx, results.len()); }
            process::exit(2);
        }
    } else {
        // 歧義清單
        if !args.quiet {
            let title = if args.list { "Matches" } else { "Ambiguous results" };
            eprintln!("{}:", title);
        }
        for (i, acc) in results.iter().enumerate() {
            let otp_str = if args.otp {
                format!("{} - ", request_otp_placeholder(acc))
            } else {
                "".to_string()
            };
            println!("{:2}) {}{}{}", i + 1, otp_str, acc.issuer.as_deref().map(|s| format!("[{}] ", s)).unwrap_or_default(), acc.name);
        }
        process::exit(2);
    };

    // 6. 執行結果
    if let Some(acc) = target {
        let otp = request_otp_placeholder(acc);
        let account_label = format!("{}{}", acc.issuer.as_deref().map(|s| format!("[{}] ", s)).unwrap_or_default(), acc.name);
        
        if !args.quiet {
            eprintln!("Selected: {}", account_label);
        }

        if args.stdout {
            println!("{}", otp);
        } else {
            use copypasta::{ClipboardContext, ClipboardProvider};
            let mut ctx = ClipboardContext::new().expect("Failed to initialize clipboard");
            ctx.set_contents(otp.clone()).expect("Failed to copy to clipboard");

            if !args.quiet {
                eprintln!("Copied OTP to clipboard.");
                use notify_rust::Notification;
                let _ = Notification::new()
                    .summary("jki: OTP Copied")
                    .body(&format!("Account: {}", account_label))
                    .timeout(5000)
                    .show();
            }
        }
    }
}
